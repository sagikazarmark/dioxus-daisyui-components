use dioxus::prelude::*;

use crate::components::native_select::{NativeSelect, NativeSelectColor, NativeSelectOption};

/// Enabled and disabled selects with identical Component axes.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            NativeSelect {
                id: "enabled-native-select",
                color: NativeSelectColor::Primary,
                aria_label: "Enabled select",
                placeholder: "Enabled",
                options: vec![NativeSelectOption::new(String::from("one"), "One")],
            }
            NativeSelect {
                id: "disabled-native-select",
                color: NativeSelectColor::Primary,
                aria_label: "Disabled select",
                placeholder: "Disabled",
                disabled: true,
                options: vec![NativeSelectOption::new(String::from("one"), "One")],
            }
        }
    }
}
