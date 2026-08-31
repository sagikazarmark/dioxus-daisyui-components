use dioxus::prelude::*;

use crate::components::checkbox::{Checkbox, CheckboxColor};
use crate::components::fieldset::{Fieldset, FieldsetLegend};
use crate::components::label::Label;
use crate::components::switch::{Switch, SwitchColor};

/// Default labels beside the registry's own controls, inside a Fieldset.
///
/// Fieldset and its legend preserve the native grouping elements. The captions
/// inside use daisyUI's current `label` class; the legacy `fieldset-label`
/// appearance remains visible in the Appearance Example only.
///
/// Both controls are `button` elements the primitives render, which are
/// labelable: the caption's `for` reaches them exactly as it reaches an input,
/// and clicking the caption operates the control.
///
/// The last row is the case daisyUI matches for itself: a label inside an
/// `input` group is restyled by `.label:is(.input>*)`, so nothing extra is
/// emitted for it and the same `label` value of the axis does both jobs.
#[component]
pub fn Example() -> Element {
    rsx! {
        Fieldset { class: "bg-base-100 border-base-300 rounded-box w-full max-w-sm border p-4",
            FieldsetLegend { "Build" }

            div { class: "flex items-center gap-2",
                Checkbox { id: "controls-release", color: CheckboxColor::Primary }
                Label {
                    html_for: "controls-release",
                    "Optimise for release",
                }
            }

            div { class: "flex items-center gap-2",
                Switch { id: "controls-watch", color: SwitchColor::Primary }
                Label {
                    html_for: "controls-watch",
                    "Rebuild on change",
                }
            }

            label { class: "input w-full",
                Label { html_for: "controls-target", "Target" }
                input {
                    id: "controls-target",
                    class: "grow",
                    placeholder: "wasm32-unknown-unknown",
                }
            }
        }
    }
}
