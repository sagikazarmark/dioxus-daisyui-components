use dioxus::prelude::*;

use crate::components::context_menu::{ContextMenu, ContextMenuContent, ContextMenuTrigger};
use crate::examples::context_menu::items::Items;

/// A surface with a menu behind it, and what the last command reported.
///
/// Right-click the surface (or long-press it on a touch screen) and the menu
/// opens where the pointer was. Escape and a click outside dismiss it, the
/// arrow keys move through the commands and skip the disabled one, and Enter
/// runs the one that is focused.
///
/// What the surface looks like is the caller's: this component emits nothing on
/// the trigger, because daisyUI has no class for a right-clickable region.
///
/// Nothing here carries an `id` on the `ContextMenu` itself. The primitive puts
/// one there and then looks the element up by it, to tell a click inside the
/// menu from a click outside, and to suppress scrolling around it, so an `id`
/// of a caller's own lands on top of it and leaves both looking for an element
/// that is not there. The component's documentation records it; the trigger and
/// the menu are addressed instead.
#[component]
pub fn Example() -> Element {
    let mut selected = use_signal(|| String::from("nothing"));

    rsx! {
        div { class: "flex flex-col items-start gap-4",
            ContextMenu {
                ContextMenuTrigger {
                    id: "overview-surface",
                    class: "border-base-300 bg-base-200 rounded-box grid h-32 w-72 place-items-center border border-dashed text-sm",
                    "Right click here"
                }
                ContextMenuContent { id: "overview-menu",
                    Items { on_select: move |value| selected.set(value) }
                }
            }

            p { class: "text-sm opacity-70",
                "Last command: "
                span { "data-testid": "selected", "{selected}" }
            }
        }
    }
}
