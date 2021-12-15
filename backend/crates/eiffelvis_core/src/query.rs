use std::ops::ControlFlow;

use crate::{algorithms::depth_first, graph::*};

use ahash::RandomState;
use indexmap::IndexSet;

/// An iterator that takes graph nodes and yields the subgraph
/// with given nodes as root.
pub struct SubGraphs<'a, G>
where
    G: Graph,
{
    graph: &'a G,
    index: <IndexSet<G::Idx> as IntoIterator>::IntoIter,
}

impl<'a, G> Iterator for SubGraphs<'a, G>
where
    G: Graph,
{
    type Item = NodeType<'a, G>;

    /// Yields the next node, guarantees nodes are yielded in order as defined by [crate::graph::Graph::cmp_index]
    fn next(&mut self) -> Option<Self::Item> {
        self.index.next().map(|i| self.graph.index(i))
    }
}

pub trait GraphQuery: Iterator {
    /// Consumes the iterator, used up nodes will be treated as root nodes in given graph
    fn roots_for_graph<'a, G>(self, graph: &'a G) -> SubGraphs<'a, G>
    where
        G: Graph,
        Self: Iterator<Item = NodeType<'a, G>> + Sized,
    {
        // Simple visit map, all nodes will be only visited once
        let mut index = IndexSet::with_capacity_and_hasher(8, RandomState::default());
        for node in self {
            depth_first(graph, node.id(), &mut |i| {
                if index.insert(i) {
                    ControlFlow::Continue(())
                } else {
                    ControlFlow::Break(())
                }
            });
        }

        // Sort the visit map, as depth_first does not preserve insertion order
        index.sort();

        SubGraphs {
            graph,
            index: index.into_iter(),
        }
    }
}

impl<I: Iterator> GraphQuery for I {}
