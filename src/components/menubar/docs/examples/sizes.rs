use dioxus::prelude::*;

use crate::components::menubar::{
    Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger, MenubarTriggerSize,
};

/// Every value of the trigger size axis, smallest first.
///
/// `Default` emits no class and renders at the same size as daisyUI's explicit
/// `btn-md`, which is why it sits in the middle of the bar rather than at one
/// end of it.
///
/// The menu's own size axis (which sizes the items inside a popup rather than
/// the trigger above it) is on the popup and is shown where the popups are
/// opened.
#[component]
pub fn Example() -> Element {
    rsx! {
        Menubar { "data-axis": "size",
            for (index , size) in MenubarTriggerSize::ALL.iter().copied().enumerate() {
                MenubarMenu { index,
                    MenubarTrigger { size, "{size:?}" }
                    MenubarContent { class: "w-40",
                        MenubarItem { index: 0usize, value: "{size:?}", "An item" }
                    }
                }
            }
        }
    }
}
