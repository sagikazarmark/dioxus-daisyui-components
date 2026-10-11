use dioxus::prelude::*;
use schemaform_dioxus::{RenderConfiguration, SchemaForm, use_form};

use crate::components::schemaform_daisyui::{Addon, Options, findings, structure};
use crate::components::schemaform_daisyui_collection::CollectionAddon;
use crate::examples::schemaform_daisyui::schemas::{definition, form_data};
use crate::examples::schemaform_daisyui_collection::schemas::{ARRAYS_DATA, ARRAYS_SCHEMA};

/// The add-on applied to a configuration composed by hand.
///
/// This form keeps the adapter's built-in controls and adopts the daisyUI shell, the collection
/// [`Addon::structure`] puts in the bundle, and the daisyUI presenter in both slots. Structure
/// renderers are fixed when a form is bound, so the configuration is built once.
#[component]
pub fn Example() -> Element {
    let definition = use_hook(|| definition(ARRAYS_SCHEMA, None));
    let form = use_form(definition, form_data(ARRAYS_DATA)).expect("the form should be created");
    let form_to_bind = form.clone();
    let bound = use_hook(move || {
        RenderConfiguration::builder()
            .structure(CollectionAddon.structure(structure(), Options::default()))
            .summary_presenter(findings())
            .local_presenter(findings())
            .build()
            .bind(&form_to_bind)
            .expect("the built-in controls should bind under the daisyUI structure")
    });

    rsx! {
        SchemaForm { form: bound, on_submit: move |_| {} }
    }
}
