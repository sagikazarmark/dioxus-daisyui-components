use dioxus::prelude::*;

use crate::components::countdown::{Countdown, CountdownDigits, CountdownValue};

/// Unsupported values clamp once, while caller styling and labels remain usable.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-end gap-6",
            Countdown { class: "font-mono text-3xl",
                CountdownValue {
                    id: "below-range",
                    value: -20,
                    style: "/**/ --digits /**/ : 9 !important;",
                }
            }
            Countdown { class: "font-mono text-3xl",
                CountdownValue { id: "above-range", value: 1_200 }
            }
            Countdown { class: "font-mono text-3xl",
                CountdownValue {
                    id: "caller-styled",
                    value: 7,
                    digits: CountdownDigits::Two,
                    aria_label: "7 seconds remaining",
                    aria_live: "off",
                    style: None::<String>,
                    "STYLE": "COLOR : rgb(12, 34, 56); LETTER-SPACING : 0.125rem; --caller-payload: \"left;right\"; --VALUE : 888 !important; /**/ --value /**/ : 888 !important; --digits /**/ : 9 !important;",
                }
            }
        }
    }
}
