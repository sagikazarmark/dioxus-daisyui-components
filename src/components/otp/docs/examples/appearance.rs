use dioxus::prelude::*;

use crate::components::otp::{Otp, OtpAppearance};

/// Every value of the appearance axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "appearance", class: "flex flex-wrap items-center gap-4",
            for appearance in OtpAppearance::ALL.iter().copied() {
                Otp {
                    appearance,
                    aria_label: format!("{appearance:?} one-time code"),
                }
            }
        }
    }
}
