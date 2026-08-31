use dioxus::prelude::*;

use crate::components::input::{Input, InputAppearance, InputColor, InputSize};

/// Caller classes and native attributes on the rendered input.
#[component]
pub fn Example() -> Element {
    rsx! {
        Input {
            id: "caller-attributes",
            class: "rounded-none",
            color: InputColor::Primary,
            size: InputSize::Lg,
            appearance: InputAppearance::Ghost,
            r#type: "email",
            name: "contact",
            placeholder: "maintainer@example.com",
            required: true,
        }
    }
}
