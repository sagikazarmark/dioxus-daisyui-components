use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::hover_card;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::{ContentAlign, ContentSide};

/// Which side of the trigger the panel is on.
///
/// This is an axis of Tailwind utilities rather than of daisyUI classes,
/// because daisyUI has none: its own floating boxes are dropdowns, and ADR-0015
/// records why a hover card cannot be one. The value is also what the primitive
/// is told, so that the `data-side` it reports and the side the panel is
/// actually on are the same thing.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum HoverCardSide {
    /// Above the trigger.
    Top,
    /// To the right of the trigger, in either writing direction; the
    /// primitive's sides are physical rather than logical.
    Right,
    /// Under the trigger, which is where a hover card usually goes and where
    /// this axis leaves it.
    #[default]
    Bottom,
    /// To the left of the trigger, mirroring [`HoverCardSide::Right`].
    Left,
}

impl HoverCardSide {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Top, Self::Right, Self::Bottom, Self::Left];

    /// The Tailwind utility that puts the panel on this side, as a complete
    /// string literal so Tailwind's scanner can see it.
    ///
    /// No offset goes with it. A gap between the trigger and the panel is a gap
    /// the pointer crosses, and a pointer that leaves the trigger without
    /// arriving on the panel closes the card.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Top => "bottom-full",
            Self::Right => "left-full",
            Self::Bottom => "top-full",
            Self::Left => "right-full",
        }
    }

    /// What the primitive is told, so that the `data-side` it reports says the
    /// same thing the utility above did.
    const fn side(self) -> ContentSide {
        match self {
            Self::Top => ContentSide::Top,
            Self::Right => ContentSide::Right,
            Self::Bottom => ContentSide::Bottom,
            Self::Left => ContentSide::Left,
        }
    }
}

/// Where the panel sits along the side [`HoverCardSide`] put it on.
///
/// The utility depends on both axes rather than on this one alone: aligning to
/// the start of a trigger means the left edge under a panel that is above or
/// below it, and the top edge beside a panel that is to one side. daisyUI's
/// dropdown expresses the same pair as two independent classes because it
/// resolves them in one anchor rule; here the two make one utility, and the two
/// axes stay separate in the API because that is how a caller thinks about
/// them.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum HoverCardAlign {
    /// The start edge of the trigger.
    Start,
    /// Centred on the trigger, which is the primitive's own default.
    #[default]
    Center,
    /// The end edge of the trigger, mirroring [`HoverCardAlign::Start`].
    End,
}

impl HoverCardAlign {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Start, Self::Center, Self::End];

    /// The Tailwind utilities that align the panel on a given side, as complete
    /// string literals so Tailwind's scanner can see them.
    pub const fn class(self, side: HoverCardSide) -> &'static str {
        match (side, self) {
            (HoverCardSide::Top | HoverCardSide::Bottom, Self::Start) => "left-0",
            (HoverCardSide::Top | HoverCardSide::Bottom, Self::Center) => {
                "left-1/2 -translate-x-1/2"
            }
            (HoverCardSide::Top | HoverCardSide::Bottom, Self::End) => "right-0",
            (HoverCardSide::Left | HoverCardSide::Right, Self::Start) => "top-0",
            (HoverCardSide::Left | HoverCardSide::Right, Self::Center) => {
                "top-1/2 -translate-y-1/2"
            }
            (HoverCardSide::Left | HoverCardSide::Right, Self::End) => "bottom-0",
        }
    }

    /// What the primitive is told, so that the `data-align` it reports says the
    /// same thing the utilities above did.
    const fn align(self) -> ContentAlign {
        match self {
            Self::Start => ContentAlign::Start,
            Self::Center => ContentAlign::Center,
            Self::End => ContentAlign::End,
        }
    }
}

/// daisyUI's size axis for a card, which sets the padding of the body and the
/// size of the title rather than anything on the card itself.
///
/// [`HoverCardSize::Default`] emits no class, which renders at the same size as
/// daisyUI's explicit `card-md`.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum HoverCardSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

impl HoverCardSize {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Xs, Self::Sm, Self::Default, Self::Lg, Self::Xl];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Xs => "card-xs",
            Self::Sm => "card-sm",
            Self::Default => "",
            Self::Lg => "card-lg",
            Self::Xl => "card-xl",
        }
    }
}

/// daisyUI's border axis for a card.
///
/// [`HoverCardBorder::Default`] emits nothing, which is daisyUI's borderless
/// card: a panel that reads against the page by its fill and its shadow rather
/// than by an edge.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum HoverCardBorder {
    #[default]
    Default,
    Solid,
    Dashed,
}

impl HoverCardBorder {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::Solid, Self::Dashed];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Solid => "card-border",
            Self::Dashed => "card-dash",
        }
    }
}

/// Whether [`HoverCard`] emits the utilities that make it the box the panel is
/// positioned against.
///
/// daisyUI would have given this element `dropdown`, which is exactly these two
/// utilities and a set of reveal rules that cannot be used here (ADR-0015). So
/// the two are emitted directly, on the inverted convention the appearance axes
/// follow: [`HoverCardAppearance::Default`] emits classes and
/// [`HoverCardAppearance::None`] emits nothing (ADR-0004).
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum HoverCardAppearance {
    #[default]
    Default,
    None,
}

impl HoverCardAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "relative inline-block",
            Self::None => "",
        }
    }
}

/// Whether [`HoverCardContent`] emits the utilities that take the panel out of
/// the flow and put it on the side the axes name.
///
/// Switching it off leaves the panel where the document would have put it,
/// which is what a caller who positions it themselves (anchor positioning, a
/// floating-element library, a panel that is not floating at all) needs
/// (ADR-0004). The side and alignment axes still travel to the primitive, so
/// what it reports stays true even when nothing here is placing the panel.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum HoverCardPositioning {
    #[default]
    Default,
    None,
}

impl HoverCardPositioning {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities this value emits beside the side and alignment
    /// ones, as complete string literals so Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "absolute z-10",
            Self::None => "",
        }
    }
}

/// Whether [`HoverCardContent`] emits the utilities that paint the card.
///
/// daisyUI's `card` rounds a box and lays it out but paints nothing: the fill
/// and the shadow in its own examples are Tailwind utilities on the same
/// element. They are emitted here instead, on the inverted convention the other
/// appearance axes follow (ADR-0004), and a floating panel needs them more
/// than a card in the flow does, since a panel that does not paint over what is
/// behind it is a panel with the page showing through it.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum HoverCardContentAppearance {
    #[default]
    Default,
    None,
}

impl HoverCardContentAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "bg-base-100 shadow-sm",
            Self::None => "",
        }
    }
}

/// The element a hover card's trigger and panel live in, and the box the panel
/// is positioned against.
///
/// **There is no state to bridge.** daisyUI's card is a box: it has no open
/// state, no hidden state and no class that reveals anything, and the primitive
/// mounts the panel only while the card is open. What daisyUI would have
/// supplied instead (the positioning of a floating box) it supplies only for
/// dropdowns, and ADR-0015 records why a hover card cannot be one. So the
/// positioning is Tailwind utilities here and in [`HoverCardContent`], and both
/// are defeatable.
///
/// The open state travels through to the primitive untouched, so a controlled
/// caller and an uncontrolled one both get its own behaviour: opening on
/// `mouseenter` and on `focus`, staying open while the pointer is on the panel
/// itself, and closing on leave and on blur.
///
/// Classes passed by the caller concatenate with this element's own; every other
/// attribute the caller passes overrides them.
#[component]
pub fn HoverCard(
    /// Whether to emit the utilities that make this element the panel's
    /// positioning context.
    #[props(default)]
    appearance: HoverCardAppearance,
    /// The controlled open state of the card.
    #[props(default)]
    open: ReadSignal<Option<bool>>,
    /// Whether the card starts open when it is not controlled.
    #[props(default)]
    default_open: bool,
    /// Called when the open state changes.
    #[props(default)]
    on_open_change: Callback<bool>,
    /// Whether the card is disabled, which leaves the trigger inert and the
    /// panel unopenable.
    #[props(default)]
    disabled: ReadSignal<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(div {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        hover_card::HoverCard {
            open,
            default_open,
            on_open_change,
            disabled,
            attributes: merged,
            {children}
        }
    }
}

/// What the card opens from.
///
/// It emits nothing. daisyUI has no class for the thing a hover card is
/// attached to, and what it looks like is the caller's: a link, a name, an
/// avatar. The primitive's own element is a `div` with `tabindex="0"` and
/// `role="button"`, which is what makes a hover card on plain text reachable by
/// keyboard, and it is what carries `aria-describedby` while the panel is up.
#[component]
pub fn HoverCardTrigger(
    /// The id of this element. Declared rather than left to the attribute list,
    /// because the primitive generates one and puts it on the element; an id
    /// arriving as an attribute would land on top of that one rather than
    /// replace it, and the two would disagree.
    #[props(default)]
    id: ReadSignal<Option<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        hover_card::HoverCardTrigger { id, attributes, {children} }
    }
}

/// The panel itself, carrying daisyUI's `card` classes.
///
/// The panel is positioned by utilities rather than by daisyUI, which is
/// ADR-0015's decision: `dropdown-content` would have positioned it, and the
/// rules that come with it take pointer events off the first child of an open
/// dropdown, which here is the trigger the card opens and closes on.
///
/// The side and alignment axes do two things at once: they emit the utilities
/// that place the panel, and they are what the primitive is told, so the
/// `data-side` and `data-align` it reports agree with where the panel actually
/// is.
///
/// Classes passed by the caller concatenate with the panel's own: a width in
/// particular, which daisyUI's own card examples always carry and which this
/// component deliberately does not emit.
#[component]
pub fn HoverCardContent(
    /// Which side of the trigger the panel is on.
    #[props(default)]
    side: HoverCardSide,
    /// Where the panel sits along that side.
    #[props(default)]
    align: HoverCardAlign,
    /// daisyUI's size axis, which sets the body's padding and the title's size.
    #[props(default)]
    size: HoverCardSize,
    /// daisyUI's border axis.
    #[props(default)]
    border: HoverCardBorder,
    /// Whether to emit the utilities that take the panel out of the flow.
    #[props(default)]
    positioning: HoverCardPositioning,
    /// Whether to emit the utilities that paint the panel.
    #[props(default)]
    appearance: HoverCardContentAppearance,
    /// The id of this element. Declared rather than left to the attribute list,
    /// because the primitive generates one and then points the trigger's
    /// `aria-describedby` at it; an id arriving as an attribute would replace
    /// the one the trigger is still naming.
    #[props(default)]
    id: ReadSignal<Option<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let size = size.class();
    let border = border.class();
    let appearance = appearance.class();
    // The alignment utility depends on the side as well as on the alignment,
    // and both are switched off together: a panel that is not taken out of the
    // flow has nothing to be placed against.
    let (position, alignment) = match positioning {
        HoverCardPositioning::Default => (side.class(), align.class(side)),
        _ => ("", ""),
    };
    let positioning = positioning.class();

    let base = attributes!(div {
        class: "card {size} {border} {positioning} {position} {alignment} {appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        hover_card::HoverCardContent {
            id,
            side: side.side(),
            align: align.align(),
            attributes: merged,
            {children}
        }
    }
}

/// The padded inside of the panel, carrying daisyUI's `card-body` class.
///
/// daisyUI puts every part of a card's content in here (the title, the text,
/// the actions) and the card's size axis reaches this element rather than the
/// one it is written on. A panel with content directly inside the card and no
/// body is a panel with no padding at all, which is daisyUI's arrangement
/// rather than this component's.
#[component]
pub fn HoverCardBody(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div { class: "card-body" });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged, {children} }
    }
}

/// The panel's heading, carrying daisyUI's `card-title` class.
///
/// It is a `div` rather than a heading element, because a hover card is a
/// `role="tooltip"` panel: its content is announced through the trigger's
/// `aria-describedby` rather than walked as a document outline, and a heading
/// inside it would appear in a page's outline at whatever level it was given. A
/// caller who wants one writes it as a child.
#[component]
pub fn HoverCardTitle(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "card-title",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged, {children} }
    }
}

/// A hover card's panel and its body, as one component rather than as
/// [`HoverCardContent`] wrapped around [`HoverCardBody`].
///
/// The collapse is legal: daisyUI has nothing between `.card` and `.card-body`,
/// and no caller content goes there: its own examples put a figure beside the
/// body, which is a sibling of the body rather than something between the two.
///
/// **Caller attributes land on the panel**, which is the element worth
/// reaching: a width, a maximum width and a fill all belong there, and the body
/// only pads. Reaching the body means dropping to the parts.
#[component]
pub fn HoverCardPanel(
    /// Which side of the trigger the panel is on.
    #[props(default)]
    side: HoverCardSide,
    /// Where the panel sits along that side.
    #[props(default)]
    align: HoverCardAlign,
    /// daisyUI's size axis.
    #[props(default)]
    size: HoverCardSize,
    /// daisyUI's border axis.
    #[props(default)]
    border: HoverCardBorder,
    /// Whether to emit the utilities that take the panel out of the flow.
    #[props(default)]
    positioning: HoverCardPositioning,
    /// Whether to emit the utilities that paint the panel.
    #[props(default)]
    appearance: HoverCardContentAppearance,
    /// The id of the panel.
    #[props(default)]
    id: ReadSignal<Option<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        HoverCardContent {
            side,
            align,
            size,
            border,
            positioning,
            appearance,
            id,
            attributes,
            HoverCardBody { {children} }
        }
    }
}
