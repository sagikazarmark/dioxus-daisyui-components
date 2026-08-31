use dioxus::prelude::*;

use crate::components::combobox::{Combobox, ComboboxInput, ComboboxList};
use crate::examples::combobox::options::Options;

/// What a query does to the list: it keeps the options that match, and shows
/// the empty line when it keeps none.
///
/// Both queries are the caller's rather than typed, so that the two states
/// stand still; the filtering itself is the primitive's and is the same either
/// way. An option the query drops is rendered as nothing at all, which is why
/// its list item hides itself when it is empty: daisyUI draws an empty list item
/// as a divider rule.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-start gap-8 pb-72",
            Combobox::<String> {
                id: "matching",
                open: Some(true),
                query: Some("le".to_string()),
                ComboboxInput { class: "w-40" }
                ComboboxList {
                    Options {}
                }
            }

            Combobox::<String> {
                id: "unmatched",
                open: Some(true),
                query: Some("kiwi".to_string()),
                ComboboxInput { class: "w-40" }
                ComboboxList {
                    Options {}
                }
            }
        }
    }
}
