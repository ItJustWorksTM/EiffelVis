#![allow(dead_code)]

use axum::{
    body::Full, extract::Extension, extract::Path, handler::get, http::HeaderValue,
    response::IntoResponse, AddExtensionLayer, Router,
};
use hyper::{header, Response};
use serde::Serialize;
use std::{future::Future, sync::Arc};
use uuid::Uuid;

use eiffelvis_core::{app::EiffelVisApp, types::LeanEvent};

type CoreApp = Arc<tokio::sync::RwLock<EiffelVisApp>>;

pub async fn app(
    core: CoreApp,
    address: &str,
    shutdown: impl Future<Output = ()>,
) -> anyhow::Result<()> {
    let service = Router::new()
        .route("/", get(event_dump))
        .route("/get_event/:id", get(get_event))
        .layer(AddExtensionLayer::new(core));
    let address = address.parse()?;

    let server = axum::Server::try_bind(&address)?
        .serve(service.into_make_service())
        .with_graceful_shutdown(shutdown);

    server.await.unwrap();
    Ok(())
}

fn into_json_response<T: Serialize>(data: &T) -> Option<impl IntoResponse> {
    serde_json::to_vec(&data).ok().map(|bytes| {
        let mut res = Response::new(Full::from(bytes));
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        res
    })
}

async fn event_dump(Extension(core): Extension<CoreApp>) -> impl IntoResponse {
    let lk = core.read().await;

    let dump: Vec<_> = lk
        .graph()
        .iter()
        .map(|node| LeanEvent::from(&node.data))
        .collect();

    into_json_response(&dump).unwrap()
}

async fn get_event(
    Path(find_id): Path<Uuid>,
    Extension(core): Extension<CoreApp>,
) -> impl IntoResponse {
    let lk = core.read().await;
    let event = lk.graph().get(find_id).map(|node| &node.data);
    if let Some(event) = event {
        into_json_response(event).unwrap()
    } else {
        panic!("TODO: deal with this :)");
    }
}
