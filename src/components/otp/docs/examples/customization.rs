use dioxus::prelude::*;

use crate::components::otp::{Otp, OtpAppearance, OtpColor, OtpSize};

/// Caller classes on the row and the endpoints of the supported length range.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-4",
            Otp {
                id: "caller-attributes",
                class: "rounded-none",
                color: OtpColor::Primary,
                size: OtpSize::Lg,
                appearance: OtpAppearance::Joined,
                length: 8,
                inputmode: "text",
                aria_label: "Eight-digit code",
            }
            Otp {
                id: "shortest-otp",
                length: 1,
                aria_label: "One-digit code",
            }
        }
    }
}
