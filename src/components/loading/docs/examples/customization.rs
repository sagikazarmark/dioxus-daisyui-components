use dioxus::prelude::*;

use crate::components::loading::{Loading, LoadingAnimation, LoadingSize};

/// Caller colour and attributes joined with the selected Axes.
#[component]
pub fn Example() -> Element {
    rsx! {
        Loading {
            id: "caller-attributes",
            class: "text-secondary",
            animation: LoadingAnimation::Bars,
            size: LoadingSize::Lg,
            "data-owner": "caller",
            role: "status",
            aria_label: "Loading caller data",
        }
    }
}
