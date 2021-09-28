use crate::graph_storage::ChunkedGraph;
use crate::types::BaseEvent;

use uuid::Uuid;

type EiffelGraph = ChunkedGraph<Uuid, BaseEvent, String>;

pub struct EiffelVisApp {
    graph: EiffelGraph,
}

impl EiffelVisApp {
    pub fn new(max_chunks: usize, chunk_size: usize) -> Self {
        Self {
            graph: EiffelGraph::new(max_chunks, chunk_size),
        }
    }

    pub fn push(&mut self, event: BaseEvent) {
        println!("added: {}", event.meta.id);
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

    pub fn graph(&self) -> &EiffelGraph {
        &self.graph
    }
}
