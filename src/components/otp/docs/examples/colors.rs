use dioxus::prelude::*;

use crate::components::otp::{Otp, OtpColor};

/// Every value of the colour axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex flex-wrap items-center gap-3",
            for color in OtpColor::ALL.iter().copied() {
                Otp {
                    color,
                    aria_label: format!("{color:?} one-time code"),
                }
            }
        }
    }
}
