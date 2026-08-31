use dioxus::prelude::*;

use crate::components::card::{Card, CardBody, CardBorder, CardTitle};

/// Every value of the border Axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "border", class: "flex flex-wrap items-start gap-3",
            for border in CardBorder::ALL.iter().copied() {
                Card { class: "w-48 bg-base-100", border,
                    CardBody {
                        CardTitle { "{border:?}" }
                        p { "A distinct edge." }
                    }
                }
            }
        }
    }
}
