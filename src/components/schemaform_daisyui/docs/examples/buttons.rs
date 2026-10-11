use dioxus::prelude::*;
use schemaform_dioxus::use_form;

use crate::components::schemaform_daisyui::{ButtonClasses, SchemaformDaisyui};
use crate::examples::schemaform_daisyui::schemas::{
    OVERVIEW_DATA, OVERVIEW_SCHEMA, definition, form_data,
};

/// The host's own classes on the buttons the renderers draw: ghost presence operations and a
/// secondary submit. A role given a class drops the package's colour, size and variant for it, so
/// the classes carry their own sizes. The collection add-on's item and append roles are on its
/// page.
#[component]
pub fn Example() -> Element {
    let definition = use_hook(|| definition(OVERVIEW_SCHEMA, None));
    let form = use_form(definition, form_data(OVERVIEW_DATA)).expect("the form should be created");
    let buttons = ButtonClasses::default()
        .presence("btn-ghost btn-xs")
        .submit("btn-secondary");

    rsx! {
        SchemaformDaisyui { form, buttons, on_submit: move |_| {} }
    }
}
