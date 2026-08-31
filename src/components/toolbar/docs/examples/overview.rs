use dioxus::prelude::*;

use crate::components::toolbar::{Toolbar, ToolbarButton, ToolbarButtonSize, ToolbarSeparator};

/// A toolbar of two groups with a rule between them.
///
/// The arrow keys move between the controls and Home returns to the first,
/// which is why every control states its `index`: the primitive matches a key
/// press to a control by index rather than by position. What this toolbar does
/// *not* have is a roving tab stop (Tab walks the controls one at a time) and
/// the component's documentation records that with the rest of the primitive's
/// keyboard.
///
/// The separator takes no index. Nothing navigates to it, and the primitive
/// gives it the opposite orientation to the toolbar's own, which is what makes
/// it a rule down a row.
#[component]
pub fn Example() -> Element {
    let mut last = use_signal(|| String::from("nothing yet"));

    rsx! {
        div { class: "flex flex-col gap-3",
            Toolbar { id: "overview", aria_label: "Editing",
                ToolbarButton {
                    index: 0usize,
                    size: ToolbarButtonSize::Sm,
                    on_click: move |_| last.set("Cut".into()),
                    "Cut"
                }
                ToolbarButton {
                    index: 1usize,
                    size: ToolbarButtonSize::Sm,
                    on_click: move |_| last.set("Copy".into()),
                    "Copy"
                }
                ToolbarButton {
                    index: 2usize,
                    size: ToolbarButtonSize::Sm,
                    on_click: move |_| last.set("Paste".into()),
                    "Paste"
                }

                ToolbarSeparator {}

                ToolbarButton {
                    index: 3usize,
                    size: ToolbarButtonSize::Sm,
                    on_click: move |_| last.set("Undo".into()),
                    "Undo"
                }
                ToolbarButton {
                    index: 4usize,
                    size: ToolbarButtonSize::Sm,
                    on_click: move |_| last.set("Redo".into()),
                    "Redo"
                }
            }

            p { class: "text-sm opacity-70",
                "Last action: "
                span { "data-testid": "action", "{last}" }
            }
        }
    }
}
