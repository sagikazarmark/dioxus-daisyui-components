use dioxus::prelude::*;

use crate::components::native_select::{
    NativeSelect, NativeSelectAppearance, NativeSelectOption,
};

/// Every value of the appearance axis.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "appearance", class: "flex flex-wrap items-center gap-2",
            for appearance in NativeSelectAppearance::ALL.iter().copied() {
                NativeSelect {
                    appearance,
                    aria_label: format!("{appearance:?}"),
                    placeholder: format!("{appearance:?}"),
                    options: vec![NativeSelectOption::new(String::from("one"), "One")],
                }
            }
        }
    }
}
