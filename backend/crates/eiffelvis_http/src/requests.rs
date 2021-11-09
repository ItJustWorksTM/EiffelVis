use eiffelvis_core::{app::EiffelVisApp, types::LeanEvent};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
#[serde(tag = "type")]
pub enum EiffelClientRequest {
    All(All),
    Latest(Latest),
    SinceId(SinceId),
    WithRoot(WithRoot),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct All {}

impl<T: EiffelVisApp> ClientRequest<T> for All {
    type Handler = AllHandler;
}

pub struct AllHandler {
    cursor: Option<Uuid>,
}

impl From<All> for AllHandler {
    fn from(_: All) -> Self {
        Self { cursor: None }
    }
}

impl<T: EiffelVisApp> ClientRequestHandler<T> for AllHandler {
    fn handle(&mut self, app: &T) -> Option<Vec<LeanEvent>> {
        app.head()
            .zip(self.cursor.map_or_else(
                || Some(app.dump::<LeanEvent>()),
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
    type Handler = LatestHandler;
}

pub struct LatestHandler {
    req: Latest,
    cursor: Option<Uuid>,
}

impl From<Latest> for LatestHandler {
    fn from(latest: Latest) -> Self {
        Self {
            req: latest,
            cursor: None,
        }
    }
}

impl<T: EiffelVisApp> ClientRequestHandler<T> for LatestHandler {
    fn handle(&mut self, app: &T) -> Option<Vec<LeanEvent>> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SinceId {
    id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WithRoot {
    id: Uuid,
}

impl<T: EiffelVisApp> ClientRequest<T> for WithRoot {
    type Handler = WithRootHandler;
}

pub struct WithRootHandler {
    req: WithRoot,
    cursor: Option<Uuid>,
}

impl From<WithRoot> for WithRootHandler {
    fn from(req: WithRoot) -> Self {
        Self { req, cursor: None }
    }
}

impl<T: EiffelVisApp> ClientRequestHandler<T> for WithRootHandler {
    fn handle(&mut self, app: &T) -> Option<Vec<LeanEvent>> {
        self.cursor
            .map_or_else(
                || app.get_subgraph_with_root::<LeanEvent>(self.req.id),
                |c| {
                    app.get_subgraph_with_root::<LeanEvent>(self.req.id)
                        .and_then(|e| {
                            e.iter() // TODO: reverse iterator as its more likely to be at the end
                                .position(|el| el.id == c)
                                .zip(Some(e))
                        })
                        .map(|(i, mut e)| {
                            e.drain(0..i + 1);
                            e
                        })
                },
            )
            .filter(|evs| !evs.is_empty())
            .map(|lean_events| {
                self.cursor = lean_events.last().map(|e| e.id);
                lean_events
            })
    }
}
