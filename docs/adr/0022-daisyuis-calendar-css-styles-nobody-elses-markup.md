# daisyUI's calendar CSS styles somebody else's markup

daisyUI ships a component called Calendar. The registry's calendar uses none of it, and that is
not an oversight: there is nothing in it a Dioxus component can wear.

Every rule in `components/calendar.css` names another library's markup:

```css
.cally::part(day) { … }
.cally ::part(selected) { … }
.react-day-picker .rdp-day_selected { … }
.pika-single .is-selected .pika-button { … }
```

The `.cally` rules reach into a web component's **shadow tree** through `::part()`, which matches
only elements a shadow root exposed with a `part` attribute; an element rendered from Rust into
the light DOM cannot be reached by them at any specificity. The other two are class names that
belong to two JavaScript date pickers; putting `.rdp-day_selected` on a button would be the
registry inventing an attachment to a library the app does not have.

So daisyUI's answer for a calendar is "use one of these three libraries and we will style it",
and this registry's premise is the opposite one: a Dioxus component, from a primitive, wearing
daisyUI's classes.

## What is used instead

The classes daisyUI puts on a day everywhere it draws one *itself*: in its own documentation's
examples of a date picker, a day is `btn`. So:

- a day is `btn btn-square` plus daisyUI's size scale, and `btn-ghost` until something happens to
  it;
- the two month buttons are `btn` with the button component's colour, size and look axes;
- everything else (the box, the header row, the weekday headings, the cells) is Tailwind
  utilities, because daisyUI has no class for any of them.

Every state a day is painted from is a **Bridged utility**: a Tailwind variant of an attribute the
primitive already sets on the day: `data-selected`, `data-today`, `data-month`,
`data-unavailable`, `data-selection-start`, `data-selection-between`, `data-selection-end`. Nothing
is recomputed in Rust, and there is no state to lift: unlike the dropdown's open state, no daisyUI
rule is waiting for a modifier class.

The utilities are drawn from daisyUI's theme rather than from Tailwind's palette (`bg-primary`,
`text-primary-content`, `ring-primary`, `border-base-300`), so a calendar restyles with
`[data-theme]` exactly as every other component does, and ADR-0002 still holds: no CSS ships.

## The grid is written out here

The primitive ships a collapsed `CalendarGrid` that renders the table, the weekday headings, the
week rows and the days from one call. It takes attributes for the table alone, so a class can
never reach a heading, a cell, or, most of all, a day. The registry therefore composes the grid
from the primitive's own parts and its public `use_calendar_grid`, which returns the weekdays and
the weeks. The month arithmetic, the first day of the week and the week chunking stay the
primitive's; what is added is a class per element.

This is the same move ADR-0003 records for the tabs, arrived at from the other side: there the
markup had to change for daisyUI's selectors to match, here the markup is identical and only the
authorship moves.

## Consequences

- The calendar's looks are utilities, so every one of them is behind an appearance axis that
  emits nothing (ADR-0004). There are six: the box, the view, the navigation row, the title, the
  grid and the day.
- `Calendar` and `RangeCalendar` are one component with two roots rather than two components. The
  primitive's `CalendarDay` reads whichever context is above it, so the header, the grid and the
  day are shared, and only the value differs.
- The day's `btn-ghost` is a daisyUI class rather than a utility, so it cannot be out-ranked by a
  caller's daisyUI class, which is why it lives in the appearance axis that can be switched off
  rather than in the class list that is always emitted.
- If daisyUI ever ships a calendar of its own for plain markup, this is the ADR to reopen: the
  states are already read off attributes, so a Tier 1 mapping would replace the variants without
  touching the composition.
