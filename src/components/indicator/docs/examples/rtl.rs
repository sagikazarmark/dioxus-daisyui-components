use dioxus::prelude::*;

use crate::components::indicator::{Indicator, IndicatorBlock, IndicatorInline, IndicatorItem};

/// Inline start and end under both writing directions.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-14 px-8 py-6",
            div { class: "flex flex-col items-center gap-3",
                span { class: "text-sm font-semibold", "LTR" }
                Indicator { id: "ltr-indicator", dir: "ltr",
                    IndicatorItem { id: "ltr-end", class: "badge badge-primary", "End" }
                    IndicatorItem {
                        id: "ltr-start",
                        inline: IndicatorInline::Start,
                        block: IndicatorBlock::Bottom,
                        class: "badge badge-secondary",
                        "Start"
                    }
                    div { class: "bg-base-300 grid h-24 w-40 place-items-center rounded-box", "Content" }
                }
            }

            div { class: "flex flex-col items-center gap-3",
                span { class: "text-sm font-semibold", "RTL" }
                Indicator { id: "rtl-indicator", dir: "rtl",
                    IndicatorItem { id: "rtl-end", class: "badge badge-primary", "End" }
                    IndicatorItem {
                        id: "rtl-start",
                        inline: IndicatorInline::Start,
                        block: IndicatorBlock::Bottom,
                        class: "badge badge-secondary",
                        "Start"
                    }
                    div { class: "bg-base-300 grid h-24 w-40 place-items-center rounded-box", "Content" }
                }
            }
        }
    }
}
