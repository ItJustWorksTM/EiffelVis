use crate::{domain::types::BaseEvent, graph::*};
use crate::{graph_storage::ChunkedGraph, query::GraphQuery};

use uuid::Uuid;

pub type EiffelGraph = ChunkedGraph<Uuid, BaseEvent, String>;

impl Key for Uuid {}

pub trait EiffelVisApp {
    fn push(&mut self, event: BaseEvent);
    fn graph(&self) -> &Self {
        self
    }
    fn get_event(&self, id: Uuid) -> Option<&BaseEvent>;
    fn dump<'a, T: From<&'a BaseEvent>>(&'a self) -> Vec<T>;
    fn get_subgraph_with_roots<'a, T: From<&'a BaseEvent>>(&'a self, roots: &[Uuid]) -> Vec<T>;
    fn head(&self) -> Option<Uuid>;
    fn events_starting_from<'a, T: From<&'a BaseEvent>>(&'a self, id: Uuid) -> Option<Vec<T>>;
}

impl EiffelVisApp for EiffelGraph {
    /// Inserts a new eiffel event into storage
    fn push(&mut self, event: BaseEvent) {
        let links = event.links.clone();
        self.add_node_with_edges(
            event.meta.id,
            event,
            links.into_iter().map(|link| (link.target, link.link_type)),
        );

        println!("Graph size: {}", self.node_count());
    }

    /// Looks up the event of given id
    fn get_event(&self, id: Uuid) -> Option<&BaseEvent> {
        Some(self.index(id).data())
    }

    /// Returns all current stored events
    fn dump<'a, T: From<&'a BaseEvent>>(&'a self) -> Vec<T> {
        self.items().map(|node| T::from(node.data())).collect()
    }

    fn get_subgraph_with_roots<'a, T: From<&'a BaseEvent>>(&'a self, roots: &[Uuid]) -> Vec<T> {
        roots
            .iter()
            .map(|i| self.index(*i))
            .roots_for_graph(self)
            .map(|node| T::from(node.data()))
            .collect()
    }

    fn head(&self) -> Option<Uuid> {
        self.last().and_then(|i| self.from_index(i))
    }

    fn events_starting_from<'a, T: From<&'a BaseEvent>>(&'a self, id: Uuid) -> Option<Vec<T>> {
        self.to_index(id)
            .zip(self.last())
            .filter(|(begin, end)| begin != end)
            .map(|(begin, _)| {
                let mut iter = self.items();
                iter.by_ref().take_while(|(i, _)| *i != begin).count();
                iter.map(|node| T::from(node.data())).collect::<Vec<_>>()
            })
            .filter(|v| !v.is_empty())
    }
}
