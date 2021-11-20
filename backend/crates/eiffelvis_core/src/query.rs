use std::ops::ControlFlow;

use crate::{
    algorithms::depth_first,
    graph::{Graph, Node},
};

use indexmap::IndexSet;

/// An iterator that takes graph nodes and yields the subgraph
/// with given nodes as root.
pub struct SubGraphs<G>
where
    G: Graph,
{
    graph: G,
    index: <IndexSet<G::I> as IntoIterator>::IntoIter,
}

impl<G> Iterator for SubGraphs<G>
where
    G: Graph,
{
    type Item = G::Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.index.next().map(|i| self.graph.index(i))
    }
}

pub trait GraphQuery: Iterator {
    fn roots_for_graph<G>(self, graph: G) -> SubGraphs<G>
    where
        Self: Iterator<Item = G::Node> + Sized,
        G: Graph,
    {
        let mut index = IndexSet::new();
        for node in self {
            depth_first(graph, node.id(), &mut |i| {
                if index.insert(i) {
                    ControlFlow::Continue(())
                } else {
                    ControlFlow::Break(())
                }
            });
        }

        index.sort_by(|&lhs, &rhs| graph.cmp_index(lhs, rhs));

        SubGraphs {
            graph,
            index: index.into_iter(),
        }
    }
}

impl<I: Iterator> GraphQuery for I {}
