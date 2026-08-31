use dioxus::prelude::*;

use crate::components::status::{Status, StatusColor, StatusSize};

/// Caller classes and semantic attributes joined with both selected Axes.
#[component]
pub fn Example() -> Element {
    rsx! {
        Status {
            id: "caller-attributes",
            class: "rounded-none",
            color: StatusColor::Primary,
            size: StatusSize::Lg,
            "data-owner": "caller",
            role: "img",
            aria_label: "Caller-owned status",
        }
    }
}
