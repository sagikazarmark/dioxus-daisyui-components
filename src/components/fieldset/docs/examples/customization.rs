use dioxus::prelude::*;

use crate::components::fieldset::{Fieldset, FieldsetLegend};
use crate::components::label::Label;

/// Caller classes and attributes merged onto both Compound parts.
#[component]
pub fn Example() -> Element {
    rsx! {
        Fieldset {
            id: "caller-fieldset",
            class: "border-error rounded-none border-2 p-3",
            title: "Caller fieldset",
            FieldsetLegend {
                id: "caller-legend",
                class: "text-error uppercase",
                title: "Caller legend",
                "Notifications"
            }
            Label {
                id: "caller-label",
                class: "text-error font-bold",
                title: "Caller label",
                html_for: "caller-control",
                "Email updates"
            }
            input { id: "caller-control", class: "input", r#type: "email" }
        }
    }
}
