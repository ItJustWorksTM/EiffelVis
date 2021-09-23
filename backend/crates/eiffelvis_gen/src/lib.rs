//! A library for generating mock eiffel events that actually link back to something.
//!
//! ## Glosary
//!
//! Which events with what links will be generated is dictated by [event_set::EventSet],
//! you can view it as the template that is used to stamp out many events.  
//! To actually generate events an [generator::EventGenerator] is needed with an event set
//! along with a few more rules such as history limit, max link amount and a random seed.
//! ```no_run
//! use eiffelvis_gen::event_set::{EventSet, Link, Event};
//! use eiffelvis_gen::generator::EventGenerator;
//! let my_event_set = EventSet::build()
//!     .add_link(Link::new("Link0", false))
//!     .add_link(Link::new("Link1", true))
//!     .add_event(
//!        Event::new("Event", "1.0.0")
//!            .with_link("Link0")
//!            .with_req_link("Link1"),
//!     )
//!     .build()
//!     .expect("I reference existing links and events");
//!
//! let seed = 123;
//! let max_links = 5;
//! let max_history = 100;
//! let generator = EventGenerator::new(seed, max_links, max_history, my_event_set);
//!
//! /// Events can now be iterated over via .iter()
//! for event in generator.iter().take(100) {
//!     // Do something!   
//! }
//! ```
//! Do note however that it is a never ending iterator; it only ever returns [None] when an impossible to generate [event_set::EventSet] is given.
//! Always use in combination with [Iterator::take] if you want to collect it.
//!

/// Types that are used to describe Events.
pub mod event_set;

/// Generator that stamps out events.
pub mod generator;

#[doc(hidden)]
pub(crate) mod base_event;

#[cfg(test)]
mod test {

    use rand::{thread_rng, Rng};
    use std::collections::HashMap;
    use uuid::Uuid;

    use crate::event_set::{Event, EventSet, Link};
    use crate::generator::EventGenerator;

    use crate::base_event::BaseEvent;

    fn collect_events(gen: &EventGenerator, count: usize) -> Vec<BaseEvent> {
        let ret: Vec<BaseEvent> = gen
            .iter()
            .take(count)
            .map(|bytes| serde_json::from_slice::<BaseEvent>(&bytes).unwrap())
            .collect();
        assert_eq!(ret.len(), count);
        ret
    }

    #[test]
    fn general() {
        let max_links = 20;

        let gen = EventGenerator::new(
            thread_rng().gen::<usize>(),
            max_links,
            usize::MAX,
            EventSet::build()
                .add_link("TestLinkNice")
                .add_event(Event::new("TestEventNice", "2.0.0").with_link("TestLinkNice"))
                .build()
                .unwrap(),
        );

        let mut last_run: Option<HashMap<Uuid, BaseEvent>> = None;

        for _ in 0..2 {
            let event_map: HashMap<Uuid, BaseEvent> = gen
                .iter()
                .take(100)
                .map(|bytes| serde_json::from_slice::<BaseEvent>(&bytes).unwrap())
                .map(|event| (event.meta.id, event))
                .collect();

            // Check if we make valid links
            for vec in event_map.values() {
                assert!(vec.links.len() < max_links);

                for link in &vec.links {
                    assert!(event_map.contains_key(&link.target));
                }

                // Check if the links are unique
                let mut l = vec.links.clone();
                l.sort();
                l.dedup();

                assert_eq!(vec.links.len(), l.len());
            }

            // Check if we are reproducable
            if let Some(events) = last_run {
                assert!(events.keys().all(|k| event_map.contains_key(k)));
            }

            last_run = Some(event_map);
        }
    }

    #[test]
    fn impossible() {
        for _ in 0..10 {
            let gen = EventGenerator::new(
                thread_rng().gen::<usize>(),
                10,
                100,
                EventSet::build()
                    .add_link("Link")
                    .add_event(Event::new("Event", "").with_req_link("Link"))
                    .build()
                    .unwrap(),
            );
            assert!(gen.iter().next().is_none());
        }
    }

    #[test]
    fn required() {
        for _ in 0..10 {
            let gen = EventGenerator::new(
                thread_rng().gen::<usize>(),
                10,
                100,
                EventSet::build()
                    .add_link("Link0")
                    .add_link(Link::new("Link1", false))
                    .add_link(Link::new("Link2", false))
                    .add_event(
                        Event::new("Event", "")
                            .with_link("Link0")
                            .with_req_link("Link1"),
                    )
                    .add_event(Event::new("AnyEvent", "").with_link("Link2"))
                    .build()
                    .unwrap(),
            );
            let events = collect_events(&gen, 100);

            events
                .iter()
                .inspect(|ev| {
                    assert!(ev.links.iter().filter(|l| l.link_type == "Link1").count() <= 1);
                    assert!(ev.links.iter().filter(|l| l.link_type == "Link2").count() <= 1);
                })
                .filter(|ev| ev.meta.event_type == "Event")
                .for_each(|ev| {
                    assert_eq!(ev.meta.event_type, "Event");
                    assert!(
                        ev.links.iter().any(|l| l.link_type == "Link1"),
                        "Failure: {:#?}",
                        ev
                    );
                });

            assert!(events
                .iter()
                .any(|ev| ev.links.iter().all(|l| l.link_type != "Link0")));
        }
    }
}
