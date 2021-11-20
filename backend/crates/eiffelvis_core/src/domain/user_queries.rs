use crate::{
    domain::types::BaseEvent,
    graph::*,
    tracked_query::{TrackedNodes, TrackedSubGraphs},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    /// Will return a given node if any of the filters match
    filters: Vec<Filter>,
    /// Method of collection, may add additional nodes
    collection: Collection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Filter {
    /// Do not filter at all, just give everything
    None,
    /// Open ended range of time
    Time {
        begin: Option<u64>,
        end: Option<u64>,
    },
    /// Event Type
    Type { name: String },
    /// Specific ids
    Ids { ids: Vec<Uuid> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Collection {
    /// Does not do anything
    Forward,
    /// Uses the result of the filtered nodes as roots and collects their graph
    AsRoots,
}

pub struct TrackedQuery<I> {
    filters: Vec<Filter>,
    collector: Collector<I>,
    inner: TrackedNodes<I>,
}

enum Collector<I> {
    Forward,
    SubGraph(TrackedSubGraphs<I>),
}

impl<I> TrackedQuery<I> {
    pub fn new(query: Query) -> Self {
        Self {
            filters: query.filters,
            collector: match query.collection {
                Collection::Forward => Collector::Forward,
                Collection::AsRoots => Collector::SubGraph(TrackedSubGraphs::new(vec![])),
            },
            inner: TrackedNodes::new(),
        }
    }

    pub fn handle<'a, R, G>(&'a mut self, graph: G) -> Vec<R>
    where
        G: Ref<'a>,
        G::Meta: Meta<Data = BaseEvent, Idx = I, Key = Uuid> + 'a,
        R: From<&'a BaseEvent> + 'static,
        I: Idx,
    {
        let fresh = self.inner.handle(graph);
        let iter = fresh.iter().filter(|node| {
            self.filters.iter().any(|filter| match filter {
                Filter::None => true,
                Filter::Time { begin, end } => {
                    begin
                        .map(|begin| node.data().meta.time >= begin)
                        .unwrap_or(true)
                        && end.map(|end| node.data().meta.time <= end).unwrap_or(true)
                }
                Filter::Type { ref name } => &node.data().meta.event_type == name,
                Filter::Ids { ref ids } => ids
                    .iter()
                    .map(|i| graph.index(*i))
                    .any(|i| i.id() == node.id()),
            })
        });

        match self.collector {
            Collector::Forward => iter.map(|v| R::from(&*v.data())).collect(),
            Collector::SubGraph(ref mut sub) => {
                iter.for_each(|v| sub.add_id(v.id()));
                sub.handle(graph)
                    .iter()
                    .map(|v| R::from(v.data()))
                    .collect()
            }
        }
    }
}
