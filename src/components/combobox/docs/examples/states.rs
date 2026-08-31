use dioxus::prelude::*;

use crate::components::combobox::{Combobox, ComboboxInput, ComboboxList};
use crate::examples::combobox::options::Options;

/// A disabled combobox, one that starts on a value, and a caller's classes on
/// the box.
///
/// The disabled one needs no class of its own: the primitive puts the attribute
/// on the field, which is a real `input`, and daisyUI's `.input` matches it. The
/// chosen row does need one: `.menu` matches no ARIA attribute at all, so
/// `menu-active` is the only thing telling it from the rest, and it is emitted
/// from the value this component lifted (ADR-0006).
///
/// The caller's class is a width, which is the case worth demonstrating: it is
/// the class that says whether the split reached the options: a width on the
/// box that the list inside it did not take would leave the options
/// shrink-wrapped in a wider box.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-start gap-8 pb-72",
            Combobox::<String> { id: "disabled", disabled: true,
                ComboboxInput { class: "w-40", placeholder: "Disabled" }
                ComboboxList {
                    Options {}
                }
            }

            Combobox::<String> {
                id: "selected",
                default_value: "Lemon".to_string(),
                open: Some(true),
                ComboboxInput { class: "w-40" }
                ComboboxList {
                    Options {}
                }
            }

            Combobox::<String> { open: Some(true),
                ComboboxInput { class: "w-40", placeholder: "Caller" }
                ComboboxList { id: "caller-attributes", class: "w-52",
                    Options {}
                }
            }
        }
    }
}
