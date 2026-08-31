use dioxus::prelude::*;

use crate::components::input::{Input, InputColor};

/// Enabled and disabled inputs with identical Component axes.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            Input {
                id: "enabled-input",
                color: InputColor::Primary,
                aria_label: "Enabled input",
                placeholder: "Enabled",
            }
            Input {
                id: "disabled-input",
                color: InputColor::Primary,
                aria_label: "Disabled input",
                placeholder: "Disabled",
                disabled: true,
            }
        }
    }
}
