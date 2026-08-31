use dioxus::prelude::*;

use crate::components::countdown::{Countdown, CountdownDigits, CountdownValue};

/// Several values and caller separators inside one daisyUI root.
#[component]
pub fn Example() -> Element {
    rsx! {
        Countdown { id: "clock-countdown", class: "font-mono text-4xl",
            CountdownValue { value: 1, digits: CountdownDigits::Two, aria_label: "1 hour" }
            " : "
            CountdownValue { value: 4, digits: CountdownDigits::Two, aria_label: "4 minutes" }
            " : "
            CountdownValue { value: 9, digits: CountdownDigits::Two, aria_label: "9 seconds" }
        }
    }
}
