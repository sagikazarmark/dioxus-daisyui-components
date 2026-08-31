use dioxus::prelude::*;

use crate::components::context_menu::{
    ContextMenu, ContextMenuContent, ContextMenuContentAppearance, ContextMenuTrigger,
};
use crate::examples::context_menu::items::Items;

/// A caller's own classes on the box, held open the way the axis rows are.
///
/// The first menu keeps this component's utilities and adds a width to them,
/// which reaches the items too: the list is stretched to the box, so a box with
/// a width is a menu with one.
///
/// The second switches the box utilities off and paints its own in their place,
/// which is what a caller does when they want to win against a utility rather
/// than against daisyUI (ADR-0004).
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-start gap-4",
            ContextMenu { open: Some(true),
                ContextMenuTrigger { class: "sr-only", "Caller classes" }
                ContextMenuContent { id: "caller-attributes", class: "static! w-56",
                    Items {}
                }
            }

            ContextMenu { open: Some(true),
                ContextMenuTrigger { class: "sr-only", "Caller box" }
                ContextMenuContent {
                    id: "caller-box",
                    appearance: ContextMenuContentAppearance::None,
                    class: "static! bg-primary text-primary-content rounded-none",
                    Items {}
                }
            }
        }
    }
}
