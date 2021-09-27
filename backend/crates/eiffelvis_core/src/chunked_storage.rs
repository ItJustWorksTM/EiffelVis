#![allow(dead_code)]

// Proof of concept Chunked storage as described in https://github.com/ItJustWorksTM/EiffelVis/issues/8

use std::collections::HashMap;

type Uuid = u8;
type StorageIndex = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GraphIndex(usize, usize);

#[derive(Debug)]
struct Edge {
    data: (),
    target: GraphIndex,
}

#[derive(Debug)]
struct Node {
    uuid: Uuid,
    data: (),
    edges: Vec<Edge>,
}

#[derive(Debug)]
struct GraphStorage {
    uuids: HashMap<Uuid, StorageIndex>,
    events: Vec<Node>,
    max_len: usize,
}

impl GraphStorage {
    fn new(max_len: usize) -> Self {
        Self {
            uuids: {
                let mut map = HashMap::new();
                map.reserve(max_len);
                map
            },
            events: Vec::with_capacity(max_len),
            max_len,
        }
    }

    fn is_full(&self) -> bool {
        self.events.len() == self.max_len
    }
}

#[derive(Debug)]
struct Graph {
    chunks: Vec<GraphStorage>,
    max_chunks: usize,
    chunk_size: usize,
    head: usize,
    tail: usize,
}

impl Graph {
    fn new(max_chunks: usize, chunk_size: usize) -> Self {
        Self {
            chunks: vec![GraphStorage::new(chunk_size)],
            max_chunks,
            chunk_size,
            head: 0,
            tail: 0,
        }
    }

    fn find_index(&self, key: Uuid) -> Option<GraphIndex> {
        self.chunks
            .iter()
            .enumerate()
            .find_map(|(chunk_index, chunk)| {
                chunk
                    .uuids
                    .get(&key)
                    .copied()
                    .map(|a| GraphIndex(chunk_index, a))
            })
    }

    fn push_node(&mut self, node: Node) {
        let chunk = {
            if self.chunks[self.head].is_full() {
                if self.chunks.len() != self.max_chunks {
                    self.chunks.push(GraphStorage::new(self.chunk_size));
                    println!("adding new storage!");
                }

                // Wrap around
                self.head = wrap_add(self.head, self.chunks.len());
                println!("head: {}", self.head);

                // If new head is full it means we wrapped around and we have to start dropping events
                if self.chunks[self.head].is_full() {
                    // Simply drop all events as we dont need them
                    self.chunks[self.head].uuids.clear();
                    self.chunks[self.head].events.clear();

                    // Iterate over all events and drop any edge that references the cleared chunk
                    let cleared_page = self.head;
                    self.chunks.iter_mut().for_each(|chunk| {
                        chunk.events.iter_mut().for_each(|ev| {
                            // TODO: dont drain, instead swap remove to reuse the buffer
                            ev.edges = ev
                                .edges
                                .drain(..)
                                .filter(|e| e.target.0 != cleared_page)
                                .collect();
                        });
                    });

                    self.tail = wrap_add(self.tail, self.chunks.len());
                    println!("tail: {}", self.head);
                }
            }
            &mut self.chunks[self.head]
        };
        chunk.uuids.insert(node.uuid, chunk.events.len());
        chunk.events.push(node);
    }

    fn index_mut(&mut self, index: GraphIndex) -> Option<&mut Node> {
        self.chunks
            .get_mut(index.0)
            .and_then(|chunk| chunk.events.get_mut(index.1))
    }

    pub fn index(&mut self, index: GraphIndex) -> Option<&Node> {
        self.chunks
            .get(index.0)
            .and_then(|chunk| chunk.events.get(index.1))
    }

    pub fn push(&mut self, key: Uuid, edges: &[Uuid], data: ()) -> Option<()> {
        // Dont allow overwriting existing nodes
        if self.find_index(key).is_some() {
            return None;
        }

        let node = Node {
            uuid: key,
            data,
            edges: edges
                .iter()
                // If links dont exist we simply don't store them
                .filter_map(|uuid| {
                    self.find_index(*uuid)
                        .map(|target| Edge { data: (), target })
                })
                .collect(),
        };

        self.push_node(node);

        Some(())
    }

    // Returns an iterator that yields nodes in the order they have been inserted
    pub fn iter(&self) -> GraphIterator<'_> {
        GraphIterator {
            inner: self.chunks[self.tail].events.iter(),
            graph: self,
            chunk: self.tail,
        }
    }
}

struct GraphIterator<'a> {
    inner: <&'a Vec<Node> as IntoIterator>::IntoIter,
    graph: &'a Graph,
    chunk: usize,
}

impl<'a> Iterator for GraphIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.inner.next();
        if next.is_none() {
            if self.chunk == self.graph.head {
                return None;
            }
            self.chunk = wrap_add(self.chunk, self.graph.chunks.len());
            self.inner = self.graph.chunks[self.chunk].events.iter();
            next = self.inner.next();
        }
        next
    }
}

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

    g.push(0, &[], ()).unwrap();

    for i in 1..30 {
        println!("{}:", i);
        g.push(i, &[i - 1], ()).unwrap();
        g.iter().for_each(|node| println!("{:?}", node));
    }
}
