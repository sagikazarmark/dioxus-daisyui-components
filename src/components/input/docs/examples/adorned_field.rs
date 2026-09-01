use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_field::{Binding, FieldContext, FieldMetaValues, use_field_meta_state};
use dioxus_primitives::dioxus_attributes::attributes;

use crate::components::input::InputField;

/// The closed happy path for a currency field: the suffix renders inside the
/// control box, with no caller-side `join` wrapper.
///
/// The wrapper owns the visible box, so a bare field's `class: "w-full"` moves
/// to `wrapper_attributes` when an adornment is added; `class` keeps reaching
/// the native input, where `tabular-nums` still belongs.
#[component]
pub fn Example() -> Element {
    let value = use_signal(String::new);
    let binding: Binding<String> = value.into();
    let meta = use_field_meta_state(FieldMetaValues {
        id: Some(Rc::from("amount-input")),
        name: Some(Rc::from("amount")),
        ..FieldMetaValues::default()
    });
    let context = use_hook(move || FieldContext::new(binding).with_meta(meta));
    let mut commits = use_signal(|| 0_u32);
    let mut focus_exits = use_signal(|| 0_u32);

    rsx! {
        InputField {
            context,
            label: "Amount in euros",
            description: "Always positive; the direction carries the sign.",
            class: "tabular-nums",
            inputmode: "decimal",
            placeholder: "0.00",
            suffix: rsx! { "EUR" },
            wrapper_attributes: attributes!(span { class: "w-full" }),
            on_commit: move |()| commits += 1,
            on_focus_exit: move |()| focus_exits += 1,
        }
        output { hidden: true, "data-testid": "amount-commits", "{commits}" }
        output { hidden: true, "data-testid": "amount-focus-exits", "{focus_exits}" }
    }
}
