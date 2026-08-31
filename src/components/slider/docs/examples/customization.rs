use dioxus::prelude::*;

use crate::components::slider::{
    Slider, SliderColor, SliderRange, SliderRoot, SliderThumb, SliderThumbAppearance, SliderTrack,
};

/// A caller's own classes, on the root and on a part.
///
/// The first one is the ordinary case: a width, which daisyUI's `range` sets to
/// twenty rem and which a caller overrides because a utility beats a component
/// class on cascade layers.
///
/// The second drops to the parts and redraws the handle, switching this
/// component's utilities off so that the caller's are the only ones on the
/// element (ADR-0004). The groove and the fill are left alone, so what daisyUI's
/// colour class reaches and what the caller reached are visible side by side.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            Slider {
                id: "caller-attributes",
                label: "A narrower slider",
                color: SliderColor::Secondary,
                default_value: 40.0,
                class: "w-32",
            }

            SliderRoot {
                id: "caller-thumb",
                label: "A square handle",
                color: SliderColor::Secondary,
                default_value: 40.0,
                class: "w-64",
                SliderTrack {
                    SliderRange {}
                    SliderThumb {
                        appearance: SliderThumbAppearance::None,
                        class: "bg-secondary absolute top-1/2 size-4 -translate-x-1/2 -translate-y-1/2 rounded-none",
                    }
                }
            }
        }
    }
}
