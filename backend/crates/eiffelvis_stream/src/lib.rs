//! A simple library that hides away all the details and gives you a simple message stream to consume.
//!
//! At the moment only AMPQ is supported through lapin, eiffelvis_gen support is planned.
//!
//! # Glosary
//!
//! ```no_run
//! # async {
//! let mut stream = eiffelvis_stream::ampq::AmpqStream::new(
//!         "amqp://localhost:5672/%2f".into(),
//!         "hello".into(),
//!         "eiffelvis".into(),
//!     )
//!     .await
//!     .expect("Failed to connect to ampq server");
//!
//! let message: Option<Vec<u8>> = stream.next().await;
//! # };
//! ```
//!
//! *note: tokio is used as async executor*
//!

/// Provides an event stream from an ampq connection
pub mod ampq;

/// Provides an event stream produced by eiffelvis_gen
pub mod gen;
