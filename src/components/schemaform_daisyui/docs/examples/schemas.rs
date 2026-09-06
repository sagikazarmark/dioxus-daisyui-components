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
/// and a write-only boolean; a nullable choice, a write-only choice, and a constant; and two more
/// choices the UI schema renders as a radio group and as the compound select.
pub const CONTROLS_SCHEMA: &str = r##"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "additionalProperties": false,
  "required": ["name", "quantity", "price", "active", "plan", "billing", "region"],
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
    "plan": { "type": ["string", "null"], "title": "Plan", "enum": ["starter", "team", null] },
    "recovery": { "title": "Recovery", "enum": ["email", "sms"], "writeOnly": true },
    "tier": { "title": "Tier", "const": "standard" },
    "billing": { "type": ["string", "null"], "title": "Billing", "enum": ["monthly", "yearly", null] },
    "region": { "title": "Region", "enum": ["eu", "us", "apac"] }
  }
}"##;

/// The controls in data-schema order, except that the billing cycle asks for the radio widget
/// and the region for the compound select.
pub const CONTROLS_UI_SCHEMA: &str = r##"{
  "version": 1,
  "root": {
    "type": "stack",
    "value": {
      "children": [
        {
          "type": "auto",
          "value": {
            "binding": { "origin": "root", "pointer": "" },
            "properties": { "exclude": ["billing", "region"] }
          }
        },
        {
          "type": "control",
          "value": { "binding": { "origin": "root", "pointer": "/billing" }, "widget": "daisyui:radio" }
        },
        {
          "type": "control",
          "value": { "binding": { "origin": "root", "pointer": "/region" }, "widget": "daisyui:select" }
        }
      ]
    }
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
  "tier": "standard",
  "billing": "yearly",
  "region": "eu"
}"##;

/// Two array shapes: string items, optional with a seed default and a `maxItems`, so the
/// container's presence operations have a target and append is withdrawn when the array is
/// full; and fixed-object items, required with `minItems`, so the sole item cannot be removed.
pub const ARRAYS_SCHEMA: &str = r##"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "additionalProperties": false,
  "required": ["name", "team"],
  "properties": {
    "name": { "type": "string", "title": "Name", "minLength": 2 },
    "tags": {
      "type": "array",
      "title": "Tags",
      "description": "Keywords for the badge.",
      "default": ["seed"],
      "maxItems": 3,
      "items": { "type": "string", "title": "Tag", "default": "fresh" }
    },
    "team": {
      "type": "array",
      "title": "Team",
      "minItems": 1,
      "maxItems": 4,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": ["name", "role"],
        "properties": {
          "name": { "type": "string", "title": "Member", "default": "New member" },
          "role": { "title": "Role", "enum": ["engineering", "design", "operations"], "default": "engineering" }
        }
      }
    }
  }
}"##;

/// Baseline data for [`ARRAYS_SCHEMA`].
pub const ARRAYS_DATA: &str = r##"{
  "name": "Ada",
  "tags": ["rust", "dioxus"],
  "team": [{ "name": "Ada", "role": "engineering" }, { "name": "Lin", "role": "design" }]
}"##;

/// A small form for the overview and the appearance axis: a validated string, a nullable string,
/// a boolean, a choice, and one array.
pub const OVERVIEW_SCHEMA: &str = r##"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "additionalProperties": false,
  "required": ["name", "plan", "tags"],
  "properties": {
    "name": { "type": "string", "title": "Display name", "description": "At least three characters.", "minLength": 3 },
    "nickname": { "type": ["string", "null"], "title": "Nickname" },
    "newsletter": { "type": "boolean", "title": "Product newsletter" },
    "plan": { "title": "Plan", "enum": ["starter", "team", "enterprise"] },
    "tags": {
      "type": "array",
      "title": "Tags",
      "minItems": 1,
      "maxItems": 5,
      "items": { "type": "string", "default": "new-tag" }
    }
  }
}"##;

/// Baseline data for [`OVERVIEW_SCHEMA`].
pub const OVERVIEW_DATA: &str = r##"{
  "name": "Ada",
  "nickname": null,
  "newsletter": true,
  "plan": "team",
  "tags": ["rust", "dioxus"]
}"##;

/// The Example this module renders: nothing. The schemas are text a page shows as source.
#[component]
pub fn Example() -> Element {
    rsx! {}
}
