use dioxus::prelude::*;

use crate::components::fieldset::{Fieldset, FieldsetLegend};
use crate::components::label::Label;

/// A plain fieldset whose first direct legend supplies its accessible name.
#[component]
pub fn Example() -> Element {
    rsx! {
        Fieldset { id: "default-fieldset",
            FieldsetLegend { id: "default-legend", "Project" }
            Label { id: "default-label", html_for: "default-project", "Project name" }
            input {
                id: "default-project",
                class: "input",
                name: "project",
                placeholder: "dioxus-daisyui",
            }
        }
    }
}
