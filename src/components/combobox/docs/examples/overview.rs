use dioxus::prelude::*;

use crate::components::combobox::{
    Combobox, ComboboxEmpty, ComboboxInput, ComboboxList, ComboboxOption,
};

/// A combobox that answers to nobody: it holds its own value, its own open
/// state and the text it is filtered by.
///
/// Typing narrows the list, the arrow keys walk what is left of it, Enter
/// chooses and Escape closes, all of it the primitive's. What this component
/// adds is the field's daisyUI class and the popup borrowed from daisyUI's
/// dropdown and menu.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex pb-64",
            Combobox::<String> {
                ComboboxInput { placeholder: "Pick a fruit" }
                ComboboxList {
                    for (index , fruit) in ["Apple", "Apricot", "Banana", "Blackberry", "Cherry"]
                        .iter()
                        .copied()
                        .enumerate()
                    {
                        ComboboxOption::<String> {
                            key: "{fruit}",
                            value: fruit.to_string(),
                            index,
                            "{fruit}"
                        }
                    }
                    ComboboxEmpty { "No fruit by that name" }
                }
            }
        }
    }
}
