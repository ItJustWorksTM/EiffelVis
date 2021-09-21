#[derive(Debug, Clone)]
pub enum LinkTargets {
    Any,
    Events(Vec<String>),
}

impl Default for LinkTargets {
    fn default() -> Self {
        Self::Any
    }
}

impl From<Vec<String>> for LinkTargets {
    fn from(targets: Vec<String>) -> Self {
        Self::Events(targets)
    }
}

#[derive(Debug, Clone)]
pub struct Link {
    pub name: String,
    pub allow_many: bool,
    pub targets: LinkTargets,
}

impl Default for Link {
    fn default() -> Self {
        Self {
            name: String::new(),
            allow_many: true,
            targets: LinkTargets::default(),
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
            LinkTargets::Events(vec) => vec.push(target.into()),
            LinkTargets::Any => self.targets = LinkTargets::from(vec![(target.into())]),
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
    pub name: String,
    pub version: String,
    pub links: Vec<String>,
    pub required_links: Vec<String>,
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
        self.links.push(link.into());
        self
    }

    pub fn with_req_link(mut self, link: impl Into<String>) -> Self {
        self.required_links.push(link.into());
        self
    }
}
