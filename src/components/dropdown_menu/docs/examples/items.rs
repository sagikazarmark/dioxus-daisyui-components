use dioxus::prelude::*;

use crate::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};

/// The items every menu on this page holds, the last of them disabled, which
/// is what makes arrow-key navigation skipping it observable.
const ITEMS: &[&str] = &["Edit", "Duplicate", "Archive"];

/// The item none of the menus lets a caller reach.
const DISABLED: &str = "Archive";

/// Those items in a menu, held open so that they can be seen.
///
/// Every other example on this page imports [`Items`] from here rather than
/// writing the loop out again: what they vary is the box around the items.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex pb-40",
            DropdownMenu { open: Some(true),
                DropdownMenuTrigger { "Shared items" }
                DropdownMenuContent {
                    Items {}
                }
            }
        }
    }
}

/// The items every menu on this page holds.
///
/// An item registers by the `index` it is given rather than by where it sits in
/// the DOM, which is what lets daisyUI's own list-item wrappers stand between
/// the menu and its items.
#[component]
pub fn Items(
    /// Called with an item's value when it is selected, which only a menu that
    /// is opened for real has anything to do with.
    #[props(default)]
    on_select: Callback<String>,
) -> Element {
    rsx! {
        for (index , item) in ITEMS.iter().copied().enumerate() {
            DropdownMenuItem::<String> {
                value: item.to_string(),
                index,
                disabled: item == DISABLED,
                on_select,
                "{item}"
            }
        }
    }
}
