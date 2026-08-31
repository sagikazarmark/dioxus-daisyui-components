use dioxus::prelude::*;

use crate::components::combobox::{Combobox, ComboboxColor, ComboboxInput, ComboboxList};
use crate::examples::combobox::options::Options;

/// Every value of the field's colour axis, read off closed comboboxes: what it
/// colours is the field, not the list under it.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex flex-wrap items-center gap-4",
            for color in ComboboxColor::ALL.iter().copied() {
                Combobox::<String> {
                    ComboboxInput { color, placeholder: "{color:?}" }
                    ComboboxList {
                        Options {}
                    }
                }
            }
        }
    }
}
