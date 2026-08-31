use dioxus::prelude::*;

use crate::components::progress::{Progress, ProgressColor};

/// Every kind of value a bar can have, including the one it cannot draw.
///
/// The fill is sized from the percentage the primitive works out, so a bar out
/// of five and a bar out of a hundred are the same fill at the same width when
/// they are the same fraction along.
///
/// The last one has no value at all. It announces itself indeterminate (the
/// primitive drops `aria-valuenow` and says so in `data-state`) and daisyUI's
/// own indeterminate styling cannot reach it, because that rule is written
/// against a native `progress` element's `:indeterminate` pseudo-class. What
/// renders is the track with no fill, which is honest rather than finished.
#[component]
pub fn Example() -> Element {
    let mut value = use_signal(|| 40.0_f64);

    rsx! {
        div { class: "flex w-full max-w-sm flex-col gap-3",
            Progress { id: "value-empty", value: Some(0.0), aria_label: "Nothing done" }
            Progress { id: "value-part", value: Some(40.0), aria_label: "Part of the way" }
            Progress { id: "value-full", value: Some(100.0), aria_label: "Finished" }
            Progress { id: "value-scaled", value: Some(2.0), max: 5.0, aria_label: "Two of five" }
            Progress { id: "value-indeterminate", value: None, aria_label: "Working" }

            div { class: "mt-2 flex items-center gap-3",
                Progress {
                    id: "value-driven",
                    color: ProgressColor::Primary,
                    value: Some(value()),
                    aria_label: "Driven",
                }
                button {
                    class: "btn btn-sm",
                    onclick: move |_| value.set((value() + 20.0).min(100.0)),
                    "Advance"
                }
                span { class: "text-sm opacity-70",
                    "at "
                    span { "data-testid": "value", "{value}" }
                }
            }
        }
    }
}
