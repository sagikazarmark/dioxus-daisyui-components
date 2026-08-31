use dioxus::prelude::*;

use crate::components::toggle::{Toggle, ToggleSize};

/// Every value of the size axis, smallest first.
///
/// `Default` emits no class and renders at the same size as daisyUI's explicit
/// `btn-md`, which is why it sits in the middle of the row rather than at one
/// end of it.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-wrap items-center gap-2",
            for size in ToggleSize::ALL.iter().copied() {
                Toggle { size, default_pressed: true, "{size:?}" }
            }
        }
    }
}
