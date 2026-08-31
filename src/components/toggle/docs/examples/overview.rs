use dioxus::prelude::*;

use crate::components::toggle::{Toggle, ToggleColor};

/// Three toggles that answer to nobody, one of them pressed to start with.
///
/// `default_pressed` seeds the state this component lifts, so the third is
/// pressed on the first render rather than after the first click, and daisyUI's
/// active class is on it from that same first render.
///
/// Each is a button: it takes focus, Enter and Space work on it, and it
/// announces its state through `aria-pressed` rather than through the class that
/// paints it.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            Toggle { color: ToggleColor::Primary, aria_label: "Bold", strong { "B" } }
            Toggle { color: ToggleColor::Primary, aria_label: "Italic", em { "I" } }
            Toggle {
                color: ToggleColor::Primary,
                default_pressed: true,
                aria_label: "Underline",
                u { "U" }
            }
        }
    }
}
