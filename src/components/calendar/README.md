# Calendar

A month of days, one date or one range chosen from it, drawn with daisyUI's `btn` classes and
wrapping the `dioxus-primitives` calendar. The month arithmetic, the arrow keys that walk the
grid, the month navigation, the dates that are out of range or unavailable and the `application`
role it is announced under are all the primitive's rather than reimplemented here.

```rust
rsx! {
    Calendar {
        selected_date: chosen(),
        on_date_change: move |date| chosen.set(date),

        CalendarView {
            CalendarHeader {
                CalendarNavigation {
                    CalendarPreviousMonthButton { size: CalendarButtonSize::Sm, "‹" }
                    CalendarMonthTitle {}
                    CalendarNextMonthButton { size: CalendarButtonSize::Sm, "›" }
                }
            }
            CalendarGrid {}
        }
    }
}
```

**daisyUI's own calendar CSS is unusable here, and ADR-0022 records why**: every rule in it names
another library's markup: `.cally::part(day)` reaches into a web component's shadow tree, and the
rest are `react-day-picker`'s and `pikaday`'s class names. So the days are `btn`, which is what
daisyUI puts on a day cell everywhere it draws one itself, and everything around them is Tailwind
utilities over daisyUI's own theme colours.

`Calendar` chooses one date and `RangeCalendar` chooses a run of them. They are one component
rather than two because only the root differs: the primitive's day reads whichever calendar context
is above it, so the header, the navigation, the title, the grid and the day are the same parts
under both.

## Composition

The compound parts are `Calendar` and `RangeCalendar` (the two roots), `CalendarView` (one month),
`CalendarHeader` (the band above a month), `CalendarNavigation` (the row that changes it),
`CalendarPreviousMonthButton` and `CalendarNextMonthButton`, `CalendarMonthTitle`, `CalendarGrid`
(the month itself) and `CalendarDay` (one day). There is no collapsed component: a calendar's
header is where a caller puts their own controls, so there is nothing to collapse it into.

**`CalendarView` is a part rather than something the root renders**, because it is also how a
calendar shows more than one month: a second view is a second month, offset from the first, walked
by the same arrow keys and filled in from the same value.

**The grid is composed here rather than taken whole from the primitive.** The primitive's collapsed
`CalendarGrid` renders the table, the headings, the week rows and the days from one call and takes
attributes for the table alone, so a class could never reach a heading, a cell, or a day. This
component writes the same composition out of the primitive's own parts and its public
`use_calendar_grid`, which is what makes the days *this registry's* days. The month arithmetic
stays where it was.

That is why `CalendarGrid` carries the day's axes as well as its own: it is what renders them.
`CalendarDay` is exported all the same, for a caller who writes their own grid out of the
primitive's parts.

## State bridging

**Neither tier: every state is a Bridged utility.** daisyUI has no calendar at all, so there is
no rule of its own for **Tier 1** to match an ARIA attribute against, and no modifier class for
**Tier 2** to emit. What the primitive does is report each state as a `data-*` attribute on the
day itself, so each is painted by a Tailwind variant of that attribute:

| State | Attribute | What is emitted |
| --- | --- | --- |
| the chosen day, and every day of a chosen range | `data-selected` | `bg-primary`, `text-primary-content` |
| today | `data-today` | a `ring-primary` inset ring |
| the days either side of the month on show | `data-month` | `opacity-40` |
| a day that cannot be chosen | `data-unavailable` | `opacity-30`, `line-through` |
| the middle of a range | `data-selection-between` | `bg-primary/20`, square corners |
| the two ends of a range | `data-selection-start`, `data-selection-end` | square corners on the inward side |

Nothing is recomputed in Rust and the primitive stays the only owner of every one of them, which
is what tells this apart from the Lifted state of ADR-0006 and the Mirrored state of ADR-0011.

**The month buttons need nothing at all.** The primitive puts the native `disabled` attribute on
them (at the ends of the range the calendar may navigate, and while the calendar is disabled), and
daisyUI's rule is `.btn:disabled`.

## Axes

- `appearance: CalendarAppearance` on either root: the box's fill, radius, border and padding, or
  nothing.
- `appearance: CalendarViewAppearance` on a view: the utilities that stack a header over a grid.
- `appearance: CalendarNavigationAppearance`: the utilities that put the title between the
  buttons.
- `color: CalendarButtonColor`, `size: CalendarButtonSize`, `appearance:
  CalendarButtonAppearance` on the two month buttons: `btn-primary` and the rest, `btn-xs` to
  `btn-xl`, and `btn-ghost btn-square` or nothing.
- `appearance: CalendarTitleAppearance`: the utilities that set the title apart.
- `appearance: CalendarGridAppearance` on the grid: the utilities for the table, the weekday
  headings and the cells, all three at once, because the grid renders all three.
- `size: CalendarDaySize` and `appearance: CalendarDayAppearance` on a day: daisyUI's button
  scale, and every state variant in the table above.

The colour and size class strings are the button component's, duplicated rather than depended on:
cross-Component dependencies are for public composition, while the Tailwind contract requires every
one of these classes to be a literal in the file that emits it.

Six of these axes are the inverted shape ADR-0004 describes (their `Default` emits and their
`None` emits nothing), because what they carry is utilities this component emits rather than
daisyUI component classes. A utility only ties with a caller's, and a tie is settled by
generated-stylesheet order, so switching ours off is the way to win it.
`CalendarButtonAppearance` is inverted for a different reason: what it carries *is* daisyUI
classes, and two daisyUI component classes on one element are settled by the order daisyUI wrote
them in, which is not something a caller can see either.

Every enum exposes `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

## Deviations

**The `time` crate is a dependency of this component.** A calendar's API is dates, and the
primitive's are `time::Date`, `time::Weekday` and `time::Month`, none of which that crate
re-exports. So the manifest declares `time` beside `dioxus-primitives`, pinned to the same version
the primitive itself depends on, which is what keeps the two sides of the API the same type.

**Four defaults are repeated rather than read.** `today`, `view_date`, `min_date` and `max_date`
have to carry a default here, and the primitive's own come from an extension trait that is private
to that crate, so this component computes the same values: the local date falling back to UTC, and
the first of 1925 to the last of 2050. A caller who pins the date, as every example in the preview
does, never meets them.

**A second click on the chosen day clears it.** That is the primitive's, and it is reported as
`None` rather than as the same date again.

**A disabled calendar's days still take clicks; they just choose nothing.** The primitive puts the
native attribute on the month buttons but not on the days, so a disabled day is muted by an
attribute rather than made inert. What it is not is chooseable.

**A range is chosen by two clicks, and follows the pointer between them.** The primitive tracks the
second end on hover, so the paint moves as the pointer does, which is why the middle of a range is
an axis value rather than something a caller draws.

## daisyUI classes deliberately not used

- `cally`, `react-day-picker`, `pika-single` and everything under them: daisyUI's calendar
  section in its entirety, which styles three JavaScript libraries' markup and reaches nothing
  rendered from Rust (ADR-0022).
- `card`, `card-body`: the box daisyUI would draw around a panel. It brings a body element's
  padding rules with it and a calendar has no body; the box here is four utilities, all of them
  switchable off.
- `join`, `join-item`: daisyUI's way of welding a row of controls together, which is what the two
  month buttons look like in some designs. It squares the inward corners of its children, which is
  wrong for two buttons with a title between them; a caller who wants it writes it around them.
- `btn-disabled`: daisyUI's class for a button that only looks disabled. The primitive sets the
  native attribute on the month buttons, which daisyUI styles and which also makes them inert.
- `table`, `table-zebra`, `table-pin-rows`: daisyUI's table classes. The grid is a `table`
  element, but daisyUI's table is written for rows of data: it pads cells, rules between rows and
  fills the header, none of which belongs on a month.
- The responsive prefixes daisyUI generates for the button classes (`sm:btn-lg` and the rest): a
  caller reaches those through `class`, which concatenates.
