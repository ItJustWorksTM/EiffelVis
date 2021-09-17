#[derive(Default, Clone, Debug)]
pub struct EventType {
    pub name: String,
    pub version: String,
    pub links: Vec<LinkType>,
}

#[derive(Default, Clone, Debug)]
pub struct LinkType {
    pub name: String,
    pub targets: Vec<String>,
    pub allow_many: bool,
    pub required: bool,
}

impl LinkType {
    fn new(name: String, targets: Vec<String>, allow_many: bool, required: bool) -> Self {
        Self {
            name,
            targets,
            allow_many,
            required,
        }
    }
}

#[macro_export]
macro_rules! link_type {
    ($name:ident, $many:literal, $requ:literal, $($targ:ty),*) => {
        pub struct $name;

        #[allow(clippy::from_over_into)]
        impl Into<crate::event_type::LinkType> for $name {
            fn into(self) ->crate::event_type::LinkType {
                crate::event_type::LinkType {
                    name: String::from(stringify!($name)),
                    targets: vec![$(String::from((stringify!($targ)))),*],
                    allow_many: $many,
                    required: $requ,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! event_type {
    ($name:ident, $ver:literal, $($link_types:expr),*) => {
        pub struct $name;

        #[allow(clippy::from_over_into)]
        impl Into<crate::event_type::EventType> for $name {
            fn into(self) -> crate::event_type::EventType {
                crate::event_type::EventType {
                    name: String::from(stringify!($name)),
                    version: $ver.into(),
                    links: vec![$(($link_types).into()),*],
                }
            }
        }
    };
}
