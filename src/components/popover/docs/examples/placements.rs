use dioxus::prelude::*;

use crate::components::popover::{Popover, PopoverContent, PopoverSide, PopoverTrigger};

/// Every value of the placement axis, one popover per value and all of them held
/// open.
///
/// A popover that has to stand open is one a caller controls, so these are
/// pinned with `open`. They are spaced far apart because a panel is positioned
/// out of the flow: the space it needs is padding on the row rather than height
/// of its own.
///
/// Every value emits a class, the default one included (ADR-0008): an unclassed
/// `dropdown-content` lands where the flow left it rather than against the
/// trigger.
#[component]
pub fn Example() -> Element {
    rsx! {
        div {
            "data-axis": "side",
            class: "flex flex-wrap items-center justify-center gap-40 py-32",
            for side in PopoverSide::ALL.iter().copied() {
                Popover {
                    side,
                    open: Some(true),
                    is_modal: false,
                    PopoverTrigger { "{side:?}" }
                    PopoverContent { class: "w-40", "Panel on the {side:?}." }
                }
            }
        }
    }
}
