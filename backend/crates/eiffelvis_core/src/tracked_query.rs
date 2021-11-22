use crate::{
    graph::*,
    query::{GraphQuery, SubGraphs},
};

pub struct TrackedNodes<I> {
    cursor: Option<I>,
}

impl<I> TrackedNodes<I> {
    pub fn new() -> Self {
        Self { cursor: None }
    }

    pub fn handle<'a, G>(&'a mut self, graph: &'a G) -> TrackedNodesIter<'a, I, G>
    where
        G: Graph<Idx = I>,
        I: Idx,
    {
        // TODO: reverse NodeIterator?
        let mut iter = graph.items();

        // TODO: consider building this into the Graph trait..
        if let Some(cursor) = self.cursor {
            iter.by_ref().take_while(|el| el.id() != cursor).count();
        }

        TrackedNodesIter {
            owner: self,
            inner: iter,
        }
    }
}

impl<I> Default for TrackedNodes<I> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TrackedNodesIter<'a, I, G: ItemIter> {
    owner: &'a mut TrackedNodes<I>,
    inner: NodeIterType<'a, G>,
}

impl<'a, I, G> Iterator for TrackedNodesIter<'a, I, G>
where
    G: Graph<Idx = I>,
    I: Idx,
{
    type Item = NodeType<'a, G>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.inner.next();
        if let Some(item) = &ret {
            self.owner.cursor = Some(item.id());
        }
        ret
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

    pub fn handle<'a, G>(&'a mut self, graph: &'a G) -> TrackedSubGraphsIter<'a, I, G>
    where
        G: Graph<Idx = I>,
        I: Idx,
    {
        let mut iter = self
            .ids
            .iter()
            .map(|i| graph.index(*i))
            .roots_for_graph(graph);

        if let Some(cursor) = self.cursor {
            iter.by_ref().take_while(|el| el.id() != cursor).count();
        }

        TrackedSubGraphsIter {
            owner: self,
            inner: iter,
        }
    }

    /// Note only events that are newer than the current state are brought along
    pub fn add_id(&mut self, id: I) {
        self.ids.push(id);
    }

    pub fn ids(&self) -> &Vec<I> {
        &self.ids
    }
}

pub struct TrackedSubGraphsIter<'a, I, G: Graph> {
    owner: &'a mut TrackedSubGraphs<I>,
    inner: SubGraphs<'a, G>,
}

impl<'a, I, G> Iterator for TrackedSubGraphsIter<'a, I, G>
where
    G: Graph<Idx = I>,
    I: Idx,
{
    type Item = NodeType<'a, G>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.inner.next();
        if let Some(item) = &ret {
            self.owner.cursor = Some(item.id());
        }
        ret
    }
}
