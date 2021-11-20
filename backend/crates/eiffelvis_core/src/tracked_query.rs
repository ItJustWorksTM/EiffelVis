use crate::{
    graph::{Graph, GraphMeta, Index, Node},
    query::GraphQuery,
};

pub struct TrackedNodes<I> {
    cursor: Option<I>,
}

impl<I> TrackedNodes<I> {
    pub fn new() -> Self {
        Self { cursor: None }
    }

    pub fn handle<'a, G>(&'a mut self, graph: G) -> impl Iterator<Item = G::Node> + 'a
    where
        G: Graph<I = I> + 'a,
        I: Index<G> + PartialEq + Eq,
    {
        // TODO: reverse NodeIterator?
        let mut iter = graph.nodes();

        // TODO: consider building this into the Graph trait..
        if let Some(cursor) = self.cursor {
            iter.by_ref().take_while(|el| el.id() != cursor).count();
        }

        iter.inspect(|v| self.cursor = Some(v.id()))
    }
}

impl<I> Default for TrackedNodes<I> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TrackedSubGraphs<I> {
    ids: Vec<I>,
    cursor: Option<I>,
}

impl<I> TrackedSubGraphs<I> {
    pub fn new(ids: Vec<I>) -> Self {
        Self { ids, cursor: None }
    }

    pub fn handle<'a, G>(&'a mut self, graph: G) -> impl Iterator<Item = G::Node> + 'a
    where
        G: Graph<I = I> + GraphMeta<NodeIndex = I> + 'a,
        I: Index<G> + PartialEq + Eq + 'static,
    {
        let mut iter = self
            .ids
            .iter()
            .map(|i| graph.index(*i))
            .roots_for_graph(graph);

        if let Some(cursor) = self.cursor {
            iter.by_ref().take_while(|el| el.id() != cursor).count();
        }

        iter.inspect(|v| self.cursor = Some(v.id()))
    }

    /// Note only events that are newer than the current state are brought along
    pub fn add_id(&mut self, id: I) {
        self.ids.push(id);
    }

    pub fn ids(&self) -> &Vec<I> {
        &self.ids
    }
}
