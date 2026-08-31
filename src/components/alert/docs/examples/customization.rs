use dioxus::prelude::*;

use crate::components::alert::{Alert, AlertAppearance, AlertColor, AlertDirection};

/// A caller's own class and attributes joined with the alert's.
#[component]
pub fn Example() -> Element {
    rsx! {
        Alert {
            id: "caller-alert",
            class: "rounded-none",
            role: "alert",
            color: AlertColor::Success,
            appearance: AlertAppearance::Outline,
            direction: AlertDirection::Vertical,
            "The caller deliberately made this a live region."
        }
    }
}
