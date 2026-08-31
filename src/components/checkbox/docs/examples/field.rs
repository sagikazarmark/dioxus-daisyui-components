use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_field::{
    Binding, ChangeOrigin, FieldContext, FieldMetaValues, use_field_meta_state,
};

use crate::components::checkbox::CheckboxField;

/// Field Context supplies the Binding and metadata this checkbox resolves.
#[component]
pub fn Example() -> Element {
    let mut checked = use_signal(|| false);
    let mut last_origin = use_signal(|| None);
    let mut commits = use_signal(|| 0);
    let mut focus_exits = use_signal(|| 0);
    let binding = use_hook(move || {
        Binding::new(
            ReadSignal::from(checked),
            Callback::new(move |(next, origin)| {
                checked.set(next);
                last_origin.set(Some(origin));
            }),
            Callback::new(move |()| commits += 1),
        )
        .with_focus_exit(Callback::new(move |()| focus_exits += 1))
    });
    let programmatic_binding = binding.clone();
    let meta = use_field_meta_state(FieldMetaValues {
        name: Some(Rc::from("terms")),
        required: true,
        invalid: Some(true),
        ..FieldMetaValues::default()
    });
    // Field pins this context's focus slot, so keep it stable across value updates.
    let context = use_hook(move || FieldContext::new(binding).with_meta(meta));
    let focus_request = context.focus_request();
    let last_origin = match last_origin() {
        Some(ChangeOrigin::User) => "User",
        Some(ChangeOrigin::Programmatic) => "Programmatic",
        None => "none",
    };
    let state_name = if checked() { "Checked" } else { "Unchecked" };

    rsx! {
        form { id: "field-aware-checkbox-form",
            CheckboxField {
                context,
                label: "Accept the terms",
            }
        }
        button {
            id: "focus-field-aware-checkbox",
            r#type: "button",
            class: "btn btn-sm",
            onclick: move |_| {
                programmatic_binding.write(false, ChangeOrigin::Programmatic);
                focus_request.request();
            },
            "Focus terms"
        }
        output {
            "data-testid": "field-aware-checkbox-value",
            class: "text-sm opacity-70",
            "Current state: {state_name}"
        }
        output {
            "data-testid": "field-aware-checkbox-origin",
            hidden: true,
            "Last origin: {last_origin}"
        }
        output {
            "data-testid": "field-aware-checkbox-commits",
            hidden: true,
            "Commits: {commits}"
        }
        output {
            "data-testid": "field-aware-checkbox-focus-exits",
            hidden: true,
            "Focus exits: {focus_exits}"
        }
    }
}
