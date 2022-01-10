use std::{collections::VecDeque, ops::Add};

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

pub(crate) fn make_service<T: EiffelVisHttpApp>(app: App<T>) -> Router {
    Router::new()
        .route("/", get(event_dump::<T>))
        .route("/get_event/:id", get(get_event::<T>))
        .route("/events_with_root/:id", get(events_with_root::<T>))
        .route("/ws", get(establish_websocket::<T>))
        .layer(CorsLayer::new().allow_origin(any()).allow_methods(any()))
        .layer(AddExtensionLayer::new(app))
}

/// Dumps the entire event store into a json array
pub async fn event_dump<T: EiffelVisHttpApp>(
    Extension(app): Extension<App<T>>,
) -> impl IntoResponse {
    let lk = app.graph.read().await;

    let dump = lk.dump::<&BaseEvent>();

    Json(&dump).into_response()
}

/// Returns full event that belongs to given uuid
pub async fn get_event<T: EiffelVisHttpApp>(
    Path(find_id): Path<Uuid>,
    Extension(app): Extension<App<T>>,
) -> impl IntoResponse {
    let lk = app.graph.read().await;
    if let Some(event) = lk.get_event(find_id) {
        Json(event).into_response()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::default())
            .unwrap()
            .into_response()
    }
}

/// Returns the sub-graph for given id
pub async fn events_with_root<T: EiffelVisHttpApp>(
    Path(find_id): Path<Uuid>,
    Extension(app): Extension<App<T>>,
) -> impl IntoResponse {
    let lk = app.graph.read().await;
    Json(lk.get_subgraph_with_roots::<&BaseEvent>(&[find_id])).into_response()
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

        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        let hist_max = 8;
        let mut delta_hist = VecDeque::with_capacity(hist_max);
        let mut last_heurstic = 0;
        let mut heuristic_changed = false;

        delta_hist.push_back(0);

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
                        heuristic_changed = true;
                        socket.send(Message::Text(serde_json::to_string(&res).unwrap())).await.map_err(|_| ())
                    },
                    _ => Err(())
                }
            },
            _ = interval.tick() => {
                let heuristic = app.heuristic.load(std::sync::atomic::Ordering::Relaxed);
                let average = delta_hist.iter().fold(0, Add::add) / delta_hist.len() as u64;
                let delta = heuristic - last_heurstic;

                if last_heurstic != heuristic {
                    // the delta might be too high for us to lock the graph so we need to remember
                    heuristic_changed = true;
                }

                let mut res = Ok(());
                if heuristic_changed && delta <= average  {
                    if let Some(handler) = req_handler.as_mut() {
                        info!("locking graph!");
                        let events: Vec<LeanEvent> = handler.handle(&*app.graph.read().await);
                        if !events.is_empty() {
                            res = socket.send(Message::Text(serde_json::to_string(&events).unwrap())).await.map_err(|_| ())
                        }
                    }

                    heuristic_changed = false;
                }

                if delta_hist.len() >= hist_max {
                    delta_hist.pop_front();
                }
                delta_hist.push_back(delta);
                last_heurstic = heuristic;

                res
            }
        } {}

        println!("Client disconnected");
    })
}
