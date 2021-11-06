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
use std::{future::Future, net::SocketAddr, sync::Arc};

use uuid::Uuid;

use eiffelvis_core::app::EiffelVisApp;

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
        let mut cursor = None;
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(3));
        
        while let Ok(()) = tokio::select! {
            usr = socket.recv() => {
                match usr {
                    Some(msg) => { println!("{:?}", msg); Ok(()) },
                    None => Err(())
                }
            },
            _ = interval.tick() => {
                if let Some((head, events)) =  {
                    let lk = core.read().await;

                    // If there is a head (implying there _are_ events)
                    // If we have an existing cursor (implying we have send things before)
                    // then get the range from our cursor till the end
                    // else dump the whole history (as this is the first time we have send something)
                    lk.head().zip(cursor.map_or_else(|| Some(lk.dump_lean_events()), |c| lk.events_starting_from(c).filter(|evs| !evs.is_empty())))
                } {
                    cursor = Some(head);
                    socket.send(Message::Text(serde_json::to_string(&events).unwrap())).await.map_err(|_| ())
                } else {
                    Ok(())
                }
            }
        } {}

        println!("Client disconnected");
    })
}
