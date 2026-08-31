use dioxus::prelude::*;

use crate::components::textarea::Textarea;

/// The textarea participates in a form through the browser's native behaviour.
#[component]
pub fn Example() -> Element {
    let mut value = use_signal(String::new);
    let mut commits = use_signal(|| 0_u32);
    let mut focus_exits = use_signal(|| 0_u32);

    rsx! {
        form { id: "textarea-form",
            Textarea {
                id: "form-textarea",
                value: ReadSignal::from(value),
                name: "notes",
                aria_label: "Notes",
                placeholder: "Notes",
                required: true,
                on_change: move |next| value.set(next),
                on_commit: move |()| commits += 1,
                on_focus_exit: move |()| focus_exits += 1,
            }
            output { id: "form-value", "{value}" }
            output { "data-testid": "textarea-commits", "{commits}" }
            output { hidden: true, "data-testid": "textarea-focus-exits", "{focus_exits}" }
        }
    }
}
