use dioxus::prelude::*;

use crate::components::native_select::{NativeSelect, NativeSelectOption};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Flavor {
    Orange,
    Lemon,
    Cherry,
}

/// A native enum picker: the browser draws the popup daisyUI's select styles.
#[component]
pub fn Example() -> Element {
    let mut value: Signal<Option<Flavor>> = use_signal(|| None);

    rsx! {
        NativeSelect {
            id: "default-native-select",
            aria_label: "Flavor",
            placeholder: "Pick a flavor",
            options: vec![
                NativeSelectOption::new(Flavor::Orange, "Orange"),
                NativeSelectOption::new(Flavor::Lemon, "Lemon"),
                NativeSelectOption::new(Flavor::Cherry, "Cherry"),
            ],
            on_change: move |next| value.set(next),
        }
    }
}
