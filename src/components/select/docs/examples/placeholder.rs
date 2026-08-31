use dioxus::prelude::*;

use crate::components::select::{
    Select, SelectList, SelectTrigger, SelectValue, SelectValueAppearance,
};
use crate::examples::select::options::Options;

/// Whether the placeholder is told apart from a chosen value.
///
/// daisyUI fades a native select's own placeholder and has no class for one
/// standing in for it, so the utility is this component's, keyed on the
/// attribute the primitive already sets, and switchable off (ADR-0004). Both
/// selects here show the placeholder, so what the row varies is the axis rather
/// than the state.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "placeholder", class: "flex flex-wrap items-center gap-4",
            for appearance in SelectValueAppearance::ALL.iter().copied() {
                Select::<String> {
                    SelectTrigger {
                        SelectValue { appearance, placeholder: "{appearance:?}" }
                    }
                    SelectList {
                        Options {}
                    }
                }
            }
        }
    }
}
