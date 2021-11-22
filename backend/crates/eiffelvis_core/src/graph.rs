pub trait Key: PartialEq + Eq + Copy + Clone + std::fmt::Debug + std::hash::Hash + Send {}
pub trait Idx: PartialEq + Eq + Copy + Clone + std::fmt::Debug + std::hash::Hash + Send {}
impl<T> Idx for T where T: PartialEq + Eq + Copy + Clone + std::fmt::Debug + std::hash::Hash + Send {}

pub trait Meta {
    type Idx: Idx;
    type Key: Key;
    type Data;
    type EdgeData;
}

pub trait Graph:
    Meta + ItemIter + Indexable<<Self as Meta>::Idx> + Indexable<<Self as Meta>::Key>
{
    fn cmp_index(&self, lhs: Self::Idx, rhs: Self::Idx) -> std::cmp::Ordering;

    fn node_count(&self) -> usize;

    fn add_node(&mut self, key: Self::Key, data: Self::Data) -> Option<Self::Idx>;

    fn add_edge(&mut self, a: Self::Key, b: Self::Key, data: Self::EdgeData);

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

    fn data(&self) -> &'a Self::Data;

    fn id(&self) -> Self::Idx;

    type EdgeItem: ItemEdge<'a, EdgeData = Self::EdgeData, Idx = Self::Idx>;
    type EdgeIterator: Iterator<Item = Self::EdgeItem>;

    fn edges(&self) -> Self::EdgeIterator;
}

pub trait ItemEdge<'a>
where
    Self::EdgeData: 'a,
{
    type EdgeData;
    type Idx;

    fn data(&self) -> &'a Self::EdgeData;
    fn target(&self) -> Self::Idx;
}

// TODO: slightly cursed
pub trait HasNodeIter<'a, T, _Outlives = &'a Self> {
    type NodeIterType: Iterator<Item = T>;
}
pub type NodeIterType<'a, This> = <This as HasNodeIter<'a, NodeType<'a, This>>>::NodeIterType;

pub trait ItemIter: for<'a> HasNodeIter<'a, NodeType<'a, Self>> + for<'a> HasNode<'a> {
    fn items(&self) -> NodeIterType<'_, Self>;
}

pub trait Indexable<T>: for<'a> HasNode<'a> {
    fn get(&self, id: T) -> Option<NodeType<'_, Self>>;
    fn index(&self, id: T) -> NodeType<'_, Self> {
        self.get(id).unwrap()
    }
}
