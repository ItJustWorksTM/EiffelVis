use crate::*;

use axum::{
    body::Full,
    extract::{
        ws::{Message, WebSocketUpgrade},
        Extension, Path, TypedHeader,
    },
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use eiffelvis_core::domain::{
    types::LeanEvent,
    user_queries::{Query, TrackedQuery},
};
use hyper::Response;

/// Dumps the entire event store into a json array
pub async fn event_dump<T: EiffelVisApp>(Extension(app): Extension<App<T>>) -> impl IntoResponse {
    let lk = app.read().await;

    let dump = lk.dump::<BaseEvent>();

    Json(&dump).into_response()
}

/// Returns full event that belongs to given uuid
pub async fn get_event<T: EiffelVisApp>(
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

pub async fn events_with_root<T: EiffelVisApp>(
    Path(find_id): Path<Uuid>,
    Extension(app): Extension<App<T>>,
) -> impl IntoResponse {
    let lk = app.read().await;
    Json(lk.get_subgraph_with_roots::<BaseEvent>(&[find_id])).into_response()
}

pub async fn establish_websocket<T: EiffelVisHttpApp>(
    Extension(app): Extension<App<T>>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse
where
    for<'a> &'a T: Graph<I = T::NodeIndex, K = T::NodeKey> + EiffelGraph + Send,
    for<'a> T::NodeIndex: graph::Index<&'a T>,
{
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected to websocket", user_agent.as_str());
    }

    ws.on_upgrade(move |mut socket| async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(500));

        let mut req_handler: Option<TrackedQuery<_>> = None;

        while let Ok(()) = tokio::select! {
            usr = socket.recv() => {
                match usr {
                    Some(Ok(Message::Text(ref msg))) => match serde_json::from_str::<Query>(msg) {
                        Ok(rq) => {

                            req_handler = Some(TrackedQuery::new(rq));

                            match socket.send(Message::Text(msg.clone())).await {
                                Ok(_) => Ok(()),
                                Err(_) => {
                                    println!("Failed to send message");
                                    Err(())
                                },
                            }
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
                if let Some(handler) = req_handler.as_mut() {
                    let events: Vec<LeanEvent> = handler.handle(&*app.read().await);
                    if !events.is_empty() {
                        match socket.send(Message::Text(serde_json::to_string(&events).unwrap())).await {
                            Ok(_) => Ok(()),
                            Err(_) => Err(())
                        }
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
