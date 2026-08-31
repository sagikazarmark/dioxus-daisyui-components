use dioxus::prelude::*;

use crate::components::tag_group::{TagColor, TagGroupMulti, TagList, TagOption, TagSize};

/// Every value of the size axis.
///
/// daisyUI's badge sizes set one custom property, which its height and its
/// horizontal padding are both derived from, and a font size beside it. `Default`
/// emits no class and renders at the same size as daisyUI's explicit `badge-md`.
#[component]
pub fn Example() -> Element {
    rsx! {
        TagGroupMulti::<String> {
            TagList { "data-axis": "size",
                for (index , size) in TagSize::ALL.iter().copied().enumerate() {
                    TagOption::<String> {
                        value: "{size:?}",
                        index,
                        size,
                        color: TagColor::Primary,
                        "{size:?}"
                    }
                }
            }
        }
    }
}
