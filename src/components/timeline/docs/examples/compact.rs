use dioxus::prelude::*;

use crate::components::timeline::{
    Timeline, TimelineCompact, TimelineConnector, TimelineItem, TimelineMiddle, TimelineStart,
};

/// Every value of the compact Axis, with start content on opposite sides.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "compact", class: "flex flex-wrap items-start gap-6",
            for compact in TimelineCompact::ALL.iter().copied() {
                Timeline { compact,
                    TimelineItem { class: "h-32 w-72",
                        TimelineStart { "{compact:?} content" }
                        TimelineMiddle { aria_hidden: "true",
                            span { class: "block size-3 rounded-full bg-primary" }
                        }
                        TimelineConnector { aria_hidden: "true" }
                    }
                }
            }
        }
    }
}
