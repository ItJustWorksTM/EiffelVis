use std::ops::ControlFlow;

use crate::{algorithms::depth_first, graph::*};

use indexmap::IndexSet;

/// An iterator that takes graph nodes and yields the subgraph
/// with given nodes as root.
pub struct SubGraphs<'a, G>
where
    G: Ref<'a>,
{
    graph: G,
    index: <IndexSet<<G::Meta as Meta>::Idx> as IntoIterator>::IntoIter,
}

impl<'a, G> Iterator for SubGraphs<'a, G>
where
    G: Ref<'a>,
{
    type Item = G::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.index.next().map(|i| self.graph.index(i))
    }
}

pub trait GraphQuery: Iterator {
    fn roots_for_graph<'a, G>(self, graph: G) -> SubGraphs<'a, G>
    where
        G: Ref<'a>,
        Self: Iterator<Item = G::Item> + Sized,
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
