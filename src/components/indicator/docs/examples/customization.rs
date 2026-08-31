use dioxus::prelude::*;

use crate::components::indicator::{Indicator, IndicatorBlock, IndicatorInline, IndicatorItem};

/// Caller classes and attributes joined with both parts' own.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "px-8 py-6",
            Indicator {
                id: "caller-indicator",
                class: "rounded-none outline-2 outline-primary outline",
                "data-owner": "caller-root",
                IndicatorItem {
                    id: "caller-item",
                    inline: IndicatorInline::Start,
                    block: IndicatorBlock::Bottom,
                    class: "badge badge-accent italic",
                    "data-owner": "caller-item",
                    aria_label: "Caller item",
                    "Merged"
                }
                IndicatorItem {
                    inline: IndicatorInline::End,
                    block: IndicatorBlock::Top,
                    class: "status status-success status-lg",
                    aria_label: "Online",
                }
                div { class: "bg-base-300 grid h-28 w-48 place-items-center", "Two items" }
            }
        }
    }
}
