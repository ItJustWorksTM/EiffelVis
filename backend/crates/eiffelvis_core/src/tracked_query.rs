use crate::{
    graph::*,
    query::{GraphQuery, SubGraphs},
};

/// Yields only the nodes that have been added to graph since the last call to `handle`
pub struct TrackedNodes<I> {
    cursor: Option<I>,
}

impl<I> TrackedNodes<I> {
    pub fn new() -> Self {
        Self { cursor: None }
    }

    /// Returns an iterator over the newly added nodes since the last call.
    /// Note: internal cursor is updated on calls to [Iterator::next]
    pub fn handle<'a, G>(&'a mut self, graph: &'a G) -> TrackedNodesIter<'a, I, G>
    where
        G: Graph<Idx = I>,
        I: Idx,
    {
        TrackedNodesIter {
            inner: graph.range(self.cursor, None),
            owner: self,
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
    inner: NodeRangeIterType<'a, G>,
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

/// Tracked version of [crate::query::SubGraphs], behaves in the same manner as [TrackedNodes].
pub struct TrackedSubGraphs<I> {
    ids: Vec<I>,
    cursor: Option<I>,
}

impl<I> TrackedSubGraphs<I> {
    pub fn new(ids: Vec<I>) -> Self {
        Self { ids, cursor: None }
    }

    /// Returns an iterator over the newly added nodes since the last call.
    /// Note: internal cursor is updated on calls to [Iterator::next]
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

        // This is unavoidable as the entire graph needs to collected before we can reason about it
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
