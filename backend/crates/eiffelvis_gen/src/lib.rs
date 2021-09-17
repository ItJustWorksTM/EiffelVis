pub mod event;
#[macro_use]
pub mod event_type;
pub mod random;

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::event::Event;
    use crate::random::*;
    use uuid::Uuid;

    link_type!(TestLink, true, true, TestEventNice);
    event_type!(TestEvent, "1.0.0", TestLinkNice);

    link_type!(TestLinkNice, true, true, TestEvent);
    event_type!(TestEventNice, "2.0.0", TestLink);

    #[test]
    fn test() {
        let thing = EventChainBlueprint::new(0..5, usize::MAX, 0)
            .with(TestEvent)
            .with(TestEventNice);

        let mut lol: Option<HashMap<Uuid, Event>> = None;

        for _ in 0..2 {
            let event_map: HashMap<Uuid, Event> = thing
                .iter()
                .take(100)
                .map(|bytes| serde_json::from_slice::<Event>(&bytes).unwrap())
                .map(|event| {
                    println!("{:#?}", event);
                    (event.meta.id, event)
                })
                .collect();

            // Check if we make valid links
            for vec in event_map.values() {
                for link in &vec.links {
                    assert!(event_map.contains_key(&link.target));
                }
            }

            // Check if we are reproducable
            if let Some(events) = lol {
                assert!(events.keys().all(|k| event_map.contains_key(k)));
            }

            lol = Some(event_map);
        }
    }
}
