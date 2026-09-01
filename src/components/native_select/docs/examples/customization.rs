use dioxus::prelude::*;

use crate::components::native_select::{
    NativeSelect, NativeSelectAppearance, NativeSelectColor, NativeSelectOption, NativeSelectSize,
};

/// Caller classes and native attributes on the rendered select.
#[component]
pub fn Example() -> Element {
    rsx! {
        NativeSelect {
            id: "caller-attributes",
            class: "w-64",
            color: NativeSelectColor::Primary,
            size: NativeSelectSize::Lg,
            appearance: NativeSelectAppearance::Ghost,
            name: "flavor",
            aria_label: "Customized flavor",
            placeholder: "Pick a flavor",
            required: true,
            options: vec![
                NativeSelectOption::new(String::from("orange"), "Orange"),
                NativeSelectOption::new(String::from("lemon"), "Lemon"),
            ],
        }
    }
}
