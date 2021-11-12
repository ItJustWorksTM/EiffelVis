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
    pub tags: Option<Vec<String>>,
    pub source: Option<MetaSource>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MetaSource {
    #[serde(rename = "domainId")] // Doesn't follow convention, so rename
    pub domain_id: Option<String>,
    pub host: Option<String>,
    pub name: Option<String>,
    pub serializer: Option<String>,
    pub uri: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeanEvent {
    pub id: Uuid,
    pub edges: Vec<Uuid>,
}

impl From<&BaseEvent> for LeanEvent {
    fn from(ev: &BaseEvent) -> Self {
        Self {
            id: ev.meta.id,
            edges: ev.links.iter().map(|link| link.target).collect(),
        }
    }
}
