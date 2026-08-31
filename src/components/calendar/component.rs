use dioxus::prelude::*;
use dioxus_primitives::calendar;
pub use dioxus_primitives::calendar::DateRange;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use time::{Date, Month, OffsetDateTime, UtcDateTime, Weekday};

/// Whether [`Calendar`] and [`RangeCalendar`] emit the utilities that draw the
/// box the month sits in.
///
/// daisyUI has no calendar of its own to take a class from: what it ships
/// under that name styles three JavaScript libraries' markup and reaches none
/// of this (ADR-0022), so the box is utilities, and the convention is the
/// inverted one ADR-0004 describes: [`CalendarAppearance::Default`] emits and
/// [`CalendarAppearance::None`] emits nothing. A utility this component emits
/// only ties with a caller's, and a tie is settled by generated-stylesheet
/// order rather than by the class attribute, so switching ours off is the way
/// to win it.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum CalendarAppearance {
    #[default]
    Default,
    None,
}

impl CalendarAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them. The radius is daisyUI's own box radius,
    /// which is what keeps the calendar the same shape as the card or the
    /// dropdown it is shown in.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => {
                "inline-flex flex-col gap-3 rounded-box border border-base-300 bg-base-100 p-4"
            }
            Self::None => "",
        }
    }
}

/// Whether [`CalendarView`] emits the utilities that stack a month's header
/// over its grid.
///
/// The inverted shape again, for the reason [`CalendarAppearance`] records.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum CalendarViewAppearance {
    #[default]
    Default,
    None,
}

impl CalendarViewAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "flex flex-col gap-2",
            Self::None => "",
        }
    }
}

/// Whether [`CalendarNavigation`] emits the utilities that put the month's
/// title between its two buttons.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum CalendarNavigationAppearance {
    #[default]
    Default,
    None,
}

impl CalendarNavigationAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "flex items-center justify-between gap-2",
            Self::None => "",
        }
    }
}

/// daisyUI's colour axis for the two month buttons, which is the button's own.
///
/// The class strings are the button component's, duplicated rather than
/// depended on: the registry uses no cross-component dependencies, and the
/// Tailwind contract already requires every one of these to be a literal in the
/// file that emits it.
///
/// [`CalendarButtonColor::Default`] emits no class at all, which is daisyUI's
/// uncoloured button rather than a synonym for [`CalendarButtonColor::Neutral`].
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum CalendarButtonColor {
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

impl CalendarButtonColor {
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
            Self::Neutral => "btn-neutral",
            Self::Primary => "btn-primary",
            Self::Secondary => "btn-secondary",
            Self::Accent => "btn-accent",
            Self::Info => "btn-info",
            Self::Success => "btn-success",
            Self::Warning => "btn-warning",
            Self::Error => "btn-error",
        }
    }
}

/// daisyUI's size axis for the two month buttons, duplicated for the reason
/// [`CalendarButtonColor`] records.
///
/// [`CalendarButtonSize::Default`] emits no class, which renders at the same
/// size as daisyUI's explicit `btn-md`.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum CalendarButtonSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

impl CalendarButtonSize {
    /// Every value of this axis, from the smallest to the largest, which is the
    /// order the preview renders them in.
    pub const ALL: &'static [Self] = &[Self::Xs, Self::Sm, Self::Default, Self::Lg, Self::Xl];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Xs => "btn-xs",
            Self::Sm => "btn-sm",
            Self::Default => "",
            Self::Lg => "btn-lg",
            Self::Xl => "btn-xl",
        }
    }
}

/// The look of the two month buttons: daisyUI's flat square button, or nothing
/// but `btn` and whatever the caller adds.
///
/// Unlike the axes above, both values here emit daisyUI classes rather than
/// utilities, so a caller who wants a different look has to switch this off
/// rather than out-rank it. That is the same escape hatch, for the same reason:
/// two daisyUI component classes on one element are settled by the order daisyUI
/// wrote them in, which is not something a caller can see.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum CalendarButtonAppearance {
    /// daisyUI's `btn-ghost btn-square`: no fill until it is pointed at, and as
    /// wide as it is tall, which is what an arrow beside a month wants.
    #[default]
    Default,
    /// `btn` and nothing else.
    None,
}

impl CalendarButtonAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The daisyUI class names for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "btn-ghost btn-square",
            Self::None => "",
        }
    }
}

/// Whether [`CalendarMonthTitle`] emits the utilities that set the title apart
/// from the days under it.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum CalendarTitleAppearance {
    #[default]
    Default,
    None,
}

impl CalendarTitleAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "text-sm font-medium",
            Self::None => "",
        }
    }
}

/// Whether [`CalendarGrid`] emits the utilities that lay the month out.
///
/// One axis for three elements (the table, the weekday headings and the cells)
/// because the grid renders all three itself: the primitive's own collapsed
/// grid takes attributes for the table and nothing else, so a class can only
/// reach a heading or a cell from a grid that writes them out (see the
/// component's documentation). Switching the axis off switches off all three.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum CalendarGridAppearance {
    #[default]
    Default,
    None,
}

impl CalendarGridAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for the table itself.
    ///
    /// **The table is not given a width**, and that is the load-bearing part:
    /// the box around it shrink-wraps a month, so its own width is what the
    /// table asks for. A `w-full` here would be a percentage of a width that is
    /// still being worked out from this table, and a browser resolves that
    /// cycle by handing the table whatever space is going, which stretches a
    /// calendar across its column, and blows the date pickers' `w-max` panel up
    /// to the widest box the engine will draw. So the month is as wide as its
    /// days, and a caller who wants it wider sizes the box.
    ///
    /// `table-fixed` stays, because it is what makes the columns one width once
    /// there is a width to divide: it is the [`CalendarDaySize`] a day is drawn
    /// at that decides, since nothing else in the column is as wide.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "table-fixed",
            Self::None => "",
        }
    }

    /// The Tailwind utilities for a weekday heading, which is a `th` above a
    /// column of days.
    ///
    /// The padding is vertical only. A heading sits in the same column as the
    /// days under it and the widest thing in that column sets its width, so
    /// padding on the sides of `Wed` would make one column wider than the six
    /// beside it; the labels are already narrower than a day without it.
    pub const fn heading_class(self) -> &'static str {
        match self {
            Self::Default => "py-1 text-xs font-normal opacity-60",
            Self::None => "",
        }
    }

    /// The Tailwind utilities for a cell, which holds one day. The padding is
    /// nothing so that a selected run of days reads as one band rather than as
    /// a row of separate marks.
    pub const fn cell_class(self) -> &'static str {
        match self {
            Self::Default => "p-0",
            Self::None => "",
        }
    }
}

/// daisyUI's size axis for a day, which is the button's own.
///
/// [`CalendarDaySize::Default`] emits no class, which renders at the same size
/// as daisyUI's explicit `btn-md`.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum CalendarDaySize {
    Xs,
    #[default]
    Sm,
    Default,
    Lg,
    Xl,
}

impl CalendarDaySize {
    /// Every value of this axis, from the smallest to the largest, which is the
    /// order the preview renders them in.
    pub const ALL: &'static [Self] = &[Self::Xs, Self::Sm, Self::Default, Self::Lg, Self::Xl];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Xs => "btn-xs",
            Self::Sm => "btn-sm",
            Self::Default => "",
            Self::Lg => "btn-lg",
            Self::Xl => "btn-xl",
        }
    }
}

/// Whether [`CalendarDay`] emits the classes that say what a day *is*.
///
/// Every one of them is a **Bridged utility**: a Tailwind variant of an
/// attribute the primitive already sets on the day, so nothing is recomputed in
/// Rust and the primitive stays the only owner of which day is which. daisyUI
/// has a class for none of these states: it has no calendar at all (ADR-0022),
/// so what the variants carry is utilities over daisyUI's own theme colours.
///
/// The states, and the attribute each is read off:
///
/// - the chosen day, and every day of a chosen range: `data-selected`
/// - today: `data-today`
/// - the days either side of the month being viewed: `data-month`
/// - a day inside a range, and the two ends of it: `data-selection-between`,
///   `data-selection-start`, `data-selection-end`
/// - a day that cannot be chosen: `data-unavailable`
///
/// Switched off, a day is `btn` and nothing else: the same shape, with no state
/// visible on it at all.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum CalendarDayAppearance {
    #[default]
    Default,
    None,
}

impl CalendarDayAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The classes for this value, as complete string literals so Tailwind's
    /// scanner can see them: one daisyUI class for the flat look of a day
    /// nothing has happened to, then a variant per state.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => concat!(
                "btn-ghost",
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

/// The date the calendar opens on when nobody says otherwise.
///
/// This repeats the primitive's own default, since a prop declared here has to
/// carry one, and it repeats it rather than reading it, because the extension
/// the primitive gets it from is private to that crate. The local date, falling
/// back to UTC where the platform will not give one up, which is exactly what
/// the primitive does.
fn default_today() -> Date {
    OffsetDateTime::now_local()
        .map(|now| now.date())
        .unwrap_or_else(|_| UtcDateTime::now().date())
}

/// The earliest date a calendar will navigate to, repeating the primitive's own
/// default for the reason [`default_today`] does.
fn default_min_date() -> Date {
    Date::from_calendar_date(1925, Month::January, 1).expect("a valid date")
}

/// The latest date a calendar will navigate to, repeating the primitive's own
/// default for the reason [`default_today`] does.
fn default_max_date() -> Date {
    Date::from_calendar_date(2050, Month::December, 31).expect("a valid date")
}

/// How a weekday is written above its column, repeating the primitive's own
/// default for the reason [`default_today`] does.
fn weekday_label(weekday: Weekday) -> String {
    match weekday {
        Weekday::Monday => "Mon",
        Weekday::Tuesday => "Tue",
        Weekday::Wednesday => "Wed",
        Weekday::Thursday => "Thu",
        Weekday::Friday => "Fri",
        Weekday::Saturday => "Sat",
        Weekday::Sunday => "Sun",
    }
    .to_string()
}

/// A month of days, one of which is chosen, drawn with daisyUI's `btn` classes.
///
/// Everything about how a calendar behaves is the primitive's: which days a
/// month has, the arrow keys that walk them, the month the header names, the
/// dates that are out of range, and the `application` role the whole thing is
/// announced under.
///
/// What this component adds is paint, and ADR-0022 records where it comes from:
/// daisyUI ships a calendar section that styles nothing this registry can
/// render, so the days are `btn` (the class daisyUI puts on a day cell in every
/// example where it draws one itself) and the box around them is utilities.
///
/// There is **no state to lift**. Every state a day is painted from is one the
/// primitive already reports as a `data-*` attribute on the day itself, so the
/// classes are Bridged utilities written as variants of those attributes rather
/// than modifier classes emitted from state this component keeps.
///
/// Classes passed by the caller concatenate with the calendar's own; every other
/// attribute the caller passes overrides them.
#[component]
pub fn Calendar(
    /// Whether to emit the utilities that draw the box.
    #[props(default)]
    appearance: CalendarAppearance,
    /// The chosen date, or `None` for a calendar nothing is chosen in.
    #[props(default)]
    selected_date: ReadSignal<Option<Date>>,
    /// Called when the chosen date changes. A second click on the chosen day
    /// clears it, which is the primitive's behaviour and is reported here as
    /// `None`.
    #[props(default)]
    on_date_change: Callback<Option<Date>>,
    /// The month on show, which is the date's month rather than the date.
    #[props(default = ReadSignal::new(Signal::new(default_today())))]
    view_date: ReadSignal<Date>,
    /// Called when the month on show changes.
    #[props(default)]
    on_view_change: Callback<Date>,
    /// The date drawn as today. Defaulted rather than read from the clock on
    /// every render, so that a caller can pin it, which is what a screenshot
    /// of a calendar needs.
    #[props(default = default_today())]
    today: Date,
    /// The day a week starts on.
    #[props(default = Weekday::Sunday)]
    first_day_of_week: Weekday,
    /// The earliest date the calendar will navigate to.
    #[props(default = default_min_date())]
    min_date: Date,
    /// The latest date the calendar will navigate to.
    #[props(default = default_max_date())]
    max_date: Date,
    /// The dates that cannot be chosen.
    #[props(default)]
    disabled_ranges: ReadSignal<Vec<DateRange>>,
    /// Whether the whole calendar is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// How a weekday is written above its column. The default repeats the
    /// primitive's own, since a prop declared here has to carry one.
    #[props(default = Callback::new(weekday_label))]
    on_format_weekday: Callback<Weekday, String>,
    /// How a month is written in the title. The default repeats the primitive's
    /// own.
    #[props(default = Callback::new(|month: Month| month.to_string()))]
    on_format_month: Callback<Month, String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(div {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        calendar::Calendar {
            selected_date,
            on_date_change,
            view_date,
            on_view_change,
            today,
            first_day_of_week,
            min_date,
            max_date,
            disabled_ranges,
            disabled,
            on_format_weekday,
            on_format_month,
            attributes: merged,
            {children}
        }
    }
}

/// The same month of days, with a range chosen across it rather than one day.
///
/// It is the same component in every respect but the value: the header, the
/// grid and the day below are shared, because the primitive's day reads
/// whichever calendar context is above it and renders itself accordingly. What
/// changes is what a click does (the first sets one end of the range and the
/// second the other) and the two extra attributes a day carries while it is
/// inside one, which [`CalendarDayAppearance`] paints.
///
/// Classes passed by the caller concatenate with the calendar's own; every other
/// attribute the caller passes overrides them.
#[component]
pub fn RangeCalendar(
    /// Whether to emit the utilities that draw the box.
    #[props(default)]
    appearance: CalendarAppearance,
    /// The chosen range, or `None` for a calendar nothing is chosen in.
    #[props(default)]
    selected_range: ReadSignal<Option<DateRange>>,
    /// Called when the chosen range changes.
    #[props(default)]
    on_range_change: Callback<Option<DateRange>>,
    /// The month on show.
    #[props(default = ReadSignal::new(Signal::new(default_today())))]
    view_date: ReadSignal<Date>,
    /// Called when the month on show changes.
    #[props(default)]
    on_view_change: Callback<Date>,
    /// The date drawn as today, defaulted for the reason [`Calendar`]'s is.
    #[props(default = default_today())]
    today: Date,
    /// The day a week starts on.
    #[props(default = Weekday::Sunday)]
    first_day_of_week: Weekday,
    /// The earliest date the calendar will navigate to.
    #[props(default = default_min_date())]
    min_date: Date,
    /// The latest date the calendar will navigate to.
    #[props(default = default_max_date())]
    max_date: Date,
    /// The dates that cannot be chosen.
    #[props(default)]
    disabled_ranges: ReadSignal<Vec<DateRange>>,
    /// Whether the whole calendar is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// How a weekday is written above its column. The default repeats the
    /// primitive's own, since a prop declared here has to carry one.
    #[props(default = Callback::new(weekday_label))]
    on_format_weekday: Callback<Weekday, String>,
    /// How a month is written in the title. The default repeats the primitive's
    /// own.
    #[props(default = Callback::new(|month: Month| month.to_string()))]
    on_format_month: Callback<Month, String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(div {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        calendar::RangeCalendar {
            selected_range,
            on_range_change,
            view_date,
            on_view_change,
            today,
            first_day_of_week,
            min_date,
            max_date,
            disabled_ranges,
            disabled,
            on_format_weekday,
            on_format_month,
            attributes: merged,
            {children}
        }
    }
}

/// One month of whichever calendar it is inside.
///
/// It is a part rather than something either calendar renders around its
/// children, because it is also how a calendar shows more than one month: a
/// second view is a second month, offset from the first, walked by the same
/// arrow keys and filled in from the same value. Everything below it (the
/// header, the title and the grid) reads the month from the view it is in.
#[component]
pub fn CalendarView(
    /// Whether to emit the utilities that stack the header over the grid.
    #[props(default)]
    appearance: CalendarViewAppearance,
    /// How many months past the calendar's own this view shows. Left unset, a
    /// view takes its offset from the order it is rendered in.
    #[props(default)]
    offset: Option<u8>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(div {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        calendar::CalendarView { offset, attributes: merged, {children} }
    }
}

/// The band above a month's grid, which is what a screen reader announces the
/// month as the heading of.
///
/// Nothing is emitted here. The heading is a wrapper the primitive gives a role
/// and a level to, and what it holds (the navigation, or whatever else a caller
/// puts beside it) brings its own layout.
#[component]
pub fn CalendarHeader(
    /// The id of this element.
    #[props(default)]
    id: Option<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        calendar::CalendarHeader { id, attributes, {children} }
    }
}

/// The row that moves the calendar from month to month: a button back, the
/// month's name, and a button forward.
#[component]
pub fn CalendarNavigation(
    /// Whether to emit the utilities that lay the row out.
    #[props(default)]
    appearance: CalendarNavigationAppearance,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(div {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        calendar::CalendarNavigation { attributes: merged, {children} }
    }
}

/// The button that shows the month before the one on show, carrying daisyUI's
/// `btn` classes.
///
/// The disabled state needs no bridging: the primitive puts the native
/// `disabled` attribute on the button (at the start of the range the calendar
/// is allowed to navigate, and while the calendar itself is disabled), and
/// daisyUI's rule is `.btn:disabled`.
///
/// The button renders whatever the caller puts in it, because the arrow is
/// content rather than paint: an icon, a character, or a word.
#[component]
pub fn CalendarPreviousMonthButton(
    /// daisyUI's colour axis.
    #[props(default)]
    color: CalendarButtonColor,
    /// daisyUI's size axis.
    #[props(default)]
    size: CalendarButtonSize,
    /// The look of the button.
    #[props(default)]
    appearance: CalendarButtonAppearance,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let color = color.class();
    let size = size.class();
    let appearance = appearance.class();

    let base = attributes!(button {
        class: "btn {appearance} {color} {size}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        calendar::CalendarPreviousMonthButton { attributes: merged, {children} }
    }
}

/// The button that shows the month after the one on show, mirroring
/// [`CalendarPreviousMonthButton`] in every respect.
#[component]
pub fn CalendarNextMonthButton(
    /// daisyUI's colour axis.
    #[props(default)]
    color: CalendarButtonColor,
    /// daisyUI's size axis.
    #[props(default)]
    size: CalendarButtonSize,
    /// The look of the button.
    #[props(default)]
    appearance: CalendarButtonAppearance,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let color = color.class();
    let size = size.class();
    let appearance = appearance.class();

    let base = attributes!(button {
        class: "btn {appearance} {color} {size}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        calendar::CalendarNextMonthButton { attributes: merged, {children} }
    }
}

/// The month and year on show, written by the primitive from the view it is in.
#[component]
pub fn CalendarMonthTitle(
    /// Whether to emit the utilities that set the title apart.
    #[props(default)]
    appearance: CalendarTitleAppearance,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let appearance = appearance.class();

    let base = attributes!(div {
        class: "{appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        calendar::CalendarMonthTitle { attributes: merged }
    }
}

/// The month itself: a table of weeks, with a heading per weekday and a day per
/// cell.
///
/// **The grid is written out here rather than taken whole from the primitive**,
/// and that is the one structural decision this component makes. The primitive
/// ships a collapsed `CalendarGrid` that renders the table, the headings, the
/// week rows and the days from the same data, but it takes attributes for the
/// table alone, so a class could never reach a heading or a cell. The days
/// would then be the primitive's rather than this registry's, which is to say
/// unpainted. So the grid is composed here from the primitive's own parts and
/// its own `use_calendar_grid`, which is public for exactly this: the weeks and
/// the weekday order are still the primitive's, and what is added is a class per
/// element.
///
/// The days it renders are [`CalendarDay`], which is why this part carries the
/// day's two axes as well as its own.
#[component]
pub fn CalendarGrid(
    /// Whether to emit the utilities that lay the month out.
    #[props(default)]
    appearance: CalendarGridAppearance,
    /// daisyUI's size axis for the days.
    #[props(default)]
    day_size: CalendarDaySize,
    /// Whether the days are painted from what they are.
    #[props(default)]
    day_appearance: CalendarDayAppearance,
    /// The id of the table.
    #[props(default)]
    id: Option<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let grid = calendar::use_calendar_grid();

    let heading = appearance.heading_class();
    let cell = appearance.cell_class();
    let table = appearance.class();

    let base = attributes!(table { class: "{table}" });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        calendar::CalendarGridRoot { id, attributes: merged,
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
                                CalendarDay {
                                    date,
                                    size: day_size,
                                    appearance: day_appearance,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// One day, carrying daisyUI's `btn` classes and a variant per state.
///
/// It is a part of its own as well as what [`CalendarGrid`] renders, for a
/// caller who writes their own grid out of the primitive's parts: a week
/// numbered down its side, say, or a day with something under the number.
///
/// The number is the day's own unless the caller writes something else into it.
#[component]
pub fn CalendarDay(
    /// The date this day is.
    date: Date,
    /// daisyUI's size axis.
    #[props(default)]
    size: CalendarDaySize,
    /// Whether the day is painted from what it is.
    #[props(default)]
    appearance: CalendarDayAppearance,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default)] children: Option<Element>,
) -> Element {
    let size = size.class();
    let appearance = appearance.class();

    let base = attributes!(button {
        class: "btn btn-square {size} {appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        calendar::CalendarDay { date, attributes: merged, children }
    }
}
