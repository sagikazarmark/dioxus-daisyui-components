use dioxus::prelude::*;

use crate::components::alert::{Alert, AlertAppearance, AlertColor, AlertDirection};

/// A default alert and three independent axes set together.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-2",
            Alert { id: "default-alert", "The scheduled maintenance starts at midnight." }
            Alert {
                color: AlertColor::Success,
                appearance: AlertAppearance::Soft,
                direction: AlertDirection::Vertical,
                "The deployment completed successfully."
            }
        }
    }
}
