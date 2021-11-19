use eiffelvis_core::{app::EiffelVisApp, types::BaseEvent};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait ClientRequestHandler<'a, T, D>
where
    T: EiffelVisApp,
    D: From<&'a BaseEvent>,
{
    fn handle(&mut self, app: &'a T) -> Option<Vec<D>>;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum EiffelClientRequest {
    All(All),
    SinceId(SinceId),
    WithRoot(WithRoot),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct All {}

pub struct AllHandler {
    cursor: Option<Uuid>,
}

impl From<All> for AllHandler {
    fn from(_: All) -> Self {
        Self { cursor: None }
    }
}

impl<'a, T: EiffelVisApp, D: From<&'a BaseEvent>> ClientRequestHandler<'a, T, D> for AllHandler {
    fn handle(&mut self, app: &'a T) -> Option<Vec<D>> {
        app.head()
            .zip(self.cursor.map_or_else(
                || Some(app.dump::<D>()),
                |c| app.events_starting_from(c).filter(|evs| !evs.is_empty()),
            ))
            .map(|(head, lean_events)| {
                self.cursor = Some(head);
                lean_events
            })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SinceId {
    id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WithRoot {
    ids: Vec<Uuid>,
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

impl<'a, T: EiffelVisApp, D: From<&'a BaseEvent>> ClientRequestHandler<'a, T, D>
    for WithRootHandler
{
    fn handle(&mut self, app: &'a T) -> Option<Vec<D>> {
        self.cursor
            .map_or_else(
                || app.get_subgraph_with_roots::<&'a BaseEvent>(&self.req.ids),
                |c| {
                    app.get_subgraph_with_roots::<&'a BaseEvent>(&self.req.ids)
                        .and_then(|e| {
                            e.iter() // TODO: reverse iterator as its more likely to be at the end
                                .position(|el| el.meta.id == c)
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
                self.cursor = lean_events.last().map(|e| e.meta.id);
                lean_events
            })
            .map(|mut fuck| fuck.drain(..).map(|v| D::from(v)).collect())
    }
}
