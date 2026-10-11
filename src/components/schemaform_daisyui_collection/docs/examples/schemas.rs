use dioxus::prelude::*;

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

/// The Example this module renders: nothing. The schemas are text a page shows as source.
#[component]
pub fn Example() -> Element {
    rsx! {}
}
