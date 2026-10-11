use dioxus::prelude::*;
use schemaform_dioxus::use_form;

use crate::components::schemaform_daisyui::{Addons, ButtonClasses, SchemaformDaisyui};
use crate::components::schemaform_daisyui_collection::CollectionAddon;
use crate::examples::schemaform_daisyui::schemas::{definition, form_data};
use crate::examples::schemaform_daisyui_collection::schemas::{ARRAYS_DATA, ARRAYS_SCHEMA};

/// The host's own classes on the collection's buttons: soft item actions and an accent append
/// button. The item actions keep `join-item btn-square`; the classes carry their own sizes.
#[component]
pub fn Example() -> Element {
    let definition = use_hook(|| definition(ARRAYS_SCHEMA, None));
    let form = use_form(definition, form_data(ARRAYS_DATA)).expect("the form should be created");
    let buttons = ButtonClasses::default()
        .item("btn-soft btn-sm")
        .append("btn-accent btn-sm");

    rsx! {
        SchemaformDaisyui {
            form,
            buttons,
            addons: Addons::default().with(CollectionAddon),
            on_submit: move |_| {},
        }
    }
}
