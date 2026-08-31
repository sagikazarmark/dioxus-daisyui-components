# Skeleton

Animated block and text placeholders styled with daisyUI's `skeleton` classes.

```rust
section { aria_busy: "true", aria_live: "polite",
    Skeleton { class: "h-32 w-full", aria_hidden: "true" }
    SkeletonText { aria_hidden: "true", "Account details" }
}
```

`Skeleton` renders daisyUI's empty `div.skeleton`. `SkeletonText` renders a
`span.skeleton.skeleton-text` and keeps the caller's text as its children, so the text determines
the shape clipped out of the animated gradient. Width, height, radius, shape and layout remain
caller utilities rather than props.

Both forms are presentational. They do not infer whether a placeholder is decorative, announce a
loading operation, or mark some other element busy. Mark decorative placeholders with
`aria-hidden="true"`; put `aria-busy` and, where updates should be announced, `aria-live` on the
region whose content is actually being replaced.

## State bridging

There is no state to bridge because the component has no state; neither Tier applies. The CSS
animation is presentation rather than Registry state, so the component does not lift, mirror or
otherwise track a loading value. This is ADR-0023's Presentational component decision.

`dioxus-primitives` is still declared as a dependency, as every Component declares it, because
`merge_attributes` lives there.

## Motion

daisyUI runs the `skeleton` keyframe only when `prefers-reduced-motion` is `no-preference`. Under a
reduced-motion preference the gradient remains painted but does not animate. The Registry adds no
motion of its own and does not override that media query.

## Deviations

None from daisyUI: the two parts reproduce its documented empty `div.skeleton` and text-bearing
`span.skeleton.skeleton-text`. They wrap no Primitive because placeholders have no behaviour for
one to provide.

## daisyUI classes deliberately not used

None. The Component exposes both documented skeleton forms. Caller utilities provide dimensions,
corner radius, shape, spacing and responsive changes because daisyUI defines no Skeleton Axis for
them.
