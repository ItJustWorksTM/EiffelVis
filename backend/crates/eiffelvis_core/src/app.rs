use crate::graph_storage::chunked_storage::GraphIndex;
use crate::types::BaseEvent;
use crate::{graph_storage::ChunkedGraph, types::LeanEvent};

use uuid::Uuid;

pub type EiffelGraph = ChunkedGraph<Uuid, BaseEvent, String>;

pub trait EiffelVisApp {
    fn push(&mut self, event: BaseEvent);
    fn get_event(&self, id: Uuid) -> Option<&BaseEvent>;
    fn dump_lean_events(&self) -> Vec<LeanEvent>;
    fn dump_events(&self) -> Vec<&BaseEvent>;
    fn collect_sub_graph_recursive(&self, root_id: GraphIndex) -> Vec<LeanEvent>;
    fn get_subgraph_with_root(&self, root_id: Uuid) -> Option<Vec<LeanEvent>>;
    fn head(&self) -> Option<Uuid>;
    fn events_starting_from(&self, id: Uuid) -> Option<Vec<LeanEvent>>;
}

impl EiffelVisApp for EiffelGraph {
    /// Inserts a new eiffel event into storage
    fn push(&mut self, event: BaseEvent) {
        self.push(
            event.meta.id,
            event
                .links
                .iter()
                .map(|link| (link.target, link.link_type.clone()))
                .collect(),
            event,
        );

        println!("Graph size: {}", self.len());
    }

    /// Looks up the event of given id
    fn get_event(&self, id: Uuid) -> Option<&BaseEvent> {
        self.get(id).map(|node| &node.data)
    }

    /// Returns all stored events in lean format, useful if only graph data is needed
    fn dump_lean_events(&self) -> Vec<LeanEvent> {
        self.iter()
            .map(|node| LeanEvent::from(&node.data))
            .collect()
    }

    /// Returns all stored sevents.
    fn dump_events(&self) -> Vec<&BaseEvent> {
        self.iter().map(|node| &node.data).collect()
    }

    fn collect_sub_graph_recursive(&self, root_id: GraphIndex) -> Vec<LeanEvent> {
        let root = self.index(root_id).unwrap();
        let mut ret = Vec::with_capacity(root.edges.len());
        for edge in &root.edges {
            ret.push(
                self.index(edge.target)
                    .map(|node| LeanEvent::from(&node.data))
                    .unwrap(),
            )
        }

        for edge in &root.edges {
            ret.append(&mut self.collect_sub_graph_recursive(edge.target))
        }

        ret
    }

    fn get_subgraph_with_root(&self, root_id: Uuid) -> Option<Vec<LeanEvent>> {
        let mut ret = self.collect_sub_graph_recursive(self.find_index(root_id)?);
        // include the root node as well
        ret.push(LeanEvent::from(&self.get(root_id)?.data));
        Some(ret)
    }

    fn head(&self) -> Option<Uuid> {
        self.head().map(|(_, h, _)| *h)
    }

    fn events_starting_from(&self, id: Uuid) -> Option<Vec<LeanEvent>> {
        let indices = self.find_index(id).zip(self.head().map(|l| l.0));

        indices.map(move |(begin, end)| {
            self.iter_range(begin, end)
                .map(|node| LeanEvent::from(&node.data))
                .collect()
        })
    }
}
