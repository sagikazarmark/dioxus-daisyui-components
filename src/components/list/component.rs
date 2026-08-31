use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

/// Whether a [`ListColumn`] takes the width left over in its row instead of
/// daisyUI's default second child.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ListColumnGrow {
    #[default]
    Default,
    Grow,
}

impl ListColumnGrow {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::Grow];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Grow => "list-col-grow",
        }
    }
}

/// Whether a [`ListColumn`] wraps onto a line below the rest of its row.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ListColumnWrap {
    #[default]
    Default,
    Wrap,
}

impl ListColumnWrap {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::Wrap];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Wrap => "list-col-wrap",
        }
    }
}

/// Rows of content in a vertical stack, carrying daisyUI's `list` class.
///
/// Classes passed by the caller concatenate with the list's own; every other
/// attribute the caller passes overrides the list's.
#[component]
pub fn List(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(ul { class: "list" });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        ul { ..merged, {children} }
    }
}

/// One row in a [`List`], carrying daisyUI's `list-row` class.
#[component]
pub fn ListRow(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(li { class: "list-row" });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        li { ..merged, {children} }
    }
}

/// Caller content inside a [`ListRow`] that can grow or wrap with daisyUI's
/// row-child modifiers.
#[component]
pub fn ListColumn(
    /// Whether this column takes the width left over in its row.
    #[props(default)]
    grow: ListColumnGrow,
    /// Whether this column wraps onto a line below the rest of its row.
    #[props(default)]
    wrap: ListColumnWrap,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let grow = grow.class();
    let wrap = wrap.class();

    let base = attributes!(div {
        class: "{grow} {wrap}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged, {children} }
    }
}
