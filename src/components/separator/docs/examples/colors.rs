use dioxus::prelude::*;

use crate::components::separator::{Separator, SeparatorColor};

/// Every value of the colour axis.
///
/// The colour is on the rule rather than on the element: daisyUI draws both
/// halves of the line in `::before` and `::after`, and a colour class paints
/// those. The element itself is transparent under every value, which is what
/// the browser specs read the pseudo-elements for.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex flex-col",
            for color in SeparatorColor::ALL.iter().copied() {
                Separator { color, "{color:?}" }
            }
        }
    }
}
