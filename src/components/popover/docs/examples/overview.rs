use dioxus::prelude::*;

use crate::components::popover::{Popover, PopoverContent, PopoverTrigger};

/// A popover that owns its own state: a trigger, and a panel of whatever the
/// caller wrote.
///
/// Nothing is prescribed inside the panel (no list, no items) which is what
/// separates this from the dropdown menu it borrows its box from. What the
/// primitive adds is the dismissal a `details` element has none of: Escape and a
/// click outside both close it, and focus is trapped inside while it is open.
#[component]
pub fn Example() -> Element {
    rsx! {
        Popover { id: "overview",
            PopoverTrigger { "Filters" }
            PopoverContent { class: "w-72",
                div { class: "flex flex-col gap-2",
                    h3 { class: "font-semibold", "Filters" }
                    p { class: "text-sm opacity-70",
                        "A popover holds whatever you write in it: text, controls, a form."
                    }
                    input {
                        id: "overview-search",
                        class: "input input-sm",
                        placeholder: "Name contains",
                    }
                }
            }
        }
    }
}
