pub mod event;

pub mod event_set;
pub mod event_type;
pub mod generator;

#[cfg(test)]
mod test {

    use rand::{thread_rng, Rng};
    use std::collections::HashMap;
    use uuid::Uuid;

    use crate::event_set::EventSet;
    use crate::event_type::{Event, Link};
    use crate::generator::EventGenerator;

    use crate::event::Event as EventValue;

    struct CustomSet;

    impl From<CustomSet> for EventSet {
        fn from(_: CustomSet) -> Self {
            EventSet::build()
                .add_link(Link::new("WOOP", true))
                .build()
                .expect("nice")
        }
    }

    #[test]
    fn test() {
        let max_links = 20;

        let thing = EventGenerator::new(
            thread_rng().gen::<usize>(),
            max_links,
            usize::MAX,
            EventSet::build()
                .add_link(Link::new("TestLinkNice", true))
                .add_event(Event::new("TestEventNice", "2.0.0").with_link("TestLinkNice"))
                .build()
                .unwrap(),
        );

        let mut last_run: Option<HashMap<Uuid, EventValue>> = None;

        for _ in 0..2 {
            println!("---");
            let event_map: HashMap<Uuid, EventValue> = thing
                .iter()
                .take(100)
                .map(|bytes| serde_json::from_slice::<EventValue>(&bytes).unwrap())
                .map(|event| {
                    println!("{:#?}", event);
                    (event.meta.id, event)
                })
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
}
