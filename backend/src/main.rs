use axum::{
    body::Full,
    extract::Extension,
    handler::get,
    http::HeaderValue,
    response::{IntoResponse, Json},
    routing::BoxRoute,
    AddExtensionLayer, Router,
};
use eiffelvis_gen::make_event;
use futures::StreamExt;
use hyper::{header, Response};
use lapin::{Connection, ConnectionProperties};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
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
    pub time: i64, // Not high priority?
                   // source: Source
                   // security: Security
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "type")]
    pub link_type: String,
    pub target: Uuid,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Event<T: Default> {
    pub meta: Meta,
    pub data: T,
    pub links: Vec<Link>,
}

type EiffelGraph = HashMap<Uuid, Event<serde_json::Value>>;
type EiffelGraphShared = Arc<RwLock<EiffelGraph>>;

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::init();

    let nice = HashMap::new();

    let locked_test = Arc::new(RwLock::new(nice));

    let app = app(locked_test.clone());
    let axum_job =
        axum::Server::bind(&"127.0.0.1:3000".parse().unwrap()).serve(app.into_make_service());
    let _ = tokio::join!(
        tokio::spawn(amqp(locked_test.clone())),
        tokio::spawn(axum_job)
    );
}

async fn amqp(graph: EiffelGraphShared) {
    let addr = "amqp://127.0.0.1:5672/%2f".to_string();
    let conn = Connection::connect(&addr, ConnectionProperties::default().with_tokio())
        .await
        .unwrap();

    let channel = conn.create_channel().await.unwrap();
    let mut consumer = channel
        .basic_consume(
            "hello",
            "my_consumer",
            lapin::options::BasicConsumeOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await
        .unwrap();

    tokio::spawn(async move {
        while let Some(delivery) = consumer.next().await {
            let delivery = delivery.unwrap().1;
            // info!("message: {:?}", std::str::from_utf8(&delivery.data));
            let test_event: Event<serde_json::Value> =
                serde_json::from_slice(&delivery.data).unwrap();
            let mut a = graph.write().await;
            info!("Graph size: {}", a.keys().len());
            a.insert(test_event.meta.id, test_event);
            delivery
                .ack(lapin::options::BasicAckOptions::default())
                .await
                .unwrap();
        }
    });
}

fn app(graph: EiffelGraphShared) -> Router<BoxRoute> {
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
