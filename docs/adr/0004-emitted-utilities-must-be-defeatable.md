# Emitted utilities must be defeatable

Where daisyUI has no component class for an element and the registry emits Tailwind utilities
instead, those utilities are emitted by default and the component exposes an axis value that
emits nothing.

§2 skips a `tailwind-merge` equivalent on the grounds that components emit daisyUI *component*
classes, which lose cleanly to caller utilities. That premise holds, but not for the reason
first given: it is not specificity. Both are single class selectors, so on specificity they
tie.

What decides it is cascade layers. Tailwind v4 puts everything in `@layer utilities`, and
daisyUI 5 wraps each of its declarations in a sub-layer nested inside its own rule:

```css
@layer utilities {
  .checkbox { @layer daisyui.l1.l2.l3 { border-radius: var(--radius-selector); } }
  .rounded-none { border-radius: 0; }
}
```

A declaration that is in no sub-layer outranks one that is, so the caller's `rounded-none`
wins, whatever its specificity, and wherever it falls in the file. Verified in chromium, firefox
and webkit against exactly that rule shape, with daisyUI's rule placed later in source *and*
given higher specificity; the unlayered declaration still won both times. The decision is
therefore on firmer ground than the original reasoning claimed: a caller beats a daisyUI
component class even when their utility is the weaker selector.

Utilities the registry emits itself get no such help. Tailwind generates them exactly as it
generates the caller's (unlayered, in the same `utilities` layer), so a collision is a real
tie, and a tie is resolved by generated-CSS order rather than class-attribute order. A caller
overriding our `py-4` with `py-2` silently loses while `py-8` silently wins. Making the
registry's utilities switchable means the caller never has to win that fight.

## Consequences

- Components look finished on install rather than requiring the caller to style them.
- Elements such as `DialogTitle` and `DialogDescription` carry an appearance axis whose default
  arm emits utilities, the inverse of the §7 convention that a default arm returns `""`.
- Elements styled purely with daisyUI component classes need no such axis, because caller
  utilities already override them cleanly.
- That last point is daisyUI's to keep, not ours. It rests on daisyUI sub-layering its
  declarations, which is a detail of how it emits under Tailwind v4 rather than anything it
  promises. If a release stopped doing it, every one of those overrides would fall back to a
  source-order tie, the same fight this ADR exists to keep callers out of, but across the
  whole registry. Worth re-checking on a daisyUI major.
