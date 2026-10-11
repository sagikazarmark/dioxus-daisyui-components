use dioxus::prelude::*;
use schemaform_dioxus::use_form;

use crate::components::schemaform_daisyui::{Addons, SchemaformDaisyui};
use crate::components::schemaform_daisyui_collection::CollectionAddon;
use crate::examples::schemaform_daisyui::schemas::{definition, form_data};
use crate::examples::schemaform_daisyui_collection::schemas::{ARRAYS_DATA, ARRAYS_SCHEMA};

/// The add-on in `SchemaformDaisyui`'s `addons`: each array is a fieldset of item cards.
///
/// The tags start from data and may be removed as a whole; the team needs at least one member,
/// so its sole item cannot be removed. Each card names its item and position, its actions sit in
/// a join, and append is withdrawn once an array is full.
#[component]
pub fn Example() -> Element {
    let definition = use_hook(|| definition(ARRAYS_SCHEMA, None));
    let form = use_form(definition, form_data(ARRAYS_DATA)).expect("the form should be created");

    rsx! {
        SchemaformDaisyui {
            form,
            addons: Addons::default().with(CollectionAddon),
            on_submit: move |_| {},
        }
    }
}
