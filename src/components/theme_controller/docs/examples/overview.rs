use dioxus::prelude::*;

use crate::components::theme_controller::{ThemeController, ThemeControllerAppearance};

/// A row of themes to pick one of, which is daisyUI's own theme controller.
///
/// Every control is a radio under one name, so the browser keeps exactly one
/// checked and the arrow keys walk the row. The one that is checked names the
/// theme the document is under: picking another re-declares that theme on
/// `:root` through daisyUI's `:has()` rule, with nothing in Rust to run.
///
/// The button appearance draws an input as a button, and daisyUI prints its
/// `aria-label` as the text; an input has no children to write it as.
///
/// **The themes named here are not daisyUI's**, so these controls check and
/// paint without moving the page they are documented on: daisyUI writes its
/// `:has()` rule once per theme it emits, and a value it never emitted matches
/// nothing. It has to be that way round here, because the preview enables every
/// theme daisyUI ships; an example naming one of those would re-theme the site
/// from inside its own documentation. The switcher in this page's header is this
/// same component over the real list, which is where it can be seen doing the
/// real thing.
#[component]
pub fn Example() -> Element {
    let themes = [
        ("parchment", "Parchment"),
        ("midnight", "Midnight"),
        ("seafoam", "Seafoam"),
    ];

    rsx! {
        div { class: "join",
            for (theme , title) in themes {
                ThemeController {
                    key: "{theme}",
                    id: "overview-{theme}",
                    theme,
                    appearance: ThemeControllerAppearance::Button,
                    class: "join-item",
                    name: "overview-theme",
                    aria_label: title,
                    default_checked: theme == "parchment",
                }
            }
        }
    }
}
