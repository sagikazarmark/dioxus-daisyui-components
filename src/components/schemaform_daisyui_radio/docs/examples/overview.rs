use dioxus::prelude::*;
use schemaform_dioxus::use_form;

use crate::components::schemaform_daisyui::{
    Addons, ButtonClasses, Density, SchemaformDaisyui,
};
use crate::components::schemaform_daisyui_radio::RadioAddon;
use crate::examples::schemaform_daisyui::schemas::{definition, form_data};

/// A required nullable billing cycle and an optional contact channel.
const SCHEMA: &str = r##"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "additionalProperties": false,
  "required": ["billing"],
  "properties": {
    "billing": { "type": ["string", "null"], "title": "Billing", "enum": ["monthly", "yearly", null] },
    "contact": { "title": "Contact", "enum": ["email", "phone"] }
  }
}"##;

/// Both choices ask for the radio widget; the rest of the form would render as usual.
const UI_SCHEMA: &str = r##"{
  "version": 1,
  "root": {
    "type": "stack",
    "value": {
      "children": [
        {
          "type": "control",
          "value": { "binding": { "origin": "root", "pointer": "/billing" }, "widget": "daisyui:radio" }
        },
        {
          "type": "control",
          "value": { "binding": { "origin": "root", "pointer": "/contact" }, "widget": "daisyui:radio" }
        }
      ]
    }
  }
}"##;

/// The `daisyui:radio` widget symbol, registered by the add-on in `addons`.
///
/// Each option, the null option included, is a radio item labelled by the text beside it. The
/// radio fields sit in the same frame as every other control, at the form's density and with the
/// host's presence classes: here compact, with ghost Set and Clear buttons.
#[component]
pub fn Example() -> Element {
    let definition = use_hook(|| definition(SCHEMA, Some(UI_SCHEMA)));
    let form = use_form(definition, form_data(r#"{"billing":"yearly"}"#))
        .expect("the form should be created");

    rsx! {
        SchemaformDaisyui {
            form,
            density: Density::Compact,
            buttons: ButtonClasses::default().presence("btn-ghost btn-xs"),
            addons: Addons::default().with(RadioAddon),
            on_submit: move |_| {},
        }
    }
}
