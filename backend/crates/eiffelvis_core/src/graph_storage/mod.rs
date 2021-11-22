//!
//! Storage types that implement the [crate::graph::Graph] trait.
//!

/// A storage type that employs the chunked fixed-size ringbuffer strategy
pub mod chunked_storage;
pub use chunked_storage::ChunkedGraph;
