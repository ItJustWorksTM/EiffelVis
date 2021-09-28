use std::{borrow::Cow, str::FromStr};

use futures::StreamExt;

use lapin::{tcp::TLSConfig, uri::AMQPUri, Connection, ConnectionProperties, Consumer};
use tokio_amqp::LapinTokioExt;

pub struct AmpqStream {
    addr: Cow<'static, str>,
    queue: Cow<'static, str>,
    reconnect_timeout: u64,
    consumer: Option<Consumer>,
}

impl AmpqStream {
    pub async fn new(
        addr: Cow<'static, str>,
        queue: Cow<'static, str>,
        reconnect_timeout: u64,
    ) -> Option<Self> {
        Some(Self {
            consumer: Some(make_ampq_consumer(&addr, &queue).await?),
            addr,
            queue,
            reconnect_timeout,
        })
    }

    /// Yields the next message.
    /// Returns None on failure, reconnects on the next call; thus it's recommended to wait a bit before trying again.
    pub async fn next(&mut self) -> Option<Vec<u8>> {
        if self.consumer.is_none() {
            self.consumer = make_ampq_consumer(&self.addr, &self.queue).await;
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

async fn make_ampq_consumer(addr: &str, queue: &str) -> Option<Consumer> {
    let connection = Connection::connect_uri_with_identity(
        AMQPUri::from_str(addr).ok()?,
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
            "my_consumer",
            lapin::options::BasicConsumeOptions {
                no_ack: true, // Ack by default to avoid big problems in poll_next
                ..Default::default()
            },
            lapin::types::FieldTable::default(),
        )
        .await
        .ok()?;
    Some(consumer)
}
