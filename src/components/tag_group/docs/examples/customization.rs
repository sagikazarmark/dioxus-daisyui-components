use dioxus::prelude::*;

use crate::components::tag_group::{
    TagColor, TagGroupMulti, TagList, TagOption, TagOptionAppearance, TagRemoveButton,
};

/// A caller's own classes on the tags.
///
/// The first one keeps this component's utilities and squares off the corner
/// radius daisyUI's `badge` sets, which the caller wins on cascade layers rather
/// than on specificity.
///
/// The second switches the selection utilities off and says what selected looks
/// like for itself, through the same attribute the primitive sets and this
/// component's own utilities read. That is the case the axis exists for: a
/// caller who would rather show selection as a fill than as a ring (ADR-0004).
#[component]
pub fn Example() -> Element {
    rsx! {
        TagGroupMulti::<String> {
            default_values: vec!["squared".to_string(), "filled".to_string()],
            TagList {
                TagOption::<String> {
                    id: "caller-attributes",
                    value: "squared".to_string(),
                    index: 0usize,
                    color: TagColor::Primary,
                    class: "rounded-none",
                    "A squared tag"
                    TagRemoveButton { "×" }
                }

                TagOption::<String> {
                    id: "caller-selection",
                    value: "filled".to_string(),
                    index: 1usize,
                    appearance: TagOptionAppearance::None,
                    class: "cursor-pointer data-[selected=true]:bg-primary data-[selected=true]:text-primary-content",
                    "A filled tag"
                }
            }
        }
    }
}
