use dioxus::prelude::*;

use crate::components::progress::{Progress, ProgressColor};

/// A bar part of the way along, and the label that names it.
///
/// The primitive gives the element its role and its value; nothing names it, so
/// the text beside it is tied to the bar with `aria-labelledby` and that is what
/// a screen reader announces it by.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex w-full max-w-sm flex-col gap-4",
            div { class: "flex flex-col gap-1",
                span { id: "overview-upload", class: "text-sm", "Uploading the archive" }
                Progress {
                    color: ProgressColor::Primary,
                    value: Some(40.0),
                    aria_labelledby: "overview-upload",
                }
            }

            div { class: "flex flex-col gap-1",
                span { id: "overview-restore", class: "text-sm", "Restoring 3 of 5 volumes" }
                Progress {
                    color: ProgressColor::Success,
                    value: Some(3.0),
                    max: 5.0,
                    aria_labelledby: "overview-restore",
                }
            }
        }
    }
}
