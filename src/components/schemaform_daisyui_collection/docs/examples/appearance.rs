use dioxus::prelude::*;
use schemaform_dioxus::use_form;

use crate::components::schemaform_daisyui::{Addons, Appearance, SchemaformDaisyui};
use crate::components::schemaform_daisyui_collection::CollectionAddon;
use crate::examples::schemaform_daisyui::schemas::{definition, form_data};
use crate::examples::schemaform_daisyui_collection::schemas::{ARRAYS_DATA, ARRAYS_SCHEMA};

/// Every value of the appearance axis on the same arrays.
///
/// The collection takes the form's appearance: `Default` emits its gaps, borders and widths;
/// `None` leaves the `fieldset`, `card`, `btn` and `join` classes and the layout to the caller.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "appearance", class: "grid gap-8",
            for appearance in Appearance::ALL {
                Variant { appearance }
            }
        }
    }
}

/// One form at `appearance`, under a heading naming the value.
#[component]
fn Variant(appearance: Appearance) -> Element {
    let definition = use_hook(|| definition(ARRAYS_SCHEMA, None));
    let form = use_form(definition, form_data(ARRAYS_DATA)).expect("the form should be created");

    rsx! {
        section { "data-value": "{appearance:?}",
            h3 { class: "mb-2 text-sm font-medium", "{appearance:?}" }
            SchemaformDaisyui {
                form,
                appearance,
                addons: Addons::default().with(CollectionAddon),
                on_submit: move |_| {},
            }
        }
    }
}
