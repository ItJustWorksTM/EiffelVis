use crate::{domain::types::BaseEvent, graph::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

impl EventFilterMeta {
    pub fn do_filter<'a, G, I>(&self, graph: &G, node: &NodeType<'a, G>) -> bool
    where
        G: Graph<Data = BaseEvent, Idx = I, Key = Uuid> + Indexable<usize>,
        I: Idx,
    {
        let res = match &self.pred {
            EventFilter::Type { names: ref name } => {
                name.iter().any(|name| &node.data().meta.event_type == name)
            }

            EventFilter::Id { ids } => ids
                .iter()
                .any(|id| graph.get(*id).map(|n| n.id() == node.id()).unwrap_or(false)),

            EventFilter::Tag { tags } => tags.iter().any(|tag| {
                node.data()
                    .meta
                    .tags
                    .as_ref()
                    .map(|v| v.contains(tag))
                    .unwrap_or(false)
            }),

            EventFilter::SourceHost { hosts } => hosts.iter().any(|host| {
                node.data()
                    .meta
                    .source
                    .as_ref()
                    .and_then(|s| s.host.as_ref())
                    .map(|h| h == host)
                    .unwrap_or(false)
            }),

            EventFilter::SourceName { names } => names.iter().any(|name| {
                node.data()
                    .meta
                    .source
                    .as_ref()
                    .and_then(|s| s.name.as_ref())
                    .map(|n| n == name)
                    .unwrap_or(false)
            }),
        };

        res ^ self.rev
    }
}
