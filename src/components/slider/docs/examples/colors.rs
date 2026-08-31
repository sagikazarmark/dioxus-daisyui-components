use dioxus::prelude::*;

use crate::components::slider::{Slider, SliderColor};

/// Every value of the colour axis.
///
/// daisyUI's colour classes set `color` and `--range-thumb` and nothing else, so
/// the fill this component draws with `bg-current`, the groove it draws with
/// `--range-bg` (which is `currentColor` mixed down to a tenth) and the handle
/// it fills with `--range-thumb` are all daisyUI's colour reached by daisyUI's
/// own route (ADR-0016).
///
/// `Default` emits no class, which is a range in the page's own text colour
/// rather than a synonym for neutral.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "color", class: "flex flex-wrap items-center gap-4",
            for color in SliderColor::ALL.iter().copied() {
                Slider {
                    color,
                    label: "{color:?}",
                    default_value: 60.0,
                    class: "w-40",
                }
            }
        }
    }
}
