use dioxus::prelude::*;

use crate::components::combobox::{Combobox, ComboboxInput, ComboboxList, ComboboxSide};
use crate::examples::combobox::options::Options;

/// Every value of the placement axis, each popup held open by its caller.
///
/// Held open rather than opened by hand: the primitive closes a popup whose
/// field has lost focus, so a row of popups standing side by side is a row whose
/// caller pins them.
#[component]
pub fn Example() -> Element {
    rsx! {
        div {
            "data-axis": "side",
            class: "flex flex-wrap items-center justify-center gap-40 py-40",
            for side in ComboboxSide::ALL.iter().copied() {
                Combobox::<String> { side, open: Some(true),
                    ComboboxInput { class: "w-40", placeholder: "{side:?}" }
                    ComboboxList {
                        Options {}
                    }
                }
            }
        }
    }
}
