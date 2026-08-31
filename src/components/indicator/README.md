# Indicator

A daisyUI indicator: caller content overlaid at a logical position on a container.

[Live examples](https://daisyui-components.dioxus.cc/components/indicator) ·
[their sources](docs/examples/)

This is a Presentational component and wraps no Primitive. `Indicator` renders the relatively
positioned `div.indicator`; one or more `IndicatorItem` parts and the content they decorate are its
children. Keep each item and the decorated content as direct siblings, with the items conventionally
first, to preserve daisyUI's canonical child structure. Multiple items may decorate the same
content.

`IndicatorItem` renders a `div` because it can either carry presentation itself or hold a semantic
element such as a button. daisyUI positions the item element, not whatever is inside it.

## Raw-class fusion

daisyUI's badge and status examples fuse `indicator-item` with `badge` or `status` on one element.
Pass those raw classes through `IndicatorItem`'s `class` attribute to preserve that structure:

```rust
IndicatorItem { class: "badge badge-secondary", "12" }
IndicatorItem { class: "status status-success", aria_label: "Online" }
IndicatorItem { block: IndicatorBlock::Bottom,
    button { class: "btn btn-primary", "Apply" }
}
```

The first two calls merge the caller's classes onto the same `div.indicator-item`; the control
stays a child because its native button element carries behaviour and semantics. This raw-class
escape hatch is deliberate. The Component has no dependency on the Registry's badge, status, or
button Components, and callers remain free to supply text or any other markup.

## State bridging

There is no state to bridge because the component has no state; neither Tier applies. Position is
two styling Axes, not state, and badge, status, text, or control behaviour belongs to caller
content. This is ADR-0023's Presentational component decision.

`dioxus-primitives` is still declared as a dependency, as every Component declares it, because
`merge_attributes` lives there.

## Axes

- `inline: IndicatorInline` on `IndicatorItem` - `indicator-start`, `indicator-center`,
  `indicator-end`.
- `block: IndicatorBlock` on `IndicatorItem` - `indicator-top`, `indicator-middle`,
  `indicator-bottom`.

The Axes are independent and produce all nine positions when combined. Both enums are
non-exhaustive, expose `ALL`, and emit one complete daisyUI class literal for every value. The
default is explicitly `indicator-end indicator-top` rather than an absence of modifiers.

The explicit inline default matters under RTL. daisyUI's bare `indicator-item` fallback is pinned
to physical right, but `indicator-end` is logical: it is right in LTR and left in RTL.
`indicator-start` mirrors in the other direction, while `indicator-center` remains centred. The
block Axis is unchanged by writing direction.

Caller classes concatenate and caller attributes survive on both Compound parts.

## Deviations

None from daisyUI's class structure: `Indicator` and `IndicatorItem` reproduce its documented
container and item parts, including multiple sibling items. The fixed `div` item supports both
same-element badge/status fusion and a nested native control; daisyUI does not require a particular
element type for either class.

## daisyUI classes deliberately not used

- Responsive-prefixed placement classes such as `sm:indicator-middle` - callers add them through
  `class` when placement should change at a breakpoint.
- `badge`, `status`, `btn`, and their modifiers - these style caller content rather than the
  Indicator itself, so examples write them locally and the Component takes no cross-Component
  dependency.
