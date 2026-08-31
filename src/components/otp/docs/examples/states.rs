use dioxus::prelude::*;

use crate::components::otp::{Otp, OtpColor};

/// Enabled and disabled OTP fields with identical Component axes.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-4",
            Otp {
                id: "enabled-otp",
                color: OtpColor::Primary,
                aria_label: "Enabled one-time code",
            }
            Otp {
                id: "disabled-otp",
                color: OtpColor::Primary,
                aria_label: "Disabled one-time code",
                disabled: true,
            }
        }
    }
}
