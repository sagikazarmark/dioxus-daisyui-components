use dioxus::prelude::*;

use crate::components::timeline::{
    Timeline, TimelineContentAppearance, TimelineEnd, TimelineItem, TimelineMiddle, TimelineStart,
};

/// Every shared content appearance on both sides of a timeline item.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "content-appearance", class: "flex flex-wrap items-start gap-6",
            for appearance in TimelineContentAppearance::ALL.iter().copied() {
                Timeline {
                    TimelineItem { class: "h-32 w-64",
                        TimelineStart { appearance, "{appearance:?} start" }
                        TimelineMiddle { aria_hidden: "true",
                            span { class: "block size-3 rounded-full bg-accent" }
                        }
                        TimelineEnd { appearance, "{appearance:?} end" }
                    }
                }
            }
        }
    }
}
