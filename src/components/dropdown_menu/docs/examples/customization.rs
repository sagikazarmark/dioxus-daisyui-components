use dioxus::prelude::*;

use crate::components::dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuTrigger};
use crate::examples::dropdown_menu::items::Items;

/// A caller's classes on the box, and a trigger the caller renders.
///
/// The width lands on the box and reaches the menu inside it: a width that
/// stopped at the box would leave the items shrink-wrapped in a wider one. It
/// adds rather than overrides, deliberately: what this component emits on the
/// box is Tailwind utilities, and a caller's utility only ties with one of ours.
/// The appearance axis is how a caller wins that outright (ADR-0004).
///
/// The second dropdown renders its own trigger element through the primitive's
/// `as` prop: every attribute the trigger would have carried (daisyUI's class
/// included) arrives at the caller's element, and the label comes from the
/// caller too, since the part's own children never reach it.
///
/// Still a button, and deliberately: daisyUI takes the pointer events off a
/// `[tabindex]` first child while the dropdown is open, so a trigger made
/// focusable by a tabindex rather than by being a button stops answering clicks
/// the moment it has focus.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-start gap-8 pb-48",
            DropdownMenu { open: Some(true),
                DropdownMenuTrigger { "Caller" }
                DropdownMenuContent { id: "caller-attributes", class: "w-52",
                    Items {}
                }
            }

            DropdownMenu { id: "as-trigger",
                DropdownMenuTrigger {
                    r#as: move |attributes: Vec<Attribute>| rsx! {
                        button { ..attributes,
                            span { "Rendered by the caller" }
                        }
                    },
                }
                DropdownMenuContent {
                    Items {}
                }
            }
        }
    }
}
