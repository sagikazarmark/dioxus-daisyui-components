# Progress

A progress bar styled with daisyUI's `progress` classes, wrapping the `dioxus-primitives`
progress.

```rust
Progress { color: ProgressColor::Primary, value: Some(40.0), aria_label: "Uploading" }

// Or as its parts, which is how the fill is reached.
ProgressRoot { value: Some(40.0), aria_label: "Uploading",
    ProgressIndicator {}
}
```

`value` is required and `None` means indeterminate. `max` defaults to `100`, so a value is a
percentage unless you say otherwise. The bar is named by `aria_label` or `aria_labelledby`; the
primitive gives it `role="progressbar"` and the value, but nothing names it.

## State bridging

**There is no state to bridge.** A progress bar is never open, checked, selected or disabled, and
its value is a number rather than a state daisyUI has a class for, so nothing here emits a
modifier class, and neither tier applies.

What this component has instead is a class that is only half usable, which is a different problem
with the same shape. daisyUI's `.progress` is written for a native `progress` element: the track
is a rule on the element itself and applies to anything, but the filled part is
`::-webkit-progress-value` and `::-moz-progress-bar`, pseudo-elements that exist on that element
and on no other. The primitive renders `div[role=progressbar]` with an indicator inside it,
which it must, since a native `progress` can carry no indicator and takes no ARIA value, so the
fill has nowhere to come from.

Per ADR-0002 the answer is never CSS, so the fill is drawn with Tailwind utilities on the
indicator, sized from the `--progress-value` percentage the primitive already publishes. The
colour axis survives the split intact because every one of daisyUI's colour classes sets nothing
but `color`: the track is `currentColor` mixed down to a fifth, and the fill is `bg-current`.
Recorded as ADR-0012.

## Axes

- `color: ProgressColor`: `progress-neutral`, `progress-primary`, `progress-secondary`,
  `progress-accent`, `progress-info`, `progress-success`, `progress-warning`, `progress-error`.
- `appearance: ProgressIndicatorAppearance`: whether the indicator emits the utilities that draw
  the fill.

`ProgressColor::Default` emits nothing, which leaves the bar at the page's own text colour.
`ProgressIndicatorAppearance` runs the other way, as ADR-0004 requires of utilities the registry
emits: the default arm emits them and `None` emits nothing, so a caller who wants to draw the
fill themselves switches ours off rather than out-ranking it.

Both enums expose `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

daisyUI has no size axis for a progress bar (its height is a plain `height` in the base rule)
so a taller or shorter bar is a caller's utility on the track, which is one of the two things
`class` on this component is for.

## Deviations

**The fill is the registry's, not daisyUI's.** Covered above and recorded as ADR-0012. Two things
follow that a reader should know before they meet them.

The first is that the fill does not animate. daisyUI transitions the native fill's inline size
over 300ms, on the pseudo-element; the utilities here set a width and nothing else, and the
registry adds no animation of its own. A caller who wants one adds `transition-[width]` through
`class` on the indicator, which is a Tailwind utility rather than a stylesheet, so it stays
inside ADR-0002.

The second is that **the indeterminate state has no daisyUI styling**, which is the same gap the
checkbox has for the same kind of reason. daisyUI's rule is `.progress:indeterminate`, a native
pseudo-class that matches a `progress` element with no `value` attribute and can never match a
`div`. The primitive does report the state (`data-state="indeterminate"`, and `aria-valuenow` is
absent, which is what a screen reader announces) so the bar is correct where it is read and bare
where it is looked at. What renders is the track with no fill: the utilities size the fill from
`--progress-value`, which an indeterminate bar does not set, and the fallback in that `var()` is
`0%` rather than nothing, because a width that resolves to `auto` would draw a full bar for a
value that does not exist.

**A caller's attributes land on the track**, where the dialog's land on its innermost element.
The convention is to put them on the element worth reaching, and here that is the outer one: the
track is what a caller sizes, positions and rounds, while the fill's width *is* the value. The
parts are the way to reach the fill, as they are the way to reach the dialog's outer element.

## daisyUI classes deliberately not used

- `radial-progress`: daisyUI's dial. It draws from `--value`, a **unitless** number it multiplies
  into a percentage and also rotates the head of the ring by, and the primitive publishes a
  percentage with its unit attached. Overriding the derived property with a utility would move
  the ring and leave the head behind, and computing a second, unitless copy of the value here
  would mean this component doing the arithmetic the primitive already did. A dial is a different
  component with a different markup shape, and if the registry grows one it will be its own.
- `.progress:indeterminate`: the gap above. Not a matter of taste: it cannot match this markup.
- The responsive prefixes daisyUI generates for the colour classes (`sm:progress-primary` and the
  rest): a caller reaches those through `class`, which concatenates.
