use dioxus::prelude::*;

use crate::components::countdown::{Countdown, CountdownDigits, CountdownValue};

/// A fixed value beside one driven by an ordinary Dioxus signal.
#[component]
pub fn Example() -> Element {
    let mut value = use_signal(|| 8_i32);

    rsx! {
        div { class: "flex flex-wrap items-center gap-5",
            div { class: "flex flex-col gap-1",
                span { class: "text-sm opacity-70", "Fixed" }
                Countdown { id: "fixed-countdown", class: "font-mono text-4xl",
                    CountdownValue { id: "fixed-value", value: 42 }
                }
            }

            div { class: "flex flex-col gap-1",
                span { class: "text-sm opacity-70", "Reactive" }
                Countdown { id: "dynamic-countdown", class: "font-mono text-4xl",
                    CountdownValue {
                        id: "dynamic-value",
                        value: value(),
                        digits: CountdownDigits::Two,
                    }
                }
            }

            button {
                class: "btn btn-sm",
                onclick: move |_| value.set(value() + 1),
                "Advance"
            }
        }
    }
}
