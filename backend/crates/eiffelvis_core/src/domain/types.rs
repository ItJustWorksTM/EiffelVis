use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BaseData {}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BaseMeta {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub event_type: String,
    pub version: String,
    pub time: u64,
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct BaseLink {
    #[serde(rename = "type")]
    pub link_type: String,
    pub target: Uuid,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BaseEvent {
    pub meta: BaseMeta,
    pub data: BaseData,
    pub links: Vec<BaseLink>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LeanEvent {
    pub id: Uuid,
    pub time: u64,
    pub event_type: String,
    pub edges: Vec<Uuid>,
}

impl From<&BaseEvent> for LeanEvent {
    fn from(ev: &BaseEvent) -> Self {
        Self {
            id: ev.meta.id,
            time: ev.meta.time,
            event_type: ev.meta.event_type,
            edges: ev.links.iter().map(|link| link.target).collect(),
        }
    }
}
