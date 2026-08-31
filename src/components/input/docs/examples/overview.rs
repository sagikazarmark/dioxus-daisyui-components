use dioxus::prelude::*;

use crate::components::input::Input;

/// A native text field carrying daisyUI's base class.
#[component]
pub fn Example() -> Element {
    rsx! {
        Input {
            id: "default-input",
            aria_label: "Component name",
            placeholder: "Component name",
        }
    }
}
