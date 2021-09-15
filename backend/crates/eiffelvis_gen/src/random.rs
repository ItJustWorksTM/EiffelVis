use crate::event::{Event, Link};

use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use std::{hash::Hash, ops::Range};
use uuid::Uuid;

#[derive(Clone)]
pub struct EventChainBlueprint {
    links: Range<u32>,
    link_age: Range<u32>,
    seed: Seeder,
}

impl Default for EventChainBlueprint {
    fn default() -> Self {
        Self {
            links: 0..1,
            link_age: 0..1,
            seed: Seeder::from(0),
        }
    }
}

impl EventChainBlueprint {
    pub fn new<T: Hash>(links: Range<u32>, link_age: Range<u32>, seed: T) -> Self {
        Self {
            links,
            link_age,
            seed: Seeder::from(seed),
        }
    }

    pub fn iter(&self) -> EventGenerator {
        EventGenerator::new(self.clone())
    }
}

pub struct EventGenerator {
    blueprint: EventChainBlueprint,
    rng: Pcg64,
    events: Vec<Uuid>,
}

impl EventGenerator {
    fn new(mut blueprint: EventChainBlueprint) -> Self {
        let rng = blueprint.seed.make_rng();
        Self {
            blueprint,
            rng,
            events: Vec::default(),
        }
    }
}

impl Iterator for EventGenerator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let range = &self.blueprint.links;

        let mut event = Event::default();

        let events = self.events.len();
        event.meta.id = Uuid::from_bytes(self.rng.gen());
        event.meta.time = events as i64;
        if range.start < range.end {
            let link_amount =
                (self.rng.gen_range(self.blueprint.links.clone()) as usize).min(events);

            let link_hist = (self.rng.gen_range(self.blueprint.link_age.clone()) as usize)
                .max(link_amount)
                .min(events);

            event.links = self.events[events - link_hist..events]
                .choose_multiple(&mut self.rng, link_amount)
                .map(|a| Link {
                    link_type: "".into(),
                    target: *a,
                })
                .collect();
        }

        let ret = Some(serde_json::to_vec(&event).unwrap());

        self.events.push(event.meta.id);

        ret
    }
}
