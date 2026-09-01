use dioxus::prelude::*;

use crate::components::native_select::{NativeSelect, NativeSelectColor, NativeSelectOption};

/// Every value of the colour axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex flex-wrap items-center gap-2",
            for color in NativeSelectColor::ALL.iter().copied() {
                NativeSelect {
                    color,
                    aria_label: format!("{color:?}"),
                    placeholder: format!("{color:?}"),
                    options: vec![NativeSelectOption::new(String::from("one"), "One")],
                }
            }
        }
    }
}
