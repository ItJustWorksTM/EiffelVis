use crate::types::BaseEvent;
use crate::{graph_storage::ChunkedGraph, types::LeanEvent};

use uuid::Uuid;

type EiffelGraph = ChunkedGraph<Uuid, BaseEvent, String>;

/// Mainly a wrapper struct around backing event storage,
/// aims to provide all common operations eiffelvis would need to perform.
pub struct EiffelVisApp {
    graph: EiffelGraph,
}

impl EiffelVisApp {
    pub fn new(max_chunks: usize, chunk_size: usize) -> Self {
        Self {
            graph: EiffelGraph::new(max_chunks, chunk_size),
        }
    }

    /// Inserts a new eiffel event into storage
    pub fn push(&mut self, event: BaseEvent) {
        self.graph.push(
            event.meta.id,
            event
                .links
                .iter()
                .map(|link| (link.target, link.link_type.clone()))
                .collect(),
            event,
        );
    }

    /// Returns the underlying event store implementation
    pub fn graph(&self) -> &EiffelGraph {
        &self.graph
    }

    /// Looks up the event of given id
    pub fn get_event(&self, id: Uuid) -> Option<&BaseEvent> {
        self.graph.get(id).map(|node| &node.data)
    }

    /// Returns all stored events in lean format, useful if only graph data is needed
    pub fn dump_lean_events(&self) -> Vec<LeanEvent> {
        self.graph
            .iter()
            .map(|node| LeanEvent::from(&node.data))
            .collect()
    }

    /// Returns all stored sevents.
    pub fn dump_events(&self) -> Vec<&BaseEvent> {
        self.graph.iter().map(|node| &node.data).collect()
    }
}
