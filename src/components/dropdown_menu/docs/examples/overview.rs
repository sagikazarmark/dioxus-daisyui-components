use dioxus::prelude::*;

use crate::components::dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuTrigger};
use crate::examples::dropdown_menu::items::Items;

/// A menu that owns its own open state, which is the whole component in three
/// parts: the dropdown, its trigger and the box its items sit in.
///
/// The trigger is a button carrying daisyUI's `btn`, and the box carries
/// `dropdown-content` with a `menu` inside it: daisyUI's two classes across
/// the primitive's two elements (ADR-0005).
#[component]
pub fn Example() -> Element {
    rsx! {
        DropdownMenu {
            DropdownMenuTrigger { "Actions" }
            DropdownMenuContent {
                Items {}
            }
        }
    }
}
