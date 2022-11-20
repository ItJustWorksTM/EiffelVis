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
<<<<<<< HEAD
<<<<<<< HEAD
    Type { names: Vec<StringCompare> },
    /// Specific ids
    Id { ids: Vec<Uuid> },
    /// meta.tags
    Tag { tags: Vec<StringCompare> },
    /// meta.source.host
    SourceHost { hosts: Vec<StringCompare> },
    /// meta.source.name
    SourceName { names: Vec<StringCompare> },
=======
    Type { names: Vec<String> },
=======
    Type { names: Vec<StringCompare> },
>>>>>>> bfe06c7 (backend: allow client to somewhat control string matching)
    /// Specific ids
    Id { ids: Vec<Uuid> },
    /// meta.tags
    Tag { tags: Vec<StringCompare> },
    /// meta.source.host
    SourceHost { hosts: Vec<StringCompare> },
    /// meta.source.name
<<<<<<< HEAD
    SourceName { names: Vec<String> },
>>>>>>> 1967a85 (backend/domain: split out filter and collection)
=======
    SourceName { names: Vec<StringCompare> },
>>>>>>> bfe06c7 (backend: allow client to somewhat control string matching)
}

impl EventFilterMeta {
    pub fn do_filter<'a, G, I>(&self, graph: &G, node: &NodeType<'a, G>) -> bool
    where
        G: Graph<Data = BaseEvent, Idx = I, Key = Uuid> + Indexable<usize>,
        I: Idx,
    {
        let res = match &self.pred {
<<<<<<< HEAD
<<<<<<< HEAD
            EventFilter::Type { names: ref name } => name
                .iter()
                .any(|name| name.eq(&node.data().meta.event_type)),
=======
            EventFilter::Type { names: ref name } => {
                name.iter().any(|name| &node.data().meta.event_type == name)
            }
>>>>>>> 1967a85 (backend/domain: split out filter and collection)
=======
            EventFilter::Type { names: ref name } => name
                .iter()
                .any(|name| name.eq(&node.data().meta.event_type)),
>>>>>>> bfe06c7 (backend: allow client to somewhat control string matching)

            EventFilter::Id { ids } => ids
                .iter()
                .any(|id| graph.get(*id).map(|n| n.id() == node.id()).unwrap_or(false)),

            EventFilter::Tag { tags } => tags.iter().any(|tag| {
                node.data()
                    .meta
                    .tags
                    .as_ref()
<<<<<<< HEAD
<<<<<<< HEAD
                    .map(|v| v.iter().any(|t| tag.eq(t)))
=======
                    .map(|v| v.contains(tag))
>>>>>>> 1967a85 (backend/domain: split out filter and collection)
=======
                    .map(|v| v.iter().any(|t| tag.eq(t)))
>>>>>>> bfe06c7 (backend: allow client to somewhat control string matching)
                    .unwrap_or(false)
            }),

            EventFilter::SourceHost { hosts } => hosts.iter().any(|host| {
                node.data()
                    .meta
                    .source
                    .as_ref()
                    .and_then(|s| s.host.as_ref())
<<<<<<< HEAD
<<<<<<< HEAD
                    .map(|h| host.eq(h))
=======
                    .map(|h| h == host)
>>>>>>> 1967a85 (backend/domain: split out filter and collection)
=======
                    .map(|h| host.eq(h))
>>>>>>> bfe06c7 (backend: allow client to somewhat control string matching)
                    .unwrap_or(false)
            }),

            EventFilter::SourceName { names } => names.iter().any(|name| {
                node.data()
                    .meta
                    .source
                    .as_ref()
                    .and_then(|s| s.name.as_ref())
<<<<<<< HEAD
<<<<<<< HEAD
                    .map(|n| name.eq(n))
=======
                    .map(|n| n == name)
>>>>>>> 1967a85 (backend/domain: split out filter and collection)
=======
                    .map(|n| name.eq(n))
>>>>>>> bfe06c7 (backend: allow client to somewhat control string matching)
                    .unwrap_or(false)
            }),
        };

        res ^ self.rev
    }
}
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> bfe06c7 (backend: allow client to somewhat control string matching)

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StringCompare {
    /// The value to compare with
    value: String,
    /// Make value being compared with lower case
    lower_case: bool,
    /// Don't require full match
    partial: bool,
}

impl StringCompare {
    fn compare(&self, other: &str) -> bool {
        if !self.partial {
            other == self.value.as_str()
        } else {
            other.contains(self.value.as_str())
        }
    }

    pub fn equal(&self, other: &str) -> bool {
        if !self.lower_case {
            self.compare(other)
        } else {
            // TODO: Avoid allocating with https://github.com/artichoke/focaccia if performance is bad.
            self.compare(other.to_lowercase().as_str())
        }
    }
}

impl PartialEq<String> for StringCompare {
    fn eq(&self, other: &String) -> bool {
        self.equal(other.as_str())
    }
}
<<<<<<< HEAD
=======
>>>>>>> 1967a85 (backend/domain: split out filter and collection)
=======
>>>>>>> bfe06c7 (backend: allow client to somewhat control string matching)
