use dioxus::prelude::*;
use dioxus_registry_preview::ExampleDocumentation;

use crate::components::tabs::{TabContent, TabContentAppearance, TabTrigger, Tabs, TabsAppearance};

/// One example on a component page: a title, a line about what it shows, and a
/// card that switches between the rendered example and the code behind it.
///
/// The two are the same code. What the preview tab renders is the component the
/// caller passes as children, and what the code tab prints is the source of the
/// file that component is written in, read at build time by `include_str!`, so
/// a snippet cannot drift from what it documents and a snippet that stopped
/// compiling fails the build rather than the reader.
///
/// The card is the registry's own [`Tabs`], which is the preview using what it
/// documents: every component page is rendered inside one.
#[component]
pub fn ExampleSection(documentation: ExampleDocumentation, children: Element) -> Element {
    let ExampleDocumentation {
        slug,
        title,
        description,
        source,
        ..
    } = documentation;

    rsx! {
        // The id is the slug prefixed rather than the slug itself, so that a
        // deep link to an example cannot collide with an id an example put on
        // something it rendered: `#controlled` is both a section here and a
        // component under test on three of these pages.
        section {
            id: "example-{slug}",
            "data-example": slug,
            class: "flex scroll-mt-20 flex-col gap-2",
            h2 { class: "text-xl font-semibold", "{title}" }
            p { class: "text-sm opacity-70", "{description}" }

            // The size axis is left at its default: daisyUI's `tabs-md`, which
            // is what its own documentation renders these tabs at, and which
            // gives a tab enough padding to read as a tab rather than as a word
            // above a box.
            Tabs {
                class: "mt-1",
                appearance: TabsAppearance::Lift,
                default_value: "preview".to_string(),
                TabTrigger { value: "preview".to_string(), index: 0usize, "Preview" }
                // The panel keeps its own utilities and adds the hatching, which
                // paints behind the example without displacing it: the pattern
                // is a background image and the surface it sits on is the
                // panel's background colour.
                TabContent {
                    "data-example-content": "true",
                    value: "preview".to_string(),
                    index: 0usize,
                    class: "example-preview",
                    {children}
                }

                TabTrigger { value: "code".to_string(), index: 1usize, "RSX" }
                TabContent {
                    value: "code".to_string(),
                    index: 1usize,
                    // The panel's own utilities are switched off and written
                    // here instead, because the code block brings its own
                    // padding; a panel that padded as well would inset the
                    // scrollbar along with the code (ADR-0004).
                    appearance: TabContentAppearance::None,
                    class: "border-base-300 bg-base-200",
                    pre { class: "overflow-x-auto p-4 text-xs leading-relaxed",
                        code { "{source}" }
                    }
                }
            }
        }
    }
}

/// Trusted, build-time Markdown from a Component README.
#[component]
pub fn ReadmeSection(html: &'static str) -> Element {
    if html.is_empty() {
        return rsx! {};
    }

    rsx! {
        section {
            class: "flex flex-col gap-4 [&_a]:link [&_code]:font-mono [&_h2]:mt-4 [&_h2]:text-xl [&_h2]:font-semibold [&_li]:ml-5 [&_li]:list-disc [&_p_code]:rounded [&_p_code]:bg-base-200 [&_p_code]:px-1 [&_pre]:overflow-x-auto [&_pre]:rounded-box [&_pre]:bg-base-200 [&_pre]:p-4 [&_pre]:text-xs [&_strong]:font-semibold",
            dangerous_inner_html: html,
        }
    }
}
