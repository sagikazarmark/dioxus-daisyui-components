use dioxus::prelude::*;

use crate::components::combobox::{
    Combobox, ComboboxEmpty, ComboboxInput, ComboboxList, ComboboxOption,
};

/// The options every combobox on this page holds.
const OPTIONS: &[&str] = &["Orange", "Lemon", "Cherry"];

/// The option none of the comboboxes lets a caller reach, which is what makes
/// arrow-key navigation skipping it observable.
const DISABLED: &str = "Currant";

/// Those options in a combobox, held open so that they can be seen.
///
/// Every other example on this page imports [`Options`] from here rather than
/// writing the list out again: what they vary is the field and the box.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex pb-72",
            Combobox::<String> { open: Some(true),
                ComboboxInput { placeholder: "Shared options" }
                ComboboxList {
                    Options {}
                }
            }
        }
    }
}

/// The options every combobox on this page holds, and the line that shows when
/// the query keeps none of them.
///
/// The index is the keyboard navigation order rather than the position in the
/// DOM, which is what lets daisyUI's list-item wrappers stand between the list
/// and its options, and what keeps the order steady while the query hides some
/// of them.
#[component]
pub fn Options() -> Element {
    rsx! {
        for (index , option) in OPTIONS.iter().copied().enumerate() {
            ComboboxOption::<String> {
                key: "{option}",
                value: option.to_string(),
                index,
                "{option}"
            }
        }

        // The disabled option closes the list, so that skipping it is a step
        // over the end of the list as well as over a row.
        ComboboxOption::<String> {
            value: DISABLED.to_string(),
            index: OPTIONS.len(),
            disabled: true,
            "{DISABLED}"
        }

        ComboboxEmpty { "No fruit by that name" }
    }
}
