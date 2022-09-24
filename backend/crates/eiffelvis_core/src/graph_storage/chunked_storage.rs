use crate::graph;
use ahash::RandomState;
use indexmap::IndexMap;
use std::{
    default::Default,
    hash::Hash,
    iter::Take,
    ops::{Bound, IndexMut, Range, RangeBounds},
};

#[derive(Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct ChunkedIndex(u64);

impl ChunkedIndex {
    pub fn new(gen: u32, idx: u32) -> ChunkedIndex {
        ChunkedIndex(((gen as u64) << 32) | idx as u64)
    }

    pub fn gen(self) -> u32 {
        (self.0 >> 32) as u32
    }

    pub fn idx(self) -> u32 {
        (self.0 & 0xFFFFFFFF) as u32
    }
}

impl std::fmt::Debug for ChunkedIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ChunkedIndex")
            .field(&self.gen())
            .field(&self.idx())
            .finish()
    }
}

type ChunkMap<K, N, E> = IndexMap<K, Element<N, E>, RandomState>;
type ChunkIndexIter = <Range<u32> as IntoIterator>::IntoIter;
type ChunkElementIter<'a, K, N, E> = <&'a ChunkMap<K, N, E> as IntoIterator>::IntoIter;

#[derive(Debug)]
pub struct ChunkedGraph<K: graph::Key, N, E> {
    store: Vec<ChunkMap<K, N, E>>,
    newest_generation: u32,
    max_chunks_pow: usize,
    max_elements_pow: u32,
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
    fn to_chunk_index(&self, generation: u32) -> usize {
        generation as usize & !(!0 << self.max_chunks_pow)
    }

    fn to_generation(&self, chunk_index: usize) -> u32 {
        let chunk_index = self.to_chunk_index(chunk_index.wrapping_sub(self.tail) as u32);
        self.newest_generation - (self.chunks() - 1 - chunk_index) as u32
    }

    fn max_elements(&self) -> usize {
        (1 << self.max_elements_pow) as usize
    }

    fn max_chunks(&self) -> usize {
        1 << self.max_chunks_pow
    }

    pub fn new(max_chunks: usize, chunk_size: u32) -> Self {
        if max_chunks.count_ones() != 1 {
            panic!("Chunk count must be a power of two ({} isn't)", max_chunks);
        }
        if chunk_size.count_ones() != 1 {
            panic!("Chunk size must be a power of two ({} isn't)", chunk_size);
        }
        Self {
            store: vec![ChunkMap::with_capacity_and_hasher(
                chunk_size as usize,
                Default::default(),
            )],
            newest_generation: 0,
            max_chunks_pow: max_chunks.trailing_zeros() as usize,
            max_elements_pow: chunk_size.trailing_zeros(),
            tail: 0,
        }
    }

    fn add_node(&mut self, key: K, data: N) -> ChunkedIndex {
        if self.store[self.head_chunk()].len() >= self.max_elements() {
            self.newest_generation += 1;
            if self.chunks() < self.max_chunks() {
                self.store.push(ChunkMap::with_capacity_and_hasher(
                    self.max_elements(),
                    Default::default(),
                ));
            } else {
                self.tail = self.tail_chunk();
                self.store.index_mut(self.head_chunk()).clear();
            }
        }

        let head_chunk = self.head_chunk();

        self.store[head_chunk].insert(key, Element(NodeData { data }, Vec::default()));

        ChunkedIndex::new(
            self.newest_generation,
            (self.store[head_chunk].len() - 1) as u32,
        )
    }

    fn add_edge(&mut self, a: K, b: K, data: E) {
        let new_index = self.to_index(a).unwrap();

        if let Some(node_edges) = self
            .to_index(b)
            .filter(|index| {
                index.gen() >= (self.newest_generation as u32 - (self.chunks() - 1) as u32)
            })
            .and_then(|index| {
                let chunk_index = self.to_chunk_index(index.gen());
                self.store
                    .get_mut(chunk_index)
                    .map(|a| &mut a[index.idx() as usize].1)
            })
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
                    .map(|i| ChunkedIndex::new(self.to_generation(chunk_index), i as u32))
            })
    }

    /// Translates given internal index to key
    pub fn from_index(&self, index: ChunkedIndex) -> Option<K> {
        Some(
            *self
                .store
                .get(self.to_chunk_index(index.gen()))?
                .get_index(index.idx() as usize)?
                .0,
        )
    }

    pub fn chunks(&self) -> usize {
        self.store.len()
    }

    /// Returns the index of the most recent added node
    pub fn last(&self) -> Option<ChunkedIndex> {
        let len = self.store[self.head_chunk()].len();
        if len > 0 {
            Some(ChunkedIndex::new(self.newest_generation, (len - 1) as u32))
        } else {
            None
        }
    }

    fn head_chunk(&self) -> usize {
        self.to_chunk_index(self.newest_generation)
    }

    fn tail_chunk(&self) -> usize {
        self.to_chunk_index(self.oldest_generation())
    }

    fn oldest_generation(&self) -> u32 {
        self.newest_generation - ((self.chunks() as u32) - 1)
    }

    fn abs_to_index(&self, id: usize) -> ChunkedIndex {
        ChunkedIndex::new(
            self.to_generation((id & !(self.max_elements() - 1)) >> self.max_elements_pow),
            (id & (self.max_elements() - 1)) as u32,
        )
    }

    fn index_to_abs(&self, id: ChunkedIndex) -> usize {
        self.to_chunk_index(id.gen()) * self.max_elements() + id.idx() as usize
    }

    pub fn node_count(&self) -> usize {
        (self.chunks() - 1) * self.max_elements() + self.store[self.head_chunk()].len()
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

impl<'a, K: graph::Key, N, E> graph::HasNodeIter<'a> for ChunkedGraph<K, N, E> {
    type Item = <NodeIter<'a, K, N, E> as Iterator>::Item;
    type NodeIterType = NodeIter<'a, K, N, E>;
}

impl<'a, K: graph::Key, N, E> graph::HasNodeRangeIter<'a> for ChunkedGraph<K, N, E> {
    type Item = <NodeIter<'a, K, N, E> as Iterator>::Item;
    type NodeRangeIterType = Take<NodeIter<'a, K, N, E>>;
}

impl<K: graph::Key, N, E> graph::ItemIter for ChunkedGraph<K, N, E> {
    fn items(&self) -> graph::NodeIterType<'_, Self> {
        NodeIter {
            chunks: (self.oldest_generation()..self.newest_generation + 1),
            inner: None,
            generation: self.oldest_generation(),
            graph: self,
            item: 0,
        }
    }

    fn range<R>(&self, range: R) -> graph::NodeRangeIterType<'_, Self>
    where
        R: RangeBounds<Self::Idx>,
    {
        let begin = match range.start_bound() {
            Bound::Included(&inc) => inc,
            Bound::Excluded(&inc) => self.abs_to_index(self.index_to_abs(inc) + 1),
            Bound::Unbounded => ChunkedIndex::new(self.oldest_generation(), 0),
        };

        let end = match range.end_bound() {
            Bound::Included(&inc) => self.abs_to_index(self.index_to_abs(inc) + 1),
            Bound::Excluded(&inc) => inc,
            Bound::Unbounded => ChunkedIndex::new(
                self.newest_generation,
                self.store[self.head_chunk()].len() as u32,
            ),
        };

        let mut chunks = begin.gen()..end.gen() + 1;

        // Partially consume the first iterator as IndexMap does not support this natively
        let inner = chunks.next().map(|idx| {
            let mut iter = self.store[self.to_chunk_index(idx) as usize].iter();
            iter.by_ref().take(begin.idx() as usize).count();
            iter
        });

        let total = ((end.gen() - begin.gen()) as usize + 1) * self.max_elements()
            - (((self.max_elements() as u32 - end.idx()) + begin.idx()) as usize);

        NodeIter {
            chunks,
            inner,
            generation: begin.gen(),
            graph: self,
            item: begin.idx(),
        }
        .take(total)
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
        self.store
            .get(self.to_chunk_index(index.gen()))
            .and_then(|m| m.get_index(index.idx() as usize))
            .map(|el| (index, el.1))
    }
}

impl<K: graph::Key, N, E> graph::Indexable<K> for ChunkedGraph<K, N, E> {
    fn get(&self, index: K) -> Option<graph::NodeType<'_, Self>> {
        self.to_index(index).map(|i| self.index(i))
    }
}

impl<K: graph::Key, N, E> graph::Indexable<usize> for ChunkedGraph<K, N, E> {
    /// Indexes the graph as if it was a linear storage like [Vec]
    fn get(&self, index: usize) -> Option<graph::NodeType<'_, Self>> {
        if index < self.node_count() {
            self.get(self.abs_to_index(index))
        } else {
            None
        }
    }
}

pub struct NodeIter<'a, K: graph::Key, N, E> {
    chunks: ChunkIndexIter,
    inner: Option<ChunkElementIter<'a, K, N, E>>,
    graph: &'a ChunkedGraph<K, N, E>,
    generation: u32,
    item: u32,
}

impl<'a, K: graph::Key, N, E> Iterator for NodeIter<'a, K, N, E> {
    type Item = (ChunkedIndex, &'a Element<N, E>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((_, n)) = self.inner.as_mut().and_then(Iterator::next) {
                let ret = Some((ChunkedIndex::new(self.generation, self.item), n));
                self.item += 1;
                return ret;
            }

            self.inner = Some(self.chunks.next().map(|id| {
                self.generation = id;
                self.item = 0;
                self.graph.store[self.graph.to_chunk_index(id)].iter()
            })?);
        }
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
            assert_eq!(g.abs_to_index(g.index_to_abs(no.id())), no.id());
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
        for i in 1..15i32 {
            assert_eq!(
                g.index((i - 1) as i32)
                    .edges()
                    .find_map(|i| g.from_index(i.target())),
                Some(i as i32)
            );
        }
    }
}
