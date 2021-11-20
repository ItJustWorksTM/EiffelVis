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

    pub fn handle<'a, G>(&'a mut self, graph: G) -> TrackedNodesIter<'a, I, G>
    where
        G: Ref<'a>,
        G::Meta: Meta<Idx = I> + 'a,
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

pub struct TrackedNodesIter<'a, I, G: Ref<'a>> {
    owner: &'a mut TrackedNodes<I>,
    inner: G::ItemIterator,
}

impl<'a, I, G: Ref<'a>> Iterator for TrackedNodesIter<'a, I, G>
where
    G::Meta: Meta<Idx = I> + 'a,
    I: Idx,
{
    type Item = G::Item;

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

    pub fn handle<'a, G>(&'a mut self, graph: G) -> TrackedSubGraphsIter<'a, I, G>
    where
        G: Ref<'a>,
        G::Meta: Meta<Idx = I> + 'a,
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

pub struct TrackedSubGraphsIter<'a, I, G: Ref<'a>> {
    owner: &'a mut TrackedSubGraphs<I>,
    inner: SubGraphs<'a, G>,
}

impl<'a, I, G: Ref<'a>> Iterator for TrackedSubGraphsIter<'a, I, G>
where
    G::Meta: Meta<Idx = I> + 'a,
    I: Idx,
{
    type Item = G::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.inner.next();
        if let Some(item) = &ret {
            self.owner.cursor = Some(item.id());
        }
        ret
    }
}
