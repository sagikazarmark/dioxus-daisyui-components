use dioxus::prelude::*;

use crate::components::radio_group::{RadioGroup, RadioItem, RadioItemColor};

/// A group of three options, one of them chosen to begin with.
///
/// The primitive renders a `button` with `role="radio"` for each item rather
/// than an `input`, so a wrapping `label` would not reach it; each option's
/// text is tied to its item with `aria-labelledby`, and the group is named the
/// same way.
#[component]
pub fn Example() -> Element {
    let options = [
        ("daily", "Every day"),
        ("weekly", "Once a week"),
        ("never", "Never"),
    ];

    rsx! {
        div { class: "flex flex-col gap-2",
            span { id: "overview-digest", class: "text-sm font-medium", "Send me a digest" }

            RadioGroup {
                default_value: "weekly".to_string(),
                aria_labelledby: "overview-digest",
                for (index , (value , label)) in options.into_iter().enumerate() {
                    div { class: "flex items-center gap-2",
                        RadioItem {
                            value: value.to_string(),
                            index,
                            color: RadioItemColor::Primary,
                            aria_labelledby: "overview-{value}",
                        }
                        span { id: "overview-{value}", class: "text-sm", "{label}" }
                    }
                }
            }
        }
    }
}
