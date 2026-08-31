use dioxus::prelude::*;

use crate::components::select::{
    Select, SelectGroup, SelectGroupLabel, SelectList, SelectOption, SelectTrigger, SelectValue,
};

/// The options every select on this page holds, grouped the way daisyUI's menu
/// groups a list: a title, then the rows it labels.
const GROUPS: &[(&str, &[&str])] = &[("Citrus", &["Orange", "Lemon"]), ("Berries", &["Cherry"])];

/// The option none of the selects lets a caller reach, which is what makes
/// arrow-key navigation and typeahead skipping it observable.
const DISABLED: &str = "Currant";

/// Those options in a select, held open so that they can be seen.
///
/// Every other example on this page imports [`Options`] from here rather than
/// writing the groups out again: what they vary is the field and the box.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex pb-72",
            Select::<String> { open: Some(true),
                SelectTrigger {
                    SelectValue { placeholder: "Shared options" }
                }
                SelectList {
                    Options {}
                }
            }
        }
    }
}

/// The options every select on this page holds, in their groups.
///
/// The index is counted across the groups rather than within one, because it is
/// the keyboard navigation order for the whole list: an option registers by
/// the index it is given rather than by where it sits in the DOM, which is what
/// lets the groups and daisyUI's list-item wrappers stand between the list and
/// its options.
#[component]
pub fn Options() -> Element {
    rsx! {
        for (group , (label , options)) in GROUPS.iter().enumerate() {
            SelectGroup { key: "{label}",
                SelectGroupLabel { "{label}" }
                for (offset , option) in options.iter().copied().enumerate() {
                    SelectOption::<String> {
                        key: "{option}",
                        value: option.to_string(),
                        index: index_of(group, offset),
                        "{option}"
                    }
                }
                // The disabled option closes the last group, so that skipping
                // it is a step over the end of the list as well as over a row.
                if group + 1 == GROUPS.len() {
                    SelectOption::<String> {
                        value: DISABLED.to_string(),
                        index: index_of(group, options.len()),
                        disabled: true,
                        "{DISABLED}"
                    }
                }
            }
        }
    }
}

/// Where an option falls in the keyboard navigation order, counted across the
/// groups before its own.
fn index_of(group: usize, offset: usize) -> usize {
    GROUPS[..group]
        .iter()
        .map(|(_, options)| options.len())
        .sum::<usize>()
        + offset
}
