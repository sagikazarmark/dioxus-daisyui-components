use dioxus::prelude::*;

use crate::components::indicator::{Indicator, IndicatorItem};

/// The explicit end/top default, with badge classes fused onto the item.
#[component]
pub fn Example() -> Element {
    rsx! {
        Indicator { id: "default-indicator",
            IndicatorItem { id: "default-item", class: "badge badge-primary", "4" }
            button { class: "btn", "Inbox" }
        }
    }
}
