use std::borrow::Cow;

use futures::StreamExt;

pub use lapin::uri::AMQPUri;
use lapin::{tcp::TLSConfig, Connection, ConnectionProperties, Consumer};
use tokio_amqp::LapinTokioExt;

pub use lapin::auth::Credentials;

pub struct AmpqStream {
    addr: AMQPUri,
    queue: Cow<'static, str>,
    consumer_tag: Cow<'static, str>,
    consumer: Option<Consumer>,
}

impl AmpqStream {
    pub async fn new(
        addr: AMQPUri,
        queue: Cow<'static, str>,
        consumer_tag: Cow<'static, str>,
    ) -> Option<Self> {
        Some(Self {
            consumer: Some(make_ampq_consumer(&addr, &queue, &consumer_tag).await?),
            addr,
            queue,
            consumer_tag,
        })
    }

    /// Yields the next message.
    /// Returns None on failure, reconnects on the next call; thus it's recommended to wait a bit before trying again.
    pub async fn next(&mut self) -> Option<Vec<u8>> {
        if self.consumer.is_none() {
            self.consumer = make_ampq_consumer(&self.addr, &self.queue, &self.consumer_tag).await;
        }
        if let Some(consumer) = &mut self.consumer {
            if let Some(Ok((_, delivery))) = consumer.next().await {
                Some(delivery.data)
            } else {
                None
            }
        } else {
            self.consumer = None;
            None
        }
    }
}

async fn make_ampq_consumer(addr: &AMQPUri, queue: &str, consumer_tag: &str) -> Option<Consumer> {
    let connection = Connection::connect_uri_with_identity(
        addr.clone(),
        ConnectionProperties::default().with_tokio(),
        TLSConfig::default(),
    )
    .await
    .ok()?;
    let consumer = connection
        .create_channel()
        .await
        .ok()?
        .basic_consume(
            queue,
            consumer_tag,
            lapin::options::BasicConsumeOptions {
                no_ack: true, // We never have a need to ACK ourselves so let the library handle it
                ..Default::default()
            },
            lapin::types::FieldTable::default(),
        )
        .await
        .ok()?;
    Some(consumer)
}
