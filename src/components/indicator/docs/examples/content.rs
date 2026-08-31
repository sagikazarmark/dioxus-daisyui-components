use dioxus::prelude::*;

use crate::components::indicator::{Indicator, IndicatorBlock, IndicatorInline, IndicatorItem};

/// Badge, status, text, and control content written without another Registry Component.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-start gap-10 px-8 py-6",
            Indicator {
                IndicatorItem { class: "badge badge-primary", "New" }
                div { class: "bg-base-300 grid h-24 w-32 place-items-center rounded-box", "Badge" }
            }

            Indicator {
                IndicatorItem {
                    class: "status status-success status-lg",
                    aria_label: "Online",
                }
                div { class: "bg-base-300 grid h-24 w-32 place-items-center rounded-box", "Status" }
            }

            Indicator {
                IndicatorItem {
                    inline: IndicatorInline::Center,
                    block: IndicatorBlock::Middle,
                    class: "bg-neutral text-neutral-content rounded-box px-3 py-1 text-xs",
                    "Members only"
                }
                div { class: "bg-base-300 grid h-24 w-40 place-items-center rounded-box", "Text" }
            }

            Indicator {
                IndicatorItem { block: IndicatorBlock::Bottom,
                    button { class: "btn btn-primary btn-sm", "Apply" }
                }
                div { class: "bg-base-300 grid h-24 w-40 place-items-center rounded-box", "Control" }
            }
        }
    }
}
