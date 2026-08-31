use dioxus::prelude::*;

use crate::components::slider::{Slider, SliderSize};

/// Every value of the size axis.
///
/// daisyUI's size classes set `--range-thumb-size` and nothing else. Its own
/// rules take the control's height from that property and the native groove's
/// from half of it, and the utilities this component draws the parts with derive
/// theirs the same way, so one property sizes all four elements (ADR-0016).
///
/// `Default` emits no class and renders at the same size as daisyUI's explicit
/// `range-md`.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-wrap items-center gap-4",
            for size in SliderSize::ALL.iter().copied() {
                Slider {
                    size,
                    label: "{size:?}",
                    default_value: 60.0,
                    class: "w-40",
                }
            }
        }
    }
}
