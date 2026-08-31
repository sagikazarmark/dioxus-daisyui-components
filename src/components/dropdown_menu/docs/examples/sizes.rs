use dioxus::prelude::*;

use crate::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuSize, DropdownMenuTrigger,
};
use crate::examples::dropdown_menu::items::Items;

/// Every value of the menu's size axis, which sizes the items rather than the
/// box; the box has no size of its own, it is whatever the items need.
///
/// It is an axis on the content rather than on the dropdown, because daisyUI's
/// `menu-*` sizes are the menu's.
#[component]
pub fn Example() -> Element {
    rsx! {
        div {
            "data-axis": "size",
            class: "flex flex-wrap items-center gap-24 pb-48",
            for size in DropdownMenuSize::ALL.iter().copied() {
                DropdownMenu { open: Some(true),
                    DropdownMenuTrigger { "{size:?}" }
                    DropdownMenuContent { size,
                        Items {}
                    }
                }
            }
        }
    }
}
