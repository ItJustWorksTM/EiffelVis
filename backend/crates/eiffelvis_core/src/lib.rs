//!
//! The *core* logic library for eiffelvis.
//!
//! Aims to provide a common interface that "frontends" such as eiffelvis_http don't have manage eiffel events.
//!

/// eiffelvis domains specicifc api
pub mod domain;

/// DAG storage implementations
pub mod graph_storage;

/// Graph traits
pub mod graph;

/// Graph traversal algorithms
pub mod algorithms;

/// Generic graph query functionality
pub mod query;

/// Generic tracked graph query functionality
pub mod tracked_query;

pub mod chunked_index_map;