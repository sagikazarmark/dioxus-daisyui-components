use dioxus::prelude::*;

use crate::components::otp::Otp;

/// The OTP field participates in a form through native input behaviour.
#[component]
pub fn Example() -> Element {
    let mut value = use_signal(String::new);
    let mut commits = use_signal(|| 0_u32);
    let mut focus_exits = use_signal(|| 0_u32);

    rsx! {
        form { id: "otp-form",
            Otp {
                id: "form-otp",
                name: "code",
                required: true,
                aria_label: "One-time code",
                value: ReadSignal::from(value),
                on_change: move |next| value.set(next),
                on_commit: move |()| commits += 1,
                on_focus_exit: move |()| focus_exits += 1,
            }
            output { id: "otp-value", "{value}" }
            output { "data-testid": "otp-commits", "{commits}" }
            output {
                class: "hidden",
                "data-testid": "otp-focus-exits",
                "{focus_exits}"
            }
        }
    }
}
