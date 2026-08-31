use dioxus::prelude::*;

use crate::components::tag_group::{
    TagColor, TagGroupEmpty, TagGroupLabel, TagGroupMulti, TagList, TagOption, TagRemoveButton,
};

/// A set of labels, any number of which can be selected, and every one of which
/// can be removed.
///
/// The whole group is one tab stop: focus enters it and the arrow keys move
/// between tags. Enter and Space select, Escape clears the selection, and
/// Delete or Backspace removes the tag that is focused, which works because
/// each tag has a remove button in it, not because a prop says so.
///
/// The colour is per tag, which is what daisyUI's badge classes are: a label
/// list is a set of differently coloured tags rather than one colour applied to
/// a set.
#[component]
pub fn Example() -> Element {
    let labels = [
        ("bug", TagColor::Error),
        ("docs", TagColor::Info),
        ("good first issue", TagColor::Success),
    ];
    let mut selected = use_signal(Vec::<String>::new);

    rsx! {
        div { class: "flex flex-col gap-3",
            TagGroupMulti::<String> {
                id: "overview",
                default_values: vec!["bug".to_string()],
                on_values_change: move |values| selected.set(values),
                TagGroupLabel { "Labels" }
                TagList {
                    TagGroupEmpty { "Every label has been removed." }
                    for (index , (label , color)) in labels.into_iter().enumerate() {
                        TagOption::<String> {
                            value: label.to_string(),
                            index,
                            color,
                            "{label}"
                            TagRemoveButton { "×" }
                        }
                    }
                }
            }

            p { class: "text-sm opacity-70",
                "Selected: "
                span { "data-testid": "selected",
                    if selected().is_empty() {
                        "nothing"
                    } else {
                        "{selected().join(\", \")}"
                    }
                }
            }
        }
    }
}
