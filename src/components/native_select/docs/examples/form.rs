use dioxus::prelude::*;

use crate::components::native_select::{NativeSelect, NativeSelectOption};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Status {
    Pending,
    Active,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Flavor {
    Orange,
    Lemon,
}

/// The select participates in a form through the browser's native behaviour:
/// an explicit form value submits its own string, a positional option submits
/// its index.
#[component]
pub fn Example() -> Element {
    let mut value: Signal<Option<Status>> = use_signal(|| None);
    let mut commits = use_signal(|| 0_u32);
    let mut focus_exits = use_signal(|| 0_u32);
    let mut commits_seen_by_submit = use_signal(|| 0_u32);

    rsx! {
        form {
            id: "native-select-form",
            class: "flex flex-wrap items-center gap-2",
            onsubmit: move |event| {
                event.prevent_default();
                commits_seen_by_submit.set(commits());
            },
            NativeSelect {
                id: "form-native-select",
                name: "status",
                aria_label: "Status",
                placeholder: "Pick a status",
                required: true,
                options: vec![
                    NativeSelectOption::new(Status::Pending, "Pending").form_value("pending"),
                    NativeSelectOption::new(Status::Active, "Active").form_value("active"),
                ],
                on_change: move |next| value.set(next),
                on_commit: move |()| commits += 1,
                on_focus_exit: move |()| focus_exits += 1,
            }
            NativeSelect {
                id: "form-positional-native-select",
                name: "flavor",
                aria_label: "Flavor",
                default_value: Flavor::Lemon,
                options: vec![
                    NativeSelectOption::new(Flavor::Orange, "Orange"),
                    NativeSelectOption::new(Flavor::Lemon, "Lemon"),
                ],
            }
            output { id: "native-select-form-value", {format!("{:?}", value())} }
            output { "data-testid": "form-native-select-commits", "{commits}" }
            output { hidden: true, "data-testid": "form-native-select-focus-exits",
                "{focus_exits}"
            }
            output { "data-testid": "form-native-select-submit-commits",
                "{commits_seen_by_submit}"
            }
        }
    }
}
