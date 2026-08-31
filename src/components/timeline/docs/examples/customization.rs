use dioxus::prelude::*;

use crate::components::timeline::{
    Timeline, TimelineCompact, TimelineConnector, TimelineContentAppearance, TimelineDirection,
    TimelineEnd, TimelineItem, TimelineMiddle, TimelineSnap, TimelineStart,
};

/// Caller classes and attributes joined with all six parts.
#[component]
pub fn Example() -> Element {
    rsx! {
        Timeline {
            id: "caller-timeline",
            class: "min-h-64",
            "data-owner": "caller-timeline",
            direction: TimelineDirection::Vertical,
            compact: TimelineCompact::Compact,
            snap: TimelineSnap::Icon,
            TimelineItem {
                id: "caller-item",
                class: "min-h-28",
                "data-owner": "caller-item",
                TimelineConnector {
                    id: "caller-connector-incoming",
                    class: "bg-primary",
                    "data-owner": "caller-connector-incoming",
                    aria_hidden: "true",
                }
                TimelineStart {
                    id: "caller-start",
                    class: "font-semibold",
                    "data-owner": "caller-start",
                    appearance: TimelineContentAppearance::Box,
                    "Caller start"
                }
                TimelineMiddle {
                    id: "caller-middle",
                    class: "text-primary",
                    "data-owner": "caller-middle",
                    aria_hidden: "true",
                    span { class: "block size-3 rounded-full bg-current" }
                }
                TimelineEnd {
                    id: "caller-end",
                    class: "italic",
                    "data-owner": "caller-end",
                    appearance: TimelineContentAppearance::Box,
                    "Caller end"
                }
                TimelineConnector {
                    id: "caller-connector-outgoing",
                    class: "bg-secondary",
                    "data-owner": "caller-connector-outgoing",
                    aria_hidden: "true",
                }
            }
        }
    }
}
