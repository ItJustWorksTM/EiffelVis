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

    fn find_index(&self, key: K) -> Option<GraphIndex> {
        self.chunks
            .iter()
            .enumerate()
            .find_map(|(chunk_index, chunk)| {
                chunk.get_index_of(&key).map(|i| GraphIndex(chunk_index, i))
            })
    }

    pub fn get(&self, key: K) -> Option<&Node<N, E>> {
        self.find_index(key).and_then(|inner| self.index(inner))
    }

    pub fn index(&self, index: GraphIndex) -> Option<&Node<N, E>> {
        self.chunks
            .get(index.0)
            .and_then(|chunk| chunk.get_index(index.1))
            .map(|(_, n)| n)
    }

    pub fn push(&mut self, key: K, mut edges: Vec<(K, E)>, data: N) -> Option<()> {
        // Dont allow overwriting existing nodes
        if self.find_index(key).is_some() {
            return None;
        }

        let node = Node {
            data,
            edges: edges
                .drain(..)
                // If links dont exist we simply don't store them
                .filter_map(|(uuid, data)| {
                    self.find_index(uuid).map(|target| Edge { data, target })
                })
                .collect(),
        };

        let chunk = {
            if self.chunks[self.head].len() == self.chunk_size {
                if self.chunks.len() != self.max_chunks {
                    self.chunks.push(IndexMap::with_capacity(self.chunk_size));
                }

                self.head = wrap_add(self.head, self.chunks.len());

                // If new head is full it means we wrapped around and we have to start dropping events
                if self.chunks[self.head].len() == self.chunk_size {
                    // Simply drop all events as we dont need them
                    self.chunks[self.head].clear();

                    // Iterate over all events and drop any edge that references the cleared chunk
                    let cleared_page = self.head;
                    self.chunks.iter_mut().for_each(|chunk| {
                        chunk.iter_mut().for_each(|(_, node)| {
                            node.edges.retain(|e| e.target.0 != cleared_page);
                        });
                    });

                    self.tail = wrap_add(self.tail, self.chunks.len());
                }
            }
            &mut self.chunks[self.head]
        };

        chunk.insert(key, node);

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
fn test() {
    let mut g = Graph::new(3, 3);

    g.push(0, vec![], "This is the beginning!").unwrap();

    for i in 1..30 {
        println!("{}:", i);
        g.push(i, vec![(i - 1, format!("targets {}", i))], "more data")
            .unwrap();

        for node in &g {
            println!("{:?}", node);
        }
    }

    println!("{}", g.len());
}
