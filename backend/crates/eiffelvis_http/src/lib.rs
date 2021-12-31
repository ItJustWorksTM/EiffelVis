//!
//! An HTTP frontend for eiffelvis_core
//!

mod handlers;

use handlers::*;

use axum::{routing::get, AddExtensionLayer, Router};
pub use axum_server::Handle;
use axum_server::{self, tls_rustls::RustlsConfig};
use tracing::info;

use std::{
    io,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

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
    address: IpAddr,
    port: u16,
    handle: Handle,
    tls: Option<(String, String)>,
) -> io::Result<()> {
    let tls = match tls {
        Some((cert, key)) => Some(RustlsConfig::from_pem_file(cert, key).await?),
        _ => None,
    };

    let service = make_service(app);

    serve_service(SocketAddr::new(address, port), service, handle, tls).await
}

/// Serves an axum router on the given address, with optional TLS
async fn serve_service(
    addr: SocketAddr,
    service: Router,
    handle: Handle,
    tls: Option<RustlsConfig>,
) -> io::Result<()> {
    match tls {
        Some(config) => {
            info!("Binding to {:?} using tls", addr);
            axum_server::bind_rustls(addr, config)
                .handle(handle)
                .serve(service.into_make_service())
                .await
        }
        None => {
            info!("Binding to {:?} without tls", addr);
            axum_server::bind(addr)
                .handle(handle)
                .serve(service.into_make_service())
                .await
        }
    }
}
