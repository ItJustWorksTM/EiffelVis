use axum::{
    body::Full, extract::Extension, handler::get, http::HeaderValue, response::IntoResponse,
    routing::BoxRoute, AddExtensionLayer, Router,
};
use futures::StreamExt;
use hyper::{header, Response};
use lapin::{tcp::TLSConfig, uri::AMQPUri, Connection, ConnectionProperties};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{collections::HashMap, str::FromStr};
use tokio::sync::RwLock;
use tokio_amqp::*;
use tower_http::trace::DefaultMakeSpan;
use tracing::info;
use uuid::Uuid;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Meta {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub event_type: String,
    pub version: String,
    pub time: i64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "type")]
    pub link_type: String,
    pub target: Uuid,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Event {
    pub meta: Meta,
    pub data: serde_json::Value,
    pub links: Vec<Link>,
}

type EiffelGraph = HashMap<Uuid, Event>;
type EiffelGraphShared = Arc<RwLock<EiffelGraph>>;

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::init();

    let locked_graph = Arc::new(RwLock::new(HashMap::new()));

    let http_addr = "127.0.0.1:3000".parse().unwrap();
    let axum_app = axum_app(locked_graph.clone());
    let axum_job = axum::Server::try_bind(&http_addr)
        .unwrap()
        .serve(axum_app.into_make_service());

    let ampq_addr = "amqp://localhost:5672/%2f";
    let consumer = make_ampq_channel(ampq_addr).await.unwrap();
    let ampq_job = ampq_app(consumer, locked_graph);

    let axum_handle = tokio::spawn(axum_job);
    ampq_job.await;
    axum_handle.abort();
}

async fn make_ampq_channel(addr: &str) -> Result<lapin::Consumer, lapin::Error> {
    let connection = Connection::connect_uri_with_identity(
        AMQPUri::from_str(addr).unwrap(),
        ConnectionProperties::default().with_tokio(),
        TLSConfig::default(),
    )
    .await?;
    let consumer = connection
        .create_channel()
        .await?
        .basic_consume(
            "hello",
            "my_consumer",
            lapin::options::BasicConsumeOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;

    Ok(consumer)
}

async fn ampq_app(mut consumer: lapin::Consumer, graph: EiffelGraphShared) {
    while let Some(Ok((_, delivery))) = consumer.next().await {
        if let Ok(test_event) = serde_json::from_slice::<Event>(&delivery.data) {
            let mut a = graph.write().await;
            let hm = a.insert(test_event.meta.id, test_event);
            info!("Graph size: {} + {:#?}", a.keys().len(), hm);
        } else {
            info!("Failed to deserialize into event");
        }

        if delivery
            .ack(lapin::options::BasicAckOptions::default())
            .await
            .is_err()
        {
            info!("ampq failed to ack");
            break;
        }
    }
    info!("ampq app exiting");
}

fn axum_app(graph: EiffelGraphShared) -> Router<BoxRoute> {
    Router::new()
        .route("/", get(handler))
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(AddExtensionLayer::new(graph))
        .boxed()
}

async fn handler(Extension(graph): Extension<EiffelGraphShared>) -> impl IntoResponse {
    let bytes = serde_json::to_vec(&*graph.read().await).unwrap();
    let mut res = Response::new(Full::from(bytes));
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    res
}
