use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_field::{FieldContext, FieldMetaValues, use_field_meta_state};

use crate::components::field::{
    Field, FieldDescription, FieldDescriptionAppearance, FieldError, FieldLabel,
};

/// Caller classes and attributes merged onto every Compound part.
#[component]
pub fn Example() -> Element {
    let meta = use_field_meta_state(FieldMetaValues {
        id: Some(Rc::from("caller-control")),
        errors: vec![Rc::from("Caller-visible error")],
        ..FieldMetaValues::default()
    });
    let context = use_hook(move || FieldContext::empty().with_meta(meta));

    rsx! {
        Field {
            id: "caller-field",
            context,
            class: "max-w-sm border border-error p-3",
            title: "Caller field",
            FieldLabel {
                id: "caller-label",
                class: "font-bold uppercase",
                title: "Caller label",
                "Account code"
            }
            input { id: "caller-control", class: "input" }
            FieldDescription {
                id: "caller-description",
                appearance: FieldDescriptionAppearance::None,
                class: "min-w-full whitespace-pre-wrap italic",
                title: "Caller description",
                "Use the code from your invitation."
            }
            FieldError {
                id: "caller-error",
                class: "underline",
                title: "Caller error",
            }
        }
    }
}
