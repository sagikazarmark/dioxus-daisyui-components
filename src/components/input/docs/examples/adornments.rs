use dioxus::prelude::*;

use crate::components::input::Input;

/// Adornments render inside the control box through the `span.input` wrapper.
///
/// Adornments are non-interactive by contract: icons, text, and `kbd` hints.
/// Decorative ones carry `aria-hidden`. daisyUI paints the disabled state from
/// the nested input, so the wrapper needs no class of its own for it.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            Input {
                id: "prefix-input",
                prefix: rsx! { "https://" },
                aria_label: "Registry URL",
                placeholder: "registry.example.com",
            }
            Input {
                id: "suffix-input",
                class: "tabular-nums",
                suffix: rsx! { "EUR" },
                inputmode: "decimal",
                aria_label: "Amount in euros",
                placeholder: "0.00",
            }
            Input {
                id: "both-input",
                prefix: rsx! { "$" },
                suffix: rsx! { ".00" },
                inputmode: "numeric",
                aria_label: "Amount in dollars",
                placeholder: "0",
            }
            Input {
                id: "icon-adorned-input",
                prefix: rsx! {
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        fill: "none",
                        class: "size-4 stroke-current opacity-50",
                        "aria-hidden": "true",
                        path {
                            d: "m21 21-4.34-4.34M11 18a7 7 0 1 0 0-14 7 7 0 0 0 0 14Z",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                        }
                    }
                },
                suffix: rsx! {
                    kbd { class: "kbd kbd-sm", "aria-hidden": "true", "⌘K" }
                },
                r#type: "search",
                aria_label: "Search components",
                placeholder: "Search",
            }
            Input {
                id: "disabled-adorned-input",
                suffix: rsx! { "EUR" },
                aria_label: "Disabled amount",
                placeholder: "0.00",
                disabled: true,
            }
        }
    }
}
