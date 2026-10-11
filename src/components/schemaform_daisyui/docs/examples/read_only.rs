use dioxus::prelude::*;
use schemaform_dioxus::use_form;

use crate::components::schemaform_daisyui::SchemaformDaisyui;
use crate::examples::schemaform_daisyui::schemas::{
    OVERVIEW_DATA, OVERVIEW_SCHEMA, definition, form_data,
};

/// A whole form made read-only through its handle, as a host does once an answer
/// is final. Every control becomes output that stays in the tab order, presence
/// operations disappear, and the shell omits its submit button; clearing the
/// flag brings the form back.
#[component]
pub fn Example() -> Element {
    let definition = use_hook(|| definition(OVERVIEW_SCHEMA, None));
    let form = use_form(definition, form_data(OVERVIEW_DATA)).expect("the form should be created");
    let mut read_only = use_signal(|| true);
    let handle = form.clone();
    use_effect(move || {
        let _ = handle.set_read_only(read_only());
    });

    rsx! {
        div { class: "flex flex-col gap-4",
            label { class: "flex items-center gap-2",
                input {
                    r#type: "checkbox",
                    class: "toggle",
                    checked: read_only(),
                    oninput: move |event| read_only.set(event.checked()),
                }
                "Read-only"
            }
            SchemaformDaisyui { form, on_submit: move |_| {} }
        }
    }
}
