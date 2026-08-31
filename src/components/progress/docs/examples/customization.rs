use dioxus::prelude::*;

use crate::components::progress::{Progress, ProgressColor};

/// A caller's own classes and attributes, on the element they belong on.
///
/// daisyUI gives a progress bar one height and no size axis, so a thicker bar
/// is a caller's utility, and this is where the collapsed component's
/// attributes land, which is the track rather than the fill. The fill's width
/// is the value; the track is the part a caller sizes, positions and rounds.
///
/// The height here wins over daisyUI's own on cascade layers rather than on
/// specificity, which is the same way every caller utility beats a daisyUI
/// component class (ADR-0004).
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex w-full max-w-sm flex-col gap-3",
            Progress {
                color: ProgressColor::Accent,
                value: Some(70.0),
                class: "h-4 rounded-none",
                id: "caller-attributes",
                aria_label: "Caller classes and attributes",
            }
        }
    }
}
