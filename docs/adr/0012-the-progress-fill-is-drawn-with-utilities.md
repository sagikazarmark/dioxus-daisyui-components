# The progress fill is drawn with utilities

The `radial-progress` omission below is superseded by
issue #63. A dedicated
`RadialProgress` now fixes its range at 0 through 100 so one normalized value can drive both the
Primitive's progressbar semantics and daisyUI's unitless `--value`.

`ProgressRoot` keeps daisyUI's `progress` class and its colour axis; `ProgressIndicator` draws
the filled part with Tailwind utilities (`block h-full w-[var(--progress-value,0%)] bg-current`)
rather than with any daisyUI class.

daisyUI's progress is written for the native element. The track is a rule on `.progress` itself
and applies to whatever carries the class, but the fill is drawn in
`::-webkit-progress-value` and `::-moz-progress-bar`, which exist on a `progress` element and
nowhere else. The primitive renders `div[role=progressbar]` with an indicator element inside,
and it has to: a native `progress` cannot contain an indicator, and it takes its value from an
attribute rather than from ARIA, so the primitive could not report an indeterminate bar or a
custom maximum through it.

So one daisyUI class covers two elements' worth of styling, and only one of the two lands. §2's
rule for that case is to drop the daisyUI class for the element it cannot style and use
utilities there, never CSS (ADR-0002). What makes it hold together here rather than fork is
that daisyUI's colour classes set nothing but `color`: `.progress-primary { color: var(--color-primary) }`,
with the track painting `currentColor` mixed down to a fifth and the native fill painting
`currentColor` outright. A fill drawn with `bg-current` is the same colour by the same route, so
the axis is passed through untouched and there is no second colour mapping to keep in step.

The width comes from `--progress-value`, the percentage the primitive computes from `value` and
`max` and publishes on the root. Reading it back through a utility means the arithmetic happens
once, in the primitive, and the fill cannot disagree with `aria-valuenow`. The `0%` fallback in
that `var()` is load-bearing: an indeterminate bar sets no such property, and a width that
resolved to `auto` would draw a *full* bar for a value that does not exist.

## Consequences

- The utilities are defeatable (ADR-0004), so a caller who wants a different fill switches ours
  off rather than out-ranking it.
- The collapsed `Progress` puts caller attributes on the **track**, where the collapsed `Dialog`
  puts them on its innermost element. Both follow the same rule, the element worth reaching,
  and here that is the outer one, because the fill's width is the value rather than a style. The
  parts remain the way to reach it.
- The fill does not animate, where daisyUI transitions the native one. The registry adds no
  animation of its own; a caller adds a transition utility if they want it.
- daisyUI's `:indeterminate` styling is unreachable for the same reason the fill is, and is
  documented as a gap rather than approximated. The state still reaches the accessibility tree,
  which is the half that matters.
- `radial-progress` is not offered. It is driven by a unitless `--value` that the primitive's
  percentage cannot supply, and half of it (the ring) could be reached by overriding daisyUI's
  derived property while the other half, the head of the ring, could not. Half a dial is worse
  than none.
