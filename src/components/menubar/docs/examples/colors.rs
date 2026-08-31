use dioxus::prelude::*;

use crate::components::menubar::{
    Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger, MenubarTriggerColor,
};

/// Every value of the trigger colour axis.
///
/// The axis is on the trigger rather than on the bar: daisyUI's colours are
/// per-button, and a bar-wide colour would be an API daisyUI does not have. The
/// triggers are `btn` because the bar cannot be a daisyUI `menu` (ADR-0018
/// records why) which is also what gives them these axes at all.
///
/// `Default` emits no class, which is daisyUI's uncoloured button rather than a
/// synonym for neutral.
#[component]
pub fn Example() -> Element {
    rsx! {
        Menubar { "data-axis": "color",
            for (index , color) in MenubarTriggerColor::ALL.iter().copied().enumerate() {
                MenubarMenu { index,
                    MenubarTrigger { color, "{color:?}" }
                    MenubarContent { class: "w-40",
                        MenubarItem { index: 0usize, value: "{color:?}", "An item" }
                    }
                }
            }
        }
    }
}
