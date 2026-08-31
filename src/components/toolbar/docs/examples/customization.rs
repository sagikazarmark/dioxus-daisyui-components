use dioxus::prelude::*;

use crate::components::toolbar::{
    Toolbar, ToolbarAppearance, ToolbarButton, ToolbarButtonSize, ToolbarSeparator,
};

/// A caller's own classes, and the layout utilities switched off.
///
/// The first toolbar keeps this component's layout and adds a surface of its
/// own, which is the common case: daisyUI has no toolbar to paint, so the paint
/// is always the caller's.
///
/// The second switches the layout utilities off and writes daisyUI's `join` in
/// their place: the fused row this component deliberately is not, reachable in
/// two classes because nothing here stands in its way (ADR-0004).
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col items-start gap-3",
            Toolbar {
                id: "caller-attributes",
                class: "bg-base-200 rounded-box p-1",
                aria_label: "Painted toolbar",
                ToolbarButton { index: 0usize, size: ToolbarButtonSize::Sm, "Cut" }
                ToolbarButton { index: 1usize, size: ToolbarButtonSize::Sm, "Copy" }
                ToolbarSeparator {}
                ToolbarButton { index: 2usize, size: ToolbarButtonSize::Sm, "Paste" }
            }

            Toolbar {
                id: "caller-join",
                appearance: ToolbarAppearance::None,
                class: "join",
                aria_label: "Joined toolbar",
                ToolbarButton {
                    index: 0usize,
                    size: ToolbarButtonSize::Sm,
                    class: "join-item",
                    "Left"
                }
                ToolbarButton {
                    index: 1usize,
                    size: ToolbarButtonSize::Sm,
                    class: "join-item",
                    "Centre"
                }
                ToolbarButton {
                    index: 2usize,
                    size: ToolbarButtonSize::Sm,
                    class: "join-item",
                    "Right"
                }
            }
        }
    }
}
