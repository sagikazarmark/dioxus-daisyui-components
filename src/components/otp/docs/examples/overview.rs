use dioxus::prelude::*;

use crate::components::otp::Otp;

/// A four-digit one-time-code field using daisyUI's visual boxes.
#[component]
pub fn Example() -> Element {
    rsx! {
        Otp {
            id: "default-otp",
            aria_label: "One-time code",
        }
    }
}
