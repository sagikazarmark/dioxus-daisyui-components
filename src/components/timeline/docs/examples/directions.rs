use dioxus::prelude::*;

use crate::components::timeline::{
    Timeline, TimelineConnector, TimelineDirection, TimelineEnd, TimelineItem, TimelineMiddle,
    TimelineStart,
};

/// Every value of the direction Axis, with a fixed item for measurable lines.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "direction", class: "flex flex-wrap items-start gap-6",
            for direction in TimelineDirection::ALL.iter().copied() {
                Timeline { direction,
                    TimelineItem { class: "h-32 w-72",
                        TimelineConnector { aria_hidden: "true" }
                        TimelineStart { "{direction:?}" }
                        TimelineMiddle { aria_hidden: "true",
                            span { class: "block size-3 rounded-full bg-primary" }
                        }
                        TimelineEnd { "Timeline" }
                        TimelineConnector { aria_hidden: "true" }
                    }
                }
            }
        }
    }
}
