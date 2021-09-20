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

use crate::event_type::{Event, LinkTargets};
use crate::{event::Event as EventValue, event::Link as LinkValue, event_set::EventSet};

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
    history: VecDeque<EventValue>,
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
    fn can_generate(&self, event: &Event) -> bool {
        let event_set = &self.inner.event_set;
        let history = &self.history;
        event.required_links.iter().all(|l| {
            history.iter().any(|e| {
                let link = event_set.links.get(l).unwrap();
                match &link.targets {
                    LinkTargets::Any => true,
                    LinkTargets::Events(evs) => evs.contains(&e.meta.event_type),
                }
            })
        })
    }
}

impl Iterator for Iter<'_> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let meta_event = {
            let mut q = self
                .inner
                .event_set
                .events
                .iter()
                .collect::<Vec<(&String, &Event)>>();
            q.shuffle(self.rng.get_mut());
            q
        }
        .iter()
        .find(|(_, e)| self.can_generate(e))?
        .1;

        let mut event = EventValue::default();

        let mut dep_map: HashMap<String, Vec<&EventValue>> = HashMap::new();

        meta_event
            .required_links
            .iter()
            .chain(meta_event.links.iter())
            .filter_map(|a| self.inner.event_set.links.get(a))
            .for_each(|a| {
                match &a.targets {
                    LinkTargets::Any => {
                        dep_map.insert("".to_string(), Vec::new());
                    }
                    LinkTargets::Events(evs) => {
                        evs.iter().for_each(|a| {
                            dep_map.insert(a.clone(), Vec::new());
                        });
                    }
                };
            });

        if let Some(v) = dep_map.get_mut("") {
            *v = self.history.iter().collect();
        }

        for x in &self.history {
            if let Some(v) = dep_map.get_mut(&x.meta.event_type) {
                v.push(x);
            }
        }

        let mut result = Vec::<LinkValue>::new();

        for (a, required) in meta_event
            .required_links
            .iter()
            .zip(std::iter::repeat(true))
            .chain(meta_event.links.iter().zip(std::iter::repeat(false)))
            .filter_map(|(a, required)| self.inner.event_set.links.get(a).map(|a| (a, required)))
        {
            let mut rand =
                self.rng.borrow_mut().gen::<usize>() % (self.inner.max_links - result.len());

            rand += required as usize;

            if !a.allow_many {
                rand = rand.clamp(0, 1);
            }

            while rand != 0 {
                let ret = match &a.targets {
                    LinkTargets::Any => {
                        let t = dep_map.get_mut("").unwrap();
                        (!t.is_empty())
                            .then(|| t.swap_remove(self.rng.borrow_mut().gen_range(0..t.len())))
                    }
                    LinkTargets::Events(evs) => {
                        let mut e = evs.iter().collect::<Vec<&String>>();
                        e.shuffle(&mut *self.rng.borrow_mut());
                        e.iter().find_map(|&a| {
                            dep_map.get_mut(a).and_then(|t| {
                                (!t.is_empty()).then(|| {
                                    t.swap_remove(self.rng.borrow_mut().gen_range(0..t.len()))
                                })
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

                    LinkValue {
                        target: pick.meta.id,
                        link_type: a.name.clone(),
                    }
                });

                if let Some(ret) = ret {
                    result.push(ret);
                    rand -= 1;
                } else {
                    break;
                }
            }
        }

        event.meta.event_type = meta_event.name.clone();
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
