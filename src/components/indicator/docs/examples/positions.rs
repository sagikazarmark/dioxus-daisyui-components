use dioxus::prelude::*;

use crate::components::indicator::{Indicator, IndicatorBlock, IndicatorInline, IndicatorItem};

/// All nine positions, rendered as multiple items on one container.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "px-8 py-6",
            Indicator { id: "position-grid",
                for (block_index , block) in IndicatorBlock::ALL.iter().copied().enumerate() {
                    for (inline_index , inline) in IndicatorInline::ALL.iter().copied().enumerate() {
                        IndicatorItem {
                            inline,
                            block,
                            "data-inline": "{inline:?}",
                            "data-block": "{block:?}",
                            class: "badge badge-secondary size-7 p-0",
                            "{block_index * 3 + inline_index + 1}"
                        }
                    }
                }
                div {
                    id: "position-target",
                    class: "bg-base-300 grid h-40 w-64 place-items-center rounded-box",
                    "All nine positions"
                }
            }
        }
    }
}
