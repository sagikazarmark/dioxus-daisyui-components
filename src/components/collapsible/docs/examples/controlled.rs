use dioxus::prelude::*;

use crate::components::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

/// A disclosure whose open state belongs to its caller.
///
/// Opening and closing travel out through `on_open_change` and back in through
/// `open`, so nothing happens unless the caller lets it: the button below
/// opens the same panel the title does, and a caller who declined to write the
/// state back would have a title that does nothing.
///
/// This is the lift ADR-0006 describes, and the reason it is a lift rather than
/// a mirror: the primitive's collapsible takes a controlled `open` prop, where
/// an accordion's item does not.
#[component]
pub fn Example() -> Element {
    let mut open = use_signal(|| false);
    let mut changes = use_signal(|| 0_u32);

    rsx! {
        div { class: "flex w-full max-w-md flex-col items-start gap-3",
            button {
                id: "toggle",
                class: "btn btn-sm",
                onclick: move |_| {
                    let next = !open();
                    open.set(next);
                    changes += 1;
                },
                if open() {
                    "Close from outside"
                } else {
                    "Open from outside"
                }
            }

            Collapsible {
                id: "controlled",
                open: Some(open()),
                on_open_change: move |next| {
                    open.set(next);
                    changes += 1;
                },
                CollapsibleTrigger { id: "controlled-trigger", "Release notes" }
                CollapsibleContent { id: "controlled-panel",
                    "Every change in this release, and the one before it."
                }
            }

            p { class: "text-sm opacity-70",
                "Changed "
                span { "data-testid": "changes", "{changes}" }
                " times, now "
                span { "data-testid": "state", if open() { "open" } else { "closed" } }
            }
        }
    }
}
