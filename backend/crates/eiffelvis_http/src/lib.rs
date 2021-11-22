//!
//! An HTTP frontend for eiffelvis_core
//!

mod handlers;

use handlers::*;

use axum::{routing::get, AddExtensionLayer, Router};

use std::{future::Future, net::SocketAddr, sync::Arc};

use uuid::Uuid;

use eiffelvis_core::domain::{app::EiffelVisApp, types::BaseEvent};

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
