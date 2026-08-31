use dioxus::prelude::*;

use crate::components::input::Input;

/// The input participates in a form through the browser's native behaviour.
#[component]
pub fn Example() -> Element {
    let mut value = use_signal(String::new);
    let mut commits = use_signal(|| 0_u32);
    let mut focus_exits = use_signal(|| 0_u32);
    let mut commits_seen_by_submit = use_signal(|| 0_u32);

    rsx! {
        form {
            id: "input-form",
            onsubmit: move |event| {
                event.prevent_default();
                commits_seen_by_submit.set(commits());
            },
            Input {
                id: "form-input",
                value: ReadSignal::from(value),
                name: "component",
                aria_label: "Component",
                placeholder: "Component",
                required: true,
                on_change: move |next| value.set(next),
                on_commit: move |()| commits += 1,
                on_focus_exit: move |()| focus_exits += 1,
            }
            output { id: "form-value", "{value}" }
            output { "data-testid": "input-commits", "{commits}" }
            output { hidden: true, "data-testid": "input-focus-exits", "{focus_exits}" }
            output { "data-testid": "input-submit-commits", "{commits_seen_by_submit}" }
        }
    }
}
