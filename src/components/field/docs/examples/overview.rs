use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_field::{Binding, FieldContext, FieldMetaValues, use_field_meta_state};

use crate::components::field::{Field, FieldDescription, FieldError, FieldLabel};
use crate::components::input::Input;

/// A complete Field whose producer-owned errors can be cleared and restored.
#[component]
pub fn Example() -> Element {
    let value = use_signal(String::new);
    let binding: Binding<String> = value.into();
    let mut meta = use_field_meta_state(FieldMetaValues {
        id: Some(Rc::from("profile-name")),
        name: Some(Rc::from("profile_name")),
        required: true,
        errors: vec![Rc::from("Choose a profile name.")],
        touched: true,
        ..FieldMetaValues::default()
    });
    let context = use_hook(move || FieldContext::new(binding).with_meta(meta));

    rsx! {
        Field { id: "profile-name-field", context, class: "max-w-sm",
            FieldLabel { id: "profile-name-label", "Profile name" }
            Input { placeholder: "daisy-user" }
            FieldDescription {
                "This is shown on your public profile."
            }
            FieldError {}
        }
        button {
            id: "toggle-profile-name-validity",
            r#type: "button",
            class: "btn btn-sm mt-3",
            onclick: move |_| {
                if meta.invalid() {
                    meta.set_errors(Vec::new());
                } else {
                    meta.set_errors(vec![Rc::from("Choose a profile name.")]);
                }
            },
            "Toggle validity"
        }
    }
}
