use std::{sync::Arc, time::Duration};

use eiffelvis_core::app::EiffelVisApp;
use tracing::info;

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::init();

    let graph = Arc::new(tokio::sync::RwLock::new(EiffelVisApp::new(10, 10)));

    let http_server = tokio::spawn(eiffelvis_http::app(
        graph.clone(),
        "127.0.0.1:3001",
        shutdown_signal(),
    ));

    let mut event_parser = eiffelvis_stream::ampq::AmpqStream::new(
        "amqp://localhost:5672/%2f".into(),
        "hello".into(),
        5,
    )
    .await
    .expect("Failed to connect to ampq server");

    let event_parser = tokio::spawn(async move {
        loop {
            if let Some(bytes) = event_parser.next().await {
                if let Ok(des) = serde_json::from_slice(&bytes) {
                    graph.write().await.push(des);
                } else {
                    info!("Received new message but failed to deserialize");
                }
            } else {
                info!("Event stream failed, sleeping for 5 seconds to retry");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    });

    let event_parser = async move {
        tokio::select! {
            _ = event_parser => Err(()),
            () = shutdown_signal() => Ok(())
        }
    };

    let (res, _) = tokio::join!(http_server, event_parser);
    res.unwrap().unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
