use dioxus::prelude::*;

use crate::components::collapsible::{
    Collapsible, CollapsibleContent, CollapsibleMarker, CollapsibleTrigger,
};

/// A disclosure that answers to nobody, open to start with.
///
/// `default_open` seeds the state this component lifts, so the panel is open on
/// the first render rather than after the first click, and daisyUI's open class
/// is on the root from that same first render.
///
/// The title is a button: it takes focus, Enter and Space work on it, and it
/// announces through `aria-expanded` what the panel below it is doing.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex w-full max-w-md flex-col gap-3",
            Collapsible { id: "overview", marker: CollapsibleMarker::Arrow, default_open: true,
                CollapsibleTrigger { "What ships when I install a component?" }
                CollapsibleContent {
                    "One directory: the component's module and nothing else. The manifest and these notes stay here."
                }
            }

            Collapsible { marker: CollapsibleMarker::Arrow,
                CollapsibleTrigger { "Does the panel exist while it is closed?" }
                CollapsibleContent {
                    "The element does: it is a row of daisyUI's grid. What you wrote inside it is mounted only while the panel is open, unless you set keep_mounted."
                }
            }
        }
    }
}
