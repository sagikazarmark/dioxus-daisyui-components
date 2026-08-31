use dioxus::prelude::*;

use crate::components::combobox::{
    Combobox, ComboboxInput, ComboboxList, ComboboxOption, ComboboxOptionAppearance,
};

/// The option the keyboard is on, painted and switched off.
///
/// This is the one thing a combobox cannot borrow from the select. A select
/// moves real DOM focus onto its options, so daisyUI's own `:focus-visible` rule
/// highlights them; a combobox never does, because focus has to stay in the
/// field for typing to keep working. The option the arrow keys are on is named
/// by `aria-activedescendant` and reported as `data-highlighted` instead, so the
/// paint is a Bridged utility over that attribute (ADR-0021) and, being a
/// utility this component emits, it comes with a value that emits nothing.
///
/// Both are open and neither is highlighted until an arrow key is pressed in the
/// field.
#[component]
pub fn Example() -> Element {
    rsx! {
        div {
            "data-axis": "option-appearance",
            class: "flex flex-wrap items-start gap-8 pb-72",
            for appearance in ComboboxOptionAppearance::ALL.iter().copied() {
                Combobox::<String> { open: Some(true),
                    ComboboxInput { class: "w-40", placeholder: "{appearance:?}" }
                    ComboboxList {
                        for (index , fruit) in ["Orange", "Lemon", "Cherry"]
                            .iter()
                            .copied()
                            .enumerate()
                        {
                            ComboboxOption::<String> {
                                key: "{fruit}",
                                appearance,
                                value: fruit.to_string(),
                                index,
                                "{fruit}"
                            }
                        }
                    }
                }
            }
        }
    }
}
