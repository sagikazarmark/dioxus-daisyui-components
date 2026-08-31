use dioxus::prelude::*;

use crate::components::toggle::{Toggle, ToggleColor};

/// Every state a toggle renders in.
///
/// The pressed pair differ in a class this component emits, which is the whole
/// of Tier 2: `aria-pressed` says the same thing to a screen reader either way,
/// and daisyUI reads none of it.
///
/// The disabled pair emit nothing extra at all. The primitive sets the native
/// `disabled` attribute, daisyUI's rule is `.btn:disabled`, and the attribute
/// also makes the button inert, so the look and the behaviour come from the
/// same place.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            Toggle { id: "released", color: ToggleColor::Primary, aria_label: "Released",
                "Released"
            }
            Toggle {
                id: "pressed",
                color: ToggleColor::Primary,
                default_pressed: true,
                aria_label: "Pressed",
                "Pressed"
            }

            Toggle { id: "disabled", disabled: true, aria_label: "Disabled", "Disabled" }
            Toggle {
                id: "disabled-pressed",
                disabled: true,
                default_pressed: true,
                aria_label: "Disabled pressed",
                "Disabled pressed"
            }
        }
    }
}
