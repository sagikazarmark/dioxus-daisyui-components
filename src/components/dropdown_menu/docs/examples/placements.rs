use dioxus::prelude::*;

use crate::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuSide, DropdownMenuTrigger,
};
use crate::examples::dropdown_menu::items::Items;

/// Every value of the placement axis, one menu per value and all of them held
/// open.
///
/// The primitive closes a menu that nothing in it is focused, so a row of menus
/// standing open side by side is a row whose caller pins them. They are spaced
/// far apart because a menu is positioned out of the flow: the space it needs
/// is padding on the row rather than height of its own.
#[component]
pub fn Example() -> Element {
    rsx! {
        div {
            "data-axis": "side",
            class: "flex flex-wrap items-center justify-center gap-40 py-40",
            for side in DropdownMenuSide::ALL.iter().copied() {
                DropdownMenu { side, open: Some(true),
                    DropdownMenuTrigger { "{side:?}" }
                    DropdownMenuContent {
                        Items {}
                    }
                }
            }
        }
    }
}
