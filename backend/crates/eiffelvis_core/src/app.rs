use crate::graph_storage::ChunkedGraph;
use crate::types::BaseEvent;

use uuid::Uuid;

pub type EiffelGraph = ChunkedGraph<Uuid, BaseEvent, String>;

pub trait EiffelVisApp {
    fn push(&mut self, event: BaseEvent);
    fn get_event(&self, id: Uuid) -> Option<&BaseEvent>;
    fn dump<'a, T: From<&'a BaseEvent>>(&'a self) -> Vec<T>;
    fn get_subgraph_with_root<'a, T: From<&'a BaseEvent>>(
        &'a self,
        root_id: Uuid,
    ) -> Option<Vec<T>>;
    fn head(&self) -> Option<Uuid>;
    fn events_starting_from<'a, T: From<&'a BaseEvent>>(&'a self, id: Uuid) -> Option<Vec<T>>;
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

    /// Returns all current stored events
    fn dump<'a, T: From<&'a BaseEvent>>(&'a self) -> Vec<T> {
        self.iter().map(|node| T::from(&node.data)).collect()
    }

    fn get_subgraph_with_root<'a, T: From<&'a BaseEvent>>(
        &'a self,
        root_id: Uuid,
    ) -> Option<Vec<T>> {
        Some(
            self.collect_sub_graph_recursive(self.find_index(root_id)?)
                .drain(..)
                .map(|node| T::from(&node.data))
                .collect(),
        )
    }

    fn head(&self) -> Option<Uuid> {
        self.head().map(|(_, h, _)| *h)
    }

    fn events_starting_from<'a, T: From<&'a BaseEvent>>(&'a self, id: Uuid) -> Option<Vec<T>> {
        let indices = self.find_index(id).zip(self.head().map(|l| l.0));

        indices.map(move |(begin, end)| {
            self.iter_range(begin, end)
                .map(|node| T::from(&node.data))
                .collect()
        })
    }
}
