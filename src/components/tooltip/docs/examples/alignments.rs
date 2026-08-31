use dioxus::prelude::*;

use crate::components::tooltip::{Tooltip, TooltipAlign, TooltipContent, TooltipTrigger};

/// Every value of the alignment axis, each held open on the same side.
///
/// Alignment is where the bubble sits along the side the placement opened it
/// on, so it is only legible on a trigger wider than the bubble; these are
/// stretched for that reason.
#[component]
pub fn Example() -> Element {
    rsx! {
        div {
            "data-axis": "align",
            class: "flex flex-wrap items-center gap-8 px-8 py-12",
            for align in TooltipAlign::ALL.iter().copied() {
                Tooltip { align, default_open: true,
                    TooltipTrigger { class: "block w-48 rounded-box border border-base-300 p-2 text-center text-sm",
                        "{align:?}"
                    }
                    TooltipContent { "{align:?}" }
                }
            }
        }
    }
}
