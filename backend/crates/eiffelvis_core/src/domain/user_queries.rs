use crate::{
    domain::types::BaseEvent,
    graph::*,
    tracked_query::{TrackedNodes, TrackedSubGraphs},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Describes a query to collect nodes from an eiffel graph,
/// to match an event **all** filters need to match.
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
        begin: Option<u128>,
        end: Option<u128>,
    },
    /// Event Type
    Type { name: String },
    /// Specific ids
    Ids { ids: Vec<Uuid> },
}

/// Used collection method,
/// selected variant is run **after** filtering has been done,
/// this means you can get nodes back that may not filfull the filter requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Collection {
    /// Does not do anything
    Forward,
    /// Uses the result of the filtered nodes as roots and collects their graph
    AsRoots,
}

/// A tracked query, only returns new matches.
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

    /// Collects new nodes that match the query since the last time this method was called
    /// If it was not called before it behaves as the non-tracked version.
    pub fn handle<'a, R, G>(&'a mut self, graph: &'a G) -> Vec<R>
    where
        G: Graph<Data = BaseEvent, Idx = I, Key = Uuid>,
        R: From<&'a BaseEvent> + 'static,
        I: Idx,
    {
        let fresh = self.inner.handle(graph);
        let iter = fresh.filter(|node| {
            self.filters.iter().all(|filter| match filter {
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
                    .filter_map(|i| graph.get(*i))
                    .any(|i| i.id() == node.id()),
            })
        });

        match self.collector {
            Collector::Forward => iter.map(|v| R::from(&*v.data())).collect(),
            Collector::SubGraph(ref mut sub) => {
                iter.for_each(|v| sub.add_id(v.id()));
                sub.handle(graph).map(|v| R::from(v.data())).collect()
            }
        }
    }
}
