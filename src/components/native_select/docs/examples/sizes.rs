use dioxus::prelude::*;

use crate::components::native_select::{NativeSelect, NativeSelectOption, NativeSelectSize};

/// Every value of the size axis, smallest to largest.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-wrap items-center gap-2",
            for size in NativeSelectSize::ALL.iter().copied() {
                NativeSelect {
                    size,
                    aria_label: format!("{size:?}"),
                    placeholder: format!("{size:?}"),
                    options: vec![NativeSelectOption::new(String::from("one"), "One")],
                }
            }
        }
    }
}
