//!
//! An HTTP frontend for eiffelvis_core
//!

use axum::{
    body::Full, extract::Extension, extract::Path, handler::get, http::StatusCode,
    response::IntoResponse, AddExtensionLayer, Json, Router,
};
use hyper::Response;
use std::{future::Future, sync::Arc};
use uuid::Uuid;

use eiffelvis_core::app::EiffelVisApp;

type CoreApp = Arc<tokio::sync::RwLock<EiffelVisApp>>;

/// Takes an eiffelvis app and binds the http server on the given address.
/// This is likely the only function you'll ever need to call.
/// `shutdown` is used to trigger graceful shutdown, tokio::signal is useful for this.
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

/// Dumps the entire event store into a json array
async fn event_dump(Extension(core): Extension<CoreApp>) -> impl IntoResponse {
    let lk = core.read().await;

    let dump = lk.dump_lean_events();

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
