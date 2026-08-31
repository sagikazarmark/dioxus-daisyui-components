mod home;
mod installation;

pub use home::{ComponentsPage, HomePage};
pub use installation::InstallationPage;

use dioxus_registry_preview::{BrowserTestPolicy, ListingPolicy, NavigationPlacement};

/// The section of the navigation a component page is listed under.
///
/// Groups belong to the Preview rather than the Registry. Each Component's
/// documentation module assigns its page to one of them.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ComponentGroup {
    Actions,
    DataInput,
    Navigation,
    Overlays,
    DataDisplay,
}

impl ComponentGroup {
    /// Every group, in navigation order.
    pub const ALL: &'static [Self] = &[
        Self::Actions,
        Self::DataInput,
        Self::Navigation,
        Self::Overlays,
        Self::DataDisplay,
    ];

    /// The heading the navigation writes above the group.
    pub const fn title(self) -> &'static str {
        match self {
            Self::Actions => "Actions",
            Self::DataInput => "Data input",
            Self::Navigation => "Navigation",
            Self::Overlays => "Overlays",
            Self::DataDisplay => "Data display",
        }
    }

    /// The stable ID authored by Component documentation modules.
    pub const fn id(self) -> &'static str {
        match self {
            Self::Actions => "actions",
            Self::DataInput => "data-input",
            Self::Navigation => "navigation",
            Self::Overlays => "overlays",
            Self::DataDisplay => "data-display",
        }
    }
}

/// How the Preview renders one catalog page.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PageKind {
    Component(ComponentPage),
    Rsx,
}

impl PageKind {
    /// The stable page-kind name rendered in the DOM catalog.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Component(_) => "generated-component",
            Self::Rsx => "rsx",
        }
    }
}

/// One page in the Preview's complete ordered catalog.
pub type PageDescriptor = dioxus_registry_preview::PageDescriptor<ComponentGroup, PageKind>;

const HOME_PAGE: PageDescriptor = PageDescriptor::new(
    "home",
    "/",
    "dioxus-daisyui",
    "A dx components registry of Dioxus components that pair dioxus-primitives behaviour with daisyUI class names.",
    NavigationPlacement::Standalone("Getting started"),
    ListingPolicy::Unlisted,
    BrowserTestPolicy::Disabled,
    PageKind::Rsx,
);

const INSTALLATION_PAGE: PageDescriptor = PageDescriptor::new(
    "installation",
    "/docs/installation",
    "Installation",
    "Set up Tailwind and daisyUI, then install a Component into your app.",
    NavigationPlacement::Standalone("Getting started"),
    ListingPolicy::Listed,
    BrowserTestPolicy::Disabled,
    PageKind::Rsx,
);

const COMPONENTS_PAGE: PageDescriptor = PageDescriptor::new(
    "components",
    "/components",
    "Components",
    "Every Component the Registry publishes.",
    NavigationPlacement::Standalone("Components"),
    ListingPolicy::Unlisted,
    BrowserTestPolicy::Disabled,
    PageKind::Rsx,
);

// Root Registry membership is the one list of Component pages. The generated
// dispatch keeps each page as a literal Dioxus component call, as ADR-0009
// requires, rather than storing component function pointers in a table.
dioxus_registry_preview::component_pages! {
    manifest: "../../../component.json",
    group: ComponentGroup {
        "actions" => ComponentGroup::Actions,
        "data-input" => ComponentGroup::DataInput,
        "navigation" => ComponentGroup::Navigation,
        "overlays" => ComponentGroup::Overlays,
        "data-display" => ComponentGroup::DataDisplay,
    },
    default: button,
    catalog: PageDescriptor {
        path: "/components",
        component: PageKind::Component,
        custom: [HOME_PAGE, INSTALLATION_PAGE, COMPONENTS_PAGE],
    },
}

impl ComponentPage {
    /// This Component page's descriptor in the complete catalog.
    pub fn descriptor(self) -> &'static PageDescriptor {
        PAGE_CATALOG
            .iter()
            .find(|page| matches!(page.kind, PageKind::Component(component) if component == self))
            .expect("every generated Component page must have a catalog descriptor")
    }
}

/// Each standalone navigation heading, in catalog order.
pub fn standalone_sections() -> impl Iterator<Item = &'static str> {
    PAGE_CATALOG
        .iter()
        .enumerate()
        .filter_map(|(index, page)| match page.navigation {
            NavigationPlacement::Standalone(section) if page.listing.is_listed() => {
                (!PAGE_CATALOG[..index].iter().any(|earlier| {
                    earlier.listing.is_listed()
                        && matches!(
                            earlier.navigation,
                            NavigationPlacement::Standalone(earlier_section)
                                if earlier_section == section
                        )
                }))
                .then_some(section)
            }
            _ => None,
        })
}

/// The listed standalone pages under one navigation heading.
pub fn standalone_pages(section: &'static str) -> impl Iterator<Item = &'static PageDescriptor> {
    PAGE_CATALOG.iter().filter(move |page| {
        page.listing.is_listed()
            && matches!(
                page.navigation,
                NavigationPlacement::Standalone(page_section) if page_section == section
            )
    })
}

/// The listed Component pages in one group, in Registry manifest order.
pub fn component_pages(group: ComponentGroup) -> impl Iterator<Item = &'static PageDescriptor> {
    PAGE_CATALOG.iter().filter(move |page| {
        page.listing.is_listed()
            && matches!(page.navigation, NavigationPlacement::Group(page_group) if page_group == group)
    })
}
