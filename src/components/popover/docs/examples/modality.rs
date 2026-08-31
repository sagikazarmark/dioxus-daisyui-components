use dioxus::prelude::*;

use crate::components::popover::{Popover, PopoverContent, PopoverTrigger};

/// A modal popover and a non-modal one, which differ in where the keyboard can
/// go while the panel is open.
///
/// The modal one traps focus inside the panel: tabbing walks its own controls
/// and does not leave. The other does not, so the keyboard walks on into the
/// page behind it, which is what a panel of reference text beside a form
/// wants, and what a panel of controls does not.
///
/// Both are dismissed the same way, by Escape or by a click outside.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-start gap-4",
            Popover { id: "modal",
                PopoverTrigger { "Modal" }
                PopoverContent { class: "w-64",
                    div { class: "flex flex-col gap-2",
                        p { class: "text-sm", "Focus is trapped in here." }
                        button { id: "modal-first", class: "btn btn-sm", "First" }
                        button { id: "modal-last", class: "btn btn-sm", "Last" }
                    }
                }
            }

            Popover { id: "modeless", is_modal: false,
                PopoverTrigger { "Not modal" }
                PopoverContent { class: "w-64",
                    div { class: "flex flex-col gap-2",
                        p { class: "text-sm", "The keyboard can walk out of here." }
                        button { id: "modeless-only", class: "btn btn-sm", "Only" }
                    }
                }
            }

            button { id: "outside", class: "btn btn-sm btn-ghost", "Outside" }
        }
    }
}
