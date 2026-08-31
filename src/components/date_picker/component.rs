use dioxus::prelude::*;
use dioxus_primitives::calendar;
use dioxus_primitives::calendar::DateRange;
use dioxus_primitives::date_picker;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::popover;
use dioxus_primitives::{ContentAlign, ContentSide};
use time::{Date, Month, OffsetDateTime, UtcDateTime};

/// daisyUI's colour axis for the field, which colours its border and the
/// outline it takes when a segment inside it is focused.
///
/// The class strings are the input's rather than a picker's (daisyUI has no
/// date picker) and they land on the element daisyUI wrote them for: `.input`
/// is a flex row that a caller may put things inside, which is exactly what the
/// primitive's field is.
///
/// [`DatePickerColor::Default`] emits no class at all, which is daisyUI's own
/// uncoloured field rather than a synonym for [`DatePickerColor::Neutral`].
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum DatePickerColor {
    #[default]
    Default,
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}

impl DatePickerColor {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[
        Self::Default,
        Self::Neutral,
        Self::Primary,
        Self::Secondary,
        Self::Accent,
        Self::Info,
        Self::Success,
        Self::Warning,
        Self::Error,
    ];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Neutral => "input-neutral",
            Self::Primary => "input-primary",
            Self::Secondary => "input-secondary",
            Self::Accent => "input-accent",
            Self::Info => "input-info",
            Self::Success => "input-success",
            Self::Warning => "input-warning",
            Self::Error => "input-error",
        }
    }
}

/// daisyUI's size axis for the field, which sets its height and the smallest
/// font it renders at.
///
/// [`DatePickerSize::Default`] emits no class, which renders at the same size
/// as daisyUI's explicit `input-md`.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum DatePickerSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

impl DatePickerSize {
    /// Every value of this axis, from the smallest to the largest, which is the
    /// order the preview renders them in.
    pub const ALL: &'static [Self] = &[Self::Xs, Self::Sm, Self::Default, Self::Lg, Self::Xl];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Xs => "input-xs",
            Self::Sm => "input-sm",
            Self::Default => "",
            Self::Lg => "input-lg",
            Self::Xl => "input-xl",
        }
    }
}

/// Whether the segments of the field are painted at all.
///
/// A segment is a `span` the primitive makes editable, not an `input`, so
/// daisyUI's rules for what is inside a field reach it for its font and nothing
/// else. What is emitted here is what tells a segment apart from the text
/// around it: the one being typed into, the ones nothing has been typed into
/// yet, and the numbers lining up as they are typed.
///
/// The inverted convention (ADR-0004): [`DatePickerSegmentAppearance::Default`]
/// emits and [`DatePickerSegmentAppearance::None`] emits nothing, because what
/// it carries is utilities this component emits rather than daisyUI classes.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum DatePickerSegmentAppearance {
    #[default]
    Default,
    None,
}

impl DatePickerSegmentAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for a segment, as complete string literals so
    /// Tailwind's scanner can see them.
    ///
    /// The faded state is written as a variant of `no-date`, which is the
    /// attribute the primitive sets on a segment nothing has been typed into:
    /// a Bridged utility, with nothing recomputed here.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => concat!(
                "rounded-sm px-0.5 tabular-nums",
                " focus:bg-primary focus:text-primary-content focus:outline-none",
                " [&[no-date=true]]:opacity-50",
                " data-[disabled=true]:opacity-50",
            ),
            Self::None => "",
        }
    }

    /// The Tailwind utilities for the separator between two segments, which is
    /// quieter than the numbers it stands between.
    pub const fn separator_class(self) -> &'static str {
        match self {
            Self::Default => "opacity-50",
            Self::None => "",
        }
    }
}

/// The look of the button that opens the calendar.
///
/// It is a daisyUI button, and this is which one: the flat square that sits
/// inside a field without filling it, or `btn` and whatever the caller adds.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum DatePickerTriggerAppearance {
    /// daisyUI's `btn-ghost btn-square btn-sm`, sized to sit inside the field.
    #[default]
    Default,
    /// `btn` and nothing else.
    None,
}

impl DatePickerTriggerAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The daisyUI class names for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "btn-ghost btn-square btn-sm",
            Self::None => "",
        }
    }
}

/// Which side of the field the calendar opens on.
///
/// Tailwind utilities rather than daisyUI classes, because daisyUI's only
/// floating box is the dropdown and this cannot be one: `.dropdown-content` is
/// hidden unless `dropdown-open` is on the element above it, and the open state
/// of a date picker lives in a context the primitive keeps to itself; there is
/// nothing here to emit the class from. The hover card reaches the same place
/// from the other direction (ADR-0015).
///
/// The value is also what the primitive is told, so that the `data-side` it
/// reports and the side the calendar is actually on are the same thing.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum DatePickerSide {
    /// Above the field.
    Top,
    /// To the right of the field, in either writing direction; the primitive's
    /// sides are physical rather than logical.
    Right,
    /// Under the field, which is where a date picker usually opens.
    #[default]
    Bottom,
    /// To the left of the field, mirroring [`DatePickerSide::Right`].
    Left,
}

impl DatePickerSide {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Top, Self::Right, Self::Bottom, Self::Left];

    /// The Tailwind utility that puts the calendar on this side, as a complete
    /// string literal so Tailwind's scanner can see it.
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

/// Where the calendar sits along the side [`DatePickerSide`] put it on.
///
/// The utility depends on both axes rather than on this one alone, for the
/// reason the hover card's does: aligning to the start of a field means its left
/// edge under a panel above or below it, and its top edge beside a panel to one
/// side.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum DatePickerAlign {
    /// The start edge of the field, which is where a calendar usually lines up.
    #[default]
    Start,
    /// Centred on the field.
    Center,
    /// The end edge of the field, mirroring [`DatePickerAlign::Start`].
    End,
}

impl DatePickerAlign {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Start, Self::Center, Self::End];

    /// The Tailwind utilities that align the calendar on a given side, as
    /// complete string literals so Tailwind's scanner can see them.
    pub const fn class(self, side: DatePickerSide) -> &'static str {
        match (side, self) {
            (DatePickerSide::Top | DatePickerSide::Bottom, Self::Start) => "left-0",
            (DatePickerSide::Top | DatePickerSide::Bottom, Self::Center) => {
                "left-1/2 -translate-x-1/2"
            }
            (DatePickerSide::Top | DatePickerSide::Bottom, Self::End) => "right-0",
            (DatePickerSide::Left | DatePickerSide::Right, Self::Start) => "top-0",
            (DatePickerSide::Left | DatePickerSide::Right, Self::Center) => {
                "top-1/2 -translate-y-1/2"
            }
            (DatePickerSide::Left | DatePickerSide::Right, Self::End) => "bottom-0",
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

/// Whether [`DatePickerPopover`] emits the utilities that make it the box the
/// calendar is positioned against.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum DatePickerPopoverAppearance {
    #[default]
    Default,
    None,
}

impl DatePickerPopoverAppearance {
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

/// Whether [`DatePickerContent`] emits the utilities that take the calendar out
/// of the flow and put it on the side the axes name.
///
/// Switching it off leaves the calendar where the document would have put it,
/// which is what a caller who positions it themselves needs (ADR-0004). The side
/// and alignment axes still travel to the primitive, so what it reports stays
/// true even when nothing here is placing the panel.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum DatePickerPositioning {
    #[default]
    Default,
    None,
}

impl DatePickerPositioning {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities this value emits beside the side and alignment
    /// ones, as complete string literals so Tailwind's scanner can see them.
    ///
    /// The width is one of them, and it is load-bearing rather than cosmetic: an
    /// absolutely positioned panel is offered the width of the box it is
    /// positioned against, which here is the field, so without it the month
    /// would be squeezed into the width of a date rather than the field sitting
    /// under a month.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "absolute z-10 mt-1 w-max",
            Self::None => "",
        }
    }
}

/// Whether [`DatePickerContent`] emits the utilities that draw the box around
/// the calendar.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum DatePickerContentAppearance {
    #[default]
    Default,
    None,
}

impl DatePickerContentAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "rounded-box border border-base-300 bg-base-100 p-4 shadow-sm",
            Self::None => "",
        }
    }
}

/// Whether the month inside the popup is painted.
///
/// [`DatePickerCalendar`] renders a whole month rather than parts a caller
/// composes, so this one axis covers all of it: the row that changes the month,
/// the title, the grid and every state a day can be in. Each of those is the
/// same mapping the calendar component makes: daisyUI has no calendar
/// (ADR-0022), so the days are `btn` and the states are Tailwind variants of
/// the attributes the primitive sets.
///
/// Switched off, the month is a bare table of unstyled buttons, which is what a
/// caller who wants to draw their own wants and, since this component ships
/// its own month rather than parts, is the escape hatch that keeps the popup
/// theirs.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum DatePickerCalendarAppearance {
    #[default]
    Default,
    None,
}

impl DatePickerCalendarAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The utilities that stack a month's header over its grid.
    pub const fn view_class(self) -> &'static str {
        match self {
            Self::Default => "flex flex-col gap-2",
            Self::None => "",
        }
    }

    /// The utilities that put the title between the two month buttons.
    pub const fn navigation_class(self) -> &'static str {
        match self {
            Self::Default => "flex items-center justify-between gap-2",
            Self::None => "",
        }
    }

    /// The daisyUI classes for a month button.
    pub const fn button_class(self) -> &'static str {
        match self {
            Self::Default => "btn btn-ghost btn-square btn-sm",
            Self::None => "",
        }
    }

    /// The utilities that set the month's title apart.
    pub const fn title_class(self) -> &'static str {
        match self {
            Self::Default => "text-sm font-medium",
            Self::None => "",
        }
    }

    /// The utilities for the table the month is laid out in.
    ///
    /// **The table is not given a width.** The panel it sits in is `w-max`,
    /// which asks the month how wide it wants to be, so a `w-full` here would
    /// be a percentage of the answer to the question it is being asked, and a
    /// browser resolves that cycle by handing the table every pixel it can,
    /// which is what left the panel a million wide. `table-fixed` stays: it is
    /// what makes the seven columns one width once there is a width to divide,
    /// and what decides that width is the day drawn in them.
    pub const fn grid_class(self) -> &'static str {
        match self {
            Self::Default => "table-fixed",
            Self::None => "",
        }
    }

    /// The utilities for a weekday heading.
    ///
    /// The padding is vertical only, so that a label never out-sizes the day
    /// under it: the heading shares a column with a week of days, and the
    /// widest thing in a column is what `table-fixed` divides the table by.
    pub const fn heading_class(self) -> &'static str {
        match self {
            Self::Default => "py-1 text-xs font-normal opacity-60",
            Self::None => "",
        }
    }

    /// The utilities for a cell, which holds one day.
    pub const fn cell_class(self) -> &'static str {
        match self {
            Self::Default => "p-0",
            Self::None => "",
        }
    }

    /// The daisyUI classes for a day, and a variant per state the primitive
    /// reports it in: the same set the calendar component emits, duplicated
    /// rather than depended on.
    pub const fn day_class(self) -> &'static str {
        match self {
            Self::Default => concat!(
                "btn btn-square btn-sm btn-ghost",
                " data-[selected=true]:bg-primary data-[selected=true]:text-primary-content",
                " data-[today=true]:ring-2 data-[today=true]:ring-primary data-[today=true]:ring-inset",
                " data-[month=last]:opacity-40 data-[month=next]:opacity-40",
                " data-[unavailable=true]:opacity-30 data-[unavailable=true]:line-through",
                " data-[selection-between=true]:bg-primary/20 data-[selection-between=true]:text-base-content",
                " data-[selection-between=true]:rounded-none",
                " data-[selection-start=true]:rounded-e-none data-[selection-end=true]:rounded-s-none",
            ),
            Self::None => "",
        }
    }
}

/// The date the picker allows from, repeating the primitive's own default: a
/// prop declared here has to carry one, and the primitive gets its from an
/// extension trait that crate keeps to itself.
fn default_min_date() -> Date {
    Date::from_calendar_date(1925, Month::January, 1).expect("a valid date")
}

/// The date the picker allows until, repeating the primitive's own default for
/// the reason [`default_min_date`] does.
fn default_max_date() -> Date {
    Date::from_calendar_date(2050, Month::December, 31).expect("a valid date")
}

/// The date the calendar opens on and draws as today, repeating the primitive's
/// own default for the reason [`default_min_date`] does: the local date,
/// falling back to UTC where the platform will not give one up.
fn default_today() -> Date {
    OffsetDateTime::now_local()
        .map(|now| now.date())
        .unwrap_or_else(|_| UtcDateTime::now().date())
}

/// Where the side and the alignment are kept for the parts that need both.
///
/// The utilities that place the calendar are written on the content, and which
/// ones they are depends on the pair, so the pair is provided by the root
/// rather than passed down by the caller twice.
#[derive(Copy, Clone)]
struct Placement {
    side: DatePickerSide,
    align: DatePickerAlign,
}

/// A date typed into a field or chosen from a calendar, styled with daisyUI's
/// `input` and `btn` classes.
///
/// The segmented field, the calendar, the popover that holds it, the arrow keys
/// that walk both, and the range of dates the picker will accept are all the
/// primitive's rather than reimplemented here.
///
/// **Nothing is lifted.** The open state lives in a context the primitive keeps
/// to itself, so this component could not emit a class from it even if daisyUI
/// had one to emit, which is also why the popup is not a daisyUI dropdown. What
/// each part is painted from is an attribute the primitive already sets on it.
///
/// This element carries nothing itself: it is where the value lives, and what
/// the eye sees is the field and the popup below it.
#[component]
pub fn DatePicker(
    /// The chosen date, or `None` for a picker nothing has been chosen in.
    #[props(default)]
    selected_date: ReadSignal<Option<Date>>,
    /// Called when the chosen date changes.
    #[props(default)]
    on_value_change: Callback<Option<Date>>,
    /// Whether the picker is disabled, which leaves the segments and the
    /// calendar inert.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// Whether the field refuses typing, leaving the calendar the only way in.
    #[props(default)]
    read_only: ReadSignal<bool>,
    /// The earliest date the picker will accept.
    #[props(default = default_min_date())]
    min_date: Date,
    /// The latest date the picker will accept.
    #[props(default = default_max_date())]
    max_date: Date,
    /// The dates it will not accept between those two.
    #[props(default)]
    disabled_ranges: ReadSignal<Vec<DateRange>>,
    /// Which side of the field the calendar opens on.
    #[props(default)]
    side: DatePickerSide,
    /// Where it sits along that side.
    #[props(default)]
    align: DatePickerAlign,
    /// Whether moving past the last segment wraps around to the first. The
    /// default repeats the primitive's own, since a prop declared here has to
    /// carry one.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    roving_loop: ReadSignal<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    use_context_provider(|| Placement { side, align });

    rsx! {
        date_picker::DatePicker {
            selected_date,
            on_value_change,
            disabled,
            read_only,
            min_date,
            max_date,
            disabled_ranges,
            roving_loop,
            attributes,
            {children}
        }
    }
}

/// The element the calendar is positioned against, which is also the popover
/// the primitive opens and closes.
///
/// It emits the two utilities daisyUI's `dropdown` would have carried, and
/// nothing else, because the reveal rules that come with `dropdown` cannot be
/// used here (see [`DatePickerSide`]).
///
/// The open state is the primitive's alone: it lives in the picker's own
/// context, and this part neither reads it nor writes it.
#[component]
pub fn DatePickerPopover(
    /// Whether to emit the utilities that make this the positioning box.
    #[props(default)]
    appearance: DatePickerPopoverAppearance,
    /// Whether focus is trapped inside the calendar while it is open. The
    /// default repeats the primitive's own.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    is_modal: ReadSignal<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(div {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        date_picker::DatePickerPopover {
            // The picker's own open state, which this component does not have
            // and does not need: the primitive reads it from its context and
            // ignores what is passed here.
            open: None,
            is_modal,
            attributes: merged,
            {children}
        }
    }
}

/// The date the field holds, and the only thing the segments can be written
/// inside.
///
/// It renders no element at all: it is where the primitive keeps the year, the
/// month and the day of the value being typed, and a segment reads its own from
/// there. A segment written outside it has nothing to read, which is why this
/// part exists in an API that otherwise mirrors what is on screen.
///
/// Written with nothing inside it, it renders the three segments in
/// year-month-day order with a separator between each pair: the primitive's own
/// order, put together again here so that the segments are this registry's
/// rather than unstyled ones.
#[component]
pub fn DatePickerInputValue(
    /// Whether the segments this renders are painted.
    #[props(default)]
    appearance: DatePickerSegmentAppearance,
    /// What a segment with no year in it shows. The default repeats the
    /// primitive's own, since a prop declared here has to carry one.
    #[props(default = Callback::new(|_| "Y".to_string()))]
    on_format_year_placeholder: Callback<(), String>,
    /// What a segment with no month in it shows.
    #[props(default = Callback::new(|_| "M".to_string()))]
    on_format_month_placeholder: Callback<(), String>,
    /// What a segment with no day in it shows.
    #[props(default = Callback::new(|_| "D".to_string()))]
    on_format_day_placeholder: Callback<(), String>,
    #[props(default)] children: Option<Element>,
) -> Element {
    let children = children.unwrap_or_else(|| {
        rsx! {
            DatePickerYearSegment { appearance }
            DatePickerSeparator { appearance }
            DatePickerMonthSegment { appearance }
            DatePickerSeparator { appearance }
            DatePickerDaySegment { appearance }
        }
    });

    rsx! {
        date_picker::DatePickerInputValue {
            on_format_year_placeholder,
            on_format_month_placeholder,
            on_format_day_placeholder,
            children,
        }
    }
}

/// The field, carrying daisyUI's `input` class.
///
/// `.input` is a flex row that lays out whatever is inside it, which is exactly
/// what this element holds: three segments, the separators between them, and
/// the button that opens the calendar. So the class goes on as daisyUI wrote it
/// and the caveat the select's field carries does not apply here, because nothing
/// about `.input` asks the element to be a form control.
///
/// Written with nothing inside it, it holds the three segments and nothing
/// else. A field that holds anything more (the button that opens the calendar,
/// a label, a different order of segments) is one the caller writes out, and
/// then the segments go inside a [`DatePickerInputValue`].
#[component]
pub fn DatePickerInput(
    /// daisyUI's colour axis.
    #[props(default)]
    color: DatePickerColor,
    /// daisyUI's size axis.
    #[props(default)]
    size: DatePickerSize,
    /// Whether the segments this renders are painted, for the case where
    /// nothing is written inside it.
    #[props(default)]
    segment_appearance: DatePickerSegmentAppearance,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default)] children: Option<Element>,
) -> Element {
    let color = color.class();
    let size = size.class();

    let base = attributes!(div {
        class: "input {color} {size}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    let children = children.unwrap_or_else(|| {
        rsx! {
            DatePickerInputValue { appearance: segment_appearance }
        }
    });

    rsx! {
        date_picker::DatePickerInput { attributes: merged, children }
    }
}

/// The year of the date, as a segment that is typed into or stepped with the
/// arrow keys.
#[component]
pub fn DatePickerYearSegment(
    /// Whether the segment is painted.
    #[props(default)]
    appearance: DatePickerSegmentAppearance,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(span {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        date_picker::DatePickerYearSegment { attributes: merged }
    }
}

/// The month of the date, mirroring [`DatePickerYearSegment`].
#[component]
pub fn DatePickerMonthSegment(
    /// Whether the segment is painted.
    #[props(default)]
    appearance: DatePickerSegmentAppearance,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(span {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        date_picker::DatePickerMonthSegment { attributes: merged }
    }
}

/// The day of the date, mirroring [`DatePickerYearSegment`].
#[component]
pub fn DatePickerDaySegment(
    /// Whether the segment is painted.
    #[props(default)]
    appearance: DatePickerSegmentAppearance,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(span {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        date_picker::DatePickerDaySegment { attributes: merged }
    }
}

/// What stands between two segments, out of the accessibility tree and out of
/// the tab order, both of which are the primitive's doing.
#[component]
pub fn DatePickerSeparator(
    /// The character between the segments.
    #[props(default = '-')]
    symbol: char,
    /// Whether the separator is painted.
    #[props(default)]
    appearance: DatePickerSegmentAppearance,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let appearance = appearance.separator_class();

    let base = attributes!(span {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        date_picker::DatePickerSeparator { symbol, attributes: merged }
    }
}

/// The button that opens the calendar, carrying daisyUI's `btn` classes.
///
/// It renders whatever the caller puts in it, because a calendar icon is
/// content rather than paint.
#[component]
pub fn DatePickerTrigger(
    /// The look of the button.
    #[props(default)]
    appearance: DatePickerTriggerAppearance,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(button {
        class: "btn {appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        popover::PopoverTrigger { attributes: merged, {children} }
    }
}

/// The panel the calendar opens in.
///
/// It is positioned with utilities rather than with daisyUI's `dropdown-content`
/// (see [`DatePickerSide`]), and the box it draws is the same one the popover
/// component draws: a fill, daisyUI's own box radius, a border and a shadow.
#[component]
pub fn DatePickerContent(
    /// Whether to emit the utilities that take the panel out of the flow.
    #[props(default)]
    positioning: DatePickerPositioning,
    /// Whether to emit the utilities that draw the box.
    #[props(default)]
    appearance: DatePickerContentAppearance,
    /// The id of this element. Declared rather than left to the attribute list,
    /// because the primitive generates one and then looks the element up by it
    /// to trap focus inside it.
    #[props(default)]
    id: ReadSignal<Option<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let position = use_context::<Placement>();

    let placed = positioning.class();
    let side = match positioning {
        DatePickerPositioning::Default => position.side.class(),
        DatePickerPositioning::None => "",
    };
    let align = match positioning {
        DatePickerPositioning::Default => position.align.class(position.side),
        DatePickerPositioning::None => "",
    };
    let appearance = appearance.class();

    let base = attributes!(div {
        class: "{placed} {side} {align} {appearance}",
    });
    let mut merged = merge_attributes(vec![base, attributes]);

    // The merged class travels through the primitive's own `class` prop, which
    // is the one part that takes one; left in the attribute list it would
    // arrive at an element that already has a class attribute on it.
    let class = take_class(&mut merged);

    rsx! {
        popover::PopoverContent {
            id,
            class,
            side: position.side.side(),
            align: position.align.align(),
            attributes: merged,
            {children}
        }
    }
}

/// The month inside the popup: its header, its grid and its days.
///
/// **It is a whole month rather than parts a caller composes.** The calendar
/// component publishes the parts; here they would be a second set of the same
/// ones, and a picker's popup is fixed furniture: what varies is the date, not
/// the layout. What a caller can still do is switch the paint off and write
/// their own inside, which is what [`DatePickerCalendarAppearance::None`] is
/// for.
///
/// The class strings are the calendar component's, duplicated rather than
/// depended on: the registry uses no cross-component dependencies, and the
/// Tailwind contract already requires every one of them to be a literal in the
/// file that emits it.
#[component]
pub fn DatePickerCalendar(
    /// Whether the month is painted.
    #[props(default)]
    appearance: DatePickerCalendarAppearance,
    /// The date drawn as today. Defaulted rather than read from the clock on
    /// every render, so that a caller can pin it, which is what a screenshot
    /// of a calendar needs.
    #[props(default = default_today())]
    today: Date,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        date_picker::DatePickerCalendar { today, attributes,
            calendar::CalendarView { class: "{appearance.view_class()}",
                calendar::CalendarHeader {
                    calendar::CalendarNavigation { class: "{appearance.navigation_class()}",
                        calendar::CalendarPreviousMonthButton { class: "{appearance.button_class()}",
                            "‹"
                        }
                        calendar::CalendarMonthTitle { class: "{appearance.title_class()}" }
                        calendar::CalendarNextMonthButton { class: "{appearance.button_class()}",
                            "›"
                        }
                    }
                }
                MonthGrid { appearance }
            }
        }
    }
}

/// The table of days, which has to be its own component: the weeks it is built
/// from are a hook, and a hook belongs to something rendered inside the view
/// that provides the month it reads.
///
/// The grid is written out rather than taken from the primitive's own collapsed
/// one for the reason ADR-0022 records: that one takes attributes for the table
/// and nothing inside it, so no class could reach a heading, a cell or a day.
#[component]
fn MonthGrid(appearance: DatePickerCalendarAppearance) -> Element {
    let grid = calendar::use_calendar_grid();

    let heading = appearance.heading_class();
    let cell = appearance.cell_class();
    let day = appearance.day_class();

    rsx! {
        calendar::CalendarGridRoot { class: "{appearance.grid_class()}",
            calendar::CalendarGridHead {
                calendar::CalendarGridHeaderRow {
                    for weekday in grid.weekdays().iter().cloned() {
                        calendar::CalendarGridDayHeader {
                            key: "{weekday.weekday():?}",
                            weekday: weekday.weekday(),
                            class: "{heading}",
                            {weekday.label().to_string()}
                        }
                    }
                }
            }
            calendar::CalendarGridBody {
                for week in grid.weeks() {
                    calendar::CalendarGridWeek { key: "{week[0]}",
                        for date in week.iter().copied() {
                            calendar::CalendarGridCell { key: "{date}", class: "{cell}",
                                calendar::CalendarDay { date, class: "{day}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Takes the class out of a merged attribute list, so that it can be passed to
/// a primitive that takes one as a prop of its own.
///
/// `merge_attributes` has already concatenated the caller's class with this
/// component's by the time this runs, so there is exactly one to take, as long
/// as it is text, which is the only kind of class `rsx!` produces and the only
/// kind that could have been concatenated in the first place. Anything else is
/// left where it is, to travel on as an attribute.
fn take_class(attributes: &mut Vec<Attribute>) -> String {
    let class = attributes.iter().position(|attribute| {
        attribute.name == "class"
            && matches!(attribute.value, dioxus::core::AttributeValue::Text(_))
    });

    match class.map(|index| attributes.remove(index).value) {
        Some(dioxus::core::AttributeValue::Text(class)) => class,
        _ => String::new(),
    }
}
