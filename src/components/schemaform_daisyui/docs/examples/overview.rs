use dioxus::prelude::*;
use schemaform_dioxus::use_form;

use crate::components::schemaform_daisyui::SchemaformDaisyui;
use crate::examples::schemaform_daisyui::schemas::{
    OVERVIEW_DATA, OVERVIEW_SCHEMA, definition, form_data,
};

/// A form bound through every seam the package fills, in one component.
///
/// The controls render as daisyUI fields, the array as a fieldset of item
/// cards, the shell lays the summary, the body and a primary submit button out,
/// and the finding presenter frames the summary as an alert once something
/// blocks submission — clear the display name and press Submit to see it. The
/// core owns every piece of state; a submission hands back an immutable
/// snapshot, shown below as text.
#[component]
pub fn Example() -> Element {
    let definition = use_hook(|| definition(OVERVIEW_SCHEMA, None));
    let form = use_form(definition, form_data(OVERVIEW_DATA)).expect("the form should be created");
    let mut submitted = use_signal(String::new);

    rsx! {
        div { class: "flex flex-col gap-4",
            SchemaformDaisyui {
                form,
                on_submit: move |snapshot: schemaform::SubmissionSnapshot| {
                    submitted.set(snapshot.form_data().to_string());
                },
            }
            if !submitted().is_empty() {
                pre { class: "text-xs", role: "status", "{submitted}" }
            }
        }
    }
}
