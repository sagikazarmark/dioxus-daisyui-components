use dioxus::prelude::*;

use crate::components::countdown::{Countdown, CountdownDigits, CountdownValue};

/// Every digit width, all driven by the same single-digit integer.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "digits", class: "flex flex-wrap items-end gap-6",
            for digits in CountdownDigits::ALL.iter().copied() {
                Countdown { class: "font-mono text-4xl",
                    CountdownValue { value: 4, digits }
                }
            }
        }
    }
}
