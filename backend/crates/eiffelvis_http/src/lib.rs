//!
//! An HTTP frontend for eiffelvis_core
//!

mod requests;
use requests::{ClientRequestHandler, EiffelClientRequest};

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
use std::{future::Future, net::SocketAddr, sync::Arc};

use uuid::Uuid;

use eiffelvis_core::{
    app::EiffelVisApp,
    types::{BaseEvent, LeanEvent},
};

use crate::requests::{AllHandler, WithRootHandler};

pub trait EiffelVisHttpApp: EiffelVisApp + Send + Sync + 'static {}
impl<T> EiffelVisHttpApp for T where T: EiffelVisApp + Send + Sync + 'static {}
type App<T> = Arc<tokio::sync::RwLock<T>>;

/// Takes an eiffelvis app and binds the http server on the given address.
/// This is likely the only function you'll ever need to call.
/// `shutdown` is used to trigger graceful shutdown, tokio::signal is useful for this.
pub async fn app<T: EiffelVisHttpApp>(
    app: App<T>,
    address: String,
    port: u16,
    shutdown: impl Future<Output = ()>,
) -> anyhow::Result<()> {
    let service = Router::new()
        .route("/", get(event_dump::<T>))
        .route("/get_event/:id", get(get_event::<T>))
        .route("/events_with_root/:id", get(events_with_root::<T>))
        .route("/ws", get(establish_websocket::<T>))
        .layer(AddExtensionLayer::new(app));
    let address = address.parse()?;

    let server = axum::Server::try_bind(&SocketAddr::new(address, port))?
        .serve(service.into_make_service())
        .with_graceful_shutdown(shutdown);

    server.await.unwrap();
    Ok(())
}

/// Dumps the entire event store into a json array
async fn event_dump<T: EiffelVisHttpApp>(Extension(app): Extension<App<T>>) -> impl IntoResponse {
    let lk = app.read().await;

    let dump = lk.dump::<BaseEvent>();

    Json(&dump).into_response()
}

/// Returns full event that belongs to given uuid
async fn get_event<T: EiffelVisHttpApp>(
    Path(find_id): Path<Uuid>,
    Extension(app): Extension<App<T>>,
) -> impl IntoResponse {
    let lk = app.read().await;
    if let Some(event) = lk.get_event(find_id) {
        Json(event).into_response()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::default())
            .unwrap()
    }
}

async fn events_with_root<T: EiffelVisHttpApp>(
    Path(find_id): Path<Uuid>,
    Extension(app): Extension<App<T>>,
) -> impl IntoResponse {
    let lk = app.read().await;
    if let Some(event) = lk.get_subgraph_with_roots::<BaseEvent>(&[find_id]) {
        Json(event).into_response()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::default())
            .unwrap()
    }
}

async fn establish_websocket<T: EiffelVisHttpApp>(
    Extension(app): Extension<App<T>>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected to websocket", user_agent.as_str());
    }

    ws.on_upgrade(move |mut socket| async move {

        let mut interval = tokio::time::interval(std::time::Duration::from_millis(500));

        type BoxedHandler<T> = Box<dyn for<'a> ClientRequestHandler<'a, T, LeanEvent> + Send>;
        let mut req_handler: Option<BoxedHandler<T>> = None;

        while let Ok(()) = tokio::select! {
            usr = socket.recv() => {
                match usr {
                    Some(Ok(Message::Text(ref msg))) => match serde_json::from_str::<EiffelClientRequest>(&msg) {
                        Ok(rq) => {
                            println!("client request! {:?}", rq);
                             req_handler = Some(match rq {
                                EiffelClientRequest::All(all) => Box::new(AllHandler::from(all)),
                                EiffelClientRequest::WithRoot(root) => Box::new(WithRootHandler::from(root)),
                                _ => todo!()
                            });

                            // TODO: actually do something sensible
                            socket.send(Message::Text(msg.clone())).await.unwrap();

                            Ok(())
                        },
                        err => {
                            println!("Warning, bad message: {:?} \n {:?}", err, usr);
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
                        th.handle(&*app.read().await)
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
