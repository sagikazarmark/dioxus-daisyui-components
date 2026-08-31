use dioxus::prelude::*;

use crate::components::menubar::{
    Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger, MenubarTriggerSize,
};

/// A menu nothing opens, and a bar that is disabled outright.
///
/// A disabled menu keeps its trigger in the bar and refuses to open it, which is
/// the primitive's: the state travels down to the trigger and to every item
/// inside it.
///
/// Neither emits a daisyUI class on the trigger. The primitive does not set the
/// native `disabled` attribute on a menubar trigger the way it does on a
/// toolbar's button, so a disabled trigger looks like an enabled one; the gap
/// is documented rather than papered over with a class the state does not
/// support.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3",
            Menubar { id: "states",
                MenubarMenu { index: 0usize,
                    MenubarTrigger { size: MenubarTriggerSize::Sm, "Open me" }
                    MenubarContent { class: "w-40",
                        MenubarItem { index: 0usize, value: "first", "An item" }
                        MenubarItem { index: 1usize, value: "second", disabled: true, "A disabled item" }
                    }
                }

                MenubarMenu { index: 1usize, disabled: true,
                    MenubarTrigger { size: MenubarTriggerSize::Sm, "Disabled menu" }
                    MenubarContent { class: "w-40",
                        MenubarItem { index: 0usize, value: "never", "Never reached" }
                    }
                }
            }

            Menubar { id: "disabled", disabled: true,
                MenubarMenu { index: 0usize,
                    MenubarTrigger { size: MenubarTriggerSize::Sm, "File" }
                    MenubarContent { class: "w-40",
                        MenubarItem { index: 0usize, value: "new", "New" }
                    }
                }
                MenubarMenu { index: 1usize,
                    MenubarTrigger { size: MenubarTriggerSize::Sm, "Edit" }
                    MenubarContent { class: "w-40",
                        MenubarItem { index: 0usize, value: "cut", "Cut" }
                    }
                }
            }
        }
    }
}
