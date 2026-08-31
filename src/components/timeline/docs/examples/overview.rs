use dioxus::prelude::*;

use crate::components::timeline::{
    Timeline, TimelineConnector, TimelineContentAppearance, TimelineEnd, TimelineItem,
    TimelineMiddle, TimelineStart,
};

/// First, middle, and last connector arrangements with content on both sides.
#[component]
pub fn Example() -> Element {
    rsx! {
        Timeline { id: "overview-timeline", class: "w-full overflow-x-auto",
            TimelineItem { id: "overview-first",
                TimelineStart { "2024" }
                TimelineMiddle { aria_hidden: "true",
                    span { class: "block size-3 rounded-full bg-primary" }
                }
                TimelineEnd { appearance: TimelineContentAppearance::Box, "Prototype" }
                TimelineConnector { id: "overview-first-outgoing", aria_hidden: "true" }
            }
            TimelineItem { id: "overview-middle",
                TimelineConnector { id: "overview-middle-incoming", aria_hidden: "true" }
                TimelineStart { "2025" }
                TimelineMiddle { aria_hidden: "true",
                    span { class: "block size-3 rounded-full bg-secondary" }
                }
                TimelineEnd { appearance: TimelineContentAppearance::Box, "Public beta" }
                TimelineConnector { id: "overview-middle-outgoing", aria_hidden: "true" }
            }
            TimelineItem { id: "overview-last",
                TimelineConnector { id: "overview-last-incoming", aria_hidden: "true" }
                TimelineStart { "2026" }
                TimelineMiddle { aria_hidden: "true",
                    span { class: "block size-3 rounded-full bg-accent" }
                }
                TimelineEnd { appearance: TimelineContentAppearance::Box, "Stable release" }
            }
        }
    }
}
