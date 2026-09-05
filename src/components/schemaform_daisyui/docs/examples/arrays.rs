use dioxus::prelude::*;
use schemaform_dioxus::use_form;

use crate::components::schemaform_daisyui::SchemaformDaisyui;
use crate::examples::schemaform_daisyui::schemas::{
    ARRAYS_DATA, ARRAYS_SCHEMA, definition, form_data,
};

/// Two homogeneous arrays as daisyUI collections.
///
/// Each array is a focusable fieldset named by its legend; each item a card
/// titled by the item noun and its position, with insert, move and remove as a
/// join of square icon buttons whose accessible names carry the position. The
/// adapter decides which affordances exist: the first item cannot move up, the
/// last cannot move down, append disappears at `maxItems`, and the team's sole
/// remaining member cannot be removed at `minItems`. Removing the optional tags
/// array offers to materialize it again. Identity, focus after a mutation and
/// the live-region announcements are the adapter's; the renderer only places
/// what it is handed.
#[component]
pub fn Example() -> Element {
    let definition = use_hook(|| definition(ARRAYS_SCHEMA, None));
    let form = use_form(definition, form_data(ARRAYS_DATA)).expect("the form should be created");

    rsx! {
        SchemaformDaisyui { form, on_submit: move |_| {} }
    }
}
