use crate::graph_storage::chunked_storage::GraphIndex;
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

    fn collect_sub_graph_recursive(&self, root_id: GraphIndex) -> Vec<LeanEvent> {
        let root = self.graph.index(root_id).unwrap();
        let mut ret = Vec::with_capacity(root.edges.len());
        for edge in &root.edges {
            ret.push(
                self.graph
                    .index(edge.target)
                    .map(|node| LeanEvent::from(&node.data))
                    .unwrap(),
            )
        }

        for edge in &root.edges {
            ret.append(&mut self.collect_sub_graph_recursive(edge.target))
        }

        ret
    }

    pub fn get_subgraph_with_root(&self, root_id: Uuid) -> Option<Vec<LeanEvent>> {
        let mut ret = self.collect_sub_graph_recursive(self.graph.find_index(root_id)?);
        // include the root node as well
        ret.push(LeanEvent::from(&self.graph.get(root_id)?.data));
        Some(ret)
    }

    pub fn head(&self) -> Option<Uuid> {
        self.graph.head().map(|(_, h, _)| *h)
    }

    pub fn events_starting_from(&self, id: Uuid) -> Option<Vec<LeanEvent>> {
        let indices = self
            .graph
            .find_index(id)
            .zip(self.graph.head().map(|l| l.0));

        indices.map(move |(begin, end)| {
            self.graph
                .iter_range(begin, end)
                .map(|node| LeanEvent::from(&node.data))
                .collect()
        })
    }
}
