use eiffelvis_core::{app::EiffelVisApp, types::LeanEvent};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum EiffelClientRequest {
    All(All),
    Latest(Latest),
    SinceId(SinceId),
}

pub trait ClientRequest<T>: Into<Self::Handler>
where
    T: EiffelVisApp,
{
    type Handler: ClientRequestHandler<T>;

    fn into_handler(self) -> Self::Handler {
        self.into()
    }
}

pub trait ClientRequestHandler<T>
where
    T: EiffelVisApp,
{
    fn handle(&mut self, app: &T) -> Option<Vec<LeanEvent>>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct All {}

impl<T: EiffelVisApp> ClientRequest<T> for All {
    type Handler = AllFunctionality;
}

pub struct AllFunctionality {
    cursor: Option<Uuid>,
}

impl From<All> for AllFunctionality {
    fn from(_: All) -> Self {
        Self { cursor: None }
    }
}

impl<T: EiffelVisApp> ClientRequestHandler<T> for AllFunctionality {
    fn handle(&mut self, app: &T) -> Option<Vec<LeanEvent>> {
        app.head()
            .zip(self.cursor.map_or_else(
                || Some(app.dump_lean_events()),
                |c| app.events_starting_from(c).filter(|evs| !evs.is_empty()),
            ))
            .map(|(head, lean_events)| {
                self.cursor = Some(head);
                lean_events
            })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Latest {
    amount: u32,
}

impl<T: EiffelVisApp> ClientRequest<T> for Latest {
    type Handler = LatestFunctionality;
}

pub struct LatestFunctionality {
    req: Latest,
    cursor: Option<Uuid>,
}

impl From<Latest> for LatestFunctionality {
    fn from(latest: Latest) -> Self {
        Self {
            req: latest,
            cursor: None,
        }
    }
}

impl<T: EiffelVisApp> ClientRequestHandler<T> for LatestFunctionality {
    fn handle(&mut self, app: &T) -> Option<Vec<LeanEvent>> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SinceId {
    id: Uuid,
}
