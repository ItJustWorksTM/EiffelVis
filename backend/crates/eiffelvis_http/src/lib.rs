//!
//! An HTTP frontend for eiffelvis_core
//!

use axum::{
    body::Full,
    extract::{
        ws::{Message, WebSocketUpgrade},
        Extension, Path, TypedHeader,
    },
    handler::get,
    http::StatusCode,
    response::IntoResponse,
    AddExtensionLayer, Json, Router,
};

use hyper::Response;
use serde::{Deserialize, Serialize};
use std::{future::Future, net::SocketAddr, sync::Arc};

use uuid::Uuid;

use eiffelvis_core::{app::EiffelVisApp, types::LeanEvent};

mod requests;

use requests::{ClientRequest, ClientRequestHandler, EiffelClientRequest};

use crate::requests::AllFunctionality;

type CoreApp = Arc<tokio::sync::RwLock<EiffelVisApp>>;

/// Takes an eiffelvis app and binds the http server on the given address.
/// This is likely the only function you'll ever need to call.
/// `shutdown` is used to trigger graceful shutdown, tokio::signal is useful for this.
pub async fn app(
    core: CoreApp,
    address: String,
    port: u16,
    shutdown: impl Future<Output = ()>,
) -> anyhow::Result<()> {
    let service = Router::new()
        .route("/", get(event_dump))
        .route("/get_event/:id", get(get_event))
        .route("/ws", get(establish_websocket))
        .layer(AddExtensionLayer::new(core));
    let address = address.parse()?;

    let server = axum::Server::try_bind(&SocketAddr::new(address, port))?
        .serve(service.into_make_service())
        .with_graceful_shutdown(shutdown);

    server.await.unwrap();
    Ok(())
}

/// Dumps the entire event store into a json array
async fn event_dump(Extension(core): Extension<CoreApp>) -> impl IntoResponse {
    let lk = core.read().await;

    let dump = lk.dump_events();

    Json(&dump).into_response()
}

/// Returns full event that belongs to given uuid
async fn get_event(
    Path(find_id): Path<Uuid>,
    Extension(core): Extension<CoreApp>,
) -> impl IntoResponse {
    let lk = core.read().await;
    if let Some(event) = lk.get_event(find_id) {
        Json(event).into_response()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::default())
            .unwrap()
    }
}

async fn establish_websocket(
    Extension(core): Extension<CoreApp>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected to websocket", user_agent.as_str());
    }

    ws.on_upgrade(move |mut socket| async move {

        let mut interval = tokio::time::interval(std::time::Duration::from_millis(500));

        let mut req_handler: Option<Box<dyn ClientRequestHandler>> = None;

        while let Ok(()) = tokio::select! {
            usr = socket.recv() => {
                match usr {
                    Some(Ok(Message::Text(msg))) => match serde_json::from_str::<EiffelClientRequest>(&msg) {
                        Ok(rq) => {
                            println!("client request! {:?}", rq);
                            req_handler = Some(match rq {
                                EiffelClientRequest::All(all) => Box::new(all.into_handler()),
                                EiffelClientRequest::Latest(latest) => Box::new(latest.into_handler()),

                                _ => todo!()
                            });

                            // TODO: actually do something sensible
                            socket.send(Message::Text(msg)).await.unwrap();

                            Ok(())
                        },
                        err => {
                            println!("Warning, bad message: {:?}", err);
                            Ok(())
                        }
                    },
                    _ => {
                        Err(())}
                }
            },
            _ = interval.tick() => {
                // TODO: cleanup
                if let Some(th) = req_handler.as_mut() {
                    if let Some(events) = {
                        let lk = core.read().await;
                        th.handle(&*lk)
                    } {
                        socket.send(Message::Text(serde_json::to_string(&events).unwrap())).await.map_err(|_| ())
                    } else {
                        Ok(())
                    }
                } else {
                    Ok(())
                }

            }
        } {}

        println!("Client disconnected");
    })
}
