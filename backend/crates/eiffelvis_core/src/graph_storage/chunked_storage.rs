use crate::graph::{Edge, Graph, GraphMeta, GraphMut, Node, *};
use indexmap::IndexMap;
use std::{hash::Hash, ops::IndexMut};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ChunkedIndex(usize, usize);

#[derive(Debug)]
pub struct ChunkedGraph<K: GraphKey, N, E> {
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

impl<K: GraphKey, N, E> ChunkedGraph<K, N, E> {
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

    pub fn from_index(&self, index: ChunkedIndex) -> Option<K> {
        Some(*self.store.get(index.0)?.get_index(index.1)?.0)
    }

    fn to_relative(&self, index: usize) -> ChunkedIndex {
        let a = index % self.max_elements;
        let b = (index - a) / self.chunks();
        let b = (self.tail + b) % self.chunks();
        ChunkedIndex(b, a)
    }

    fn to_absolute(&self, index: ChunkedIndex) -> usize {
        (index.0 * self.max_elements) + index.1 - self.tail * self.max_elements
    }

    pub fn chunks(&self) -> usize {
        self.store.len()
    }

    pub fn last(&self) -> Option<ChunkedIndex> {
        let head = self.head_chunk();
        let len = self.store[head].len();
        if len > 0 {
            Some(ChunkedIndex(head, len - 1))
        } else {
            None
        }
    }

    pub fn head_chunk(&self) -> usize {
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

impl<'a, N, E> Node for &'a Element<N, E> {
    type Data = &'a N;
    fn data(self) -> Self::Data {
        &self.0.data
    }

    type Edge = &'a EdgeData<E>;
    type EdgeIterator = EdgeIter<'a, E>;
    fn edges(self) -> Self::EdgeIterator {
        EdgeIter {
            inner: self.1.iter(),
        }
    }
}

impl<'a, E> Edge for &'a EdgeData<E> {
    type Data = &'a E;
    type NodeIndex = ChunkedIndex;

    fn target(self) -> Self::NodeIndex {
        self.target
    }

    fn data(self) -> Self::Data {
        &self.data
    }
}

impl<K: GraphKey, N, E> GraphMeta for ChunkedGraph<K, N, E> {
    type NodeIndex = ChunkedIndex;
    type NodeKey = K;
    type NodeData = N;
    type EdgeData = E;
}

impl<'a, K: GraphKey, N, E> Graph for &'a ChunkedGraph<K, N, E> {
    type K = K;
    type I = ChunkedIndex;

    type Node = &'a Element<N, E>;
    type Edge = &'a EdgeData<E>;

    type NodeIterator = NodeIter<'a, K, N, E>;

    fn nodes(self) -> Self::NodeIterator {
        NodeIter {
            inner: self.store[self.tail].iter(),
            chunk: self.tail,
            graph: self,
            item: 0,
        }
    }
}

impl<K: GraphKey, N, E> GraphMut for ChunkedGraph<K, N, E> {
    fn add_node(&mut self, key: K, data: N) -> Self::NodeIndex {
        self.add_node(key, data)
    }

    fn add_edge(&mut self, a: K, b: K, data: E) {
        self.add_edge(a, b, data)
    }
}

impl<'a, K: GraphKey, N, E> Index<&'a ChunkedGraph<K, N, E>> for ChunkedIndex {
    fn index(self, graph: &'a ChunkedGraph<K, N, E>) -> &'a Element<N, E> {
        &graph.store[self.0][self.1]
    }
}

impl<'a, K: GraphKey, N, E> Index<&'a ChunkedGraph<K, N, E>> for K {
    fn index(self, graph: &'a ChunkedGraph<K, N, E>) -> &'a Element<N, E> {
        Index::index(graph.to_index(self).unwrap(), graph)
    }
}

pub struct NodeIter<'a, K: GraphKey, N, E> {
    inner: <&'a IndexMap<K, Element<N, E>> as IntoIterator>::IntoIter,
    graph: &'a ChunkedGraph<K, N, E>,
    chunk: usize,
    item: usize,
}

impl<'a, K: GraphKey, N, E> Iterator for NodeIter<'a, K, N, E> {
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

impl<K: GraphKey, N, E> std::ops::Index<ChunkedIndex> for ChunkedGraph<K, N, E> {
    type Output = Element<N, E>;

    fn index(&self, index: ChunkedIndex) -> &Self::Output {
        todo!()
    }
}

#[test]
fn test_forward_link_single() {
    impl GraphKey for i32 {}

    let mut g = ChunkedGraph::new(3, 3);

    g.add_node(0, "This is the beginning!");

    for i in 1..9 {
        g.add_node_with_edges(
            i,
            "more data",
            vec![(0, format!("targets {}", i))].into_iter(),
        );
    }

    // assert_eq!(g.iter_range(NodeIndex(0, 0), NodeIndex(1, 2)).count(), 5);

    // All nodes should now have links to the first node (which are stored on the first node itself)
    let zeroth = g.index(0).edges().count();
    assert_eq!(zeroth, 8);
}

#[test]
fn test_forward_link_many() {
    let mut g = ChunkedGraph::new(3, 3);

    g.add_node(0, String::from("0"));

    for i in 1..9 {
        g.add_node_with_edges(i, format!("{}", i), std::iter::once((i - 1, "")));
    }

    // Now i - 1 node should store an edge to node i
    // 0 -> 1, 1 -> 2 ...
    for i in 1..8 {
        assert_eq!(
            g.index(i - 1)
                .edges()
                .next()
                .and_then(|i| g.from_index(i.target())),
            Some(i)
        );
    }
}

pub struct NodeIndexIter<'a, K: GraphKey, N, E> {
    inner: <std::ops::Range<usize> as IntoIterator>::IntoIter,
    graph: &'a ChunkedGraph<K, N, E>,
}

impl<'a, K: GraphKey, N, E> Iterator for NodeIndexIter<'a, K, N, E> {
    type Item = ChunkedIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|i| self.graph.to_relative(i))
    }
}