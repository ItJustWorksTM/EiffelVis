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
        let thing = EventChainBlueprint::new(0..5, 19..20, 0);

        let mut lol: Option<HashMap<Uuid, Vec<Uuid>>> = None;

        for _ in 0..10 {
            let event_map: HashMap<Uuid, Vec<Uuid>> = thing
                .iter()
                .take(100)
                .map(|bytes| serde_json::from_slice::<Event>(&bytes).unwrap())
                .map(|event| {
                    println!("{:#?}", event);
                    (
                        event.meta.id,
                        event.links.iter().map(|link| link.target).collect(),
                    )
                })
                .collect();

            // Check if we make valid links
            for vec in event_map.values() {
                for id in vec {
                    assert!(event_map.contains_key(id));
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
