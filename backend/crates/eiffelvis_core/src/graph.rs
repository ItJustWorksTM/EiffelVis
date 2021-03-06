use std::ops::RangeBounds;

pub trait Key: PartialEq + Eq + Copy + Clone + std::fmt::Debug + std::hash::Hash + Send {}
pub trait Idx:
    PartialEq + Eq + PartialOrd + Ord + Copy + Clone + std::fmt::Debug + std::hash::Hash + Send
{
}
impl<T> Idx for T where
    T: PartialEq + PartialOrd + Ord + Eq + Copy + Clone + std::fmt::Debug + std::hash::Hash + Send
{
}

/// Meta graph trait, defines the base types of a graph such as key and data types
pub trait Meta {
    type Idx: Idx;
    type Key: Key;
    type Data;
    type EdgeData;
}

/// Main graph trait
pub trait Graph:
    Meta + ItemIter + Indexable<<Self as Meta>::Idx> + Indexable<<Self as Meta>::Key>
{
    /// Returns the total amount of nodes this graph holds
    fn node_count(&self) -> usize;

    /// Creates a new edge with given data without any edges
    fn add_node(&mut self, key: Self::Key, data: Self::Data) -> Option<Self::Idx>;

    /// Creates a new edge between a and b with given edge data
    fn add_edge(&mut self, a: Self::Key, b: Self::Key, data: Self::EdgeData);

    /// Convenience function, implemented in terms of [Graph::add_node] and [Graph::add_edge]
    fn add_node_with_edges<I>(
        &mut self,
        key: Self::Key,
        data: Self::Data,
        edges: I,
    ) -> Option<Self::Idx>
    where
        I: Iterator<Item = (Self::Key, Self::EdgeData)>,
    {
        let id = self.add_node(key, data);
        if id.is_some() {
            for (target, data) in edges {
                self.add_edge(key, target, data);
            }
        }
        id
    }
}

pub trait HasNode<'a, _Outlives = &'a Self>: Meta {
    type NodeType: Item<
        'a,
        Data = <Self as Meta>::Data,
        EdgeData = <Self as Meta>::EdgeData,
        Idx = <Self as Meta>::Idx,
    >;
}
pub type NodeType<'a, This> = <This as HasNode<'a>>::NodeType;

pub trait Item<'a>
where
    Self::Data: 'a,
    Self::EdgeData: 'a,
{
    type Data;
    type EdgeData;
    type Idx;

    /// Returns the data associated with this node
    fn data(&self) -> &'a Self::Data;

    /// Returns the id associated with this node
    fn id(&self) -> Self::Idx;

    type EdgeItem: ItemEdge<'a, EdgeData = Self::EdgeData, Idx = Self::Idx>;
    type EdgeIterator: Iterator<Item = Self::EdgeItem>;

    /// Returns an iterator over the edges this node has
    fn edges(&self) -> Self::EdgeIterator;
}

pub trait ItemEdge<'a>
where
    Self::EdgeData: 'a,
{
    type EdgeData;
    type Idx;

    /// Returns the data associated with this edge
    fn data(&self) -> &'a Self::EdgeData;

    /// Returns the index of the node this edge targets
    fn target(&self) -> Self::Idx;
}

pub trait HasNodeIter<'a, _Outlives = &'a Self> {
    type Item;
    type NodeIterType: Iterator<Item = Self::Item>;
}
pub type NodeIterType<'a, This> = <This as HasNodeIter<'a>>::NodeIterType;

pub trait HasNodeRangeIter<'a, _Outlives = &'a Self> {
    type Item;
    type NodeRangeIterType: Iterator<Item = Self::Item>;
}
pub type NodeRangeIterType<'a, This> = <This as HasNodeRangeIter<'a>>::NodeRangeIterType;

pub trait ItemIter:
    for<'a> HasNodeRangeIter<'a, Item = NodeType<'a, Self>>
    + for<'a> HasNodeIter<'a, Item = NodeType<'a, Self>>
    + for<'a> HasNode<'a>
{
    fn items(&self) -> NodeIterType<'_, Self>;

    fn range<R>(&self, range: R) -> NodeRangeIterType<'_, Self>
    where
        R: RangeBounds<Self::Idx>;
}

pub trait Indexable<T>: for<'a> HasNode<'a> {
    fn get(&self, id: T) -> Option<NodeType<'_, Self>>;
    fn index(&self, id: T) -> NodeType<'_, Self> {
        self.get(id).unwrap()
    }
}
