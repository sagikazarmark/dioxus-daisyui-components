use dioxus::prelude::*;

use crate::components::collapsible::{
    Collapsible, CollapsibleContent, CollapsibleMarker, CollapsibleTrigger,
};

/// Every value of the marker axis, which is the sign daisyUI draws in the
/// corner of the title.
///
/// daisyUI draws it as an `::after` on the title, and turns it as the root
/// gains the open class, so the marker and the state are the same thing seen
/// twice. `Default` draws none, which is a title with nothing in its corner
/// rather than a disclosure that cannot be opened.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "marker", class: "grid w-full gap-3 sm:grid-cols-3",
            for marker in CollapsibleMarker::ALL.iter().copied() {
                Collapsible { marker,
                    CollapsibleTrigger { "{marker:?}" }
                    CollapsibleContent { "The sign in the corner above is this axis." }
                }
            }
        }
    }
}
