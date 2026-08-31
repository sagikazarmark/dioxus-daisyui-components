use dioxus::prelude::*;

use crate::components::popover::{Popover, PopoverContent, PopoverTrigger};

/// The open state lifted all the way out to the caller, which is what this
/// component does internally either way.
///
/// The popover reports through the callback and this example writes the state
/// back, so what is on screen is what the page decided, including the half an
/// uncontrolled popover cannot do, which is a button inside the panel closing
/// it.
#[component]
pub fn Example() -> Element {
    let mut open = use_signal(|| false);
    let mut changes = use_signal(|| 0_u32);

    rsx! {
        div { class: "flex flex-wrap items-center gap-3",
            Popover {
                id: "controlled",
                open: Some(open()),
                on_open_change: move |next| {
                    open.set(next);
                    changes += 1;
                },
                PopoverTrigger { id: "controlled-trigger", "Deploy" }
                PopoverContent { class: "w-64",
                    div { class: "flex flex-col gap-2",
                        p { class: "text-sm", "Ship the current branch?" }
                        button {
                            id: "controlled-confirm",
                            class: "btn btn-sm btn-primary",
                            onclick: move |_| open.set(false),
                            "Ship it"
                        }
                    }
                }
            }

            p { class: "text-sm opacity-70",
                span { "data-testid": "state", if open() { "open" } else { "closed" } }
                " · changed "
                span { "data-testid": "changes", "{changes}" }
                " times"
            }
        }
    }
}
