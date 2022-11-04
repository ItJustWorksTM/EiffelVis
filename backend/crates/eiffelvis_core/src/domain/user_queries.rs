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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFilterMeta {
    /// Reverses the predicate's result if true
    #[serde(default)]
    rev: bool,
    pred: EventFilter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EventFilter {
    /// Event Type
    Type { names: Vec<String> },
    /// Specific ids
    Id { ids: Vec<Uuid> },
    /// meta.tags
    Tag { tags: Vec<String> },
    /// meta.source.host
    SourceHost { hosts: Vec<String> },
    /// meta.source.name
    SourceName { names: Vec<String> },
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

        let fresh = inner.handle(graph);
        let iter = fresh.filter(|node| {
            if self.event_filters.is_empty() {
                true
            } else {
                // Event filters itterator that returns nodes that contain the imput given (id filter has to be an exact match, all others do not.)
                self.event_filters.iter().filter(|v| !v.is_empty()).any(|filters| {
                    filters.iter().all(|filter| match &filter.pred {
                        EventFilter::Type { names: ref name } => name.iter().any(|name| node.data().meta.event_type.to_lowercase().contains(&name.to_lowercase())),
                        EventFilter::Id { ids } =>
                            ids.iter().any(|id| graph.get(*id).map(|n| n.id() == node.id()).unwrap_or(false)),
                        EventFilter::Tag { tags } =>
                            tags.iter().any(|tag| node.data().meta.tags.as_ref().map(|v| v.contains(tag)).unwrap_or(false)),
                        EventFilter::SourceHost { hosts } =>
                            hosts.iter().any(|host|node.data().meta.source.as_ref().and_then(|s| s.host.as_ref()).map(|h| h.to_lowercase().contains(&host.to_lowercase())).unwrap_or(false)),
                        EventFilter::SourceName { names } =>
                            names.iter().any(|name| node.data().meta.source.as_ref().and_then(|s| s.name.as_ref()).map(|n| n.to_lowercase().contains(&name.to_lowercase())).unwrap_or(false)),
                    } ^ filter.rev)
                })
            }
        });

        match self.collector {
            Collector::Forward => iter.map(|v| R::from(v.data())).collect(),
            Collector::SubGraph(ref mut sub) => {
                iter.for_each(|v| sub.add_id(v.id()));
                sub.handle(graph).map(|v| R::from(v.data())).collect()
            }
        }
    }
}
