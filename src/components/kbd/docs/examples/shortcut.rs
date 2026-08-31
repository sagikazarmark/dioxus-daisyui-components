use dioxus::prelude::*;

use crate::components::kbd::Kbd;

/// A multi-key shortcut composed by the caller.
#[component]
pub fn Example() -> Element {
    rsx! {
        p { id: "shortcut",
            "Open the command palette with "
            Kbd { "Ctrl" }
            " + "
            Kbd { "Shift" }
            " + "
            Kbd { "P" }
            "."
        }
    }
}
