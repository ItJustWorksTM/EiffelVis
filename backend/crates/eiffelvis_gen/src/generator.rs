use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    hash::Hash,
    time::{SystemTime, UNIX_EPOCH},
};

use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use uuid::Uuid;

use crate::event_set::{EventBorrow, LinkBorrow};
use crate::{base_event::BaseEvent, base_event::BaseLink, event_set::EventSet};

pub struct EventGenerator {
    pub seed: Seeder,
    inner: Inner,
}

struct Inner {
    event_set: EventSet,
    max_links: usize,
    history_max: usize,
}

impl EventGenerator {
    pub fn new(seed: impl Hash, max_links: usize, history_max: usize, event_set: EventSet) -> Self {
        Self {
            inner: Inner {
                event_set,
                history_max,
                max_links,
            },
            seed: seed.into(),
        }
    }

    pub fn iter(&self) -> Iter {
        Iter::new(&self.inner, self.seed.clone().make_rng())
    }
}

impl Default for EventGenerator {
    fn default() -> Self {
        Self {
            inner: Inner {
                event_set: EventSet::default(),
                max_links: 5,
                history_max: 100,
            },
            seed: rand::random::<usize>().into(),
        }
    }
}

pub struct Iter<'a> {
    inner: &'a Inner,
    rng: RefCell<Pcg64>,
    history: VecDeque<BaseEvent>,
}

impl<'a> Iter<'a> {
    fn new(inner: &'a Inner, rng: Pcg64) -> Self {
        Self {
            inner,
            rng: RefCell::new(rng),
            history: VecDeque::new(),
        }
    }
}

impl Iter<'_> {
    fn can_generate(&self, event: &EventBorrow) -> bool {
        if event.link_count() > self.inner.max_links {
            return false;
        }
        let history = &self.history;
        event.links().all(|(link, req)| {
            if history.is_empty() {
                !req
            } else {
                history.iter().any(|e| match (link.targets(), req) {
                    (Some(mut evs), true) => evs.any(|a| a.name() == e.meta.event_type.as_str()),
                    _ => true,
                })
            }
        })
    }
}

impl Iterator for Iter<'_> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let event_set = &self.inner.event_set;
        let meta_event = {
            let mut q = event_set.events().collect::<Vec<EventBorrow>>();
            q.shuffle(self.rng.get_mut());
            *q.iter().find(|&e| self.can_generate(e))?
        };

        let mut event = BaseEvent::default();

        let mut dep_map: HashMap<&str, Vec<&BaseEvent>> = HashMap::new();

        meta_event.links().for_each(|(a, _)| {
            match a.targets() {
                None => {
                    dep_map.insert("", Vec::new());
                }
                Some(evs) => {
                    evs.for_each(|a| {
                        dep_map.insert(a.name(), Vec::new());
                    });
                }
            };
        });

        if let Some(v) = dep_map.get_mut("") {
            *v = self.history.iter().collect();
        }

        for x in &self.history {
            if let Some(v) = dep_map.get_mut(x.meta.event_type.as_str()) {
                v.push(x);
            }
        }

        let mut select_event = |a: LinkBorrow| {
            match a.targets() {
                None => {
                    let t = dep_map.get_mut("").unwrap();
                    (!t.is_empty())
                        .then(|| t.swap_remove(self.rng.borrow_mut().gen_range(0..t.len())))
                }
                Some(evs) => {
                    let mut e = evs.collect::<Vec<EventBorrow>>();
                    e.shuffle(&mut *self.rng.borrow_mut());
                    e.iter().find_map(|&a| {
                        dep_map.get_mut(a.name()).and_then(|t| {
                            (!t.is_empty())
                                .then(|| t.swap_remove(self.rng.borrow_mut().gen_range(0..t.len())))
                        })
                    })
                }
            }
            .map(|pick| {
                for (_, v) in dep_map.iter_mut() {
                    if let Some(pos) = v.iter().position(|&r| r.meta.id == pick.meta.id) {
                        v.remove(pos);
                        continue;
                    }
                }

                BaseLink {
                    target: pick.meta.id,
                    link_type: a.name().to_string(),
                }
            })
        };

        let mut result = Vec::<BaseLink>::new();

        // TODO: shuffle links first

        // First do the required links ONCE
        result.extend(
            meta_event
                .links()
                .filter(|(_, required)| *required)
                .map(|(link, _)| select_event(link).unwrap()), // unwrap safe because we checked that we can generate this event
        );

        // Then randomly select
        for (a, _) in meta_event.links() {
            if self.inner.max_links < result.len() {
                break;
            }

            let budget = self.inner.max_links - result.len();

            let mut rand = self.rng.borrow_mut().gen::<usize>() % budget;

            if !a.multiple_allowed() {
                rand = rand.clamp(0, 1);
            }

            while rand != 0 {
                if let Some(ret) = select_event(a) {
                    result.push(ret);
                    rand -= 1;
                } else {
                    break;
                }
            }
        }

        event.meta.event_type = meta_event.name().to_string();
        event.meta.id = Uuid::from_bytes(self.rng.get_mut().gen());

        // TODO: allow specifying a starting time and a time delay between events
        event.meta.time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        event.links = result;

        let event_bytes = serde_json::to_vec(&event).unwrap();

        self.history.push_back(event);

        if self.history.len() > self.inner.history_max {
            self.history.pop_front();
        }

        Some(event_bytes)
    }
}
