use std::ops::ControlFlow;

use crate::graph::*;

pub fn depth_first<'a, G, F>(graph: G, index: <G::Meta as Meta>::Idx, callback: &mut F)
where
    G: Ref<'a>,
    F: FnMut(<G::Meta as Meta>::Idx) -> ControlFlow<()>,
{
    let node = graph.index(index);

    if let ControlFlow::Continue(()) = callback(index) {
        for edge in node.edges() {
            depth_first(graph, edge.target(), callback);
        }
    }
}
