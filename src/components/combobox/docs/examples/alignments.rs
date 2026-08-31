use dioxus::prelude::*;

use crate::components::combobox::{Combobox, ComboboxAlign, ComboboxInput, ComboboxList};
use crate::examples::combobox::options::Options;

/// Every value of the alignment axis, which moves a popup along the side the
/// placement opened it on.
#[component]
pub fn Example() -> Element {
    rsx! {
        div {
            "data-axis": "align",
            class: "flex flex-wrap items-center justify-center gap-32 pb-72",
            for align in ComboboxAlign::ALL.iter().copied() {
                Combobox::<String> { align, open: Some(true),
                    ComboboxInput { class: "w-40", placeholder: "{align:?}" }
                    ComboboxList {
                        Options {}
                    }
                }
            }
        }
    }
}
