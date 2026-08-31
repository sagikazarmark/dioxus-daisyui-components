use dioxus::prelude::*;

use crate::components::popover::{
    Popover, PopoverContent, PopoverContentAppearance, PopoverTrigger,
};

/// The panel's appearance axis, which is the utilities this component emits
/// where daisyUI has none.
///
/// `dropdown-content` positions the panel and nothing else: the fill, the
/// corners, the padding and the shadow are Tailwind utilities in daisyUI's own
/// examples, so they are emitted here, and `None` emits nothing at all, which
/// is how a caller wins a tie against a utility rather than trying to out-rank
/// it (ADR-0004).
///
/// Both are held open, and neither is modal, so that the two panels can be
/// looked at side by side.
#[component]
pub fn Example() -> Element {
    rsx! {
        div {
            "data-axis": "appearance",
            class: "flex flex-wrap items-center justify-center gap-40 py-24",
            for appearance in PopoverContentAppearance::ALL.iter().copied() {
                Popover { open: Some(true), is_modal: false,
                    PopoverTrigger { "{appearance:?}" }
                    PopoverContent { appearance, class: "w-40", "Panel: {appearance:?}." }
                }
            }
        }
    }
}
