# Date picker

A date typed into a segmented field or chosen from a calendar, styled with daisyUI's `input` and
`btn` classes and wrapping the `dioxus-primitives` date picker. The segments, the digit-at-a-time
typing, the arrow keys that step a value and move between segments, the popover, the calendar and
the range of dates the picker accepts are all the primitive's rather than reimplemented here.

```rust
rsx! {
    DatePicker {
        selected_date: chosen(),
        on_value_change: move |date| chosen.set(date),

        DatePickerPopover {
            DatePickerInput {
                DatePickerInputValue {}

                DatePickerTrigger { aria_label: "Open the calendar", "📅" }
            }

            DatePickerContent {
                DatePickerCalendar {}
            }
        }
    }
}
```

**The field is daisyUI's wrapping `input`, which is the one place daisyUI wrote for this shape.**
`.input` is a flex row with a border, a radius and a focus outline, laying out whatever is inside
it, and what is inside it here is three segments, the separators between them and the button that
opens the calendar. Nothing about the class asks the element to be a form control, so it carries
over intact.

## Composition

The compound parts are `DatePicker` (the value), `DatePickerPopover` (the box the calendar is
positioned against), `DatePickerInput` (the field), `DatePickerInputValue` holding
`DatePickerYearSegment`, `DatePickerMonthSegment` and `DatePickerDaySegment` with
`DatePickerSeparator` between them, `DatePickerTrigger` (the button) and `DatePickerContent` (the
panel), holding `DatePickerCalendar` (the month).

**`DatePickerInputValue` renders no element at all**, and is where the primitive keeps the year,
the month and the day being typed: a segment reads its own from there, and one written outside it
has nothing to read. Left empty it renders the three segments in year-month-day order with a
separator between each pair, which is the common case; the field renders one of these itself when
nothing is written inside it.

**The popup is not a daisyUI dropdown**, and the reason is one this registry has met before from
the other side (ADR-0015). daisyUI hides `.dropdown-content` unless `dropdown-open` is on the
element above it, and that class would have to be emitted from Rust, but a date picker's open
state lives in a context the primitive keeps to itself, with no controlled prop and no callback
this component can reach. There is nothing here to emit it from. So the panel is positioned with
utilities keyed on a side and an alignment axis, exactly as the hover card's is, and the box it
draws is the popover component's.

**The month is a whole calendar rather than parts.** The calendar component publishes the parts;
inside a picker's popup they would be a second set of the same ones, and what varies there is the
date rather than the layout. `DatePickerCalendar` therefore renders the header, the month buttons,
the title, the grid and the days itself, with one axis switching all of it off for a caller who
wants to draw their own.

The class strings for that month are the calendar component's, duplicated rather than depended on:
cross-Component dependencies are for public composition, while the Tailwind contract requires every
one of these classes to be a literal in the file that emits it.

**The grid inside it is written out** rather than taken from the primitive's collapsed
`CalendarGrid`, for the reason ADR-0022 records: that one takes attributes for the table and
nothing inside it, so no class could reach a weekday heading, a cell or a day.

## State bridging

**Neither tier, and nothing is lifted.** daisyUI has no date picker and no calendar this registry
can use, so there is no rule for **Tier 1** to match and no modifier class for **Tier 2** to emit;
and the one class that would have been emitted from state (`dropdown-open`) belongs to a
dropdown this popup cannot be.

Every state is a **Bridged utility** instead: a Tailwind variant of an attribute the primitive
already sets, with nothing recomputed in Rust.

| State | Attribute | What is emitted |
| --- | --- | --- |
| a segment nothing has been typed into | `no-date` | `opacity-50` |
| a segment of a disabled picker | `data-disabled` | `opacity-50` |
| the chosen day, and today, and the days of the neighbouring months, and the days that cannot be chosen | `data-selected`, `data-today`, `data-month`, `data-unavailable` | the calendar component's own set |

**Two states need nothing at all.** The month buttons go inert at the ends of the range the picker
accepts, and the primitive puts the native `disabled` attribute on them, which daisyUI's
`.btn:disabled` matches. And the focused segment is painted by a plain `:focus` variant, because
focus is where the browser puts it rather than something the primitive reports.

## Axes

- `color: DatePickerColor` and `size: DatePickerSize` on the field: `input-primary` and the rest,
  `input-xs` to `input-xl`.
- `appearance: DatePickerSegmentAppearance` on a segment or a separator: the padding, the
  tabular figures, the focused fill and the faded empty state, or nothing.
- `appearance: DatePickerTriggerAppearance` on the button: daisyUI's `btn-ghost btn-square
  btn-sm`, or `btn` alone.
- `side: DatePickerSide` and `align: DatePickerAlign` on the root: which side the calendar opens
  on and where it sits along it. Both are also what the primitive is told, so the `data-side` and
  `data-align` it reports say what the utilities did.
- `appearance: DatePickerPopoverAppearance`: the two utilities that make the popover the box the
  calendar is positioned against.
- `positioning: DatePickerPositioning` and `appearance: DatePickerContentAppearance` on the panel:
  whether it is taken out of the flow and placed at all, and whether the box is drawn.
- `appearance: DatePickerCalendarAppearance` on the month: every class the month is drawn with,
  or none of them.

The colour and size class strings are the input's and the button's, duplicated for the reason
above.

Six of these axes are the inverted shape ADR-0004 describes (their `Default` emits and their
`None` emits nothing), because what they carry is utilities this component emits rather than
daisyUI component classes. `DatePickerTriggerAppearance` is inverted for the reason the calendar's
button axis is: what it carries *is* daisyUI classes, and two of those on one element are settled
by the order daisyUI wrote them in.

Every enum exposes `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

## Deviations

**The `time` crate is a dependency of this component**, for the reason the calendar's docs record:
a date picker's API is `time::Date`, and the primitive re-exports none of it.

**Three defaults are repeated rather than read**: the earliest and latest dates the picker
accepts, and the date the calendar draws as today. The primitive's own come from an extension
trait private to that crate, so this component computes the same values: the first of 1925, the
last of 2050, and the local date falling back to UTC.

**A segment is a `span`, not an `input`.** The primitive makes it editable and gives it the
`spinbutton` role, which is what a screen reader announces and what the arrow keys step. daisyUI's
rules for an `input` inside a field therefore do not reach it: there is no `input` in there at all,
so what a segment looks like is this component's utilities rather than daisyUI's.

**A disabled picker is faded but still editable.** The primitive marks every segment with
`data-disabled` and takes them out of its own focus collection, which is what this component fades
them from, but the elements stay `contenteditable` and their key handler still writes, so a
disabled picker can be typed into. Nothing here can close that without reimplementing the
behaviour this component keeps a primitive for; the browser specs assert it as it is, so the day
upstream fixes it is the day this paragraph is wrong rather than a day nobody notices.

**A picker with no popup is a picker with a field.** `DatePickerContent` is optional: a picker that
only wants typing leaves it out, and the trigger with it.

**The calendar closes when a segment is focused.** That is the primitive's, and it is what keeps
the two ways of setting a date from fighting.

## daisyUI classes deliberately not used

- `dropdown`, `dropdown-content`, `dropdown-open` and the positioning family: daisyUI's only
  floating box. The open state of a date picker is not reachable from here, so the modifier class
  that reveals the content could never be emitted; the panel is placed with utilities instead.
- `cally`, `react-day-picker`, `pika-single`: daisyUI's calendar section, which styles three
  JavaScript libraries' markup and reaches nothing rendered from Rust (ADR-0022).
- `input-ghost`: daisyUI's borderless field. Not exposed for the reason the button's other looks
  are not: a caller reaches it through `class`, which concatenates.
- `validator`, `validator-hint`: daisyUI's validation paint, which reads `:user-invalid` off a
  form control the browser has validated. There is no form control here (the segments are `span`s)
  and what a picker accepts is already the `min_date`, `max_date` and `disabled_ranges` the
  primitive enforces.
- `join`, `join-item`: daisyUI's way of welding a field to a button beside it. The button here is
  *inside* the field rather than beside it, which is the shape `.input` was written for.
- The responsive prefixes daisyUI generates for these classes (`sm:input-lg` and the rest): a
  caller reaches those through `class`, which concatenates.
