use dioxus::prelude::*;
use schemaform_dioxus::use_form;

use crate::components::schemaform_daisyui::{Appearance, SchemaformDaisyui};
use crate::examples::schemaform_daisyui::schemas::{
    OVERVIEW_DATA, OVERVIEW_SCHEMA, definition, form_data,
};

/// Every value of the appearance axis on the same form.
///
/// `Default` emits the Tailwind utilities the package lays itself out with;
/// `None` emits none of them, so what remains is daisyUI's component classes
/// (`fieldset`, `card`, `btn`, `join`, `alert`, `input`, ...) and the layout is
/// the caller's to decide. The same elements, ids and markers render under both
/// values. The axis is fixed when the form is bound, like the renderers.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "appearance", class: "grid gap-8",
            for appearance in Appearance::ALL {
                Variant { appearance }
            }
        }
    }
}

/// One form at `appearance`, under a heading naming the value. Its own component so each
/// variant owns its form; the wrapper is the one element the axis row lists per value.
#[component]
fn Variant(appearance: Appearance) -> Element {
    let definition = use_hook(|| definition(OVERVIEW_SCHEMA, None));
    let form = use_form(definition, form_data(OVERVIEW_DATA)).expect("the form should be created");

    rsx! {
        section { "data-value": "{appearance:?}",
            h3 { class: "mb-2 text-sm font-medium", "{appearance:?}" }
            SchemaformDaisyui { form, appearance, on_submit: move |_| {} }
        }
    }
}
