use dioxus::prelude::*;

use crate::components::tag_group::{TagColor, TagGroup, TagGroupLabel, TagList, TagOption};

/// A group where one tag is selected at a time, with one tag disabled.
///
/// This is the single-selection root: choosing another tag replaces the choice
/// rather than adding to it, choosing the selected one again clears it, and
/// Escape clears it from anywhere in the group.
///
/// The disabled tag is inert and skipped by the arrow keys, and it looks like
/// every other one: daisyUI has no disabled badge, and drawing one here would
/// be this registry inventing a look. The class below is how a caller says
/// otherwise, through the attribute the primitive already sets.
#[component]
pub fn Example() -> Element {
    let priorities = ["Low", "Medium", "High"];
    // Seeded from the same value the group is, so the readout is right on the
    // first render rather than one selection later: the primitive reports a
    // change rather than a state.
    let mut selected = use_signal(|| Some("Medium".to_string()));

    rsx! {
        div { class: "flex flex-col gap-3",
            TagGroup::<String> {
                id: "priority",
                default_value: Some("Medium".to_string()),
                on_value_change: move |value| selected.set(value),
                TagGroupLabel { "Priority" }
                TagList {
                    for (index , priority) in priorities.into_iter().enumerate() {
                        TagOption::<String> {
                            value: priority.to_string(),
                            index,
                            color: TagColor::Primary,
                            disabled: priority == "High",
                            class: "data-[disabled=true]:opacity-50",
                            "{priority}"
                        }
                    }
                }
            }

            p { class: "text-sm opacity-70",
                "Priority: "
                span { "data-testid": "priority",
                    match selected() {
                        Some(value) => value,
                        None => "none".to_string(),
                    }
                }
            }
        }
    }
}
