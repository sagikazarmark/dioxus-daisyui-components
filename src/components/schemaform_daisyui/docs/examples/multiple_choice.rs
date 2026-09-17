use dioxus::prelude::*;
use schemaform_dioxus::use_form;

use crate::components::button::Button;
use crate::components::schemaform_daisyui::SchemaformDaisyui;
use crate::examples::schemaform_daisyui::schemas::{definition, form_data};

/// A unique array of finite choices is one control. The adapter owns membership,
/// including unknown members and array presence; resetting demonstrates node-to-widget updates.
#[component]
pub fn Example() -> Element {
    let definition = use_hook(|| definition(r#"{
      "$schema": "https://json-schema.org/draft/2020-12/schema",
      "type": "object", "additionalProperties": false,
      "properties": {
        "topics": {
          "type": "array", "title": "Topics", "description": "Choose topics for your badge.",
          "uniqueItems": true, "minItems": 1,
          "items": { "enum": ["Rust", "Dioxus", "Web"] }
        },
        "legacy": {
          "type": "array", "title": "Legacy topics", "uniqueItems": true,
          "items": { "enum": ["Rust", "Dioxus"] }
        }
      }
    }"#, None));
    let form = use_form(definition, form_data(r#"{"topics":["Rust"],"legacy":["retired"]}"#))
        .expect("the form should be created");
    let reset = form.clone();
    let mut submitted = use_signal(String::new);
    rsx! {
        SchemaformDaisyui {
            form,
            on_submit: move |snapshot: schemaform::SubmissionSnapshot| submitted.set(snapshot.form_data().to_string()),
        }
        Button { r#type: "button", onclick: move |_| { reset.reset().expect("reset should succeed"); }, "Reset topics" }
        output { "aria-label": "Submitted topics", "{submitted}" }
    }
}
