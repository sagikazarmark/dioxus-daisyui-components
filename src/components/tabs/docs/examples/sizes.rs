use dioxus::prelude::*;

use crate::components::tabs::{TabTrigger, Tabs, TabsSize};

/// The tabs each bar renders.
const TABS: &[(&str, &str)] = &[
    ("overview", "Overview"),
    ("usage", "Usage"),
    ("cost", "Cost"),
];

/// Every value of the size axis, as tab bars with no panels under them.
///
/// A size is the tab's own height, and leaving the panels out is what makes
/// that height the set's height too; a set with a panel in it is as tall as
/// whatever the caller put in the panel, which is not the axis. daisyUI's tabs
/// are a navigation bar as often as they are a panel switcher, so this is also
/// a shape it ships.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "size", class: "flex flex-col items-start gap-4",
            for size in TabsSize::ALL.iter().copied() {
                Tabs { size, default_value: "usage".to_string(),
                    for (index , (value , label)) in TABS.iter().copied().enumerate() {
                        TabTrigger { value: value.to_string(), index, "{size:?} {label}" }
                    }
                }
            }
        }
    }
}
