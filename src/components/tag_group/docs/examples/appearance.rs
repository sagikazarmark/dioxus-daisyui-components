use dioxus::prelude::*;

use crate::components::tag_group::{
    TagColor, TagGroupAppearance, TagGroupEmpty, TagGroupEmptyAppearance, TagGroupLabel,
    TagGroupMulti, TagList, TagListAppearance, TagOption, TagOptionAppearance,
};

/// All four appearance axes, which are the utilities this component emits where
/// daisyUI has no class of its own.
///
/// The tag's is the one that carries state: it is what marks a selected tag,
/// since daisyUI's badge has no selected look, and it is written as a variant of
/// the attribute the primitive already sets rather than as a class this
/// component recomputes. Every tag in that row is selected, so the row shows
/// what the axis does rather than what the state does.
///
/// The group's stacks the label above the tags, the list's lays the tags out, and
/// the empty state's mutes what it says. Each `None` emits nothing at all, which
/// is how a caller wins a tie against a utility rather than trying to out-rank it
/// (ADR-0004).
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            div { "data-axis": "tag", class: "flex flex-wrap items-center gap-6",
                for appearance in TagOptionAppearance::ALL.iter().copied() {
                    TagGroupMulti::<String> {
                        default_values: vec!["selected".to_string()],
                        TagList {
                            TagOption::<String> {
                                value: "selected".to_string(),
                                index: 0usize,
                                color: TagColor::Primary,
                                appearance,
                                "{appearance:?}"
                            }
                        }
                    }
                }
            }

            div { "data-axis": "group", class: "flex flex-wrap items-start gap-6",
                for appearance in TagGroupAppearance::ALL.iter().copied() {
                    TagGroupMulti::<String> { appearance,
                        TagGroupLabel { "Group: {appearance:?}" }
                        TagList {
                            TagOption::<String> { value: "one".to_string(), index: 0usize, "one" }
                        }
                    }
                }
            }

            div { "data-axis": "list", class: "flex flex-wrap items-start gap-6",
                for appearance in TagListAppearance::ALL.iter().copied() {
                    TagGroupMulti::<String> {
                        TagList { appearance,
                            TagOption::<String> {
                                value: "list".to_string(),
                                index: 0usize,
                                "List: {appearance:?}"
                            }
                            TagOption::<String> { value: "second".to_string(), index: 1usize, "second" }
                        }
                    }
                }
            }

            div { "data-axis": "empty", class: "flex flex-wrap items-start gap-6",
                for appearance in TagGroupEmptyAppearance::ALL.iter().copied() {
                    TagGroupMulti::<String> {
                        TagList {
                            TagGroupEmpty { appearance, "Empty: {appearance:?}" }
                        }
                    }
                }
            }
        }
    }
}
