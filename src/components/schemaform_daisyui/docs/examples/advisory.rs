use dioxus::prelude::*;
use schemaform_dioxus::{SubmissionMode, use_form};

use crate::components::schemaform_daisyui::SchemaformDaisyui;
use crate::examples::schemaform_daisyui::schemas::{OVERVIEW_SCHEMA, definition, form_data};

/// Advisory submission delivers data and findings to its own host callback without
/// moving focus. Switching mode on the mounted form also changes its presentation.
#[component]
pub fn Example() -> Element {
    let definition = use_hook(|| definition(OVERVIEW_SCHEMA, None));
    let form = use_form(definition, form_data(r#"{"name":"A","plan":"team","tags":[]}"#))
        .expect("the form should be created");
    let mut advisory = use_signal(|| true);
    let mut submitted = use_signal(String::new);
    rsx! {
        label { class: "flex items-center gap-2",
            input { r#type: "checkbox", class: "checkbox", checked: advisory(), oninput: move |event| advisory.set(event.checked()) }
            "Advisory mode"
        }
        SchemaformDaisyui {
            form,
            submission_mode: if advisory() { SubmissionMode::Advisory } else { SubmissionMode::Gated },
            on_submit: move |_| submitted.set("Validated submission".to_owned()),
            on_advisory_submit: move |submission: schemaform::AdvisorySubmission| {
                submitted.set(format!("Advisory submission: {} findings", submission.findings().count()));
            },
        }
        output { "aria-label": "Submission result", "{submitted}" }
    }
}
