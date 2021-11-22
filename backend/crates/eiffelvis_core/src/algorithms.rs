use std::ops::ControlFlow;

use crate::graph::*;

pub fn depth_first<G, F>(graph: &G, index: G::Idx, callback: &mut F)
where
    G: Graph,
    F: FnMut(G::Idx) -> ControlFlow<()>,
{
    let node = graph.index(index);

    if let ControlFlow::Continue(()) = callback(index) {
        for edge in node.edges() {
            depth_first(graph, edge.target(), callback);
        }
    }
}
