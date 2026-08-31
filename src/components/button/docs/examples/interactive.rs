use dioxus::prelude::*;

use crate::components::button::{Button, ButtonColor};

/// A button that reports its activations, beside a disabled one carrying the
/// very same handler.
///
/// daisyUI draws the disabled look from the `disabled` attribute rather than
/// from a class of its own, and the attribute also makes the button inert, so
/// nothing here is emitted for the state, and the count is what says the
/// disabled button never fired.
#[component]
pub fn Example() -> Element {
    let mut activations = use_signal(|| 0_u32);

    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            Button {
                color: ButtonColor::Primary,
                onclick: move |_| activations += 1,
                "Activate"
            }

            Button { disabled: true, onclick: move |_| activations += 1, "Disabled" }

            Button { onclick: move |_| activations.set(0), "Reset" }

            p { class: "text-sm opacity-70",
                "Activated "
                span { "data-testid": "activations", "{activations}" }
                " times"
            }
        }
    }
}
