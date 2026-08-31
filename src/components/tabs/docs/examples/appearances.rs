use dioxus::prelude::*;

use crate::components::tabs::{TabContent, TabTrigger, Tabs, TabsAppearance};

/// The tabs each set renders, and a fourth that is disabled.
const TABS: &[(&str, &str)] = &[
    ("overview", "Overview"),
    ("usage", "Usage"),
    ("cost", "Cost"),
    ("archived", "Archived"),
];

/// Every value of the appearance axis, one whole set of tabs per value.
///
/// A set is the smallest thing an appearance is observable on: daisyUI
/// expresses one across the tabs element, the active tab and the panel at once.
/// Each set opens on its second tab, because the lifted appearance rounds a
/// panel's corner only when the active tab is not the one the row starts with.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "appearance", class: "flex flex-col gap-4",
            for appearance in TabsAppearance::ALL.iter().copied() {
                Tabs { appearance, default_value: "usage".to_string(),
                    for (index , (value , label)) in TABS.iter().copied().enumerate() {
                        TabTrigger {
                            value: value.to_string(),
                            index,
                            disabled: value == "archived",
                            "{appearance:?} {label}"
                        }
                        TabContent { value: value.to_string(), index, "{appearance:?} {label}" }
                    }
                }
            }
        }
    }
}
