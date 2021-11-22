use std::ops::ControlFlow;

use crate::graph::*;

/// A recursive depth first traversal implementation,
/// if user provided closure returns [ControlFlow::Break] that particular node's edges will not be visitted,
/// but will not stop the whole traversal.
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
