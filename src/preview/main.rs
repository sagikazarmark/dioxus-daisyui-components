use dioxus::prelude::*;

pub use dioxus_daisyui_components::components;
mod example;
mod fixtures;
mod pages;
mod shell;
mod theme;

pub(crate) use pages::example_modules as examples;

use pages::{ComponentPage, PAGE_CATALOG, PageDescriptor};
use shell::Shell;
use theme::Theme;

const TAILWIND: Asset = asset!("/assets/style.css");

fn main() {
    dioxus::launch(Preview);
}

/// Every address the preview has: a documentation page, under a theme.
///
/// The page is a path and the theme is a query parameter of every route, which
/// splits them the way they are reached: a page is navigated to and linked
/// between, and a theme restyles whatever page is already open. Both stay in
/// the URL so that a browser test reaches an address by asking for it rather
/// than by clicking its way there, and a screenshot is named after the address
/// it was taken at.
///
/// A theme that is absent or unrecognised resolves to its default, so a bad
/// theme renders the page under `light` rather than an error page. A path that
/// names no page at all is the one thing that cannot fall back, and lands on
/// [`Route::NotFound`].
#[derive(Routable, Clone, Debug, PartialEq)]
enum Route {
    #[route("/?:theme")]
    Home { theme: Theme },
    #[route("/docs/installation?:theme")]
    Installation { theme: Theme },
    #[route("/components?:theme")]
    Components { theme: Theme },
    #[route("/components/:component?:theme")]
    Component {
        component: ComponentPage,
        theme: Theme,
    },
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

impl Route {
    /// The catalog page this route renders, if it names one.
    pub fn page(&self) -> Option<&'static PageDescriptor> {
        let route = self.to_string();
        let path = route.split('?').next().unwrap_or(&route);

        PAGE_CATALOG.iter().find(|page| page.path == path)
    }

    /// What the document is titled at this address, which is what a bookmark
    /// and a browser tab go by.
    pub fn title(&self) -> String {
        let Some(descriptor) = self.page() else {
            return String::from("Not found · dioxus-daisyui-components");
        };
        if descriptor.id == "home" {
            descriptor.title.to_owned()
        } else {
            format!("{} · dioxus-daisyui-components", descriptor.title)
        }
    }

    /// The same address under another theme, which is what the header's theme
    /// switcher links to, so switching keeps the reader where they were.
    pub fn with_theme(&self, theme: Theme) -> String {
        let route = self.to_string();
        let path = route.split('?').next().unwrap_or(&route);

        format!("{path}?theme={theme}")
    }
}

/// A catalog page's address under one theme.
pub fn page_url(page: &PageDescriptor, theme: Theme) -> String {
    format!("{}?theme={theme}", page.path)
}

#[component]
fn Preview() -> Element {
    rsx! {
        document::Stylesheet { href: TAILWIND }

        Router::<Route> {}
    }
}

#[component]
fn Home(theme: Theme) -> Element {
    let route = Route::Home { theme };
    let title = route
        .page()
        .expect("Home must be in the page catalog")
        .title;

    rsx! {
        Shell { route, theme,
            pages::HomePage { theme, title }
        }
    }
}

#[component]
fn Installation(theme: Theme) -> Element {
    let route = Route::Installation { theme };
    let title = route
        .page()
        .expect("Installation must be in the page catalog")
        .title;

    rsx! {
        Shell { route, theme,
            pages::InstallationPage { title }
        }
    }
}

#[component]
fn Components(theme: Theme) -> Element {
    let route = Route::Components { theme };
    let title = route
        .page()
        .expect("Components must be in the page catalog")
        .title;

    rsx! {
        Shell { route, theme,
            pages::ComponentsPage { theme, title }
        }
    }
}

#[component]
fn Component(component: ComponentPage, theme: Theme) -> Element {
    let descriptor: &'static PageDescriptor = component.descriptor();

    rsx! {
        Shell { route: Route::Component { component, theme }, theme,
            article { class: "flex flex-col gap-10",
                header { class: "flex flex-col gap-2",
                    h1 { class: "text-3xl font-bold", "{descriptor.title}" }
                    p { class: "opacity-70", "{descriptor.description}" }
                }

                {component.view()}

                if descriptor.id == "radio_group" {
                    fixtures::RadioGroupFocusFixtures {}
                }
            }
        }
    }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    let path = segments.join("/");

    rsx! {
        Shell { route: Route::NotFound { segments: segments.clone() }, theme: Theme::default(),
            article { class: "flex flex-col items-start gap-4",
                h1 { class: "text-3xl font-bold", "No page at /{path}" }
                p { class: "opacity-70",
                    "The sidebar lists every component the registry publishes."
                }
                Link {
                    class: "btn btn-primary",
                    to: Route::Home {
                        theme: Theme::default(),
                    },
                    "Back to the start"
                }
            }
        }
    }
}
