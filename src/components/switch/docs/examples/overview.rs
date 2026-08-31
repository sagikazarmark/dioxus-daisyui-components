use dioxus::prelude::*;

use crate::components::switch::{Switch, SwitchColor};

/// A switch beside the text that names it.
///
/// The primitive renders a `button` with `role="switch"` rather than an
/// `input`, so a wrapping `label` would not reach it; the text is tied to the
/// switch with `aria-labelledby` instead, which is what a screen reader
/// announces it by.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3",
            div { class: "flex items-center gap-2",
                Switch { aria_labelledby: "overview-telemetry" }
                span { id: "overview-telemetry", class: "text-sm", "Send anonymous telemetry" }
            }

            div { class: "flex items-center gap-2",
                Switch {
                    color: SwitchColor::Primary,
                    default_value: true,
                    aria_labelledby: "overview-logs",
                }
                span { id: "overview-logs", class: "text-sm", "Ship logs to the collector" }
            }
        }
    }
}
