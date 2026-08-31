use dioxus::prelude::*;

use crate::components::tabs::{TabContent, TabContentAppearance, TabTrigger, Tabs, TabsAppearance};

/// The tabs each set renders.
const TABS: &[(&str, &str)] = &[
    ("overview", "Overview"),
    ("usage", "Usage"),
    ("cost", "Cost"),
];

/// Every value of the panel's own appearance axis, one set per value.
///
/// daisyUI's `tab-content` draws a border but leaves it transparent and paints
/// no background, so the utilities its own examples use are emitted by
/// `TabContent` instead. That inverts the usual convention: the default emits
/// classes and `None` emits nothing, because a utility this component emits
/// only ties with a caller's, and switching ours off is how a caller wins.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "panel", class: "flex flex-col gap-4",
            for appearance in TabContentAppearance::ALL.iter().copied() {
                Tabs {
                    appearance: TabsAppearance::Lift,
                    default_value: "usage".to_string(),
                    for (index , (value , label)) in TABS.iter().copied().enumerate() {
                        TabTrigger { value: value.to_string(), index, "Panel {appearance:?} {label}" }
                        TabContent {
                            appearance,
                            value: value.to_string(),
                            index,
                            "Panel {appearance:?} {label}"
                        }
                    }
                }
            }
        }
    }
}
