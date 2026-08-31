use dioxus::prelude::*;

use crate::components::collapsible::{
    Collapsible, CollapsibleContent, CollapsibleMarker, CollapsibleTrigger,
};

/// A disabled disclosure, and one whose panel stays in the document while it is
/// closed.
///
/// The disabled one is inert: the primitive puts the native `disabled`
/// attribute on the title button, so it takes no clicks and no focus. daisyUI
/// has no disabled collapse, so it looks like any other; `data-[disabled=true]`
/// through `class` is how a caller says otherwise, which is what the second half
/// of the first one does.
///
/// The kept one mounts its children whether it is open or not. Everything else
/// here mounts them when the panel opens, which is the primitive's default and
/// the reason a closed panel weighs nothing.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex w-full max-w-md flex-col gap-3",
            Collapsible {
                id: "disabled",
                disabled: true,
                marker: CollapsibleMarker::Plus,
                class: "data-[disabled=true]:opacity-50",
                CollapsibleTrigger { "Rollback (not available on this plan)" }
                CollapsibleContent { "Upgrade to reach this." }
            }

            Collapsible { id: "kept", marker: CollapsibleMarker::Plus, keep_mounted: true,
                CollapsibleTrigger { "Kept mounted while closed" }
                CollapsibleContent {
                    p { id: "kept-content", "This paragraph is in the document either way." }
                }
            }
        }
    }
}
