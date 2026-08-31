use dioxus::prelude::*;

use crate::components::file_input::FileInput;

/// The file input participates in a form through the browser's native behaviour.
#[component]
pub fn Example() -> Element {
    let mut selected = use_signal(String::new);

    rsx! {
        form { id: "fileinput-form" }
        FileInput {
            id: "form-fileinput",
            accept: "image/png,image/jpeg",
            multiple: true,
            name: "attachments",
            form: "fileinput-form",
            required: true,
            aria_label: "Choose images for the form",
            onchange: move |event: FormEvent| {
                let names = event
                    .files()
                    .iter()
                    .map(|file| file.name())
                    .collect::<Vec<_>>()
                    .join(", ");
                selected.set(names);
            },
        }
        output { id: "selected-files", class: "block pt-2", "{selected}" }
    }
}
