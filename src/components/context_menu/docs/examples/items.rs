use dioxus::prelude::*;

use crate::components::context_menu::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
};

/// The commands every menu on this page holds, the last of them disabled,
/// which is what makes arrow-key navigation skipping it observable.
const ITEMS: &[&str] = &["Edit", "Duplicate", "Archive"];

/// The command none of the menus lets a caller reach.
const DISABLED: &str = "Archive";

/// Those commands in a menu, held open so that they can be seen.
///
/// The menu is controlled open because the primitive closes one that nothing in
/// it is focused, and pulled back into the flow with `static!` because a real
/// one is pinned to the pointer with an inline `position: fixed`. Both are the
/// preview showing a menu that would otherwise only exist mid-gesture.
///
/// A disabled item is muted by daisyUI's own `menu-disabled`, which this
/// component emits on the item's wrapper; the primitive reports the state as
/// `data-disabled`, which daisyUI matches nowhere.
#[component]
pub fn Example() -> Element {
    rsx! {
        ContextMenu { open: Some(true),
            ContextMenuTrigger { class: "sr-only", "Shared commands" }
            ContextMenuContent { id: "items", class: "static! w-fit",
                Items {}
            }
        }
    }
}

/// Those commands in a menu.
///
/// Every example on this page imports [`Items`] from here rather than writing
/// the loop out again: what they vary is the box around the items.
///
/// An item registers by the `index` it is given rather than by where it sits in
/// the DOM, which is what lets daisyUI's own list-item wrappers stand between
/// the menu and its items.
#[component]
pub fn Items(
    /// Called with an item's value when it is selected.
    #[props(default)]
    on_select: Callback<String>,
) -> Element {
    rsx! {
        for (index , item) in ITEMS.iter().copied().enumerate() {
            ContextMenuItem {
                value: item.to_string(),
                index,
                disabled: item == DISABLED,
                on_select,
                "{item}"
            }
        }
    }
}
