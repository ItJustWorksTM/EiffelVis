use std::collections::HashMap;

use crate::meta_event::{Event, Link, LinkTargets};

#[derive(Default)]
pub struct EventSetBuilder {
    links: HashMap<String, Link>,
    events: HashMap<String, Event>,
}

#[derive(Default, Debug)]
pub struct EventSet {
    pub links: HashMap<String, Link>,
    pub events: HashMap<String, Event>,
}

impl EventSet {
    pub fn build() -> EventSetBuilder {
        EventSetBuilder::new()
    }
}

impl EventSetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_event_set(mut self, event_set: impl Into<EventSet>) -> Self {
        let event_set: EventSet = event_set.into();
        println!("{:#?}", event_set);
        self.links.extend(event_set.links);
        self.events.extend(event_set.events);
        self
    }

    pub fn add_event(mut self, event: Event) -> Self {
        self.events.insert(event.name.clone(), event);
        self
    }

    pub fn add_link(mut self, link: Link) -> Self {
        self.links.insert(link.name.clone(), link);
        self
    }

    pub fn build(self) -> Option<EventSet> {
        let links_valid = self.links.iter().all(|(_, link)| {
            if let LinkTargets::Events(evs) = &link.targets {
                evs.iter().all(|event| self.events.contains_key(event))
            } else {
                true
            }
        });

        let events_valid = self.events.iter().all(|(_, event)| {
            event.links.iter().all(|link| self.links.contains_key(link))
                && event
                    .required_links
                    .iter()
                    .all(|link| self.links.contains_key(link))
        });

        if links_valid && events_valid {
            Some(EventSet {
                links: self.links,
                events: self.events,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::EventSet;
    use crate::meta_event::{Event, Link};

    #[test]
    fn typical() {
        assert!(EventSet::build()
            .add_link(Link::new("NLink", false).with_target("N"))
            .add_link(Link::new("AnyLink", true))
            .add_link(Link::new("ReqLink", true))
            .add_event(
                Event::new("Nice", "0.0.0")
                    .with_req_link("NLink")
                    .with_req_link("ReqLink"),
            )
            .add_event(Event::new("N", "0.0.0").with_link("AnyLink"))
            .build()
            .is_some());
    }

    #[test]
    fn missing_link() {
        assert!(EventSet::build()
            .add_event(Event::new("Missing", "Link").with_link("missing :("))
            .build()
            .is_none(),);
    }

    #[test]
    fn missing_event() {
        assert!(EventSet::build()
            .add_link(Link::new("Missing event", true).with_target("missing :("))
            .build()
            .is_none(),);
    }

    struct FromComposite;

    impl From<FromComposite> for EventSet {
        fn from(_: FromComposite) -> Self {
            EventSet::build()
                .add_link(Link::new("Nice", true))
                .build()
                .unwrap()
        }
    }

    #[test]
    fn composite() {
        assert!(EventSet::build()
            .add_event_set(FromComposite)
            .add_event(Event::new("NiceEvent", "1.0.0").with_link("Nice"))
            .build()
            .is_some());
    }
}
