use dioxus::prelude::*;

use crate::components::switch::{Switch, SwitchColor};

/// Every value of the colour axis, rendered on.
///
/// On is the state daisyUI expresses a toggle's colour in (its rule is
/// `.toggle-primary[aria-checked=true]`) so a switch that is off renders the
/// same whatever this axis says.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex flex-wrap items-center gap-3",
            for color in SwitchColor::ALL.iter().copied() {
                Switch { color, default_value: true, aria_label: "{color:?}" }
            }
        }
    }
}
