use ahash::RandomState;
use indexmap::IndexMap;
use indexmap::{map::Iter as IndexMapIter, Equivalent};
use std::{
    default::Default,
    hash::{BuildHasher, Hash},
    iter::Take,
    ops::{Bound, Index, IndexMut, Range, RangeBounds},
};

pub struct ChunkedIndexMap<K, V, S = RandomState> {
    inner: Vec<IndexMap<K, V, S>>,
    head_generation: u32,
    max_chunks_pow: usize,
    max_elements_pow: u32,
    tail: usize,
}

impl<K, V> ChunkedIndexMap<K, V> {
    pub fn new(max_chunks: usize, chunk_size: u32) -> Self {
        if max_chunks.count_ones() != 1 {
            panic!("Chunk count must be a power of two ({} isn't)", max_chunks);
        }
        if chunk_size.count_ones() != 1 {
            panic!("Chunk size must be a power of two ({} isn't)", chunk_size);
        }
        Self {
            inner: vec![IndexMap::with_capacity_and_hasher(
                chunk_size as usize,
                Default::default(),
            )],
            head_generation: 0,
            max_chunks_pow: max_chunks.trailing_zeros() as usize,
            max_elements_pow: chunk_size.trailing_zeros(),
            tail: 0,
        }
    }
}

impl<K, V, S> ChunkedIndexMap<K, V, S> {
    pub fn insert(&mut self, key: K, value: V) -> ChunkedIndex
    where
        S: BuildHasher + Default,
        K: Hash + Eq,
    {
        if self.head().len() >= self.max_elements() {
            self.head_generation += 1;
            if self.chunks() < self.max_chunks() {
                self.inner.push(IndexMap::with_capacity_and_hasher(
                    self.max_elements(),
                    Default::default(),
                ));
            } else {
                self.tail = self.tail_chunk();
                self.inner.index_mut(self.head_chunk()).clear();
            }
        }

        let el = {
            let head = self.head_mut();
            head.insert(key, value);
            head.len()
        };

        ChunkedIndex::new(self.head_generation, el as u32 - 1)
    }

    pub fn get_full<Q>(&self, key: &Q) -> Option<(ChunkedIndex, &K, &V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
        K: Hash + Eq,
        S: BuildHasher,
    {
        self.get_index_of(key)
            .and_then(|ix| self.get_index(ix).zip(Some(ix)))
            .map(|((k, v), ix)| (ix, k, v))
    }

    pub fn get_index(&self, index: ChunkedIndex) -> Option<(&K, &V)> {
        self.inner
            .get(self.generation_to_chunk(index.gen()))
            .and_then(|m| m.get_index(index.idx() as usize))
    }

    pub fn get_index_mut(&mut self, index: ChunkedIndex) -> Option<(&K, &mut V)> {
        let chunk = self.generation_to_chunk(index.gen());
        self.inner
            .get_mut(chunk)
            .and_then(|m| m.get_index_mut(index.idx() as usize))
            .map(|(k,v)|(&*k, v))
    }

    pub fn get_index_of<Q>(&self, key: &Q) -> Option<ChunkedIndex>
    where
        Q: ?Sized + Hash + Equivalent<K>,
        K: Hash + Eq,
        S: BuildHasher,
    {
        self.inner
            .iter()
            .enumerate()
            .find_map(|(chunk_index, chunk)| {
                chunk
                    .get_index_of(key)
                    .map(|i| ChunkedIndex::new(self.chunk_to_generation(chunk_index), i as u32))
            })
    }

    pub fn get_index_of_linear(&self, index: usize) -> Option<ChunkedIndex> {
        (index < self.len()).then(|| self.abs_to_index(index))
    }

    pub fn iter(&self) -> Iter<'_, K, V, S> {
        self.iter_range(..)
    }

    pub fn iter_range<R>(&self, range: R) -> Iter<'_, K, V, S>
    where
        R: RangeBounds<ChunkedIndex>,
    {
        let begin = match range.start_bound() {
            Bound::Included(&inc) => inc,
            Bound::Excluded(&inc) => self.abs_to_index(self.index_to_abs(inc) + 1),
            Bound::Unbounded => ChunkedIndex::new(self.tail_generation(), 0),
        };

        let end = match range.end_bound() {
            Bound::Included(&inc) => self.abs_to_index(self.index_to_abs(inc) + 1),
            Bound::Excluded(&inc) => inc,
            Bound::Unbounded => ChunkedIndex::new(self.head_generation, self.head().len() as u32),
        };

        let mut chunks = begin.gen()..end.gen() + 1;

        // Partially consume the first iterator as IndexMap does not support this natively
        let inner = chunks.next().map(|idx| {
            let mut iter = self.inner[self.generation_to_chunk(idx) as usize].iter();
            iter.by_ref().take(begin.idx() as usize).count();
            iter
        });

        let total = ((end.gen() - begin.gen()) as usize + 1) * self.max_elements()
            - (((self.max_elements() as u32 - end.idx()) + begin.idx()) as usize);

        _Iter {
            chunks,
            inner,
            generation: begin.gen(),
            graph: self,
            item: begin.idx(),
        }
        .take(total)
    }

    pub fn len(&self) -> usize {
        (self.chunks() - 1) * self.max_elements() + self.head().len()
    }

    pub fn chunks(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<K, V, S> ChunkedIndexMap<K, V, S> {
    fn head(&self) -> &IndexMap<K, V, S> {
        self.inner.index(self.head_chunk())
    }

    fn head_mut(&mut self) -> &mut IndexMap<K, V, S> {
        self.inner.index_mut(self.head_chunk())
    }

    fn head_chunk(&self) -> usize {
        self.generation_to_chunk(self.head_generation)
    }

    fn tail_chunk(&self) -> usize {
        self.generation_to_chunk(self.tail_generation())
    }

    pub fn head_generation(&self) -> u32 {
        self.head_generation
    }

    pub fn tail_generation(&self) -> u32 {
        self.head_generation - ((self.chunks() as u32) - 1)
    }

    fn generation_to_chunk(&self, generation: u32) -> usize {
        generation as usize & !(!0 << self.max_chunks_pow)
    }

    fn chunk_to_generation(&self, chunk_index: usize) -> u32 {
        let chunk_index = self.generation_to_chunk(chunk_index.wrapping_sub(self.tail) as u32);
        self.head_generation - (self.chunks() - 1 - chunk_index) as u32
    }

    fn max_elements(&self) -> usize {
        (1 << self.max_elements_pow) as usize
    }

    fn max_chunks(&self) -> usize {
        1 << self.max_chunks_pow
    }

    fn abs_to_index(&self, id: usize) -> ChunkedIndex {
        ChunkedIndex::new(
            self.chunk_to_generation((id & !(self.max_elements() - 1)) >> self.max_elements_pow),
            (id & (self.max_elements() - 1)) as u32,
        )
    }

    fn index_to_abs(&self, id: ChunkedIndex) -> usize {
        self.generation_to_chunk(id.gen()) * self.max_elements() + id.idx() as usize
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct ChunkedIndex(u32, u32);

impl ChunkedIndex {
    pub fn new(gen: u32, idx: u32) -> ChunkedIndex {
        ChunkedIndex(gen, idx)
    }

    pub fn gen(self) -> u32 {
        self.0
    }

    pub fn idx(self) -> u32 {
        self.1
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

type RangeIter<T> = <Range<T> as IntoIterator>::IntoIter;

pub struct _Iter<'a, K, V, S> {
    chunks: RangeIter<u32>,
    inner: Option<IndexMapIter<'a, K, V>>,
    graph: &'a ChunkedIndexMap<K, V, S>,
    generation: u32,
    item: u32,
}

impl<'a, K, V, S> Iterator for _Iter<'a, K, V, S> {
    type Item = (ChunkedIndex, &'a V);

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
                self.graph.inner[self.graph.generation_to_chunk(id)].iter()
            })?);
        }
    }
}

pub type Iter<'a, K, V, S> = Take<_Iter<'a, K, V, S>>;
