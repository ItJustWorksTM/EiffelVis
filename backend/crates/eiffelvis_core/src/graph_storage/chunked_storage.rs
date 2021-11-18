use crate::graph::{
    Edge as EdgeTrait, Graph as GraphTrait, GraphMeta as GraphMetaTrait, GraphMut as GraphMutTrait,
    Node as NodeTrait, *,
};
use indexmap::IndexMap;
use std::{hash::Hash, ops::IndexMut};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct NodeIndex(usize, usize);

#[derive(Debug)]
pub struct Graph<K: GraphKey, N, E> {
    store: Vec<IndexMap<K, Element<N, E>>>,
    max_chunks: usize,
    max_elements: usize,
    tail: usize,
}

#[derive(Debug)]
pub struct Element<N, E>(Node<N>, Vec<Edge<E>>);

#[derive(Debug)]
pub struct Node<N> {
    pub data: N,
}

#[derive(Debug)]
pub struct Edge<E> {
    pub data: E,
    pub target: NodeIndex,
}

impl<K: GraphKey, N, E> Graph<K, N, E> {
    pub fn new(max_chunks: usize, chunk_size: usize) -> Self {
        Self {
            store: vec![IndexMap::with_capacity(chunk_size)],
            max_chunks,
            max_elements: chunk_size,
            tail: 0,
        }
    }

    pub fn to_index(&self, key: K) -> Option<NodeIndex> {
        self.store
            .iter()
            .enumerate()
            .find_map(|(chunk_index, chunk)| {
                chunk.get_index_of(&key).map(|i| NodeIndex(chunk_index, i))
            })
    }

    pub fn from_index(&self, index: NodeIndex) -> Option<K> {
        Some(*self.store.get(index.0)?.get_index(index.1)?.0)
    }

    fn to_relative(&self, index: usize) -> NodeIndex {
        let a = index % self.max_elements;
        let b = (index - a) / self.chunks();
        let b = (self.tail + b) % self.chunks();
        NodeIndex(b, a)
    }

    fn to_absolute(&self, index: NodeIndex) -> usize {
        (index.0 * self.max_elements) + index.1 - self.tail * self.max_elements
    }

    pub fn chunks(&self) -> usize {
        self.store.len()
    }

    pub fn last(&self) -> Option<NodeIndex> {
        let head = self.head_chunk();
        let len = self.store[head].len();
        if len > 0 {
            Some(NodeIndex(head, len - 1))
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

    pub fn cmp_index(&self, lhs: NodeIndex, rhs: NodeIndex) -> std::cmp::Ordering {
        let lhs = self.to_absolute(lhs);
        let rhs = self.to_absolute(rhs);
        lhs.cmp(&rhs)
    }
}

impl<'a, N, E> NodeTrait for &'a Element<N, E> {
    type Data = &'a N;
    fn data(self) -> Self::Data {
        &self.0.data
    }

    type Edge = &'a Edge<E>;
    type EdgeIterator = EdgeIter<'a, E>;
    fn edges(self) -> Self::EdgeIterator {
        EdgeIter {
            inner: self.1.iter(),
        }
    }
}

impl<'a, E> EdgeTrait for &'a Edge<E> {
    type Data = &'a E;
    type NodeIndex = NodeIndex;

    fn target(self) -> Self::NodeIndex {
        self.target
    }

    fn data(self) -> Self::Data {
        &self.data
    }
}

impl<K: GraphKey, N, E> GraphMetaTrait for Graph<K, N, E> {
    type NodeIndex = NodeIndex;
    type NodeKey = K;
    type NodeData = N;
    type EdgeData = E;
}

impl<'a, K: GraphKey, N, E> GraphTrait for &'a Graph<K, N, E> {
    type K = K;
    type I = NodeIndex;

    type Node = &'a Element<N, E>;
    type Edge = &'a Edge<E>;

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

impl<K: GraphKey, N, E> GraphMutTrait for Graph<K, N, E> {
    fn add_node(&mut self, key: K, data: N) -> Self::NodeIndex {
        if self.store[self.head_chunk()].len() >= self.max_elements {
            if self.chunks() < self.max_chunks {
                self.store.push(IndexMap::with_capacity(self.max_elements));
            } else {
                self.tail = (self.tail + 1) % self.store.len();
                self.store.index_mut(self.head_chunk()).clear();
            }
        }

        let head_chunk = self.head_chunk();

        self.store[head_chunk].insert(key, Element(Node { data }, Vec::default()));

        NodeIndex(head_chunk, self.store[head_chunk].len() - 1)
    }

    fn add_edge(&mut self, a: Self::NodeKey, b: Self::NodeKey, data: Self::EdgeData) {
        let new_index = self.to_index(a).unwrap();

        if let Some(node_edges) = self
            .to_index(b)
            .and_then(|index| self.store.get_mut(index.0).map(|a| &mut a[index.1].1))
        {
            node_edges.push(Edge {
                data,
                target: new_index,
            });
        }
    }
}

impl<'a, K: GraphKey, N, E> Index<&'a Graph<K, N, E>> for NodeIndex {
    fn index(self, graph: &'a Graph<K, N, E>) -> &'a Element<N, E> {
        &graph.store[self.0][self.1]
    }
}

impl<'a, K: GraphKey, N, E> Index<&'a Graph<K, N, E>> for K {
    fn index(self, graph: &'a Graph<K, N, E>) -> &'a Element<N, E> {
        Index::index(graph.to_index(self).unwrap(), graph)
    }
}

pub struct NodeIter<'a, K: GraphKey, N, E> {
    inner: <&'a IndexMap<K, Element<N, E>> as IntoIterator>::IntoIter,
    graph: &'a Graph<K, N, E>,
    chunk: usize,
    item: usize,
}

impl<'a, K: GraphKey, N, E> Iterator for NodeIter<'a, K, N, E> {
    type Item = (NodeIndex, &'a Element<N, E>);

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
                let ret = (NodeIndex(self.chunk, self.item), n);
                self.item += 1;
                ret
            })
    }
}

pub struct EdgeIter<'a, E> {
    inner: <&'a Vec<Edge<E>> as IntoIterator>::IntoIter,
}

impl<'a, E> Iterator for EdgeIter<'a, E> {
    type Item = &'a Edge<E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[test]
fn test_forward_link_single() {
    impl GraphKey for i32 {}

    let mut g = Graph::new(3, 3);

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
    let mut g = Graph::new(3, 3);

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
    graph: &'a Graph<K, N, E>,
}

impl<'a, K: GraphKey, N, E> Iterator for NodeIndexIter<'a, K, N, E> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|i| self.graph.to_relative(i))
    }
}
