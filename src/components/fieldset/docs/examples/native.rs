use dioxus::prelude::*;

use crate::components::fieldset::{Fieldset, FieldsetLegend};
use crate::components::label::Label;

/// Native form ownership and the disabled fieldset's first-legend exception.
#[component]
pub fn Example() -> Element {
    rsx! {
        form { id: "fieldset-form" }
        Fieldset {
            id: "disabled-fieldset",
            class: "bg-base-200 border-base-300 rounded-box w-full max-w-sm border p-4",
            disabled: true,
            form: "fieldset-form",
            name: "account-settings",
            FieldsetLegend {
                "Disabled account group"
                input {
                    id: "legend-exception",
                    r#type: "checkbox",
                    aria_label: "Control inside the first legend",
                }
            }
            Label { html_for: "disabled-email", "Recovery email" }
            input {
                id: "disabled-email",
                class: "input w-full",
                r#type: "email",
                name: "account-email",
                value: "reader@example.com",
                form: "fieldset-form",
            }
        }
    }
}
