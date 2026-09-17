use dioxus::prelude::*;
use schemaform_dioxus::use_form;

use crate::components::schemaform_daisyui::SchemaformDaisyui;
use crate::examples::schemaform_daisyui::schemas::{definition, form_data};

/// Format is a widget annotation, not a validation rule. Unknown formats and
/// numeric controls stay text inputs; write-only strings always use password.
#[component]
pub fn Example() -> Element {
    let definition = use_hook(|| {
        definition(
            r#"{
              "$schema": "https://json-schema.org/draft/2020-12/schema",
              "type": "object",
              "additionalProperties": false,
              "properties": {
                "email": { "type": "string", "format": "email", "title": "Email" },
                "idn-email": { "type": "string", "format": "idn-email", "title": "International email" },
                "uri": { "type": "string", "format": "uri", "title": "Website" },
                "uri-reference": { "type": "string", "format": "uri-reference", "title": "URI reference" },
                "iri": { "type": "string", "format": "iri", "title": "International website" },
                "iri-reference": { "type": "string", "format": "iri-reference", "title": "IRI reference" },
                "date": { "type": "string", "format": "date", "title": "Date" },
                "date-time": { "type": "string", "format": "date-time", "title": "Date and time" },
                "time": { "type": "string", "format": "time", "title": "Time" },
                "unknown": { "type": "string", "format": "custom", "title": "Custom format" },
                "plain": { "type": "string", "title": "Plain text" },
                "number": { "type": "number", "format": "date", "title": "Number" },
                "secret": { "type": "string", "format": "email", "writeOnly": true, "title": "Secret email" }
              }
            }"#,
            None,
        )
    });
    let form = use_form(definition, form_data("{}")) .expect("the form should be created");
    rsx! { SchemaformDaisyui { form, on_submit: move |_| {} } }
}
