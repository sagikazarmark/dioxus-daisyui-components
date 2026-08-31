use dioxus::prelude::*;

use crate::components::status::{Status, StatusColor, StatusSize};

/// Every size, from smallest to largest, using one fixed colour.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-wrap items-center gap-6",
            for size in StatusSize::ALL.iter().copied() {
                Status {
                    color: StatusColor::Primary,
                    size,
                    role: "img",
                    aria_label: "{size:?} status size",
                }
            }
        }
    }
}
