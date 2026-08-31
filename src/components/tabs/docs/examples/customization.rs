use dioxus::prelude::*;

use crate::components::tabs::{TabContent, TabTrigger, Tabs, TabsAppearance};

/// The tabs this set renders.
const TABS: &[(&str, &str)] = &[
    ("overview", "Overview"),
    ("usage", "Usage"),
    ("cost", "Cost"),
];

/// A caller's own classes and attributes on a boxed set.
///
/// The class squares off a corner radius daisyUI itself sets: the two are both
/// single class selectors, and the caller's wins on cascade layers rather than
/// on specificity.
#[component]
pub fn Example() -> Element {
    rsx! {
        Tabs {
            id: "caller-attributes",
            appearance: TabsAppearance::Box,
            class: "rounded-none",
            default_value: "usage".to_string(),
            for (index , (value , label)) in TABS.iter().copied().enumerate() {
                TabTrigger { value: value.to_string(), index, "Caller {label}" }
                TabContent { value: value.to_string(), index, "Caller {label}" }
            }
        }
    }
}
