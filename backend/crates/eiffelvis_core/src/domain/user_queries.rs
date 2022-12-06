use crate::{
    domain::{event_filter::EventFilterMeta, types::BaseEvent},
    graph::*,
    tracked_query::{TrackedNodes, TrackedSubGraphs},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Describes a query to collect nodes from an eiffel graph,
/// to match an event **all** filters need to match.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    #[serde(default)]
    /// Filters used to determine initial range of nodes to be filtered
    range_filter: RangeFilter,
    /// Filters to be applied over individual events
    event_filters: Vec<Vec<EventFilterMeta>>,
    /// Method of collection, may add additional nodes
    collection: Collection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RangeFilterBound {
    Absolute { val: i64 },
    Time { val: u64 },
    Ids { val: Uuid },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct RangeFilter {
    begin: Option<RangeFilterBound>,
    end: Option<RangeFilterBound>,
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
    range_filter: RangeFilter,
    event_filters: Vec<Vec<EventFilterMeta>>,
    collector: Collector<I>,
    inner: Option<TrackedNodes<I>>,
}

enum Collector<I> {
    Forward,
    SubGraph(TrackedSubGraphs<I>),
}

impl<I> Collector<I> {
    fn collect_nodes<'a, G, R, IterT>(&'a mut self, graph: &'a G, iter: IterT) -> Vec<R>
    where
        G: Graph<Data = BaseEvent, Idx = I, Key = Uuid> + Indexable<usize>,
        R: From<&'a BaseEvent> + 'static,
        I: Idx,
        IterT: Iterator<Item = NodeType<'a, G>>,
    {
        match self {
            Collector::Forward => iter.map(|v| R::from(v.data())).collect(),
            Collector::SubGraph(ref mut sub) => {
                iter.for_each(|v| sub.add_id(v.id()));
                sub.handle(graph).map(|v| R::from(v.data())).collect()
            }
        }
    }
}

impl<I> TrackedQuery<I> {
    pub fn new(query: Query) -> Self {
        Self {
            range_filter: query.range_filter,
            event_filters: query.event_filters,
            collector: match query.collection {
                Collection::Forward => Collector::Forward,
                Collection::AsRoots => Collector::SubGraph(TrackedSubGraphs::new(vec![])),
            },
            inner: None,
        }
    }

    /// Collects new nodes that match the query since the last time this method was called
    /// If it was not called before it behaves as the non-tracked version.
    pub fn handle<'a, R, G>(&'a mut self, graph: &'a G) -> Vec<R>
    where
        G: Graph<Data = BaseEvent, Idx = I, Key = Uuid> + Indexable<usize>,
        R: From<&'a BaseEvent> + 'static,
        I: Idx,
    {
        let inner = if let Some(ref mut inner) = self.inner {
            inner
        } else {
            let range_bound_to_idx = |rg: &Option<RangeFilterBound>| match rg {
                Some(rg) => match *rg {
                    RangeFilterBound::Ids { val } => graph.get(val).map(|d| Some(d.id())),
                    RangeFilterBound::Time { val } => graph
                        .items()
                        .find(|node| node.data().meta.time >= val)
                        .map(|d| Some(d.id())),
                    RangeFilterBound::Absolute { val } => {
                        let val = if val < 0 {
                            (graph.node_count()
                                - (val.unsigned_abs() as usize).min(graph.node_count()))
                                as usize
                        } else {
                            val as usize
                        };
                        graph.get(val).map(|no| Some(no.id()))
                    }
                },
                _ => Some(None),
            };

            let begin = range_bound_to_idx(&self.range_filter.begin);
            let end = range_bound_to_idx(&self.range_filter.end);

            let new = match begin.zip(end) {
                Some(s) => match s {
                    (Some(d), Some(b)) => TrackedNodes::range(d..=b),
                    (Some(d), None) => TrackedNodes::range(d..),
                    (None, Some(b)) => TrackedNodes::range(..=b),
                    _ => TrackedNodes::range(..),
                },
                _ => return Vec::new(), // TODO: decide error handling
            };

            self.inner = Some(new);
            self.inner.as_mut().unwrap()
        };

        if self.event_filters.is_empty() {
            let iter = inner.handle(graph);
            self.collector.collect_nodes(graph, iter)
        } else {
            let iter = inner.handle(graph).filter(|node| {
                self.event_filters
                    .iter()
                    .filter(|v| !v.is_empty())
                    .any(|filters| filters.iter().all(|filter| filter.do_filter(graph, node)))
            });
            self.collector.collect_nodes(graph, iter)
        }
    }
}
