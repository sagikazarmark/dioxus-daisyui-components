use dioxus::prelude::*;

use crate::components::theme_controller::{ThemeController, ThemeControllerAppearance};

/// The appearance that emits nothing, inside a control the caller draws.
///
/// daisyUI's icon theme controller is a `swap`, and a swap is markup rather
/// than paint: the classes go on a `label` and on the two glyphs beside the
/// input, and daisyUI reaches them with `.swap input:checked ~ .swap-on`, a
/// sibling of the element this component renders. So the component contributes
/// the one thing only it can, which is a checked input carrying the theme, and
/// the caller writes the rest exactly as daisyUI documents it.
///
/// The label is also what makes the glyphs clickable: `.swap > *` stacks every
/// child in one grid cell and `.swap input` strips the input of its own
/// appearance, so what is on screen is the glyph and what a click reaches is
/// the input under it.
#[component]
pub fn Example() -> Element {
    rsx! {
        label { class: "swap swap-rotate text-3xl",
            ThemeController {
                id: "swap-dark",
                theme: "seafoam",
                appearance: ThemeControllerAppearance::None,
                aria_label: "Seafoam theme",
            }
            span { class: "swap-off", "🌞" }
            span { class: "swap-on", "🌚" }
        }
    }
}
