use dioxus::prelude::*;
use schemaform::FormDefinition;
use schemaform_dioxus::use_form;
use serde_json::json;

use crate::components::schemaform_daisyui::{Density, SchemaformDaisyui};

/// Identical absent fields in both layouts; submit reveals the exact object.
#[component]
pub fn Example() -> Element {
    rsx! { div { "data-axis": "density", class: "grid gap-8",
        for density in Density::ALL { Variant { density } }
    } }
}

#[component]
fn Variant(density: Density) -> Element {
    let definition = use_hook(|| FormDefinition::compile(json!({
        "$schema":"https://json-schema.org/draft/2020-12/schema",
        "type":"object","additionalProperties":false,"properties":{
            "name":{"type":"string","title":"Name","minLength":2},
            "active":{"type":"boolean","title":"Active"},
            "tags":{"type":"array","title":"Tags","uniqueItems":true,
                "maxItems":2,
                "items":{"type":"string","enum":["one","two","three"]}}
        }
    })).unwrap());
    let form = use_form(definition, json!({})).unwrap();
    let mut submitted = use_signal(String::new);
    rsx! { section { "data-value": "{density:?}",
        SchemaformDaisyui { form, density, on_submit: move |snapshot: schemaform::SubmissionSnapshot| submitted.set(snapshot.form_data().to_string()) }
        output { "data-submitted": "", "{submitted}" }
    } }
}
