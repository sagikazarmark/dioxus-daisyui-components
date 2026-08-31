use dioxus::prelude::*;

use crate::components::input::Input;
use crate::components::label::{Label, LabelAppearance};

/// daisyUI's floating label, which wants its control inside it.
///
/// This is the value of the appearance axis that changes the markup rather than
/// only the paint: `.floating-label` reads the placeholder state of the control
/// it contains, and lifts its own `span` out of the field once there is
/// something in it. So the caption is a `span` and the control is a sibling of
/// it, both inside the label, which the component cannot enforce and its
/// documentation records.
///
/// `html_for` is emitted all the same. It is harmless with the control inside,
/// and it keeps the association explicit rather than implied by nesting.
#[component]
pub fn Example() -> Element {
    let filled = use_signal(|| String::from("dropdown_menu"));

    rsx! {
        div { class: "flex flex-col gap-3",
            Label {
                id: "floating-empty",
                appearance: LabelAppearance::Floating,
                html_for: "floating-empty-input",
                span { "Registry URL" }
                Input {
                    id: "floating-empty-input",
                    placeholder: "Registry URL",
                }
            }

            Label {
                id: "floating-filled",
                appearance: LabelAppearance::Floating,
                html_for: "floating-filled-input",
                span { "Component" }
                Input {
                    id: "floating-filled-input",
                    placeholder: "Component",
                    value: ReadSignal::from(filled),
                }
            }
        }
    }
}
