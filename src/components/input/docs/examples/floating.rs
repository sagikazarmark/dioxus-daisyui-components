use dioxus::prelude::*;

use crate::components::input::Input;
use crate::components::label::{Label, LabelAppearance};

/// `floating-label` belongs to the label around the input.
#[component]
pub fn Example() -> Element {
    rsx! {
        Label {
            appearance: LabelAppearance::Floating,
            html_for: "floating-input",
            span { "Registry URL" }
            Input {
                id: "floating-input",
                placeholder: "Registry URL",
            }
        }
    }
}
