use dioxus::prelude::*;

use crate::components::radio_group::{RadioGroup, RadioGroupAppearance, RadioItem};

/// Both values of the group's appearance axis.
///
/// daisyUI has no class for a radio group, so the utilities that lay one out
/// are this component's own, and a caller who wants their own layout switches
/// them off rather than out-ranking them, which is what the second value is for
/// (ADR-0004). The group on the right lays itself out in a grid instead.
#[component]
pub fn Example() -> Element {
    let options = ["Low", "Normal", "High"];

    rsx! {
        div { "data-axis": "appearance", class: "flex flex-wrap items-start gap-6",
            for appearance in RadioGroupAppearance::ALL.iter().copied() {
                RadioGroup {
                    appearance,
                    class: if appearance == RadioGroupAppearance::None { "grid grid-cols-3 gap-2" },
                    default_value: "Normal".to_string(),
                    aria_label: "{appearance:?}",
                    for (index , option) in options.into_iter().enumerate() {
                        div { class: "flex items-center gap-2 rounded-box border border-base-300 p-2",
                            RadioItem {
                                value: option.to_string(),
                                index,
                                aria_labelledby: "{appearance:?}-{option}",
                            }
                            span { id: "{appearance:?}-{option}", class: "text-sm", "{option}" }
                        }
                    }
                }
            }
        }
    }
}
