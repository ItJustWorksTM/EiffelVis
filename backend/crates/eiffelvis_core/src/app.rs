use crate::algorithms::depth_first;
use crate::graph_storage::ChunkedGraph;
use crate::{graph::*, types::BaseEvent};
use std::ops::ControlFlow;

use indexmap::IndexSet;
use uuid::Uuid;
impl GraphKey for Uuid {}

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
        self.nodes().map(|(_, node)| T::from(node.data())).collect()
    }

    fn get_subgraph_with_root<'a, T: From<&'a BaseEvent>>(
        &'a self,
        root_id: Uuid,
    ) -> Option<Vec<T>> {
        let mut index = IndexSet::<_>::default();

        depth_first(self, self.to_index(root_id).unwrap(), &mut |i| {
            if index.insert(i) {
                ControlFlow::Continue(())
            } else {
                ControlFlow::Break(())
            }
        });

        index.sort_by(|&lhs, &rhs| self.cmp_index(lhs, rhs));

        Some(
            index
                .drain(..)
                .map(|index| T::from(self.index(index).data()))
                .collect(),
        )
    }

    fn head(&self) -> Option<Uuid> {
        self.last().and_then(|i| self.from_index(i))
    }

    fn events_starting_from<'a, T: From<&'a BaseEvent>>(&'a self, id: Uuid) -> Option<Vec<T>> {
        self.to_index(id)
            .zip(self.last())
            .filter(|(begin, end)| {
                println!("{:?}", (begin, end));
                begin != end
            })
            .map(|(begin, _)| {
                println!("hello!");
                let mut iter = self.nodes();
                iter.by_ref().take_while(|(i, _)| *i != begin).count();
                iter.map(|(_, node)| T::from(node.data()))
                    .collect::<Vec<_>>()
            })
            .filter(|v| !v.is_empty())
    }
}
