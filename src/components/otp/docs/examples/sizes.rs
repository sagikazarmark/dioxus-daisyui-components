use dioxus::prelude::*;

use crate::components::otp::{Otp, OtpSize};

/// Every value of the size axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-wrap items-center gap-3",
            for size in OtpSize::ALL.iter().copied() {
                Otp {
                    size,
                    aria_label: format!("{size:?} one-time code"),
                }
            }
        }
    }
}
