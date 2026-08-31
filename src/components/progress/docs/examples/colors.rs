use dioxus::prelude::*;

use crate::components::progress::{Progress, ProgressColor};

/// Every value of the colour axis, part of the way along so that both halves of
/// the bar are on screen.
///
/// The colour class lands on the track and sets nothing but `color`. That one
/// property paints both parts: the track is `currentColor` mixed down to a
/// fifth, and the fill this registry draws is `currentColor` outright, which
/// is what lets the axis survive the fill being ours rather than daisyUI's
/// (ADR-0012).
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex w-full max-w-sm flex-col gap-2",
            for color in ProgressColor::ALL.iter().copied() {
                Progress { color, value: Some(60.0), aria_label: "{color:?}" }
            }
        }
    }
}
