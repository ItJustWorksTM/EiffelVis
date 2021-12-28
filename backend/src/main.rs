//!
//! Combines all individual eiffelvis_* libraries into the final product.
//!
//! Responsibilities are mainly owning the async executor and setting initial parameters through cli
//!

use std::{sync::Arc, time::Duration};

use eiffelvis_core::{domain::app::EiffelVisApp, graph_storage::ChunkedGraph};
use structopt::StructOpt;
use tracing::info;

/// Command line options
#[derive(StructOpt, Debug)]
#[structopt(name = "EiffelVis")]
struct Cli {
    /// HTTP host address
    #[structopt(short, long, default_value = "127.0.0.1")]
    address: String,

    /// HTTP host port
    #[structopt(short, long, default_value = "3001")]
    port: u16,

    /// AMQP URI
    #[structopt(short = "r", long, default_value = "amqp://localhost:5672/%2f")]
    rmq_uri: String,

    /// AMQP queue
    #[structopt(short = "q", long, default_value = "hello")]
    rmq_queue: String,

    /// AMQP reconnect timeout
    #[structopt(short = "t", long, default_value = "3001")]
    timeout: u64,

    /// Maximum amount of chunks stored in memory
    #[structopt(long, default_value = "8")]
    max_chunks: usize,

    /// Maximum amount of events a single chunk will hold
    #[structopt(long, default_value = "128")]
    chunk_size: u32,

    #[structopt(long)]
    tls_cert: Option<String>,

    #[structopt(long)]
    tls_key: Option<String>,
}

/// Starts all the services that make up EiffelVis.
#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    let cli = Cli::from_args();

    let graph = Arc::new(tokio::sync::RwLock::new(ChunkedGraph::new(
        cli.max_chunks,
        cli.chunk_size,
    )));

    let http_server_handle = eiffelvis_http::Handle::new();
    let http_server = tokio::spawn(eiffelvis_http::app(
        graph.clone(),
        cli.address.parse().unwrap(),
        cli.port,
        http_server_handle.clone(),
        cli.tls_cert.zip(cli.tls_key),
    ));

    let mut event_parser = eiffelvis_stream::ampq::AmpqStream::new(
        cli.rmq_uri.into(),
        cli.rmq_queue.into(),
        "eiffelvis".into(),
    )
    .await
    .expect("Failed to connect to ampq server");

    let timeout = cli.timeout;
    let event_parser = tokio::spawn(async move {
        loop {
            if let Some(bytes) = event_parser.next().await {
                if let Ok(des) = serde_json::from_slice(&bytes) {
                    EiffelVisApp::push(&mut *graph.write().await, des);
                } else {
                    info!("Received new message but failed to deserialize");
                }
            } else {
                info!("Event stream failed, sleeping for 5 seconds to retry");
                tokio::time::sleep(Duration::from_secs(timeout)).await;
            }
        }
    });

    tokio::spawn(async move {
        shutdown_signal().await;
        http_server_handle.graceful_shutdown(None);
    });

    tokio::select! {
        res = event_parser => res.unwrap(),
        res = http_server => res.unwrap().unwrap(),
    };
}

#[doc(hidden)]
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
