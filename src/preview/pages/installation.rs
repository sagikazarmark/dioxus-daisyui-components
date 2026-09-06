use dioxus::prelude::*;

/// The installation page.
///
/// It is a placeholder in one sense only: the prose it carries is the
/// repository's README, written out here so that the documentation site is
/// reachable end to end, and the reference documentation it links on to has
/// still to be written. The steps themselves are the real ones.
#[component]
pub fn InstallationPage(title: &'static str) -> Element {
    rsx! {
        article { class: "flex flex-col gap-10",
            header { class: "flex flex-col gap-2",
                h1 { class: "text-3xl font-bold", "{title}" }
                p { class: "opacity-70",
                    "The registry ships no CSS. Your app supplies Tailwind and daisyUI, and "
                    code { class: "kbd kbd-sm", "dx" }
                    " copies a component into your own source tree."
                }
            }

            Step { number: 1, title: "Load Tailwind and daisyUI",
                p {
                    code { class: "kbd kbd-sm", "dx" }
                    " runs its own pinned Tailwind over a single stylesheet at your crate root,
                     and that file is the only input it processes. Install daisyUI next to your
                     crate:"
                }
                Code { language: "shell", source: INSTALL }
                p {
                    "And write "
                    code { class: "kbd kbd-sm", "tailwind.css" }
                    " at your crate root:"
                }
                Code { language: "css", source: STYLESHEET }
            }

            Step { number: 2, title: "Bundle the generated stylesheet",
                p {
                    code { class: "kbd kbd-sm", "dx" }
                    " writes the compiled CSS next to your assets; bundle it from your app:"
                }
                Code { language: "rust", source: BUNDLE }
            }

            Step { number: 3, title: "Install a component",
                Code { language: "shell", source: ADD }
                p {
                    "The first install creates "
                    code { class: "kbd kbd-sm", "src/components/" }
                    " and prints the one manual step it leaves to you: add "
                    code { class: "kbd kbd-sm", "mod components;" }
                    " to your "
                    code { class: "kbd kbd-sm", "main.rs" }
                    ". Cargo dependencies are added for you."
                }
                p {
                    "Then rebuild. "
                    code { class: "kbd kbd-sm", "dx serve" }
                    " and "
                    code { class: "kbd kbd-sm", "dx build" }
                    " run Tailwind over your stylesheet, which is where the component's class
                     names are picked up. There are no other setup steps."
                }
            }

            Step { number: 4, title: "Use it",
                p {
                    "Every daisyUI styling axis is its own prop, so they combine freely the way
                     daisyUI's classes do. Your own classes concatenate with the component's, and
                     your own attributes override it:"
                }
                Code { language: "rust", source: USAGE }
            }
        }
    }
}

const INSTALL: &str = "npm install daisyui@5";

const STYLESHEET: &str = r#"@import "tailwindcss";
@plugin "daisyui";"#;

const BUNDLE: &str = r#"const TAILWIND: Asset = asset!("/assets/tailwind.css");

rsx! {
    document::Stylesheet { href: TAILWIND }
}"#;

const ADD: &str = "dx components add button --path /path/to/dioxus-daisyui-components";

const USAGE: &str = r#"use crate::components::button::{Button, ButtonColor, ButtonSize};

rsx! {
    Button {
        color: ButtonColor::Primary,
        size: ButtonSize::Lg,
        class: "w-full",
        onclick: move |_| tracing::info!("clicked"),
        "Save"
    }
}"#;

/// One numbered step, as a heading and whatever the step is made of.
///
/// The prose is dimmed, and only the prose: a code block in the step sits on
/// the code surface, which is one colour across the site (ADR-0033) and would
/// come out a different one under the opacity.
#[component]
fn Step(number: u8, title: &'static str, children: Element) -> Element {
    rsx! {
        section { class: "flex flex-col gap-3",
            h2 { class: "flex items-center gap-3 text-xl font-semibold",
                span { class: "badge badge-neutral badge-sm", "{number}" }
                "{title}"
            }
            div { class: "flex flex-col gap-3 text-sm [&>p]:opacity-80", {children} }
        }
    }
}

/// A block of code, on the surface an example's code tab prints on and at the
/// same measurements, so the two read as one kind of block. It is not
/// highlighted: the tab's highlighting is worked out at compile time from a
/// Rust source file, and these are shell, CSS and fragments (ADR-0033).
#[component]
fn Code(language: &'static str, source: &'static str) -> Element {
    rsx! {
        pre {
            "data-language": language,
            class: "overflow-x-auto rounded-box border border-base-300 bg-code-surface p-4 text-xs leading-relaxed text-code-content",
            code { "{source}" }
        }
    }
}
