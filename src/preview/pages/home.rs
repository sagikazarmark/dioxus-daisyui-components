use dioxus::prelude::*;

use crate::pages::{ComponentGroup, component_pages};
use crate::theme::Theme;
use crate::{Route, page_url};

/// The landing page: what the registry is, and the two ways in.
#[component]
pub fn HomePage(theme: Theme, title: &'static str) -> Element {
    rsx! {
        article { class: "flex flex-col gap-10",
            header { class: "flex flex-col items-start gap-4",
                h1 { class: "text-4xl font-bold tracking-tight", "{title}" }
                p { class: "max-w-2xl text-lg opacity-70",
                    "A "
                    code { class: "kbd kbd-sm", "dx components" }
                    " registry of Dioxus components that pair dioxus-primitives behaviour with
                     daisyUI class names."
                }

                div { class: "flex flex-wrap gap-2",
                    Link { class: "btn btn-primary", to: Route::Installation { theme }, "Get started" }
                    Link { class: "btn", to: Route::Components { theme }, "Browse components" }
                }
            }

            div { class: "grid gap-4 sm:grid-cols-3",
                Point { title: "Copied, not depended on",
                    "A component is copied into your own source tree, where it emits daisyUI class
                     names and nothing else."
                }
                Point { title: "No CSS of its own",
                    "Your app supplies Tailwind and daisyUI. The registry ships no stylesheet and
                     defines no theme, so your "
                    code { class: "kbd kbd-xs", "[data-theme]" }
                    " restyles everything at once."
                }
                Point { title: "Behaviour from the primitives",
                    "Focus management, keyboard navigation, ARIA wiring and dismissal are
                     dioxus-primitives'. The registry owns the mapping to daisyUI's selectors."
                }
            }
        }
    }
}

/// One of the three cards on the landing page.
#[component]
fn Point(title: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "card border border-base-300 bg-base-100",
            div { class: "card-body gap-2 p-5",
                h2 { class: "card-title text-base", "{title}" }
                p { class: "text-sm opacity-70", {children} }
            }
        }
    }
}

/// The component index, which is the sidebar's component list written out as
/// cards for a wider viewport.
///
/// It is grouped as the sidebar is, and by the same [`ComponentGroup`]: a
/// reader who has learned where a component lives from one of the two finds it
/// in the same place in the other. The cards carry the description as well,
/// which is what this page is for: the sidebar is a list of names to navigate
/// by, and this is the list read to find out what the registry has.
#[component]
pub fn ComponentsPage(theme: Theme, title: &'static str) -> Element {
    rsx! {
        article { class: "flex flex-col gap-10",
            header { class: "flex flex-col gap-2",
                h1 { class: "text-3xl font-bold", "{title}" }
                p { class: "opacity-70",
                    "Every component the registry publishes. Each one installs on its own with "
                    code { class: "kbd kbd-sm", "dx components add" }
                    "."
                }
            }

            for group in ComponentGroup::ALL.iter().copied() {
                section { key: "{group.title()}", class: "flex flex-col gap-4",
                    h2 { class: "text-xl font-semibold tracking-tight", "{group.title()}" }

                    ul { class: "grid gap-4 sm:grid-cols-2",
                        for page in component_pages(group) {
                            li { key: "{page.id}", class: "contents",
                                Link {
                                    class: "card border border-base-300 bg-base-100 transition-colors hover:border-primary",
                                    to: page_url(page, theme),
                                    div { class: "card-body gap-2 p-5",
                                        h3 { class: "card-title text-base", "{page.title}" }
                                        p { class: "text-sm opacity-70", "{page.description}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
