use dioxus::prelude::*;

use crate::components::tag_group::{TagColor, TagGroupMulti, TagList, TagOption};

/// Every value of the colour axis.
///
/// daisyUI's badge colours set `--badge-color` and `--badge-fg`, which paint the
/// fill, the border and the text. The selected ring this component emits reads
/// the first of them back, so a selected tag is ringed in its own colour rather
/// than in one this registry chose.
///
/// `Default` emits no class, which is daisyUI's uncoloured badge rather than a
/// synonym for neutral.
#[component]
pub fn Example() -> Element {
    rsx! {
        TagGroupMulti::<String> {
            TagList { "data-axis": "color",
                for (index , color) in TagColor::ALL.iter().copied().enumerate() {
                    TagOption::<String> { value: "{color:?}", index, color, "{color:?}" }
                }
            }
        }
    }
}
