use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Data {}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Meta {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub event_type: String,
    pub version: String,
    pub time: i64, // Not high priority?
                   // source: Source
                   // security: Security
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "type")]
    pub link_type: String,
    pub target: Uuid,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Event {
    pub meta: Meta,
    pub data: Data,
    pub links: Vec<Link>,
}
