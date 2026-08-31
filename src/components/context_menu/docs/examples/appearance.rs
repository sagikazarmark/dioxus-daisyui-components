use dioxus::prelude::*;

use crate::components::context_menu::{
    ContextMenu, ContextMenuContent, ContextMenuContentAppearance, ContextMenuTrigger,
};
use crate::examples::context_menu::items::Items;

/// The appearance axis, which is the utilities this component emits where
/// daisyUI has no class of its own.
///
/// daisyUI's floating menus are `dropdown-content`, which is a position rather
/// than a look: the fill, the corners and the shadow are utilities beside it in
/// its own examples. A context menu is pinned to the pointer instead, so the
/// position is the primitive's and only the look is left to emit. `None` emits
/// nothing at all, which is how a caller wins a tie against a utility rather
/// than trying to out-rank it (ADR-0004).
///
/// The menus are controlled open and pulled back into the flow with `static!`,
/// for the reasons the size example records.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "appearance", class: "flex flex-wrap items-start gap-4",
            for appearance in ContextMenuContentAppearance::ALL.iter().copied() {
                ContextMenu { open: Some(true),
                    ContextMenuTrigger { class: "sr-only", "{appearance:?}" }
                    ContextMenuContent { appearance, class: "static!",
                        Items {}
                    }
                }
            }
        }
    }
}
