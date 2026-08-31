use dioxus::prelude::*;

use crate::components::drag_and_drop_list::{
    DragAndDropIndicatorAppearance, DragAndDropList, DragAndDropListAppearance,
    DragAndDropListItemAppearance,
};

/// The three axes, each drawn and each switched off.
///
/// daisyUI's `list` and `list-row` are the only classes this component emits;
/// everything else (the box around the rows, what a row being carried looks
/// like, and the line it would land on) is utilities, so every one of them
/// comes with a value that emits nothing (ADR-0004).
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8",
            div { "data-axis": "appearance", class: "flex flex-wrap items-start gap-6",
                for appearance in DragAndDropListAppearance::ALL.iter().copied() {
                    div { class: "w-56",
                        DragAndDropList {
                            appearance,
                            items: rows(&format!("{appearance:?}")),
                            aria_label: "Box {appearance:?}",
                        }
                    }
                }
            }

            div { "data-axis": "item-appearance", class: "flex flex-wrap items-start gap-6",
                for appearance in DragAndDropListItemAppearance::ALL.iter().copied() {
                    div { class: "w-56",
                        DragAndDropList {
                            item_appearance: appearance,
                            items: rows(&format!("{appearance:?}")),
                            aria_label: "Rows {appearance:?}",
                        }
                    }
                }
            }

            div { "data-axis": "indicator-appearance", class: "flex flex-wrap items-start gap-6",
                for appearance in DragAndDropIndicatorAppearance::ALL.iter().copied() {
                    div { class: "w-56",
                        DragAndDropList {
                            indicator_appearance: appearance,
                            items: rows(&format!("{appearance:?}")),
                            aria_label: "Lines {appearance:?}",
                        }
                    }
                }
            }
        }
    }
}

/// Two rows named after the value of the axis they are showing.
fn rows(label: &str) -> Vec<Element> {
    (1..=2)
        .map(|position| {
            let label = label.to_string();
            rsx! {
                div { class: "list-col-grow", "{label} {position}" }
            }
        })
        .collect()
}
