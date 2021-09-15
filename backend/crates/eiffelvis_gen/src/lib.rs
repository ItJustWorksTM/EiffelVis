use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
pub struct Event<T: Default> {
    pub meta: Meta,
    pub data: T,
    pub links: Vec<Link>,
}

pub fn make_event() -> Vec<u8> {
    serde_json::to_vec(&Event {
        links: vec![Link {
            link_type: "fuck".to_string(),
            target: Uuid::new_v4(),
        }],
        data: (),
        ..Default::default()
    })
    .unwrap()
}

#[cfg(test)]
mod test {
    use super::make_event;

    #[test]
    fn test() {
        println!("{:?}", make_event());
    }
}
