use dioxus::prelude::*;

use crate::components::alert::{Alert, AlertAppearance, AlertColor};

/// Every value of the appearance axis on one fixed colour.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "appearance", class: "flex flex-col gap-2",
            for appearance in AlertAppearance::ALL.iter().copied() {
                Alert { color: AlertColor::Info, appearance, "{appearance:?} alert" }
            }
        }
    }
}
