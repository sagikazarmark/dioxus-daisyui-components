use dioxus::prelude::*;

use crate::components::textarea::{Textarea, TextareaColor};

/// Enabled and disabled textareas with identical Component axes.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            Textarea {
                id: "enabled-textarea",
                color: TextareaColor::Primary,
                aria_label: "Enabled textarea",
                placeholder: "Enabled",
            }
            Textarea {
                id: "disabled-textarea",
                color: TextareaColor::Primary,
                aria_label: "Disabled textarea",
                placeholder: "Disabled",
                disabled: true,
            }
        }
    }
}
