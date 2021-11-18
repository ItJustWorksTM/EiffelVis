use std::ops::ControlFlow;

use crate::graph::{Edge, Graph, Index, Node};

pub fn depth_first<G, I, F>(graph: G, index: I, callback: &mut F)
where
    G: Graph<I = I>,
    I: Index<G>,
    F: FnMut(G::NodeIndex) -> ControlFlow<()>,
{
    let node = graph.index(index);

    if let ControlFlow::Continue(()) = callback(index) {
        for edge in node.edges() {
            depth_first(graph, edge.target(), callback);
        }
    }
}
