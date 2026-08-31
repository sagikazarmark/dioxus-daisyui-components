use dioxus::prelude::*;

use crate::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuContentAppearance, DropdownMenuTrigger,
};
use crate::examples::dropdown_menu::items::Items;

/// The box's own appearance axis, whose switched-off value leaves the menu
/// unpainted: positioned by daisyUI and drawn by nobody.
///
/// daisyUI's `dropdown-content` only positions the element; the fill, the
/// corners and the shadow are utilities this component emits, and a utility it
/// emits is one a caller can only tie with, so switching them off has to be an
/// axis of its own (ADR-0004).
#[component]
pub fn Example() -> Element {
    rsx! {
        div {
            "data-axis": "appearance",
            class: "flex flex-wrap items-center gap-40 pb-40",
            for appearance in DropdownMenuContentAppearance::ALL.iter().copied() {
                DropdownMenu { open: Some(true),
                    DropdownMenuTrigger { "{appearance:?}" }
                    DropdownMenuContent { appearance,
                        Items {}
                    }
                }
            }
        }
    }
}
