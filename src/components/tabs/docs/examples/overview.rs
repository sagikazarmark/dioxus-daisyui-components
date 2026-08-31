use dioxus::prelude::*;

use crate::components::tabs::{TabContent, TabTrigger, Tabs, TabsAppearance};

/// A set of tabs written out by hand, which is the shape every other example
/// here builds with a loop.
///
/// A trigger and the panel it reveals are adjacent siblings, both direct
/// children of `Tabs`: trigger, its panel, the next trigger, its panel. That is
/// daisyUI's own tabs markup, and the only shape its adjacent-sibling rule
/// reaches a panel in.
#[component]
pub fn Example() -> Element {
    rsx! {
        Tabs { appearance: TabsAppearance::Lift, default_value: "overview".to_string(),
            TabTrigger { value: "overview".to_string(), index: 0usize, "Overview" }
            TabContent { value: "overview".to_string(), index: 0usize,
                "What the service does, in one paragraph."
            }

            TabTrigger { value: "usage".to_string(), index: 1usize, "Usage" }
            TabContent { value: "usage".to_string(), index: 1usize,
                "How much of it you have used this month."
            }

            TabTrigger { value: "cost".to_string(), index: 2usize, "Cost" }
            TabContent { value: "cost".to_string(), index: 2usize, "What that came to." }
        }
    }
}
