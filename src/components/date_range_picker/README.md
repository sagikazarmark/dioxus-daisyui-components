# Date range picker

A run of days chosen from a calendar, styled with daisyUI's `input` and `btn` classes and wrapping
the `dioxus-primitives` date range picker. The two-click range selection, the popover, the range
calendar and the dates the picker accepts are all the primitive's rather than reimplemented here.

[Live examples](https://daisyui-components.dioxus.cc/components/date_range_picker) ·
[their sources](docs/examples/)

**The primitive's segmented range input is not published**, and that is the one thing to know
before reaching for this: it never settles once a range exists. See Deviations.

It stands beside the date picker rather than being an axis of it, because what differs is the parts
rather than the paint: a root of its own, a value that is a range rather than a date, and a calendar
that selects one. The alert dialog stands beside the dialog for the same reason.

What the field *shows* is the caller's, because there is nothing here to show it with: the range is
written however that app writes a date, beside the button that opens the calendar.

**The field is daisyUI's wrapping `input`, which is the one place daisyUI wrote for this shape.**
`.input` is a flex row with a border, a radius and a focus outline, laying out whatever is inside
it: here the range as the caller writes it and the button that opens the calendar. Nothing about
the class asks the element to be a form control, so it carries over intact.

## Composition

The compound parts are `DateRangePicker` (the value), `DateRangePickerPopover` (the box the
calendar is positioned against), `DateRangePickerInput` (the field), `DateRangePickerTrigger` (the
button) and `DateRangePickerContent` (the panel), holding `DateRangePickerCalendar` (the month).

**The field renders nothing of its own**, unlike the date picker's: the parts that would have filled
it in are the ones this component does not publish.

**The popup is not a daisyUI dropdown**, and the reason is one this registry has met before from
the other side (ADR-0015). daisyUI hides `.dropdown-content` unless `dropdown-open` is on the
element above it, and that class would have to be emitted from Rust, but a range picker's open
state lives in a context the primitive keeps to itself, with no controlled prop and no callback
this component can reach. There is nothing here to emit it from. So the panel is positioned with
utilities keyed on a side and an alignment axis, exactly as the hover card's is, and the box it
draws is the popover component's.

**The month is a whole calendar rather than parts.** The calendar component publishes the parts;
inside a picker's popup they would be a second set of the same ones, and what varies there is the
date rather than the layout. `DateRangePickerCalendar` therefore renders the header, the month
buttons, the title, the grid and the days itself, with one axis switching all of it off for a
caller who wants to draw their own.

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
| the chosen day, and today, and the days of the neighbouring months, and the days that cannot be chosen | `data-selected`, `data-today`, `data-month`, `data-unavailable` | the calendar component's own set |
| the two ends of the range and the days between them | `data-selection-start`, `data-selection-between`, `data-selection-end` | a lighter fill and square corners, so the run reads as one band |

**One state needs nothing at all.** The month buttons go inert at the ends of the range the picker
accepts, and the primitive puts the native `disabled` attribute on them, which daisyUI's
`.btn:disabled` matches.

## Axes

- `color: DateRangePickerColor` and `size: DateRangePickerSize` on the field: `input-primary` and
  the rest, `input-xs` to `input-xl`.
- `appearance: DateRangePickerTriggerAppearance` on the button: daisyUI's `btn-ghost btn-square
  btn-sm`, or `btn` alone.
- `side: DateRangePickerSide` and `align: DateRangePickerAlign` on the root: which side the
  calendar opens on and where it sits along it. Both are also what the primitive is told, so the
  `data-side` and `data-align` it reports say what the utilities did.
- `appearance: DateRangePickerPopoverAppearance`: the two utilities that make the popover the box
  the calendar is positioned against.
- `positioning: DateRangePickerPositioning` and `appearance: DateRangePickerContentAppearance` on the
  panel: whether it is taken out of the flow and placed at all, and whether the box is drawn.
- `appearance: DateRangePickerCalendarAppearance` on the month: every class the month is drawn
  with, or none of them.

The colour and size class strings are the input's and the button's, duplicated for the reason
above.

Four of these axes are the inverted shape ADR-0004 describes (their `Default` emits and their
`None` emits nothing), because what they carry is utilities this component emits rather than
daisyUI component classes. `DateRangePickerTriggerAppearance` is inverted for the reason the
calendar's button axis is: what it carries *is* daisyUI classes, and two of those on one element
are settled by the order daisyUI wrote them in.

Every enum exposes `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

## Deviations

**The primitive's segmented range input is not published, because it never settles.** The
primitive's `DateRangePickerInputValue` keeps a signal per end of the range, fills them in from the
value it is given, and reports the range they make back through the change callback. Each hop
writes a signal whether or not the value changed, so once a complete range exists the three effects
chase each other: the value sets the two ends, the two ends make a range, the range is reported, the
report sets the value. The page stops answering: not slowly, but a locked main thread, on the
second click of a range or on the first render of a picker that starts on one.

Nothing here can close that: the loop is between the primitive's own effects, and this registry
styles rather than reimplements. So `DateRangePickerInputValue`, its two halves and the six segments
are left unpublished, and what the field holds is the caller's. The parts that are published (the
popover, the field, the trigger, the panel and the range calendar) are what a range picker is
mostly made of, and every one of them is exercised in the browser specs.

The day upstream fixes it, the segments become three more parts and this section becomes a
paragraph about the date picker's, which are published and do work.

**The `time` crate is a dependency of this component**, for the reason the calendar's docs record:
a date picker's API is `time::Date`, and the primitive re-exports none of it.

**Three defaults are repeated rather than read**: the earliest and latest dates the picker
accepts, and the date the calendar draws as today. The primitive's own come from an extension
trait private to that crate, so this component computes the same values: the first of 1925, the
last of 2050, and the local date falling back to UTC.

**A disabled picker is marked rather than painted.** With no segments in the field there is nothing
here for `data-disabled` to fade; the attribute is on the root, where a caller's own content can
read it: `group-data-[disabled=true]:opacity-50` on the field, say.

**A picker with no popup is a picker with a field.** `DateRangePickerContent` is optional: a picker
that only wants typing leaves it out, and the trigger with it.

**The range is reported once both ends are known.** The primitive fills the second half from the
second click, and reports nothing until it has both, so a caller sees one range rather than two
halves of one.

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
  form control the browser has validated. There is no form control here at all, and what a picker
  accepts is already the `min_date`, `max_date` and `disabled_ranges` the primitive enforces.
- `join`, `join-item`: daisyUI's way of welding a field to a button beside it. The button here is
  *inside* the field rather than beside it, which is the shape `.input` was written for.
- The responsive prefixes daisyUI generates for these classes (`sm:input-lg` and the rest): a
  caller reaches those through `class`, which concatenates.
