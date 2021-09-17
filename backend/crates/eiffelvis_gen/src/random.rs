use crate::{
    event::{Event, Link},
    event_type::EventType,
};

use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use std::{hash::Hash, ops::Range};
use uuid::Uuid;

#[derive(Clone)]
pub struct EventChainBlueprint {
    links: Range<u32>,
    max_link_age: usize,
    seed: Seeder,
    event_types: Vec<EventType>,
}

impl Default for EventChainBlueprint {
    fn default() -> Self {
        Self {
            links: 0..1,
            max_link_age: usize::MAX,
            seed: Seeder::from(0),
            event_types: vec![],
        }
    }
}

impl EventChainBlueprint {
    pub fn new<T: Hash>(links: Range<u32>, max_link_age: usize, seed: T) -> Self {
        Self {
            links,
            max_link_age,
            seed: Seeder::from(seed),
            event_types: vec![],
        }
    }

    pub fn with<T: Into<EventType>>(mut self, ev: T) -> Self {
        self.event_types.push(ev.into());
        self
    }

    pub fn iter(&self) -> EventGenerator {
        EventGenerator::new(self.clone())
    }
}

pub struct EventGenerator {
    blueprint: EventChainBlueprint,
    rng: Pcg64,
    events: Vec<Event>,
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
        let ev = self.blueprint.event_types.choose(&mut self.rng)?;
        let range = &self.blueprint.links;

        let mut event = Event::default();

        let events = self.events.len();

        event.meta.id = Uuid::from_bytes(self.rng.gen());
        event.meta.time = events as i64;
        event.meta.event_type = ev.name.clone();
        event.meta.version = ev.version.clone();

        if range.start < range.end {
            let required_links = ev.links.iter().filter(|l| l.required).count();
            let link_amount = (self.rng.gen_range(self.blueprint.links.clone()) as usize)
                .min(events)
                .max(required_links);

            // TODO: This is kinda wrong logic
            for link in &ev.links {
                event.links.append(
                    &mut self
                        .events
                        .iter()
                        .filter(|e| {
                            link.targets.is_empty()
                                || link.targets.iter().any(|a| *a == e.meta.event_type)
                        })
                        .choose_multiple(&mut self.rng, link_amount)
                        .iter()
                        .map(|a| Link {
                            link_type: link.name.clone(),
                            target: a.meta.id,
                        })
                        .collect(),
                );
            }
        }

        let ret = Some(serde_json::to_vec(&event).unwrap());

        self.events.push(event);

        ret
    }
}
