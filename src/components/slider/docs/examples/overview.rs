use dioxus::prelude::*;

use crate::components::slider::{Slider, SliderColor};

/// A slider that reports what it is worth, and one that keeps its own value.
///
/// The first is controlled: every move travels out through `on_change`
/// and back in through `value`, which is what an app that has to store the
/// number does. The second is not, and moves on its own.
///
/// Both are labelled. The primitive gives the handle `role="slider"` and its
/// value through `aria-valuenow`, and a slider with no label is announced as
/// "slider" and nothing else however clear the page around it is.
#[component]
pub fn Example() -> Element {
    let mut volume = use_signal(|| 40.0);
    let mut changes = use_signal(|| 0usize);
    let mut commits = use_signal(|| 0usize);
    let mut focus_exits = use_signal(|| 0usize);

    rsx! {
        div { class: "flex flex-col gap-6",
            div { class: "flex flex-col gap-2",
                p { class: "text-sm opacity-70",
                    "Volume: "
                    span { "data-testid": "volume", "{volume()}" }
                }
                Slider {
                    id: "controlled",
                    label: "Volume",
                    color: SliderColor::Primary,
                    value: Some(volume()),
                    on_change: move |next| {
                        volume.set(next);
                        changes += 1;
                    },
                    on_commit: move |()| commits += 1,
                    on_focus_exit: move |()| focus_exits += 1,
                    class: "w-64",
                }
                p { class: "text-xs opacity-60",
                    "Changes: "
                    span { "data-testid": "slider-changes", "{changes}" }
                    ", commits: "
                    span { "data-testid": "slider-commits", "{commits}" }
                }
                output {
                    class: "hidden",
                    "data-testid": "slider-focus-exits",
                    "{focus_exits}"
                }
            }

            div { class: "flex flex-col gap-2",
                p { class: "text-sm opacity-70", "Brightness, in steps of ten" }
                Slider {
                    id: "stepped",
                    label: "Brightness",
                    default_value: 30.0,
                    step: 10.0,
                    class: "w-64",
                }
            }
        }
    }
}
