use dioxus::prelude::*;

use crate::components::native_select::{NativeSelect, NativeSelectOption};

/// The placeholder is daisyUI's native pattern: a disabled first option,
/// selected while the value is `None` and still listed after a choice is made.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            NativeSelect {
                id: "placeholder-empty",
                aria_label: "Empty placeholder select",
                placeholder: "Pick a flavor",
                options: vec![
                    NativeSelectOption::new(String::from("orange"), "Orange"),
                    NativeSelectOption::new(String::from("lemon"), "Lemon"),
                ],
            }
            NativeSelect {
                id: "placeholder-chosen",
                aria_label: "Chosen placeholder select",
                placeholder: "Pick a flavor",
                default_value: String::from("lemon"),
                options: vec![
                    NativeSelectOption::new(String::from("orange"), "Orange"),
                    NativeSelectOption::new(String::from("lemon"), "Lemon"),
                ],
            }
        }
    }
}
