use dioxus::prelude::*;

use crate::components::kbd::{Kbd, KbdSize};

/// Every size, from smallest to largest.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-wrap items-center gap-3",
            for size in KbdSize::ALL.iter().copied() {
                Kbd { size, "{size:?}" }
            }
        }
    }
}
