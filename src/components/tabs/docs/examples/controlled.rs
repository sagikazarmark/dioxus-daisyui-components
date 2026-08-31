use dioxus::prelude::*;

use crate::components::tabs::{TabContent, TabTrigger, Tabs, TabsAppearance};

/// The tabs this set renders, the last of them disabled, which is what makes
/// arrow-key navigation skipping it observable.
const TABS: &[(&str, &str)] = &[
    ("overview", "Overview"),
    ("usage", "Usage"),
    ("cost", "Cost"),
    ("archived", "Archived"),
];

/// A set of tabs whose active value belongs to the caller.
///
/// The value travels out through the change callback and back in through the
/// value prop, so nothing switches unless the caller lets it. Arrow keys move
/// focus along the row without activating anything; a second press activates,
/// which is what makes a set with expensive panels usable from the keyboard.
#[component]
pub fn Example() -> Element {
    let mut active = use_signal(|| String::from("overview"));
    let mut changes = use_signal(|| 0_u32);

    rsx! {
        div { class: "flex flex-col items-start gap-4",
            Tabs {
                id: "controlled",
                appearance: TabsAppearance::Lift,
                value: Some(active()),
                on_value_change: move |value| {
                    active.set(value);
                    changes += 1;
                },
                for (index , (value , label)) in TABS.iter().copied().enumerate() {
                    TabTrigger {
                        value: value.to_string(),
                        index,
                        disabled: value == "archived",
                        "{label}"
                    }
                    TabContent { value: value.to_string(), index, "{label} panel" }
                }
            }

            p { class: "text-sm opacity-70",
                "Changed "
                span { "data-testid": "changes", "{changes}" }
                " times"
            }

            // Somewhere for focus to land after the tab bar and its panel, so
            // that the panel can be shown to sit in the focus order between the
            // two.
            a { id: "after", href: "#after", class: "link w-fit", "After the tabs" }
        }
    }
}
