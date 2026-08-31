use dioxus::prelude::*;

use crate::components::card::{Card, CardBody, CardSize, CardTitle};

/// Every value of the size Axis, smallest to largest.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-wrap items-start gap-3",
            for size in CardSize::ALL.iter().copied() {
                Card { class: "w-44 bg-base-100 shadow-sm", size,
                    CardBody {
                        CardTitle { "{size:?}" }
                        p { "Card body" }
                    }
                }
            }
        }
    }
}
