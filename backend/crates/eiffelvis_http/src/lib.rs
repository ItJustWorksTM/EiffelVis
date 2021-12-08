//!
//! An HTTP frontend for eiffelvis_core
//!

mod handlers;

use handlers::*;

use axum::{routing::get, AddExtensionLayer, Router};
use axum_server::{self, tls_rustls::RustlsConfig, Handle};
use tracing::info;

use std::{future::Future, net::SocketAddr, sync::Arc};

use tower_http::cors::{any, CorsLayer};

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
    shutdown: impl Future<Output = ()> + Send + 'static,
    tls: Option<(String, String)>,
) -> anyhow::Result<impl Future<Output = anyhow::Result<()>>> {
    let address = address.parse()?;
    let tls = match tls {
        Some((cert, key)) => Some(RustlsConfig::from_pem_file(cert, key).await.unwrap()),
        _ => None,
    };

    Ok(serv(app, SocketAddr::new(address, port), tls, shutdown))
}

async fn serv<T: EiffelVisHttpApp>(
    app: App<T>,
    addr: SocketAddr,
    tls: Option<RustlsConfig>,
    shutdown: impl Future<Output = ()> + Send + 'static,
) -> anyhow::Result<()> {
    let service = Router::new()
        .route("/", get(event_dump::<T>))
        .route("/get_event/:id", get(get_event::<T>))
        .route("/events_with_root/:id", get(events_with_root::<T>))
        .route("/ws", get(establish_websocket::<T>))
        .layer(CorsLayer::new().allow_origin(any()).allow_methods(any()))
        .layer(AddExtensionLayer::new(app));

    let handle = Handle::new();
    let stop_handle = handle.clone();
    tokio::spawn(async move {
        shutdown.await;
        stop_handle.graceful_shutdown(None);
    });

    match tls {
        Some(config) => {
            info!("Binding to {:?} using {:?} tls", addr, config);
            axum_server::bind_rustls(addr, config)
                .handle(handle)
                .serve(service.into_make_service())
                .await?;
        }
        None => {
            info!("Binding to {:?} without tls", addr);
            axum_server::bind(addr)
                .handle(handle)
                .serve(service.into_make_service())
                .await?;
        }
    }

    Ok(())
}
