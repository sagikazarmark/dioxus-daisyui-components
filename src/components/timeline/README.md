# Timeline

A daisyUI-styled chronological list with connected events.

```rust
Timeline { direction: TimelineDirection::Vertical,
    TimelineItem {
        TimelineStart { "2024" }
        TimelineMiddle { aria_hidden: "true", span { class: "size-3 rounded-full bg-primary" } }
        TimelineEnd { appearance: TimelineContentAppearance::Box, "First release" }
        TimelineConnector { aria_hidden: "true" }
    }
    TimelineItem {
        TimelineConnector { aria_hidden: "true" }
        TimelineStart { "2025" }
        TimelineMiddle { aria_hidden: "true", span { class: "size-3 rounded-full bg-primary" } }
        TimelineEnd { appearance: TimelineContentAppearance::Box, "Stable release" }
    }
}
```

This is a Presentational component and wraps no Primitive. Its six Compound parts reproduce
daisyUI's documented structure. `Timeline` renders `ul.timeline`; every `TimelineItem` renders a
direct `li`; `TimelineStart`, `TimelineMiddle`, and `TimelineEnd` render direct `div` children with
their matching `timeline-*` classes; and `TimelineConnector` renders a direct `hr`.

## Ordering

Write `TimelineItem` elements in chronological DOM order. Start and end name visual sides of the
line, not earlier and later events, so alternating content between them must not reorder the list.
This keeps reading order, source order, and chronology together in either direction.

Connector position is structural. Put a `TimelineConnector` before an item's content for the line
coming from the previous event and after its content for the line going to the next event. The
first item normally has only the outgoing connector, a middle item has both, and the last has only
the incoming connector. Connectors must remain direct `hr` children of `TimelineItem`; a wrapper
prevents daisyUI's first-child and last-child selectors from placing them.

## Accessibility

The `ul` and `li` elements expose the chronology as an ordinary list. Add an accessible name to
`Timeline` when the surrounding heading does not already identify the list. The caller owns
connector semantics: set `aria-hidden="true"` when an `hr` is only a visual continuation, or leave
it exposed when it represents a thematic break.

Content inside `TimelineMiddle` is caller-owned. Hide a marker icon with `aria-hidden="true"` when
the start or end content already says what happened. If a marker communicates information not
present in the text, give that information a textual equivalent instead of relying on colour or
shape alone.

## State bridging

There is no state to bridge because the component has no state; neither Tier applies. Direction,
compact layout, snapped markers, and boxes are styling Axes, not open, selected, or focused state.

`dioxus-primitives` is still declared as a dependency, as every Component declares it, because
`merge_attributes` lives there.

## Axes

- `direction: TimelineDirection` on `Timeline` - `timeline-horizontal` or `timeline-vertical`.
- `compact: TimelineCompact` on `Timeline` - `timeline-compact`, or nothing.
- `snap: TimelineSnap` on `Timeline` - `timeline-snap-icon`, or nothing.
- `appearance: TimelineContentAppearance` on `TimelineStart` and `TimelineEnd` - `timeline-box`,
  or nothing.

The four Axes are independent, non-exhaustive, and expose `ALL` so the Preview renders every
value. Direction is explicit for both values. `TimelineCompact::Default`, `TimelineSnap::Default`,
and `TimelineContentAppearance::Default` emit no modifier class.

Line colour is deliberately not an Axis. daisyUI gives connectors the theme's `base-300` colour
and documents ordinary background utilities for changing individual segments. Pass classes such
as `bg-primary` to `TimelineConnector`; caller classes and attributes merge on every part.

## Deviations

The structure and classes match daisyUI. The Component does not add accessibility attributes;
callers decide whether connectors and marker icons are decorative.

The Component wraps no Primitive because a timeline has no focus, keyboard interaction, or state
for one to provide.

## daisyUI classes deliberately not used

- Responsive-prefixed timeline classes - callers add them through `class` when direction or
  another modifier should change at a breakpoint.
- Colour classes - daisyUI defines no timeline colour Axis; callers paint connector segments and
  marker icons with ordinary colour utilities.
