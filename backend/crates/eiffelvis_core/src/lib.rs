//!
//! The *core* logic library for eiffelvis.
//!
//! Aims to provide a common interface that "frontends" such as eiffelvis_http don't have manage eiffel events.
//!

/// eiffelvis core api
pub mod app;

/// DAG storage implementations
pub mod graph_storage;

/// eiffelvis shared types, e.g. events
pub mod types;
