use dioxus::prelude::*;

use crate::components::menubar::{
    Menubar, MenubarAppearance, MenubarContent, MenubarContentAppearance, MenubarItem, MenubarMenu,
    MenubarTrigger, MenubarTriggerSize,
};

/// Both appearance axes, which are the utilities this component emits where
/// daisyUI has no class of its own.
///
/// The bar's axis lays the row out (daisyUI's own menu bar is a `menu`, which
/// this element cannot be, ADR-0018) and the popup's places and paints the
/// menu, since there is no `.dropdown` here for `dropdown-content` to position
/// inside. Each `None` emits nothing at all, which is how a caller wins a tie
/// against a utility rather than trying to out-rank it (ADR-0004).
///
/// The popups are opened one at a time, because a menu the primitive has closed
/// is not in the document at all.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            div { "data-axis": "appearance", class: "flex flex-col gap-4",
                for appearance in MenubarAppearance::ALL.iter().copied() {
                    Menubar { appearance,
                        MenubarMenu { index: 0usize,
                            MenubarTrigger { size: MenubarTriggerSize::Sm, "Bar: {appearance:?}" }
                            MenubarContent { class: "w-40",
                                MenubarItem { index: 0usize, value: "{appearance:?}", "An item" }
                            }
                        }
                        MenubarMenu { index: 1usize,
                            MenubarTrigger { size: MenubarTriggerSize::Sm, "Second" }
                            MenubarContent { class: "w-40",
                                MenubarItem { index: 0usize, value: "second", "An item" }
                            }
                        }
                    }
                }
            }

            Menubar { "data-axis-triggers": "content",
                for (index , appearance) in MenubarContentAppearance::ALL.iter().copied().enumerate() {
                    MenubarMenu { index,
                        MenubarTrigger { size: MenubarTriggerSize::Sm, "Popup: {appearance:?}" }
                        MenubarContent { appearance, class: "w-40",
                            MenubarItem { index: 0usize, value: "{appearance:?}", "An item" }
                        }
                    }
                }
            }
        }
    }
}
