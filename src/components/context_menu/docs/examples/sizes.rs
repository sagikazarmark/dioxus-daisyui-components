use dioxus::prelude::*;

use crate::components::context_menu::{
    ContextMenu, ContextMenuContent, ContextMenuSize, ContextMenuTrigger,
};
use crate::examples::context_menu::items::Items;

/// Every value of the size axis, one menu per value and all of them held open.
///
/// Two things the preview does here a consumer would not. The menus are
/// **controlled** open, because the primitive closes a menu that nothing in it
/// is focused: a menu that stands open on a page is one its caller pins. And
/// each menu is pulled back into the flow with `static!`, because a real one is
/// pinned to the pointer with an inline `position: fixed`, and a row of menus
/// that were all pinned to a pointer that never moved would be a row of menus
/// stacked in the corner of the viewport.
///
/// daisyUI's menu sizes the items rather than the box, which is why the boxes
/// grow with them rather than instead of them.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-wrap items-start gap-4",
            for size in ContextMenuSize::ALL.iter().copied() {
                ContextMenu { open: Some(true),
                    ContextMenuTrigger { class: "sr-only", "{size:?}" }
                    ContextMenuContent { size, class: "static!",
                        Items {}
                    }
                }
            }
        }
    }
}
