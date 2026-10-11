use dioxus::prelude::*;
use schemaform_dioxus::use_form;

use crate::components::schemaform_daisyui::SchemaformDaisyui;
use crate::examples::schemaform_daisyui::schemas::{
    CONTROLS_DATA, CONTROLS_SCHEMA, definition, form_data,
};

/// Every control kind the renderer presents.
///
/// Strings, the integer and the number are `Input`s; the write-only string is
/// a password input labelled with its replacement action. The non-nullable
/// and nullable booleans are native checkboxes, indeterminate while the data
/// holds no boolean: null, or left out as for the last two; the write-only
/// one is a replacement select over the localized labels. The plan is a
/// native select with the null option as an ordinary option. The reference is
/// read-only output, the tier a constant. Presence affordances appear where the core
/// allows a presence operation right now.
#[component]
pub fn Example() -> Element {
    let definition = use_hook(|| definition(CONTROLS_SCHEMA, None));
    let form = use_form(definition, form_data(CONTROLS_DATA)).expect("the form should be created");

    rsx! {
        SchemaformDaisyui { form, on_submit: move |_| {} }
    }
}
