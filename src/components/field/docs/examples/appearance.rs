use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_field::{FieldContext, FieldMetaValues, use_field_meta_state};

use crate::components::field::{
    Field, FieldAppearance, FieldDescription, FieldDescriptionAppearance, FieldError,
    FieldErrorAppearance,
};

/// Every utility-backed appearance axis. Each `None` value emits nothing.
#[component]
pub fn Example() -> Element {
    let error_meta = use_field_meta_state(FieldMetaValues {
        errors: vec![Rc::from("Producer error")],
        ..FieldMetaValues::default()
    });

    rsx! {
        div { class: "flex flex-col gap-6",
            div { "data-axis": "field-appearance", class: "flex flex-wrap items-start gap-6",
                for appearance in FieldAppearance::ALL.iter().copied() {
                    Field { appearance, context: FieldContext::empty(),
                        span { "{appearance:?}" }
                        span { "Field content" }
                    }
                }
            }

            Field { context: FieldContext::empty(), appearance: FieldAppearance::None,
                div {
                    "data-axis": "description-appearance",
                    class: "grid w-48 grid-cols-2 gap-4",
                    for appearance in FieldDescriptionAppearance::ALL.iter().copied() {
                        FieldDescription {
                            appearance,
                            "Supporting prose wraps"
                        }
                    }
                }
            }

            div { "data-axis": "error-appearance", class: "flex flex-wrap items-start gap-6",
                for appearance in FieldErrorAppearance::ALL.iter().copied() {
                    FieldError {
                        appearance,
                        meta: error_meta,
                    }
                }
            }
        }
    }
}
