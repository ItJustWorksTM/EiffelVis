use crate::{
    chunked_index_map::{self, ChunkedIndex, ChunkedIndexMap},
    graph,
};
use std::{default::Default, ops::RangeBounds};

pub struct ChunkedGraph<K: graph::Key, N, E> {
    inner: ChunkedIndexMap<K, Element<N, E>>,
}

#[derive(Debug)]
pub struct Element<N, E>(NodeData<N>, Vec<EdgeData<E>>);

#[derive(Debug)]
pub struct NodeData<N> {
    pub data: N,
}

#[derive(Debug)]
pub struct EdgeData<E> {
    pub data: E,
    pub target: ChunkedIndex,
}

impl<K: graph::Key, N, E> ChunkedGraph<K, N, E> {
    pub fn new(max_chunks: usize, chunk_size: u32) -> Self {
        Self {
            inner: ChunkedIndexMap::new(max_chunks, chunk_size),
        }
    }

    fn add_node(&mut self, key: K, data: N) -> ChunkedIndex {
        self.inner
            .insert(key, Element(NodeData { data }, Vec::default()))
    }

    fn add_edge(&mut self, a: K, b: K, data: E) {
        let from_index = self.inner.get_index_of(&b).unwrap();
        let new_index = self.inner.get_index_of(&a).unwrap();

        let (_, Element(_, edges)) = self.inner.get_index_mut(from_index).unwrap();

        edges.push(EdgeData {
            data,
            target: new_index,
        })
    }

    pub fn node_count(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, N, E> graph::Item<'a> for (ChunkedIndex, &'a Element<N, E>) {
    type Data = N;
    type EdgeData = E;
    type Idx = ChunkedIndex;

    fn id(&self) -> ChunkedIndex {
        self.0
    }

    fn data(&self) -> &'a N {
        &self.1 .0.data
    }

    type EdgeItem = &'a EdgeData<E>;
    type EdgeIterator = std::slice::Iter<'a, EdgeData<E>>;
    fn edges(&self) -> Self::EdgeIterator {
        self.1 .1.iter()
    }
}

impl<'a, E> graph::ItemEdge<'a> for &'a EdgeData<E> {
    type EdgeData = E;
    type Idx = ChunkedIndex;

    fn target(&self) -> ChunkedIndex {
        self.target
    }

    fn data(&self) -> &'a E {
        &self.data
    }
}

impl<K: graph::Key, N, E> graph::Meta for ChunkedGraph<K, N, E> {
    type Idx = ChunkedIndex;
    type Key = K;
    type Data = N;
    type EdgeData = E;
}

impl<'a, K: graph::Key, N, E> graph::HasNode<'a> for ChunkedGraph<K, N, E> {
    type NodeType = (ChunkedIndex, &'a Element<N, E>);
}

impl<'a, K: graph::Key, N, E> graph::HasNodeIter<'a> for ChunkedGraph<K, N, E> {
    type Item = (ChunkedIndex, &'a Element<N, E>);
    type NodeIterType = chunked_index_map::Iter<'a, K, Element<N, E>, ahash::RandomState>;
}

impl<'a, K: graph::Key, N, E> graph::HasNodeRangeIter<'a> for ChunkedGraph<K, N, E> {
    type Item = (ChunkedIndex, &'a Element<N, E>);
    type NodeRangeIterType = chunked_index_map::Iter<'a, K, Element<N, E>, ahash::RandomState>;
}

impl<K: graph::Key, N, E> graph::ItemIter for ChunkedGraph<K, N, E> {
    fn items(&self) -> graph::NodeIterType<'_, Self> {
        self.inner.iter()
    }

    fn range<R>(&self, range: R) -> graph::NodeRangeIterType<'_, Self>
    where
        R: RangeBounds<Self::Idx>,
    {
        self.inner.iter_range(range)
    }
}

impl<K: graph::Key, N, E> graph::Graph for ChunkedGraph<K, N, E> {
    fn add_node(&mut self, key: K, data: N) -> Option<ChunkedIndex> {
        Some(self.add_node(key, data))
    }

    fn add_edge(&mut self, a: K, b: K, data: E) {
        self.add_edge(a, b, data)
    }

    fn node_count(&self) -> usize {
        ChunkedGraph::node_count(self)
    }
}

impl<K: graph::Key, N, E> graph::Indexable<ChunkedIndex> for ChunkedGraph<K, N, E> {
    fn get(&self, index: ChunkedIndex) -> Option<graph::NodeType<'_, Self>> {
        self.inner.get_index(index).map(|(_, v)| (index, v))
    }
}

impl<K: graph::Key, N, E> graph::Indexable<K> for ChunkedGraph<K, N, E> {
    fn get(&self, index: K) -> Option<graph::NodeType<'_, Self>> {
        self.inner.get_full(&index).map(|(ix, _, v)| (ix, v))
    }
}

impl<K: graph::Key, N, E> graph::Indexable<usize> for ChunkedGraph<K, N, E> {
    /// Indexes the graph as if it was a linear storage like [Vec]
    fn get(&self, index: usize) -> Option<graph::NodeType<'_, Self>> {
        self.inner
            .get_index_of_linear(index)
            .map(|ix| (ix, self.inner.get_index(ix).unwrap().1))
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::graph::*;

    use std::iter::once;

    #[test]
    fn test_forward_link_single() {
        impl graph::Key for i32 {}
        let mut g = ChunkedGraph::new(4, 4);

        g.add_node(0, "This is the beginning!".into());

        for i in 1..17 {
            g.add_node_with_edges(
                i,
                format!("more data {}", i),
                once((0, format!("targets {}", i))),
            )
            .unwrap();
        }

        assert_eq!(g.items().count(), g.node_count());
        assert_eq!(g.range(..).count(), g.node_count());

        let mut last = g.items().next().unwrap().id();
        for no in g.items().skip(1) {
            assert_eq!(g.range(last..=no.id()).count(), 2);
            last = no.id();
        }

        // All nodes should now have links to the first node (which are stored on the first node itself)
        if let Some(zeroth) = g.get(0) {
            assert_eq!(zeroth.edges().count(), 8);
        }
    }

    #[test]
    fn test_forward_link_many() {
        let mut g = ChunkedGraph::new(4, 4);

        g.add_node(0, String::from("the first node"));

        for i in 1..16 {
            g.add_node_with_edges(i, format!("boy {}", i), once((i - 1, "")))
                .expect("This is valid");
        }

        assert_eq!(g.items().count(), g.node_count());

        for (i, item) in g.items().enumerate() {
            let it = g.index(i as i32);
            assert_eq!(it.data(), item.data());
        }

        // Now i - 1 node should store an edge to node i
        // 0 -> 1, 1 -> 2 ...
        // for i in 1..15i32 {
        //     assert_eq!(
        //         g.index((i - 1) as i32)
        //             .edges()
        //             .find_map(|i| g.from_index(i.target())),
        //         Some(i as i32)
        //     );
        // }
    }
}
