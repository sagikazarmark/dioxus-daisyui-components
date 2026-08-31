use dioxus::prelude::*;

use crate::components::toolbar::{Toolbar, ToolbarButton, ToolbarButtonColor};

/// Every value of the button colour axis.
///
/// The axis is on the control rather than on the toolbar: daisyUI's colours are
/// per-button, so a toolbar-wide colour would be an API daisyUI does not have,
/// and a row of differently coloured controls would then need a way back out of
/// it.
///
/// `Default` emits no class, which is daisyUI's uncoloured button rather than a
/// synonym for neutral.
#[component]
pub fn Example() -> Element {
    rsx! {
        Toolbar { "data-axis": "color", aria_label: "Colours",
            for (index , color) in ToolbarButtonColor::ALL.iter().copied().enumerate() {
                ToolbarButton { index, color, "{color:?}" }
            }
        }
    }
}
