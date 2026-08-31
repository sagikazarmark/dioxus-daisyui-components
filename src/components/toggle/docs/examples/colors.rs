use dioxus::prelude::*;

use crate::components::toggle::{Toggle, ToggleColor};

/// Every value of the colour axis, rendered pressed.
///
/// Pressed is the state worth showing the colours in: daisyUI's `btn-active`
/// darkens whatever colour the button already carries, so an unpressed row would
/// say what the button component's page already says.
///
/// `Default` emits no class, which is daisyUI's uncoloured button rather than a
/// synonym for neutral.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex flex-wrap items-center gap-2",
            for color in ToggleColor::ALL.iter().copied() {
                Toggle { color, default_pressed: true, "{color:?}" }
            }
        }
    }
}
