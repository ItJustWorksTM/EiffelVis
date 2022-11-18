use crate::event_set::{Event, EventSet, Link};

use rand::Rng;

pub struct EiffelVocabulary;

const EVENT_TYPES: [&str; 23] = [
    "EiffelActivityStartedEvent",
    "EiffelActivityTriggeredEvent",
    "EiffelActivityCanceledEvent",
    "EiffelActivityFinishedEvent",
    "EiffelArtifactCreatedEvent",
    "EiffelArtifactPublishedEvent",
    "EiffelArtifactReusedEvent",
    "EiffelTestCaseStartedEvent",
    "EiffelTestCaseTriggeredEvent",
    "EiffelTestCaseCanceledEvent",
    "EiffelTestCaseFinishedEvent",
    "EiffelTestSuiteStartedEvent",
    "EiffelTestExecutionRecipeCollectionCreatedEvent",
    "EiffelTestSuiteFinishedEvent",
    "EiffelAnnouncementPublishedEvent",
    "EiffelCompositionDefinedEvent",
    "EiffelConfidenceLevelModifiedEvent",
    "EiffelEnvironmentDefinedEvent",
    "EiffelFlowContextDefinedEvent",
    "EiffelIssueDefinedEvent",
    "EiffelIssueVerifiedEvent",
    "EiffelSourceChangeCreatedEvent",
    "EiffelSourceChangeSubmittedEvent",
];

impl From<EiffelVocabulary> for EventSet {
    fn from(_: EiffelVocabulary) -> Self {
        let mut builder = EventSet::build();

        for eventtype in EVENT_TYPES {
            // Initialize and create the event variable
            let mut event = Event::new(eventtype.to_string(), "1.0.0");

            // Create the random number generate for the links
            let _randomrange = rand::thread_rng().gen_range(1..3);

            // Loop and add the links to the event
            for linknumber in 0.._randomrange {
                event = event.with_link(format!("Link{linknumber}"));
            }

            builder = builder.add_event(event);
        }

        builder
            .add_link(Link::new("Link0", true))
            .add_link(Link::new("Link1", true))
            .add_event(
                Event::new("Event", "1.0.0")
                    .with_link("Link0")
                    .with_link("Link1"),
            )
            .build()
            .expect("This should work")
    }
}
