# Badge

A small label for a count, status or tag, styled with daisyUI's `badge` classes.

```rust
Badge {
    color: BadgeColor::Primary,
    size: BadgeSize::Lg,
    appearance: BadgeAppearance::Outline,
    "New"
}
```

The component reproduces daisyUI's canonical single `span.badge`. It assigns no role or
announcement behaviour; callers add semantic attributes when their use of the badge requires
them.

## State bridging

There is no state to bridge because the component has no state; neither Tier applies. There is no
badge Primitive to wrap, and none is appropriate for presentational markup with no focus, keyboard
interaction or ARIA wiring of its own.

`dioxus-primitives` is still declared as a dependency, as every component declares it, because
`merge_attributes` lives there.

## Axes

- `color: BadgeColor`: `badge-neutral`, `badge-primary`, `badge-secondary`, `badge-accent`,
  `badge-info`, `badge-success`, `badge-warning`, `badge-error`.
- `size: BadgeSize`: `badge-xs`, `badge-sm`, `badge-lg`, `badge-xl`.
- `appearance: BadgeAppearance`: `badge-outline`, `badge-dash`, `badge-soft`, `badge-ghost`.

All three axes are orthogonal. Each enum's `Default` emits nothing, and each enum exposes `ALL` so
the Preview and browser specs render every value. The appearance prop is not named `style` because
that name belongs to the global HTML attribute.

`BadgeColor::Default` is daisyUI's uncoloured badge, rather than a synonym for neutral.
`BadgeSize::Default` renders identically to explicit `badge-md`, so that class is not emitted.
`BadgeAppearance::Default` is the filled badge.

The colour and size class strings are deliberately duplicated from `tag_group` rather than shared.
Cross-Component dependencies are for public composition, while Tailwind's scanner must see every
complete class literal in the Component file that emits it.

## Deviations

None from daisyUI: the component reproduces daisyUI's documented `span.badge` markup. It wraps no
Primitive because a badge has no behaviour for one to provide.

## daisyUI classes deliberately not used

- `badge-md`: the default size emits nothing, and daisyUI renders an unclassed badge at exactly
  that size.
- Responsive-prefixed badge classes: callers add them through `class`.
- `indicator` and `indicator-item`: these describe separate surrounding indicator structure,
  rather than an axis of a badge.
