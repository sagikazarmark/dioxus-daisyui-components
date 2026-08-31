use dioxus::prelude::*;

use crate::components::label::Label;
use crate::components::switch::{Switch, SwitchColor};

/// A caption beside the control it names, twice.
///
/// `html_for` points at the control's id, which is what makes the caption
/// clickable and what a screen reader announces the control by. The second one
/// names a `Switch`, which the primitive renders as a `button`: a labelable
/// element, so the association works the same way it does for an input.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3",
            div { class: "flex flex-col gap-1",
                Label { html_for: "overview-project", "Project name" }
                input {
                    id: "overview-project",
                    class: "input",
                    placeholder: "dioxus-daisyui-components",
                }
            }

            div { class: "flex items-center gap-2",
                Switch { id: "overview-telemetry", color: SwitchColor::Primary }
                Label { html_for: "overview-telemetry", "Send anonymous telemetry" }
            }
        }
    }
}
