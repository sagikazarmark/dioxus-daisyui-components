use dioxus::prelude::*;

use crate::components::radio_group::{RadioGroup, RadioItem};

/// The two orientations.
///
/// `horizontal` is the primitive's prop: it decides which arrow keys move the
/// selection, and the primitive reports it as `data-orientation`. The group's
/// appearance axis is written against that attribute rather than against the
/// prop, so the layout follows the keys without the two being kept in step by
/// hand.
#[component]
pub fn Example() -> Element {
    let options = ["Low", "Normal", "High"];

    rsx! {
        div { class: "flex flex-col gap-6",
            div { class: "flex flex-col gap-2",
                span { id: "vertical-label", class: "text-sm font-medium", "Vertical, which is the default" }
                RadioGroup {
                    id: "vertical",
                    default_value: "Normal".to_string(),
                    aria_labelledby: "vertical-label",
                    for (index , option) in options.into_iter().enumerate() {
                        div { class: "flex items-center gap-2",
                            RadioItem {
                                value: option.to_string(),
                                index,
                                aria_labelledby: "vertical-{option}",
                            }
                            span { id: "vertical-{option}", class: "text-sm", "{option}" }
                        }
                    }
                }
            }

            div { class: "flex flex-col gap-2",
                span { id: "horizontal-label", class: "text-sm font-medium", "Horizontal" }
                RadioGroup {
                    id: "horizontal",
                    horizontal: true,
                    default_value: "Normal".to_string(),
                    aria_labelledby: "horizontal-label",
                    for (index , option) in options.into_iter().enumerate() {
                        div { class: "flex items-center gap-2",
                            RadioItem {
                                value: option.to_string(),
                                index,
                                aria_labelledby: "horizontal-{option}",
                            }
                            span { id: "horizontal-{option}", class: "text-sm", "{option}" }
                        }
                    }
                }
            }
        }
    }
}
