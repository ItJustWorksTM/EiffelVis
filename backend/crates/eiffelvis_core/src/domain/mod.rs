//!
//! Domain specific logic for the eiffelvis library.
//!
//! This module contains logic specialized for the EiffelGraph trait, which is a supertrait over [crate::graph::Graph]
//!

/// Contains [EiffelGraph] and implements some special operations for it
pub mod app;

/// Eiffel Event types
pub mod types;

/// Query types and functions to acquire specific data from an [EiffelGraph]
pub mod user_queries;
