pub(crate) mod base_event;

pub mod event_set;
pub mod generator;

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

        let thing = EventGenerator::new(
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
            let event_map: HashMap<Uuid, BaseEvent> = thing
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
