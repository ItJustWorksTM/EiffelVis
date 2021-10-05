use indexmap::IndexMap;
use std::{hash::Hash, iter::FusedIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphIndex(usize, usize);

pub trait GraphKey: Hash + Eq + Copy + Clone {}
impl<T: Hash + Eq + Copy + Clone> GraphKey for T {}

#[derive(Debug)]
pub struct Edge<T> {
    pub data: T,
    pub target: GraphIndex,
}

#[derive(Debug)]
pub struct Node<T, E> {
    pub data: T,
    pub edges: Vec<Edge<E>>,
}

#[derive(Debug)]
pub struct Graph<K: GraphKey, N, E> {
    chunks: Vec<IndexMap<K, Node<N, E>>>,
    max_chunks: usize,
    chunk_size: usize,
    head: usize,
    tail: usize,
}

impl<K: GraphKey, N, E> Graph<K, N, E> {
    pub fn new(max_chunks: usize, chunk_size: usize) -> Self {
        Self {
            chunks: vec![IndexMap::with_capacity(chunk_size)],
            max_chunks,
            chunk_size,
            head: 0,
            tail: 0,
        }
    }

    pub fn find_index(&self, key: K) -> Option<GraphIndex> {
        self.chunks
            .iter()
            .enumerate()
            .find_map(|(chunk_index, chunk)| {
                chunk.get_index_of(&key).map(|i| GraphIndex(chunk_index, i))
            })
    }

    #[allow(dead_code)]
    fn find_key(&self, index: GraphIndex) -> Option<K> {
        self.chunks
            .get(index.0)
            .and_then(|f| f.get_index(index.1).map(|(k, _)| *k))
    }

    pub fn get(&self, key: K) -> Option<&Node<N, E>> {
        self.find_index(key).and_then(|inner| self.index(inner))
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut Node<N, E>> {
        self.find_index(key)
            .and_then(move |inner| self.index_mut(inner))
    }

    pub fn index(&self, index: GraphIndex) -> Option<&Node<N, E>> {
        self.chunks
            .get(index.0)
            .and_then(|chunk| chunk.get_index(index.1))
            .map(|(_, n)| n)
    }

    pub fn index_mut(&mut self, index: GraphIndex) -> Option<&mut Node<N, E>> {
        self.chunks
            .get_mut(index.0)
            .and_then(|chunk| chunk.get_index_mut(index.1))
            .map(|(_, n)| n)
    }

    pub fn push(&mut self, key: K, edges: Vec<(K, E)>, data: N) -> Option<()> {
        // Dont allow overwriting existing nodes
        if self.find_index(key).is_some() {
            return None;
        }

        // make space for the new node
        if self.chunks[self.head].len() == self.chunk_size {
            if self.chunks.len() != self.max_chunks {
                self.chunks.push(IndexMap::with_capacity(self.chunk_size));
            }

            self.head = wrap_add(self.head, self.chunks.len());

            // If new head is full it means we wrapped around and we have to start dropping events
            if self.chunks[self.head].len() == self.chunk_size {
                // Simply drop all events as we dont need them
                self.chunks[self.head].clear();

                self.tail = wrap_add(self.tail, self.chunks.len());
            }
        }

        let new_index = GraphIndex(self.head, self.chunks[self.head].len());

        // Insert new edges on nodes
        for (edge_key, data) in edges {
            if let Some(node) = self.get_mut(edge_key) {
                node.edges.push(Edge {
                    data,
                    target: new_index,
                });
            }
        }

        self.chunks[self.head].insert(
            key,
            Node {
                data,
                edges: Vec::default(),
            },
        );

        Some(())
    }

    pub fn len(&self) -> usize {
        self.chunks.iter().fold(0, |total, c| total + c.len())
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // Returns an iterator that yields nodes in the order they have been inserted
    pub fn iter(&self) -> GraphIterator<'_, K, N, E> {
        GraphIterator {
            inner: self.chunks[self.tail].iter(),
            graph: self,
            chunk: self.tail,
        }
    }
}

impl<'a, K: GraphKey, N, E> IntoIterator for &'a Graph<K, N, E> {
    type IntoIter = GraphIterator<'a, K, N, E>;
    type Item = &'a Node<N, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct GraphIterator<'a, K: GraphKey, N, E> {
    inner: <&'a IndexMap<K, Node<N, E>> as IntoIterator>::IntoIter,
    graph: &'a Graph<K, N, E>,
    chunk: usize,
}

impl<'a, K: GraphKey, N, E> Iterator for GraphIterator<'a, K, N, E> {
    type Item = &'a Node<N, E>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.inner.next();
        if next.is_none() {
            if self.chunk == self.graph.head {
                return None;
            }
            self.chunk = wrap_add(self.chunk, self.graph.chunks.len());
            self.inner = self.graph.chunks[self.chunk].iter();
            next = self.inner.next();
        }
        next.map(|(_, n)| n)
    }
}

impl<K: GraphKey, N, E> FusedIterator for GraphIterator<'_, K, N, E> {}

fn wrap_add(mut val: usize, max: usize) -> usize {
    val += 1;
    if val == max {
        val = 0;
    }
    val
}

#[test]
fn test_forward_link_single() {
    let mut g = Graph::new(3, 3);

    g.push(0, vec![], "This is the beginning!").unwrap();

    for i in 1..9 {
        g.push(i, vec![(0, format!("targets {}", i))], "more data")
            .unwrap();
    }

    // All nodes should now have links to the first node (which are stored on the first node itself)
    let zeroth = g.get(0).unwrap();
    assert_eq!(zeroth.edges.len(), 8);
}

#[test]
fn test_forward_link_many() {
    let mut g = Graph::new(3, 3);

    g.push(0, vec![], "");

    for i in 1..9 {
        g.push(i, vec![(i - 1, "")], "").unwrap();
    }

    // Now i - 1 node should store an edge to node i
    // 0 -> 1, 1 -> 2 ...
    for i in 1..8 {
        assert_eq!(
            g.find_key(g.get(i - 1).unwrap().edges[0].target).unwrap(),
            i
        );
    }
}
