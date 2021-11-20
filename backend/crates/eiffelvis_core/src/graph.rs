use std::{cmp::Ordering, ops::Deref};

pub trait GraphKey: std::hash::Hash + Eq + Copy + Clone + Send + Sync + 'static {}
pub trait GraphIndex: std::hash::Hash + Eq + Copy + Clone + Send + Sync + 'static {}

pub trait GraphMeta {
    type NodeIndex: GraphIndex;
    type NodeKey: GraphKey;

    type NodeData;
    type EdgeData;
}

pub trait Index<G>: Copy
where
    G: Graph,
{
    fn index(self, graph: G) -> G::Node;
}

pub trait Graph:
    GraphMeta<NodeIndex = <Self as Graph>::I, NodeKey = <Self as Graph>::K> + Sized + Copy
{
    // TODO: Find a way to not have this
    type I: Index<Self> + GraphIndex;
    type K: Index<Self> + GraphKey;

    type D: Deref<Target = Self::NodeData>;

    type Edge: Edge<NodeIndex = Self::NodeIndex>;
    type Node: Node<Id = Self::NodeIndex, Data = Self::D, Edge = Self::Edge>;

    fn index(self, index: impl Index<Self>) -> Self::Node {
        index.index(self)
    }

    fn cmp_index(self, lhs: Self::NodeIndex, rhs: Self::NodeIndex) -> Ordering;

    type NodeIterator: Iterator<Item = Self::Node>;
    fn nodes(self) -> Self::NodeIterator;
}

pub trait Node: Copy {
    type Id;
    fn id(self) -> Self::Id;

    type Data;
    fn data(self) -> Self::Data;

    type Edge: Edge;
    type EdgeIterator: Iterator<Item = Self::Edge>;
    fn edges(self) -> Self::EdgeIterator;
}

pub trait Edge: Copy {
    type Data;
    type NodeIndex;

    fn data(self) -> Self::Data;
    fn target(self) -> Self::NodeIndex;
}

pub trait GraphMut: GraphMeta {
    fn add_node_with_edges<I>(
        &mut self,
        key: Self::NodeKey,
        data: Self::NodeData,
        edges: I,
    ) -> Self::NodeIndex
    where
        I: Iterator<Item = (Self::NodeKey, Self::EdgeData)>,
    {
        let id = self.add_node(key, data);
        for (target, data) in edges {
            self.add_edge(key, target, data);
        }
        id
    }

    fn add_node(&mut self, key: Self::NodeKey, data: Self::NodeData) -> Self::NodeIndex;
    fn add_edge(&mut self, a: Self::NodeKey, b: Self::NodeKey, data: Self::EdgeData);
}

/// Blanket impls
impl<'a, G> GraphMeta for &'a G
where
    G: GraphMeta,
{
    type NodeIndex = G::NodeIndex;
    type NodeKey = G::NodeKey;
    type NodeData = G::NodeData;
    type EdgeData = G::EdgeData;
}

impl<G> GraphMeta for &mut G
where
    G: GraphMeta,
{
    type NodeIndex = G::NodeIndex;
    type NodeKey = G::NodeKey;
    type NodeData = G::NodeData;
    type EdgeData = G::EdgeData;
}

impl<G> GraphMut for &mut G
where
    G: GraphMut,
{
    fn add_node(&mut self, key: Self::NodeKey, data: Self::NodeData) -> Self::NodeIndex {
        GraphMut::add_node(*self, key, data)
    }

    fn add_edge(&mut self, a: Self::NodeKey, b: Self::NodeKey, data: Self::EdgeData) {
        GraphMut::add_edge(*self, a, b, data)
    }
}
