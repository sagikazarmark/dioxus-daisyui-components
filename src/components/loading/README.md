# Loading

An empty loading indicator with daisyUI's animation and size Axes.

```rust
button { class: "btn btn-primary",
    Loading { size: LoadingSize::Sm, aria_hidden: "true" }
    "Save"
}

Loading { animation: LoadingAnimation::Dots, role: "status", aria_label: "Loading account" }
```

`Loading` renders an empty `span.loading`. Animation and size are independent typed Axes. Colour
remains a caller utility because daisyUI paints the mask with `currentColor`.

The Component assigns no unconditional role or live-region behaviour. A spinner inside a named
button is decorative and should be hidden from assistive technology. A standalone operation can
instead receive caller-supplied `role="status"`, `aria-label`, or the semantics appropriate to the
surrounding region.

## State bridging

There is no state to bridge because the component has no state; neither Tier applies. Loading is a
CSS presentation chosen by an Axis, not a Registry-owned loading value. In ADR-0023's terms this is
a Presentational component, so it does not lift, mirror or derive application state.

`dioxus-primitives` is still declared as a dependency, as every Component declares it, because
`merge_attributes` lives there.

## Axes

- `animation: LoadingAnimation`: `loading-dots`, `loading-ring`, `loading-ball`, `loading-bars`,
  `loading-infinity`.
- `size: LoadingSize`: `loading-xs`, `loading-sm`, `loading-lg`, `loading-xl`.

Both enums are non-exhaustive and expose `ALL` for the Preview. `LoadingAnimation::Default` emits
nothing because bare `loading` is already daisyUI's spinner. `LoadingSize::Default` also emits
nothing because bare `loading` is already medium-sized.

## Motion

daisyUI draws every animation as an SVG embedded in `mask-image`; the SVG's SMIL elements perform
the motion rather than a CSS `animation` property. Under `prefers-reduced-motion: reduce`, daisyUI
uses alternate embedded SVGs with longer SMIL durations. Motion slows rather than stopping. The
Registry adds no motion mechanism and does not override daisyUI's reduced-motion choice.

## Deviations

None from daisyUI: the Component reproduces its documented empty `span.loading`, animation
modifiers and size modifiers. It wraps no Primitive because an animated mask has no behaviour for
one to provide.

## daisyUI classes deliberately not used

- `loading-spinner`: bare `loading` uses the identical spinner mask, so the explicit default is
  redundant.
- `loading-md`: bare `loading` has the identical medium width, so the explicit default is
  redundant.
- Responsive-prefixed loading classes: callers add them through `class`, which concatenates.
- Colour utilities: colour is not a daisyUI Loading Axis; the mask uses `currentColor`, so callers
  use their ordinary text-colour utilities.
