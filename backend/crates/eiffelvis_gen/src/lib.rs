mod event;
mod random;

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::event::Event;
    use crate::random::*;
    use uuid::Uuid;

    #[test]
    fn test() {
        let thing = EventChainBlueprint::new(0..5, 10..11, 0);

        let mut lol: Option<HashMap<Uuid, Event>> = None;

        for _ in 0..10 {
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

                    let time_diff = vec.meta.time - event_map.get(&link.target).unwrap().meta.time;
                    // link age may be unintuitive
                    assert!(time_diff < 11)
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
