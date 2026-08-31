use dioxus::prelude::*;

use crate::components::select::{
    Select, SelectList, SelectListAppearance, SelectListSize, SelectTrigger, SelectValue,
};
use crate::examples::select::options::Options;

/// The list's own two axes: the size that sizes the options, and the appearance
/// that paints the box.
///
/// The size is the options' rather than the box's (daisyUI's menu has no size
/// of its own, it is whatever the options need) and it is an axis apart from
/// the trigger's because daisyUI's are apart. The appearance's switched-off
/// value leaves the popup positioned by daisyUI and painted by nobody, which is
/// how a caller wins against a utility this component emits (ADR-0004).
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8",
            // A grid rather than a wrapping row, so that every select gets a
            // column of its own: these popups stand open side by side and the
            // largest of them is wider than a trigger, which in a row would
            // leave one lying over the next.
            div {
                "data-axis": "list-size",
                class: "grid grid-cols-5 items-start gap-8 pb-72",
                for size in SelectListSize::ALL.iter().copied() {
                    Select::<String> { open: Some(true),
                        SelectTrigger {
                            SelectValue { placeholder: "{size:?}" }
                        }
                        SelectList { size,
                            Options {}
                        }
                    }
                }
            }

            div {
                "data-axis": "appearance",
                class: "flex flex-wrap items-start gap-40 pb-72",
                for appearance in SelectListAppearance::ALL.iter().copied() {
                    Select::<String> { open: Some(true),
                        SelectTrigger {
                            SelectValue { placeholder: "{appearance:?}" }
                        }
                        SelectList { appearance,
                            Options {}
                        }
                    }
                }
            }
        }
    }
}
