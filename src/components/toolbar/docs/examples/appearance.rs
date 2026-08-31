use dioxus::prelude::*;

use crate::components::toolbar::{Toolbar, ToolbarAppearance, ToolbarButton, ToolbarButtonSize};

/// The appearance axis, which is the layout utilities this component emits
/// where daisyUI has no class of its own.
///
/// `Default` lays the row out and `None` emits nothing at all, which is how a
/// caller wins a tie against a utility rather than trying to out-rank it
/// (ADR-0004): with the axis off, the toolbar is an unstyled `div` and the
/// controls fall into the flow the caller writes around them.
///
/// The keyboard is unaffected by either value. Which arrow keys move focus is
/// the primitive's `horizontal` prop, not what the row looks like, which is why
/// the two are emitted from the same prop rather than being separate decisions.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "appearance", class: "flex flex-col gap-4",
            for appearance in ToolbarAppearance::ALL.iter().copied() {
                Toolbar { appearance, aria_label: "{appearance:?}",
                    ToolbarButton { index: 0usize, size: ToolbarButtonSize::Sm,
                        "{appearance:?}: cut"
                    }
                    ToolbarButton { index: 1usize, size: ToolbarButtonSize::Sm,
                        "{appearance:?}: copy"
                    }
                }
            }
        }
    }
}
