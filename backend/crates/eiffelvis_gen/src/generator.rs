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

/// Holds on to all the data needed to generate a infinite series of events and should not need be modified after creation.  
/// Actual generated events come from the iterator [Iter] obtained through [EventGenerator::iter()].
///
/// Generated events come in the form of JSON, you will need to deserialize them on your own.  
/// The produced JSON will at least include the "meta", "data" and "links" fields each with their non optional fields :
/// ```json
/// {
///    "meta":{
///       "id":"206f25df-f6a8-545b-7492-996f84e897ee",
///       "type":"Event",
///       "version":"1.0.0",
///       "time":1632257822
///    },
///    "data":{
///   
///    },
///    "links":[
///       {
///          "type":"Link0",
///          "target":"13588b40-3c04-30f0-6437-f9712c17bc09"
///       }
///    ]
/// }
/// ```
/// ("data" is always empty for now)
pub struct EventGenerator {
    seed: Seeder,
    inner: Inner,
}

struct Inner {
    event_set: EventSet,
    max_links: usize,
    history_max: usize,
}

impl EventGenerator {
    /// Constructs a new [EventGenerator],  
    /// * `seed` : seeds the random generator used to generate random events.  
    /// * `max_links` : the upper limit on how many links a generated event can have, however links marked required ignore this limit.  
    /// * `history_max` : the upper limit on how many events will be kept to act as an event history from which new links are created.
    /// * `event_set` : the events and links that should be generated
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

    /// Creates a new **infinite** event iterator.
    /// Every [Iter] made from the same [EventGenerator] generate the same sequence of events.
    pub fn iter(&self) -> Iter {
        Iter::new(&self.inner, self.seed.clone().make_rng())
    }
}

impl Default for EventGenerator {
    fn default() -> Self {
        Self {
            inner: Inner {
                event_set: EventSet::default(),
                max_links: 10,
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
            false
        } else {
            event.links().all(|(link, req)| {
                if self.history.is_empty() {
                    !req
                } else {
                    self.history.iter().any(|e| match (link.targets(), req) {
                        (Some(mut evs), true) => {
                            evs.any(|a| a.name() == e.meta.event_type.as_str())
                        }
                        _ => true,
                    })
                }
            })
        }
    }
}

impl Iterator for Iter<'_> {
    type Item = Vec<u8>;

    /// Yields a new event, fails if no events could be created due to rule constraints such as required links.
    fn next(&mut self) -> Option<Self::Item> {
        let event_set = &self.inner.event_set;

        let meta_event = {
            let mut q = event_set.events().collect::<Vec<EventBorrow>>();
            q.shuffle(&mut *self.rng.borrow_mut());
            *q.iter().find(|&e| self.can_generate(e))?
        };

        let mut event = BaseEvent::default();

        let mut allowed_events: HashMap<&str, Vec<&BaseEvent>> = HashMap::new();

        for (link, _) in meta_event.links() {
            match link.targets() {
                None => {
                    allowed_events.insert("", Vec::new());
                }
                Some(evs) => {
                    evs.for_each(|a| {
                        allowed_events.insert(a.name(), Vec::new());
                    });
                }
            };
        }

        if let Some(v) = allowed_events.get_mut("") {
            *v = self.history.iter().collect();
        }

        for ev in &self.history {
            if let Some(v) = allowed_events.get_mut(ev.meta.event_type.as_str()) {
                v.push(ev);
            }
        }

        let mut select_event = |link: LinkBorrow| {
            match link.targets() {
                None => {
                    let t = allowed_events.get_mut("").unwrap();
                    (!t.is_empty())
                        .then(|| t.swap_remove(self.rng.borrow_mut().gen_range(0..t.len())))
                }
                Some(evs) => {
                    let mut e = evs.collect::<Vec<EventBorrow>>();
                    e.shuffle(&mut *self.rng.borrow_mut());
                    e.iter().find_map(|&a| {
                        allowed_events.get_mut(a.name()).and_then(|t| {
                            (!t.is_empty())
                                .then(|| t.swap_remove(self.rng.borrow_mut().gen_range(0..t.len())))
                        })
                    })
                }
            }
            .map(|pick| {
                for (_, v) in allowed_events.iter_mut() {
                    if let Some(pos) = v.iter().position(|&r| r.meta.id == pick.meta.id) {
                        v.remove(pos);
                        continue;
                    }
                }

                BaseLink {
                    target: pick.meta.id,
                    link_type: link.name().to_string(),
                }
            })
        };

        let mut generated_links = Vec::new();

        // First do the required links ONCE
        generated_links.extend(
            meta_event
                .links()
                .filter(|(_, required)| *required)
                .map(|(link, _)| select_event(link).unwrap()), // unwrap safe because we checked that we can generate this event
        );

        let mut generatable_events: Vec<(LinkBorrow, bool)> = meta_event
            .links()
            .filter(|(link, required)| !*required || link.multiple_allowed())
            .collect();

        let target_amount = self
            .rng
            .borrow_mut()
            .gen_range(0..self.inner.max_links - generated_links.len());

        // List of possible eiffel events
        let event_types: [&str; 23] = [
            "ActC", "ActF", "ActS", "ActT", "AnnP", "ArtC", "ArtP", "ArtR", "TCC", "TCF", "TCS",
            "TCT", "TERCC", "TSF", "TSS", "CD", "CLM", "ED", "FCD", "ID", "IV", "SCC", "SCS",
        ];

        // Random number generator used to select a random index of the above
        fn random_num(min: usize, max: usize) -> usize {
            let mut rng = rand::thread_rng();
            rng.gen_range(min..max)
        }

        // Randomly pick a link and select an event for it until we reach our target or we exhaust possible events
        while let Some((i, (link, _))) = {
            let ret = generatable_events
                .iter()
                .enumerate()
                .choose(&mut *self.rng.borrow_mut());
            ret
        } {
            if generated_links.len() >= target_amount {
                break;
            }

            let exhausted = if let Some(bl) = select_event(*link) {
                generated_links.push(bl);
                !link.multiple_allowed()
            } else {
                true
            };

            if exhausted {
                generatable_events.swap_remove(i);
            }
        }

        event.meta.event_type = event_types[random_num(0, 22)].to_string();
        event.meta.id = Uuid::from_bytes(self.rng.get_mut().gen());

        event.meta.time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        event.links = generated_links;

        let event_bytes = serde_json::to_vec(&event).unwrap();

        self.history.push_back(event);

        if self.history.len() > self.inner.history_max {
            self.history.pop_front();
        }

        Some(event_bytes)
    }
}
