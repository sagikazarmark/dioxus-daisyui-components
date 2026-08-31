# Stat

A daisyUI-styled statistics layout with titles, values, figures, descriptions, and actions.

```rust
Stats { direction: StatsDirection::Horizontal, class: "bg-base-100 shadow",
    Stat {
        StatFigure { class: "text-primary", "24h" }
        StatTitle { "Page views" }
        StatValue { "89,400" }
        StatDescription { "21% more than last month" }
        StatActions { button { class: "btn btn-xs", "Details" } }
    }
    Stat {
        StatTitle { "Downloads" }
        StatValue { "31K" }
        StatDescription { "Jan 1 through Feb 1" }
    }
}
```

This is a Presentational component and wraps no Primitive. All seven Compound parts render `div`
elements with daisyUI's documented classes: `Stats` renders `div.stats`; `Stat` renders
`div.stat`; and `StatTitle`, `StatValue`, `StatDescription`, `StatFigure`, and `StatActions` render
`div.stat-title`, `div.stat-value`, `div.stat-desc`, `div.stat-figure`, and `div.stat-actions`.
These elements add no semantic roles of their own. Callers put meaningful text, controls, and any
needed accessible names inside them.

The hierarchy is part of the API. Every `Stat` must remain a direct child of `Stats` so daisyUI's
grid places the items and its `:not(:last-child)` selector draws inter-item dividers. Every
`StatTitle`, `StatValue`, `StatDescription`, `StatFigure`, and `StatActions` must remain a direct
child of its `Stat` so each part is a grid item in the row and column daisyUI assigns it. Inserted
wrappers break those selectors and placements.

## State bridging

There is no state to bridge because the component has no state; neither Tier applies. A statistics
layout has no focus, keyboard interaction or ARIA wiring for a Primitive to provide.

`dioxus-primitives` is still declared as a dependency, as every Component declares it, because
`merge_attributes` lives there.

## Axes

- `direction: StatsDirection` on `Stats` - `stats-horizontal` or `stats-vertical`.

Both values are explicit: the default `StatsDirection::Horizontal` emits `stats-horizontal`, and
`StatsDirection::Vertical` emits `stats-vertical`. The enum exposes `ALL` so the Preview and browser
spec render both values. The direction changes the root grid flow and whether dividers appear at
the inline or block end of each non-final `Stat`.

Caller classes concatenate on all seven parts, and other caller attributes survive the merge.
Colour, centering, border, background, width, overflow, and shadow remain caller utilities rather
than Stat Axes because daisyUI defines no independent stat dimensions for them.

## Deviations

None from daisyUI: the seven parts reproduce its documented statistics structure and all render
the documented `div` elements. The Component wraps no Primitive because a statistics layout has no
behaviour for one to provide.

## daisyUI classes deliberately not used

- Responsive-prefixed direction classes such as `sm:stats-horizontal` - callers add them through
  `class` when a breakpoint should choose the direction.
- Colour, centering, border, background, and shadow utilities shown in daisyUI examples - these are
  caller styling, not classes or Axes owned by Stat.
