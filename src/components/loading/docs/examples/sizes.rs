use dioxus::prelude::*;

use crate::components::loading::{Loading, LoadingSize};

/// Every size, from smallest to largest, using one fixed mask.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-wrap items-center gap-6",
            for size in LoadingSize::ALL.iter().copied() {
                Loading { size }
            }
        }
    }
}
