use dioxus::prelude::*;

use crate::components::radio_group::{RadioGroup, RadioItem, RadioItemColor};

/// Every value of the colour axis, in one group.
///
/// Nothing is chosen here, which is the state a whole row of colours can be
/// read in: daisyUI puts a radio's colour on its border before anything is
/// chosen and draws the dot with it afterwards, and a group has one chosen item
/// by definition.
#[component]
pub fn Example() -> Element {
    rsx! {
        RadioGroup {
            "data-axis": "color",
            class: "flex-wrap items-center",
            aria_label: "Colours",
            for (index , color) in RadioItemColor::ALL.iter().copied().enumerate() {
                RadioItem {
                    value: "{color:?}",
                    index,
                    color,
                    aria_label: "{color:?}",
                }
            }
        }
    }
}
