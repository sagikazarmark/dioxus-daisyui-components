use dioxus::prelude::*;

use crate::components::timeline::{
    Timeline, TimelineConnector, TimelineDirection, TimelineEnd, TimelineItem, TimelineMiddle,
    TimelineStart,
};

/// Chronological list semantics with decorative connectors and marker icons.
#[component]
pub fn Example() -> Element {
    rsx! {
        section {
            h3 { id: "release-history", class: "mb-3 font-semibold", "Release history" }
            Timeline {
                direction: TimelineDirection::Vertical,
                aria_labelledby: "release-history",
                TimelineItem { class: "min-h-24",
                    TimelineStart { "June 2025" }
                    TimelineMiddle { aria_hidden: "true",
                        span { class: "block size-3 rounded-full bg-success" }
                    }
                    TimelineEnd { "Public beta" }
                    TimelineConnector { aria_hidden: "true" }
                }
                TimelineItem { class: "min-h-24",
                    TimelineConnector { aria_hidden: "true" }
                    TimelineStart { "August 2026" }
                    TimelineMiddle { aria_hidden: "true",
                        span { class: "block size-3 rounded-full bg-success" }
                    }
                    TimelineEnd { "Stable release" }
                }
            }
        }
    }
}
