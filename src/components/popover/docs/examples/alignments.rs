use dioxus::prelude::*;

use crate::components::popover::{Popover, PopoverAlign, PopoverContent, PopoverTrigger};

/// Every value of the alignment axis, which is where the panel sits along the
/// side it opened on.
///
/// The two axes compose the way daisyUI's classes do: on a vertical placement
/// the alignment moves the panel across the trigger, and on a horizontal one it
/// moves the panel up and down it. These are all on the default placement, so
/// what changes is the panel's edge against the trigger's.
#[component]
pub fn Example() -> Element {
    rsx! {
        div {
            "data-axis": "align",
            class: "flex flex-wrap items-center justify-center gap-40 py-24",
            for align in PopoverAlign::ALL.iter().copied() {
                Popover {
                    align,
                    open: Some(true),
                    is_modal: false,
                    PopoverTrigger { "{align:?}" }
                    PopoverContent { class: "w-40", "Aligned to the {align:?}." }
                }
            }
        }
    }
}
