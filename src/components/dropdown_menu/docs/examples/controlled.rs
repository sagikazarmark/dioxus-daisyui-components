use dioxus::prelude::*;

use crate::components::dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuTrigger};
use crate::examples::dropdown_menu::items::Items;

/// A menu whose open state belongs to its caller, and whose items report what
/// was chosen.
///
/// Opening, dismissal and selection all travel out through callbacks and back
/// in through the open prop, so nothing happens unless the caller lets it.
/// Arrow keys move through the items and skip the disabled one; Escape
/// dismisses and returns focus to the trigger.
#[component]
pub fn Example() -> Element {
    let mut open = use_signal(|| false);
    let mut changes = use_signal(|| 0_u32);
    let mut selected = use_signal(|| String::from("nothing"));

    rsx! {
        div { class: "flex flex-col items-start gap-4",
            DropdownMenu {
                id: "controlled",
                open: Some(open()),
                on_open_change: move |next| {
                    open.set(next);
                    changes += 1;
                },
                DropdownMenuTrigger { "Actions" }
                DropdownMenuContent { id: "controlled-menu",
                    Items { on_select: move |value| selected.set(value) }
                }
            }

            p { class: "text-sm opacity-70",
                "Changed "
                span { "data-testid": "changes", "{changes}" }
                " times, selected "
                span { "data-testid": "selected", "{selected}" }
            }

            // Somewhere for focus to land after the trigger, so that focus
            // coming back to the trigger can be told from focus merely having
            // left the menu.
            a { id: "after", href: "#after", class: "link w-fit", "After the menu" }
        }
    }
}
