use dioxus::prelude::*;

use crate::components::combobox::{Combobox, ComboboxInput, ComboboxList, ComboboxSize};
use crate::examples::combobox::options::Options;

/// Every value of the field's size axis, smallest to largest. The list under it
/// has a size axis of its own, since daisyUI's two are separate.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-wrap items-center gap-4",
            for size in ComboboxSize::ALL.iter().copied() {
                Combobox::<String> {
                    ComboboxInput { size, placeholder: "{size:?}" }
                    ComboboxList {
                        Options {}
                    }
                }
            }
        }
    }
}
