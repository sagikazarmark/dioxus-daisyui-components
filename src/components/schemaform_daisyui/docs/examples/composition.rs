use dioxus::prelude::*;
use schemaform_dioxus::{RenderConfiguration, SchemaForm, use_form};

use crate::components::schemaform_daisyui::{findings, structure};
use crate::examples::schemaform_daisyui::schemas::{
    ARRAYS_DATA, ARRAYS_SCHEMA, definition, form_data,
};

/// The seams composed by hand, when one of them should differ.
///
/// This form keeps the adapter's built-in controls — unstyled semantic HTML a
/// host themes through the adapter's class hooks — and adopts only the daisyUI
/// structure bundle (collection and shell) and the daisyUI presenter in both
/// slots. `controls()`, `structure()` and `findings()` are independent; any
/// slot the bundle does not fill stays the built-in. Structure renderers are
/// fixed when a form is bound, so the configuration is built once.
#[component]
pub fn Example() -> Element {
    let definition = use_hook(|| definition(ARRAYS_SCHEMA, None));
    let form = use_form(definition, form_data(ARRAYS_DATA)).expect("the form should be created");
    let form_to_bind = form.clone();
    let bound = use_hook(move || {
        RenderConfiguration::builder()
            .structure(structure())
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
