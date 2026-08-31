use dioxus::prelude::*;

use crate::components::fieldset::{Fieldset, FieldsetLegend};
use crate::components::label::Label;

/// Several explicitly labeled controls in a caller-styled fieldset.
#[component]
pub fn Example() -> Element {
    rsx! {
        Fieldset {
            id: "styled-fieldset",
            class: "bg-base-200 border-base-300 rounded-box w-full max-w-sm border p-4",
            FieldsetLegend { "Page details" }

            Label { html_for: "page-title", "Title" }
            input {
                id: "page-title",
                class: "input w-full",
                name: "title",
                placeholder: "My Dioxus app",
            }

            Label { html_for: "page-visibility", "Visibility" }
            select { id: "page-visibility", class: "select w-full", name: "visibility",
                option { value: "public", "Public" }
                option { value: "private", "Private" }
            }
        }
    }
}
