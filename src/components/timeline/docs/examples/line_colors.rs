use dioxus::prelude::*;

use crate::components::timeline::{
    Timeline, TimelineConnector, TimelineContentAppearance, TimelineEnd, TimelineItem,
    TimelineMiddle, TimelineStart,
};

/// Caller background utilities paint individual connector segments.
#[component]
pub fn Example() -> Element {
    rsx! {
        Timeline { class: "w-full overflow-x-auto",
            TimelineItem {
                TimelineStart { "Queued" }
                TimelineMiddle { aria_hidden: "true",
                    span { class: "block size-3 rounded-full bg-primary" }
                }
                TimelineEnd { appearance: TimelineContentAppearance::Box, "Build" }
                TimelineConnector { class: "bg-primary", aria_hidden: "true" }
            }
            TimelineItem {
                TimelineConnector { class: "bg-primary", aria_hidden: "true" }
                TimelineStart { "Running" }
                TimelineMiddle { aria_hidden: "true",
                    span { class: "block size-3 rounded-full bg-primary" }
                }
                TimelineEnd { appearance: TimelineContentAppearance::Box, "Test" }
                TimelineConnector { class: "bg-primary", aria_hidden: "true" }
            }
            TimelineItem {
                TimelineConnector { class: "bg-primary", aria_hidden: "true" }
                TimelineStart { "Next" }
                TimelineMiddle { aria_hidden: "true",
                    span { class: "block size-3 rounded-full bg-base-300" }
                }
                TimelineEnd { appearance: TimelineContentAppearance::Box, "Deploy" }
            }
        }
    }
}
