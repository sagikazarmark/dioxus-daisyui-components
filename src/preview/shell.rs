use dioxus::prelude::*;

use crate::components::theme_controller::{ThemeController, ThemeControllerAppearance};
use crate::pages::{
    ComponentGroup, PAGE_CATALOG, component_pages, standalone_pages, standalone_sections,
};
use crate::theme::Theme;
use crate::{Route, page_url};

/// The chrome every page is rendered inside: a top bar carrying the site
/// navigation and the theme switcher, and a sidebar carrying the page list.
///
/// It takes the current route rather than reading it, so that every link it
/// renders (the theme switcher's included) is built from the address the page
/// was reached at.
///
/// **Nothing here carries `data-theme`.** The theme is on the document root,
/// where the switcher's own theme controller puts it through daisyUI's
/// `:has()` rule, and daisyUI's base layer paints the root from it, so the
/// page needs no themed wrapper and no colour classes of its own. See ADR-0020.
#[component]
pub fn Shell(route: Route, theme: Theme, children: Element) -> Element {
    let title = route.title();
    let page_id = route.page().map_or("not-found", |page| page.id);

    rsx! {
        document::Title { "{title}" }

        // Browser tooling reads every page and its independent policies from
        // this catalog rather than inferring coverage from visible navigation.
        div { "data-page-catalog": "true", hidden: true,
            for page in PAGE_CATALOG {
                span {
                    key: "{page.id}",
                    "data-catalog-page": page.id,
                    "data-path": page.path,
                    "data-title": page.title,
                    "data-description": page.description,
                    "data-kind": page.kind.name(),
                    // The placement is non-exhaustive upstream, so a placement
                    // this protocol has no marker for is refused rather than
                    // written out as one of the two it does; a wrong marker
                    // would move a page in the browser tooling's coverage
                    // silently, and a facade that gained a third placement is a
                    // change to make here deliberately.
                    "data-navigation": match page.navigation {
                        dioxus_registry_preview::NavigationPlacement::Standalone(_) => "standalone",
                        dioxus_registry_preview::NavigationPlacement::Group(_) => "group",
                        _ => panic!("page catalog contains an unsupported navigation placement"),
                    },
                    "data-navigation-value": match page.navigation {
                        dioxus_registry_preview::NavigationPlacement::Standalone(section) => section,
                        dioxus_registry_preview::NavigationPlacement::Group(group) => group.id(),
                        _ => panic!("page catalog contains an unsupported navigation placement"),
                    },
                    "data-listing": page.listing.as_str(),
                    "data-browser-test": page.browser_tests.as_str(),
                }
            }
        }

        header { class: "sticky top-0 z-30 border-b border-base-300 bg-base-100",
            div { class: "mx-auto flex max-w-7xl items-center gap-6 px-4 py-3",
                Link {
                    class: "text-base font-bold tracking-tight",
                    to: Route::Home { theme },
                    "dioxus-daisyui-components"
                }

                // The site navigation, which is the two halves of the
                // documentation: the prose and the components.
                nav { "data-menu": "top", class: "flex items-center gap-1",
                    TopLink {
                        to: Route::Installation { theme },
                        active: matches!(route, Route::Installation { .. }),
                        "Docs"
                    }
                    TopLink {
                        to: Route::Components { theme },
                        active: matches!(route, Route::Components { .. } | Route::Component { .. }),
                        "Components"
                    }
                }

                div { class: "ml-auto",
                    ThemeSwitcher { route: route.clone(), theme }
                }
            }
        }

        div { class: "mx-auto flex max-w-7xl items-start gap-8 px-4",
            Sidebar { route, theme }

            main { "data-page": page_id, class: "min-w-0 flex-1 py-10", {children} }
        }
    }
}

/// One entry in the top navigation.
#[component]
fn TopLink(to: Route, active: bool, children: Element) -> Element {
    let emphasis = if active { "btn-active" } else { "" };

    rsx! {
        Link { class: "btn btn-ghost btn-sm {emphasis}", to, {children} }
    }
}

/// The visible page list.
///
/// **It scrolls on its own.** The sidebar is stuck below the header and is
/// bounded by what is left of the viewport under it, so a list longer than the
/// screen is reached by scrolling the sidebar rather than by scrolling the page
/// the reader is on, and the scroll stops there rather than carrying on into
/// the page once the list runs out.
///
/// **The components are grouped**, by [`ComponentGroup`], because the list is
/// long enough that a reader looking for one control would otherwise read all
/// of it. The groups are rendered from the group list and each one's pages from
/// the group itself, so the navigation states nothing about which page is where
/// and leaves that to each generated page descriptor.
///
/// The Component switcher contains listed pages only. Browser coverage comes
/// from the complete page catalog rendered by [`Shell`], so listing and test
/// policy stay independent. Each link still carries its stable ID for generic
/// navigation tooling rather than exposing only presentation text.
#[component]
fn Sidebar(route: Route, theme: Theme) -> Element {
    rsx! {
        aside {
            class: "sticky top-14 hidden max-h-[calc(100dvh-3.5rem)] w-56 shrink-0 self-start overflow-y-auto overscroll-contain py-10 lg:block",
            nav { class: "flex flex-col gap-6",
                for section in standalone_sections() {
                    ul { key: "{section}", class: "menu w-full p-0",
                        li { class: "menu-title", "{section}" }
                        for page in standalone_pages(section) {
                            li { key: "{page.id}",
                                Link {
                                    class: if route.page().is_some_and(|current| current.id == page.id) { "menu-active" },
                                    to: page_url(page, theme),
                                    "{page.title}"
                                }
                            }
                        }
                    }
                }

                div { "data-switcher": "component", class: "flex flex-col gap-6",
                    for group in ComponentGroup::ALL.iter().copied() {
                        ul { key: "{group.title()}", class: "menu w-full p-0",
                            li { class: "menu-title", "{group.title()}" }
                            for page in component_pages(group) {
                                li { key: "{page.id}",
                                    Link {
                                        "data-value": page.id,
                                        class: if route.page().is_some_and(|current| current.id == page.id) { "menu-active" },
                                        to: page_url(page, theme),
                                        "{page.title}"
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

/// The theme switcher: every theme daisyUI ships, in a menu.
///
/// **It is what themes the preview.** Its choices are the registry's own theme
/// controller, one radio group over every theme, and the checked one puts its
/// theme on the document root through daisyUI's `:has()` rule: no `data-theme`
/// anywhere, and no state in this app between the reader's click and the page
/// repainting. The preview documents the component by being themed by it
/// (ADR-0020).
///
/// **The URL records the theme, and a pick's navigation is bookkeeping.** The
/// menu has already repainted the page by the time it navigates; what the
/// navigation buys is an address that still names what is on screen, so that a
/// bookmark, a screenshot and a browser test all reach the same page the same
/// way.
///
/// **A theme that arrives from anywhere else has to be pushed back in.** The
/// controller's `default_checked` is the input's `defaultChecked`, which no
/// longer moves a control the reader has touched, so the back button and a typed
/// URL are met by rebuilding the menu from the address, which is what repaints
/// the page. A pick in the menu is not: the browser has already moved the
/// control, and rebuilding it would take focus off the group the arrow keys are
/// walking.
///
/// The choices are in the document whether the menu is open or not, which is
/// also what lets the browser tests read the themes the preview offers off it.
/// Three of them carry a marker as well: a screenshot is taken per page per
/// *baseline* theme rather than per theme, marked in `Theme::ALL`.
///
/// Open and closed are both said outright (`dropdown-open` on the wrapper and
/// `hidden` on the menu) rather than by leaving the open class off. daisyUI
/// also shows a dropdown's content on `:focus-within`, so a menu that had
/// merely lost `dropdown-open` would stay on screen for as long as the button
/// that just closed it kept focus.
///
/// This is not the registry's own dropdown menu, deliberately: that component
/// leaves the primitive to mount its content when it opens, and this menu's
/// content is part of the list of addresses the preview has.
#[component]
fn ThemeSwitcher(route: Route, theme: Theme) -> Element {
    let mut open = use_signal(|| false);

    // The theme the menu is showing, and a counter that changes when it has to
    // be rebuilt to show another one. Stored rather than signalled, because this
    // is bookkeeping about the render rather than state the render is derived
    // from, and a signal written here would ask for the render it is already in.
    let mut shown = use_hook(|| CopyValue::new((theme, 0_u32)));
    let (last, count) = shown();
    let generation = if last == theme {
        count
    } else {
        shown.set((theme, count + 1));
        count + 1
    };

    let (wrapper, menu) = if open() {
        ("dropdown-open", "")
    } else {
        ("", "hidden")
    };

    rsx! {
        div { "data-switcher": "theme", class: "dropdown dropdown-end {wrapper}",
            button {
                class: "btn btn-sm",
                aria_expanded: "{open()}",
                onclick: move |_| open.toggle(),
                "Theme: {theme.title()}"
            }
            ul {
                "data-part": "themes",
                class: "menu dropdown-content z-50 mt-1 max-h-96 w-44 flex-nowrap overflow-y-auto rounded-box bg-base-200 shadow-sm {menu}",
                // Escape is how a reader who walked the list with the arrow keys
                // puts it away, since walking it is not what closes it.
                onkeydown: move |event| {
                    if event.key() == Key::Escape {
                        open.set(false);
                    }
                },
                // The generation is on the `li` rather than on the choice inside
                // it, because a key is what orders a list: the `li`s are the
                // list, and a key on an only child is diffed by position like
                // any other.
                for option in Theme::ALL.iter().copied() {
                    // The menu closes on the pointer that picked, not on the
                    // change: the arrow keys walk a radio group and change the
                    // theme at every step, firing a `click` as they go, which
                    // is why that event cannot tell the two apart, and a menu
                    // that closed under a walk would end it at the first press.
                    // A pointer is somebody deciding; Escape, above, is how the
                    // keyboard says the same.
                    li { key: "{generation}-{option}", onpointerup: move |_| open.set(false),
                        ThemeChoice {
                            target: route.with_theme(option),
                            option,
                            checked: option == theme,
                            shown,
                        }
                    }
                }
            }
        }
    }
}

/// One theme to pick, as the registry's own theme controller.
///
/// The button appearance is daisyUI's own for a set of themes, and it is an
/// input rather than a button: the text is the `aria-label`, which daisyUI
/// prints, and daisyUI paints a checked button primary, so the theme the page
/// is under is the one control in the menu that says so.
#[component]
fn ThemeChoice(
    /// The address this choice leads to, which is where it navigates.
    target: String,
    /// The theme it names, which is also what the browser tests read off it.
    option: Theme,
    /// Whether it is the theme the address is under.
    checked: bool,
    /// The switcher's record of the theme its menu is showing.
    shown: CopyValue<(Theme, u32)>,
) -> Element {
    let mut shown = shown;

    rsx! {
        ThemeController {
            "data-value": option.name(),
            // The themes a screenshot is taken under, marked where the browser
                    // tests read the list, so the set they sweep is the preview's own
            // answer rather than a copy kept beside it.
            "data-baseline": if option.is_baseline() { "true" },
            theme: option.name(),
            appearance: ThemeControllerAppearance::Button,
            // One name over the menu, which is what has the browser keep a
            // single theme checked across it.
            name: "theme",
            class: "btn-sm btn-block btn-ghost justify-start",
            aria_label: option.title(),
            default_checked: checked,
            onchange: move |_| {
                // Recorded before the navigation it causes, so that the render
                // coming back reads as a theme the switcher already knows rather
                // than as one that arrived from outside it, which is what keeps
                // the control the reader is holding from being rebuilt.
                shown.set((option, shown().1));
                navigator().push(target.clone());
            },
        }
    }
}
