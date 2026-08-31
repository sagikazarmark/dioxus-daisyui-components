use dioxus::prelude::*;

use crate::components::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuTrigger,
};
use crate::examples::dropdown_menu::items::Items;

/// Every value of the alignment axis, which moves a menu along the side the
/// placement axis opened it on.
///
/// Every one of these is placed the default way, so the row only needs room
/// under it.
#[component]
pub fn Example() -> Element {
    rsx! {
        div {
            "data-axis": "align",
            class: "flex flex-wrap items-center justify-center gap-40 pb-40",
            for align in DropdownMenuAlign::ALL.iter().copied() {
                DropdownMenu { align, open: Some(true),
                    DropdownMenuTrigger { "{align:?}" }
                    DropdownMenuContent {
                        Items {}
                    }
                }
            }
        }
    }
}
