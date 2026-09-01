use dioxus::prelude::*;

use crate::components::native_select::{NativeSelect, NativeSelectOption};

/// A controlled select renders the value the caller holds, and keeps following
/// external writes after the user has picked.
#[component]
pub fn Example() -> Element {
    let mut value: Signal<Option<String>> = use_signal(|| Some(String::from("lemon")));
    let mut changes = use_signal(|| 0_u32);
    let mut commits = use_signal(|| 0_u32);

    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            NativeSelect {
                id: "controlled-native-select",
                aria_label: "Controlled flavor",
                placeholder: "Pick a flavor",
                value: ReadSignal::from(value),
                options: vec![
                    NativeSelectOption::new(String::from("orange"), "Orange"),
                    NativeSelectOption::new(String::from("lemon"), "Lemon"),
                    NativeSelectOption::new(String::from("cherry"), "Cherry"),
                ],
                on_change: move |next| {
                    value.set(next);
                    changes += 1;
                },
                on_commit: move |()| commits += 1,
            }
            button {
                id: "select-cherry-externally",
                r#type: "button",
                class: "btn btn-sm",
                onclick: move |_| value.set(Some(String::from("cherry"))),
                "Select cherry"
            }
            output { "data-testid": "controlled-native-select-value",
                {format!("{:?}", value())}
            }
            output { "data-testid": "native-select-changes", "{changes}" }
            output { "data-testid": "native-select-commits", "{commits}" }
        }
    }
}
