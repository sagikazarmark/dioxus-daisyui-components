use dioxus::prelude::*;

use crate::components::timeline::{
    Timeline, TimelineConnector, TimelineEnd, TimelineItem, TimelineMiddle, TimelineSnap,
};

/// Every value of the snap Axis, with enough item width to expose the offset.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "snap", class: "flex flex-wrap items-start gap-6",
            for snap in TimelineSnap::ALL.iter().copied() {
                Timeline { snap,
                    TimelineItem { class: "h-32 w-72",
                        TimelineMiddle { aria_hidden: "true",
                            span { class: "block size-3 rounded-full bg-secondary" }
                        }
                        TimelineEnd { "{snap:?} marker" }
                        TimelineConnector { aria_hidden: "true" }
                    }
                }
            }
        }
    }
}
