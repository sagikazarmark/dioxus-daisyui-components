use dioxus::prelude::*;

use crate::components::loading::{Loading, LoadingAnimation, LoadingSize};

/// Every animation mask, at a size large enough to compare their shapes.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "animation", class: "flex flex-wrap items-center gap-6",
            for animation in LoadingAnimation::ALL.iter().copied() {
                Loading { animation, size: LoadingSize::Lg }
            }
        }
    }
}
