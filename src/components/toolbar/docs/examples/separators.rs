use dioxus::prelude::*;

use crate::components::toolbar::{
    Toolbar, ToolbarButton, ToolbarButtonSize, ToolbarSeparator, ToolbarSeparatorColor,
};

/// Every value of the separator colour axis, and the rule's own orientation.
///
/// A separator's orientation is the opposite of the toolbar it sits in, which
/// the primitive works out and this component follows: the rules in the row run
/// down it, and the one in the column beside it runs across. daisyUI's names for
/// the two are the other way round (the class that draws a rule down a row is
/// `divider-horizontal`) which is the one trap the separator component
/// documents too.
///
/// The row of rules is given a height of its own because that is all it holds: a
/// divider stretches to the tallest thing beside it, and there is nothing beside
/// these.
///
/// `Default` emits no class, which is daisyUI's own rule: the page's text colour
/// mixed down to a tenth.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-start gap-6",
            Toolbar { "data-axis": "separator", class: "h-8", aria_label: "Separator colours",
                for color in ToolbarSeparatorColor::ALL.iter().copied() {
                    ToolbarSeparator { color, decorative: true }
                }
            }

            Toolbar {
                id: "vertical",
                horizontal: false,
                aria_label: "Vertical toolbar",
                ToolbarButton { index: 0usize, size: ToolbarButtonSize::Sm, "Top" }
                ToolbarSeparator {}
                ToolbarButton { index: 1usize, size: ToolbarButtonSize::Sm, "Middle" }
                ToolbarButton { index: 2usize, size: ToolbarButtonSize::Sm, "Bottom" }
            }
        }
    }
}
