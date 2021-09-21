use std::collections::HashMap;
use std::iter::repeat;

#[derive(Default, Debug)]
pub struct EventSet {
    links: HashMap<String, Link>,
    events: HashMap<String, Event>,
}

impl EventSet {
    pub fn build() -> EventSetBuilder {
        EventSetBuilder::new()
    }

    pub fn events(&self) -> impl Iterator<Item = EventBorrow> {
        self.events
            .values()
            .zip(repeat(self))
            .map(|(ev, s)| EventBorrow {
                event_set: s,
                event: ev,
            })
    }

    pub fn get_event(&self, name: &str) -> Option<EventBorrow> {
        self.events.get(name).map(|ev| EventBorrow {
            event_set: self,
            event: ev,
        })
    }
}

pub type LinkTargets = Option<Vec<String>>;

#[derive(Debug, Clone)]
pub struct Link {
    name: String,
    allow_many: bool,
    targets: LinkTargets,
}

impl Default for Link {
    fn default() -> Self {
        Self {
            name: String::new(),
            allow_many: true,
            targets: None,
        }
    }
}

impl Link {
    pub fn new(name: impl Into<String>, allow_many: bool) -> Self {
        Self {
            name: name.into(),
            allow_many,
            ..Self::default()
        }
    }

    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        match &mut self.targets {
            Some(vec) => vec.push(target.into()),
            None => self.targets = LinkTargets::from(vec![(target.into())]),
        };
        self
    }
}

impl<T: Into<String>> From<T> for Link {
    fn from(str: T) -> Self {
        Self {
            name: str.into(),
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Event {
    name: String,
    version: String,
    links: Vec<(String, bool)>,
}

impl Event {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            ..Self::default()
        }
    }

    pub fn with_link(mut self, link: impl Into<String>) -> Self {
        self.links.push((link.into(), false));
        self
    }

    pub fn with_req_link(mut self, link: impl Into<String>) -> Self {
        self.links.push((link.into(), true));
        self
    }
}

#[derive(Default)]
pub struct EventSetBuilder {
    links: HashMap<String, Link>,
    events: HashMap<String, Event>,
}

impl EventSetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_event_set(mut self, event_set: impl Into<EventSet>) -> Self {
        let event_set: EventSet = event_set.into();
        self.links.extend(event_set.links);
        self.events.extend(event_set.events);
        self
    }

    pub fn add_event(mut self, event: Event) -> Self {
        self.events.insert(event.name.clone(), event);
        self
    }

    pub fn add_link(mut self, link: impl Into<Link>) -> Self {
        let link = link.into();
        self.links.insert(link.name.clone(), link);
        self
    }

    pub fn build(self) -> Option<EventSet> {
        let links_valid = self.links.iter().all(|(_, link)| {
            if let Some(evs) = &link.targets {
                evs.iter().all(|event| self.events.contains_key(event))
            } else {
                true
            }
        });

        let events_valid = self.events.iter().all(|(_, event)| {
            event
                .links
                .iter()
                .all(|(link, _)| self.links.contains_key(link))
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

#[derive(Clone, Copy)]
pub struct EventBorrow<'a> {
    event_set: &'a EventSet,
    event: &'a Event,
}

impl<'a> EventBorrow<'a> {
    pub fn name(&self) -> &'a str {
        self.event.name.as_str()
    }

    pub fn version(&self) -> &'a str {
        self.event.version.as_str()
    }

    pub fn links(&self) -> impl Iterator<Item = (LinkBorrow<'a>, bool)> {
        self.event
            .links
            .iter()
            .zip(repeat(self.event_set))
            .filter_map(|((link, required), e)| {
                e.links.get(link).map(|a| {
                    (
                        LinkBorrow {
                            event_set: e,
                            link: a,
                        },
                        *required,
                    )
                })
            })
    }

    pub fn link_count(&self) -> usize {
        self.event.links.len()
    }

    pub fn link(&self, name: &str) -> Option<&Link> {
        self.event_set.links.get(name)
    }
}

#[derive(Clone, Copy)]
pub struct LinkBorrow<'a> {
    event_set: &'a EventSet,
    link: &'a Link,
}

impl<'a> LinkBorrow<'a> {
    pub fn name(&self) -> &'a str {
        self.link.name.as_str()
    }

    pub fn multiple_allowed(&self) -> bool {
        self.link.allow_many
    }

    pub fn targets(&self) -> Option<impl Iterator<Item = EventBorrow<'a>>> {
        self.link.targets.as_ref().map(|vec| {
            vec.iter()
                .zip(repeat(self.event_set))
                .filter_map(|(event, event_set)| event_set.get_event(event))
        })
    }
}

#[cfg(test)]
mod test {
    use crate::event_set::{Event, EventSet, Link};

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

    #[test]
    fn iterators() {
        let event_set = EventSet::build()
            .add_link(Link::new("0", true))
            .add_link(Link::new("1", true))
            .add_link(Link::new("2", true))
            .add_event(
                Event::new("Event", "1.0.0")
                    .with_link("0")
                    .with_link("1")
                    .with_link("2"),
            )
            .add_event(Event::new("Event2", ""))
            .add_event(Event::new("Event3", ""))
            .build()
            .unwrap();

        let event = event_set.get_event("Event").unwrap();
        let mut iter = event.links();

        assert!(iter.all(|v| matches!(v.0.name(), "0" | "1" | "2")));

        let mut iter = event_set.events();

        assert!(iter.all(|v| matches!(v.name(), "Event" | "Event2" | "Event3")));
    }

    #[test]
    fn get_event() {
        let event_set = EventSet::default();
        assert!(event_set.get_event("No").is_none());
    }
}
