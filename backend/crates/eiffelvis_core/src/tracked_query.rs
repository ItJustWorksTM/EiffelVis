use crate::{graph::*, query::GraphQuery};

// TODO: return iterators instead of Vec to avoid allocating..

pub struct TrackedNodes<I> {
    cursor: Option<I>,
}

impl<I> TrackedNodes<I> {
    pub fn new() -> Self {
        Self { cursor: None }
    }

    pub fn handle<'a, G>(&'a mut self, graph: G) -> Vec<G::Item>
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

        iter.inspect(|v| self.cursor = Some(v.id())).collect()
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

    pub fn handle<'a, G>(&'a mut self, graph: G) -> Vec<G::Item>
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

        iter.inspect(|v| self.cursor = Some(v.id())).collect()
    }

    /// Note only events that are newer than the current state are brought along
    pub fn add_id(&mut self, id: I) {
        self.ids.push(id);
    }

    pub fn ids(&self) -> &Vec<I> {
        &self.ids
    }
}
