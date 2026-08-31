use dioxus::prelude::*;

use crate::components::radial_progress::RadialProgress;

/// Endpoints, normalization, and one value changing reactively.
#[component]
pub fn Example() -> Element {
    let mut value = use_signal(|| 20.0_f64);

    rsx! {
        div { class: "flex flex-wrap items-center gap-4",
            RadialProgress { id: "value-zero", value: 0.0, aria_label: "Zero percent", "0%" }
            RadialProgress { id: "value-part", value: 40.0, aria_label: "Forty percent", "40%" }
            RadialProgress { id: "value-full", value: 100.0, aria_label: "One hundred percent", "100%" }
            RadialProgress { id: "value-below", value: -25.0, aria_label: "Clamped to zero", "below" }
            RadialProgress { id: "value-above", value: 140.0, aria_label: "Clamped to one hundred", "above" }
            RadialProgress { id: "value-nan", value: f64::NAN, aria_label: "Not a number normalized to zero", "NaN" }
            RadialProgress { id: "value-negative-infinity", value: f64::NEG_INFINITY, aria_label: "Negative infinity clamped to zero", "-inf" }
            RadialProgress { id: "value-positive-infinity", value: f64::INFINITY, aria_label: "Positive infinity clamped to one hundred", "+inf" }

            div { class: "flex items-center gap-3",
                RadialProgress {
                    id: "value-driven",
                    value,
                    class: "text-accent",
                    aria_label: "Reactive progress",
                    span { "data-testid": "radial-value", "{value}%" }
                }
                button {
                    class: "btn btn-sm",
                    onclick: move |_| value.set(65.0),
                    "Advance radial progress"
                }
            }
        }
    }
}
