use dioxus::prelude::*;

use crate::components::alert::{Alert, AlertColor};

/// Every value of the colour axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex flex-col gap-2",
            for color in AlertColor::ALL.iter().copied() {
                Alert { color, "{color:?} alert" }
            }
        }
    }
}
