use dioxus::prelude::*;

use crate::components::tooltip::{Tooltip, TooltipColor, TooltipContent, TooltipTrigger};

/// Every value of the colour axis, each held open.
///
/// A tooltip that is not open renders no bubble at all, so a row of colours is
/// a row of tooltips that were opened by their `default_open` and left that way.
///
/// daisyUI has no `tooltip-neutral`: an unclassed tooltip is already filled with
/// the neutral colour, which is what the first of these is.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex flex-wrap items-center gap-6 pt-12",
            for color in TooltipColor::ALL.iter().copied() {
                Tooltip { color, default_open: true,
                    TooltipTrigger { class: "text-sm underline decoration-dotted", "{color:?}" }
                    TooltipContent { "{color:?}" }
                }
            }
        }
    }
}
