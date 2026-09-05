use dioxus::prelude::*;
use schemaform_dioxus::use_form;

use crate::components::schemaform_daisyui::SchemaformDaisyui;
use crate::examples::schemaform_daisyui::schemas::{
    CONTROLS_DATA, CONTROLS_SCHEMA, CONTROLS_UI_SCHEMA, definition, form_data,
};

/// Every control kind the renderer presents, and both widget symbols.
///
/// Strings, the integer and the number are `Input`s; the write-only string is
/// a password input labelled with its replacement action. The non-nullable
/// boolean is a native checkbox, the nullable one the registry `Checkbox` with
/// null as its indeterminate state, the write-only one a replacement select
/// over the localized labels. The plan is a native select with the null option
/// as an ordinary option; the billing cycle asks for `daisyui:radio` and the
/// region for `daisyui:select` in the UI schema. The reference is read-only
/// output, the tier a constant. Presence affordances appear where the core
/// allows a presence operation right now.
#[component]
pub fn Example() -> Element {
    let definition = use_hook(|| definition(CONTROLS_SCHEMA, Some(CONTROLS_UI_SCHEMA)));
    let form = use_form(definition, form_data(CONTROLS_DATA)).expect("the form should be created");

    rsx! {
        SchemaformDaisyui { form, on_submit: move |_| {} }
    }
}
