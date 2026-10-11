use dioxus::prelude::*;
use schemaform_dioxus::use_form;

use crate::components::schemaform_daisyui::{Addons, SchemaformDaisyui};
use crate::components::schemaform_daisyui_select::SelectAddon;
use crate::examples::schemaform_daisyui::schemas::{definition, form_data};

/// A required region.
const SCHEMA: &str = r##"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "additionalProperties": false,
  "required": ["region"],
  "properties": {
    "region": { "title": "Region", "enum": ["eu", "us", "apac"] }
  }
}"##;

/// The region asks for the compound select.
const UI_SCHEMA: &str = r##"{
  "version": 1,
  "root": {
    "type": "control",
    "value": { "binding": { "origin": "root", "pointer": "/region" }, "widget": "daisyui:select" }
  }
}"##;

/// The `daisyui:select` widget symbol, registered by the add-on in `addons`.
///
/// The trigger carries the control's id and name and shows the selected option's label; the
/// listbox offers every option. Unlike the native select, the compound select takes no part in
/// native form submission.
#[component]
pub fn Example() -> Element {
    let definition = use_hook(|| definition(SCHEMA, Some(UI_SCHEMA)));
    let form = use_form(definition, form_data(r#"{"region":"eu"}"#))
        .expect("the form should be created");

    rsx! {
        SchemaformDaisyui {
            form,
            addons: Addons::default().with(SelectAddon),
            on_submit: move |_| {},
        }
    }
}
