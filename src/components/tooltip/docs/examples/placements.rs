use dioxus::prelude::*;

use crate::components::tooltip::{Tooltip, TooltipContent, TooltipSide, TooltipTrigger};

/// Every value of the placement axis, each held open.
///
/// The placement is a class on the tooltip's outer element rather than on the
/// bubble, and it places the tail as well as the bubble, which is why the
/// default value emits one too, where most axes in this registry emit nothing
/// for theirs.
///
/// The row is padded on every side, since each bubble is drawn outside the
/// element it belongs to.
#[component]
pub fn Example() -> Element {
    rsx! {
        div {
            "data-axis": "side",
            class: "flex flex-wrap items-center gap-16 px-20 py-12",
            for side in TooltipSide::ALL.iter().copied() {
                Tooltip { side, default_open: true,
                    TooltipTrigger { class: "text-sm underline decoration-dotted", "{side:?}" }
                    TooltipContent { "{side:?}" }
                }
            }
        }
    }
}
