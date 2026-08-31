use dioxus::prelude::*;

use crate::components::file_input::FileInput;

/// A native file picker carrying daisyUI's base class.
#[component]
pub fn Example() -> Element {
    rsx! {
        FileInput {
            id: "default-fileinput",
            aria_label: "Choose a file",
        }
    }
}
