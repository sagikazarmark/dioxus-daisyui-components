use dioxus::prelude::*;

use crate::components::textarea::Textarea;

/// A native multi-line text field carrying daisyUI's base class.
#[component]
pub fn Example() -> Element {
    rsx! {
        Textarea {
            id: "default-textarea",
            aria_label: "Component notes",
            placeholder: "Component notes",
        }
    }
}
