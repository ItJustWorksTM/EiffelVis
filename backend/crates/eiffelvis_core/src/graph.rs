use std::cmp::Ordering;

pub trait Key: PartialEq + Eq + Copy + Clone + std::fmt::Debug + std::hash::Hash + Send {}
pub trait Idx: PartialEq + Eq + Copy + Clone + std::fmt::Debug + std::hash::Hash + Send {}
impl<T> Idx for T where T: PartialEq + Eq + Copy + Clone + std::fmt::Debug + std::hash::Hash + Send {}

pub trait ValueIndex<Idx>: Sized {
    type Output;

    fn index(self, idx: Idx) -> Self::Output {
        self.try_index(idx).unwrap()
    }

    fn try_index(self, idx: Idx) -> Option<Self::Output>;
}

pub trait Meta {
    type Data;
    type EdgeData;
    type Idx: Idx;
    type Key: Key;
}

pub trait Ref<'a>:
    Copy
    + ValueIndex<<Self::Meta as Meta>::Idx, Output = Self::Item>
    + ValueIndex<<Self::Meta as Meta>::Key, Output = Self::Item>
where
    Self::Meta: 'a,
{
    type Meta: Meta;

    fn cmp_index(self, lhs: <Self::Meta as Meta>::Idx, rhs: <Self::Meta as Meta>::Idx) -> Ordering;

    type Item: Item<
        'a,
        Idx = <Self::Meta as Meta>::Idx,
        Data = <Self::Meta as Meta>::Data,
        EdgeData = <Self::Meta as Meta>::EdgeData,
    >;
    type ItemIterator: Iterator<Item = Self::Item>;
    fn items(self) -> Self::ItemIterator;
}

pub trait Mut {
    type Meta: Meta;

    fn add_node(
        &mut self,
        key: <Self::Meta as Meta>::Key,
        data: <Self::Meta as Meta>::Data,
    ) -> Option<<Self::Meta as Meta>::Idx>;

    fn add_edge(
        &mut self,
        a: <Self::Meta as Meta>::Key,
        b: <Self::Meta as Meta>::Key,
        data: <Self::Meta as Meta>::EdgeData,
    );

    fn add_node_with_edges<I>(
        &mut self,
        key: <Self::Meta as Meta>::Key,
        data: <Self::Meta as Meta>::Data,
        edges: I,
    ) -> Option<<Self::Meta as Meta>::Idx>
    where
        I: Iterator<Item = (<Self::Meta as Meta>::Key, <Self::Meta as Meta>::EdgeData)>,
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
