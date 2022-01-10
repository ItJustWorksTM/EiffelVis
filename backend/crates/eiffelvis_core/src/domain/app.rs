use crate::query::GraphQuery;
use crate::{domain::types::BaseEvent, graph::*};

use uuid::Uuid;

pub trait EiffelGraph:
    Graph<Key = Uuid, Data = BaseEvent, EdgeData = String> + Indexable<usize>
{
}
impl<T> EiffelGraph for T where
    T: Graph<Key = Uuid, Data = BaseEvent, EdgeData = String> + Indexable<usize>
{
}

impl Key for Uuid {}

pub trait EiffelVisApp: EiffelGraph {
    /// Inserts a new eiffel event into storage
    fn push(&mut self, event: BaseEvent) -> bool;

    /// Looks up the event of given id
    fn get_event(&self, id: Uuid) -> Option<&BaseEvent>;

    /// Returns all current stored events
    fn dump<'a, T: From<&'a BaseEvent>>(&'a self) -> Vec<T>;

    /// Collects sub-graph('s) for given root nodes
    fn get_subgraph_with_roots<'a, T: From<&'a BaseEvent>>(&'a self, roots: &[Uuid]) -> Vec<T>;
}

impl<G: EiffelGraph> EiffelVisApp for G {
    fn push(&mut self, event: BaseEvent) -> bool {
        let links = event.links.clone();
        self.add_node_with_edges(
            event.meta.id,
            event,
            links.into_iter().map(|link| (link.target, link.link_type)),
        )
        .is_some()
    }

    fn get_event(&self, id: Uuid) -> Option<&BaseEvent> {
        Some(self.get(id)?.data())
    }

    fn dump<'a, T: From<&'a BaseEvent>>(&'a self) -> Vec<T> {
        self.items().map(|node| T::from(node.data())).collect()
    }

    fn get_subgraph_with_roots<'a, T: From<&'a BaseEvent>>(&'a self, roots: &[Uuid]) -> Vec<T> {
        roots
            .iter()
            .filter_map(|i| self.get(*i))
            .roots_for_graph(self)
            .map(|node| T::from(node.data()))
            .collect()
    }
}
