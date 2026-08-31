use dioxus::prelude::*;

use crate::components::select::{Select, SelectList, SelectTrigger, SelectValue};
use crate::examples::select::options::Options;

/// A select that owns its own state, which is the whole component in four
/// parts: the select, the field, the value it shows and the list under it.
///
/// The field is daisyUI's `select`, caret and all; the popup is the dropdown's
/// structure borrowed whole (ADR-0005). The list stays in the document while
/// the popup is closed (hidden) because that is where an option registers the
/// text the field shows once it is the chosen one.
#[component]
pub fn Example() -> Element {
    rsx! {
        Select::<String> {
            SelectTrigger {
                SelectValue { placeholder: "Pick a fruit" }
            }
            SelectList {
                Options {}
            }
        }
    }
}
