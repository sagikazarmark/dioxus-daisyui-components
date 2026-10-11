//! The schemas the examples share, as the JSON text a host would receive them as.

use dioxus::prelude::*;
use schemaform::{
    CompilationProfile, FormDefinition,
    json::{FormDataLimits, parse_data_schema, parse_form_data, parse_ui_schema_v1},
};

/// Compiles `data_schema`, and `ui_schema` when given, into a form definition.
pub fn definition(data_schema: &str, ui_schema: Option<&str>) -> FormDefinition {
    let profile = CompilationProfile::default();
    let data_schema = parse_data_schema(data_schema.as_bytes(), &profile)
        .expect("the example data schema should parse");
    let compiler = FormDefinition::compiler(data_schema);
    let compiler = match ui_schema {
        Some(ui_schema) => compiler.ui_schema(
            parse_ui_schema_v1(ui_schema.as_bytes(), &profile)
                .expect("the example UI schema should parse"),
        ),
        None => compiler,
    };
    compiler
        .compile()
        .expect("the example schemas should compile")
}

/// Parses `form_data` as the baseline of a form.
pub fn form_data(form_data: &str) -> serde_json::Value {
    parse_form_data(form_data.as_bytes(), &FormDataLimits::default())
        .expect("the example form data should parse")
}

/// Every control kind the renderer presents: strings, an integer and a number; a nullable string
/// with presence operations; a write-only string; a read-only string; a non-nullable, a nullable
/// and a write-only boolean, and two booleans the data leaves out, one of them required; a
/// nullable choice, a write-only choice, and a constant.
pub const CONTROLS_SCHEMA: &str = r##"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "additionalProperties": false,
  "required": ["name", "quantity", "price", "active", "terms", "plan"],
  "properties": {
    "name": { "type": "string", "title": "Name", "description": "Shown on the badge.", "minLength": 2 },
    "quantity": { "type": "integer", "title": "Quantity", "minimum": 0 },
    "price": { "type": "number", "title": "Price", "description": "In euros, decimals allowed." },
    "nickname": { "type": ["string", "null"], "title": "Nickname" },
    "secret": { "type": "string", "title": "Secret", "writeOnly": true },
    "reference": { "type": "string", "title": "Reference", "description": "Assigned by the server.", "readOnly": true },
    "active": { "type": "boolean", "title": "Active" },
    "newsletter": { "type": ["boolean", "null"], "title": "Newsletter", "description": "Null means undecided." },
    "mfa": { "type": "boolean", "title": "MFA", "writeOnly": true },
    "terms": { "type": "boolean", "title": "Accept terms" },
    "marketing": { "type": "boolean", "title": "Marketing email" },
    "plan": { "type": ["string", "null"], "title": "Plan", "enum": ["starter", "team", null] },
    "recovery": { "title": "Recovery", "enum": ["email", "sms"], "writeOnly": true },
    "tier": { "title": "Tier", "const": "standard" }
  }
}"##;

/// Baseline data for [`CONTROLS_SCHEMA`].
pub const CONTROLS_DATA: &str = r##"{
  "name": "Ada",
  "quantity": 1,
  "price": 9.5,
  "nickname": null,
  "secret": "hunter2",
  "reference": "ref_42",
  "active": true,
  "newsletter": null,
  "mfa": true,
  "plan": "team",
  "recovery": "sms",
  "tier": "standard"
}"##;

/// A small form for the overview and the appearance axis: a validated string, a nullable string,
/// a boolean, a choice, and a multiple choice that needs one topic.
pub const OVERVIEW_SCHEMA: &str = r##"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "additionalProperties": false,
  "required": ["name", "plan", "topics"],
  "properties": {
    "name": { "type": "string", "title": "Display name", "description": "At least three characters.", "minLength": 3 },
    "nickname": { "type": ["string", "null"], "title": "Nickname" },
    "newsletter": { "type": "boolean", "title": "Product newsletter" },
    "plan": { "title": "Plan", "enum": ["starter", "team", "enterprise"] },
    "topics": {
      "type": "array",
      "title": "Topics",
      "uniqueItems": true,
      "minItems": 1,
      "items": { "enum": ["rust", "dioxus", "daisyui"] }
    }
  }
}"##;

/// Baseline data for [`OVERVIEW_SCHEMA`].
pub const OVERVIEW_DATA: &str = r##"{
  "name": "Ada",
  "nickname": null,
  "newsletter": true,
  "plan": "team",
  "topics": ["rust", "dioxus"]
}"##;

/// The Example this module renders: nothing. The schemas are text a page shows as source.
#[component]
pub fn Example() -> Element {
    rsx! {}
}
