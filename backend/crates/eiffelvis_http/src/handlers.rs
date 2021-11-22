use crate::*;
use serde::Serialize;

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
pub async fn event_dump<T: EiffelVisHttpApp>(
    Extension(app): Extension<App<T>>,
) -> impl IntoResponse {
    let lk = app.read().await;

    let dump = lk.dump::<BaseEvent>();

    Json(&dump).into_response()
}

/// Returns full event that belongs to given uuid
pub async fn get_event<T: EiffelVisHttpApp>(
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

/// Returns the sub-graph for given id
pub async fn events_with_root<T: EiffelVisHttpApp>(
    Path(find_id): Path<Uuid>,
    Extension(app): Extension<App<T>>,
) -> impl IntoResponse {
    let lk = app.read().await;
    Json(lk.get_subgraph_with_roots::<BaseEvent>(&[find_id])).into_response()
}

#[derive(Debug, Clone, Serialize)]
struct QueryRes {
    repr: String,
    error: Option<String>,
}

/// Establishes a websocket with the client,
/// [Query] in json format is expected to be send by the client
/// Backend will then use a [TrackedQuery] to gradually send results.
pub async fn establish_websocket<T: EiffelVisHttpApp>(
    Extension(app): Extension<App<T>>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected to websocket", user_agent.as_str());
    }

    ws.on_upgrade(move |mut socket| async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(500));

        let mut req_handler: Option<TrackedQuery<_>> = None;

        while let Ok(()) = tokio::select! {
            usr = socket.recv() => {
                match usr {
                    Some(Ok(Message::Text(ref msg))) => {
                        let res = match serde_json::from_str::<Query>(msg) {
                            Ok(rq) => {
                                req_handler = Some(TrackedQuery::new(rq));
                                None
                            },
                            Err(err) => Some(format!("{}", err))
                        };
                        let res = QueryRes { repr: msg.clone(), error: res };
                        println!("Request {:?}", res);
                        socket.send(Message::Text(serde_json::to_string(&res).unwrap())).await.map_err(|_| ())
                    },
                    _ => Err(())
                }
            },
            _ = interval.tick() => {
                if let Some(handler) = req_handler.as_mut() {
                    let events: Vec<LeanEvent> = handler.handle(&*app.read().await);
                    if !events.is_empty() {
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
