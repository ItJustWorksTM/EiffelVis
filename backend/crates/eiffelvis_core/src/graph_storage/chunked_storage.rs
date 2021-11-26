use crate::graph;
use indexmap::IndexMap;
use std::{hash::Hash, ops::IndexMut};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ChunkedIndex(usize, usize);

#[derive(Debug)]
pub struct ChunkedGraph<K: graph::Key, N, E> {
    store: Vec<IndexMap<K, Element<N, E>>>,
    max_chunks: usize,
    max_elements: usize,
    tail: usize,
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
    pub fn new(max_chunks: usize, chunk_size: usize) -> Self {
        Self {
            store: vec![IndexMap::with_capacity(chunk_size)],
            max_chunks,
            max_elements: chunk_size,
            tail: 0,
        }
    }

    fn add_node(&mut self, key: K, data: N) -> ChunkedIndex {
        if self.store[self.head_chunk()].len() >= self.max_elements {
            if self.chunks() < self.max_chunks {
                self.store.push(IndexMap::with_capacity(self.max_elements));
            } else {
                self.tail = (self.tail + 1) % self.store.len();
                self.store.index_mut(self.head_chunk()).clear();
            }
        }

        let head_chunk = self.head_chunk();

        self.store[head_chunk].insert(key, Element(NodeData { data }, Vec::default()));

        ChunkedIndex(head_chunk, self.store[head_chunk].len() - 1)
    }

    fn add_edge(&mut self, a: K, b: K, data: E) {
        let new_index = self.to_index(a).unwrap();

        if let Some(node_edges) = self
            .to_index(b)
            .and_then(|index| self.store.get_mut(index.0).map(|a| &mut a[index.1].1))
        {
            node_edges.push(EdgeData {
                data,
                target: new_index,
            });
        }
    }

    /// Translates given key to internal index.
    /// This is generally an relatively expensive operation, avoid frequent translations.
    pub fn to_index(&self, key: K) -> Option<ChunkedIndex> {
        self.store
            .iter()
            .enumerate()
            .find_map(|(chunk_index, chunk)| {
                chunk
                    .get_index_of(&key)
                    .map(|i| ChunkedIndex(chunk_index, i))
            })
    }

    /// Translates given internal index to key
    pub fn from_index(&self, index: ChunkedIndex) -> Option<K> {
        Some(*self.store.get(index.0)?.get_index(index.1)?.0)
    }

    /// Translates absolute index to internal index,
    /// if index > self.node_count() return index is undefined.
    fn to_relative(&self, index: usize) -> ChunkedIndex {
        let a = index % self.max_elements;
        let b = (index - a) / self.chunks();
        let b = (self.tail + b) % self.chunks();
        ChunkedIndex(b, a)
    }

    /// Translates relative index (starting from tail) to absolute
    fn to_absolute(&self, index: ChunkedIndex) -> usize {
        (index.0 * self.max_elements) + index.1 - (self.tail * self.max_elements)
    }

    pub fn chunks(&self) -> usize {
        self.store.len()
    }

    /// Returns the index of the most recent added node
    pub fn last(&self) -> Option<ChunkedIndex> {
        let head = self.head_chunk();
        let len = self.store[head].len();
        if len > 0 {
            Some(ChunkedIndex(head, len - 1))
        } else {
            None
        }
    }

    fn head_chunk(&self) -> usize {
        (self.tail + self.chunks() - 1) % self.chunks()
    }

    pub fn node_count(&self) -> usize {
        (self.chunks() - 1) * self.max_elements + self.store[self.head_chunk()].len()
    }

    pub fn cmp_index(&self, lhs: ChunkedIndex, rhs: ChunkedIndex) -> std::cmp::Ordering {
        let lhs = self.to_absolute(lhs);
        let rhs = self.to_absolute(rhs);
        lhs.cmp(&rhs)
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
    type EdgeIterator = EdgeIter<'a, E>;
    fn edges(&self) -> Self::EdgeIterator {
        EdgeIter {
            inner: self.1 .1.iter(),
        }
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

impl<'a, K: graph::Key, N, E> graph::HasNodeIter<'a, (ChunkedIndex, &'a Element<N, E>)>
    for ChunkedGraph<K, N, E>
{
    type NodeIterType = NodeIter<'a, K, N, E>;
}

impl<'a, K: graph::Key, N, E> graph::ItemIter for ChunkedGraph<K, N, E> {
    fn items(&self) -> graph::NodeIterType<'_, Self> {
        NodeIter {
            inner: self.store[self.tail].iter(),
            chunk: self.tail,
            graph: self,
            item: 0,
        }
    }
}

impl<'a, K: graph::Key, N, E> graph::Graph for ChunkedGraph<K, N, E> {
    fn cmp_index(&self, lhs: ChunkedIndex, rhs: ChunkedIndex) -> std::cmp::Ordering {
        ChunkedGraph::cmp_index(self, lhs, rhs)
    }

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

impl<'a, K: graph::Key, N, E> graph::Indexable<ChunkedIndex> for ChunkedGraph<K, N, E> {
    fn get(&self, index: ChunkedIndex) -> Option<graph::NodeType<'_, Self>> {
        self.store
            .get(index.0)
            .and_then(|m| m.get_index(index.1))
            .map(|el| (index, el.1))
    }
}

impl<'a, K: graph::Key, N, E> graph::Indexable<K> for ChunkedGraph<K, N, E> {
    fn get(&self, index: K) -> Option<graph::NodeType<'_, Self>> {
        self.to_index(index).map(|i| self.index(i))
    }
}

pub struct NodeIter<'a, K: graph::Key, N, E> {
    inner: <&'a IndexMap<K, Element<N, E>> as IntoIterator>::IntoIter,
    graph: &'a ChunkedGraph<K, N, E>,
    chunk: usize,
    item: usize,
}

impl<'a, K: graph::Key, N, E> Iterator for NodeIter<'a, K, N, E> {
    type Item = (ChunkedIndex, &'a Element<N, E>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .or_else(|| {
                if self.chunk != self.graph.head_chunk() {
                    self.chunk = (self.chunk + 1) % self.graph.chunks();
                    self.item = 0;
                    self.inner = self.graph.store[self.chunk].iter();
                    self.inner.next()
                } else {
                    None
                }
            })
            .map(|(_, n)| {
                let ret = (ChunkedIndex(self.chunk, self.item), n);
                self.item += 1;
                ret
            })
    }
}

pub struct EdgeIter<'a, E> {
    inner: <&'a Vec<EdgeData<E>> as IntoIterator>::IntoIter,
}

impl<'a, E> Iterator for EdgeIter<'a, E> {
    type Item = &'a EdgeData<E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::graph::*;

    use std::iter::once;

    // TODO: Add more correctness tests, like cmp_index, especially in cases where we have wrapped around.

    #[test]
    fn test_forward_link_single() {
        impl graph::Key for i32 {}
        let mut g = ChunkedGraph::new(3, 3);

        g.add_node(0, "This is the beginning!");

        for i in 1..9 {
            g.add_node_with_edges(i, "more data", once((0, format!("targets {}", i))));
        }

        // assert_eq!(g.iter_range(NodeIndex(0, 0), NodeIndex(1, 2)).count(), 5);

        // All nodes should now have links to the first node (which are stored on the first node itself)
        let zeroth = g.index(0).edges().count();
        assert_eq!(zeroth, 8);
    }

    #[test]
    fn test_forward_link_many() {
        let mut g = ChunkedGraph::new(3, 3);

        g.add_node(0, String::from("the first node"));

        for i in 1..9 {
            g.add_node_with_edges(i, format!("boy {}", i), once((i - 1, "")))
                .expect("This is valid");
        }

        for (i, item) in g.items().enumerate() {
            let it = g.index(i as i32);
            assert_eq!(it.data(), item.data());
        }

        // Now i - 1 node should store an edge to node i
        // 0 -> 1, 1 -> 2 ...
        for i in 1..8i32 {
            assert_eq!(
                g.index((i - 1) as i32)
                    .edges()
                    .find_map(|i| g.from_index(i.target())),
                Some(i as i32)
            );
        }
    }
}

pub struct NodeIndexIter<'a, K: graph::Key, N, E> {
    inner: <std::ops::Range<usize> as IntoIterator>::IntoIter,
    graph: &'a ChunkedGraph<K, N, E>,
}

impl<'a, K: graph::Key, N, E> Iterator for NodeIndexIter<'a, K, N, E> {
    type Item = ChunkedIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|i| self.graph.to_relative(i))
    }
}
