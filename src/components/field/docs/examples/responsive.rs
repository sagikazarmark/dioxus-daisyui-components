use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_field::{Binding, FieldContext, FieldMetaValues, use_field_meta_state};

use crate::components::field::{Field, FieldDescription, FieldLabel};
use crate::components::input::Input;
use crate::components::textarea::Textarea;

/// Long descriptions wrap without widening full-width controls or adjacent fields.
#[component]
pub fn Example() -> Element {
    let input_value = use_signal(String::new);
    let input_binding: Binding<String> = input_value.into();
    let input_meta = use_field_meta_state(FieldMetaValues {
        id: Some(Rc::from("responsive-input")),
        ..FieldMetaValues::default()
    });
    let input_context = use_hook(move || FieldContext::new(input_binding).with_meta(input_meta));

    let textarea_value = use_signal(String::new);
    let textarea_binding: Binding<String> = textarea_value.into();
    let textarea_meta = use_field_meta_state(FieldMetaValues {
        id: Some(Rc::from("responsive-textarea")),
        ..FieldMetaValues::default()
    });
    let textarea_context =
        use_hook(move || FieldContext::new(textarea_binding).with_meta(textarea_meta));

    rsx! {
        div {
            id: "responsive-fields",
            class: "grid w-full max-w-[52rem] grid-cols-1 gap-4 sm:grid-cols-2",
            Field { id: "responsive-input-field", context: input_context,
                FieldLabel { "Deployment region" }
                Input { id: "responsive-input", class: "w-full" }
                FieldDescription {
                    "Choose the region nearest your users to reduce latency while keeping data residency and operational requirements in mind."
                }
            }
            Field { id: "responsive-textarea-field", context: textarea_context,
                FieldLabel { "Implementation notes" }
                Textarea { id: "responsive-textarea", class: "w-full" }
                FieldDescription {
                    "Describe constraints, important tradeoffs, rollout considerations, and anything reviewers should understand before approving this change."
                }
            }
        }
    }
}
