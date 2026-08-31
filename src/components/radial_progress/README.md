# Radial progress

A circular daisyUI progress indicator with value semantics from the `dioxus-primitives` progress
Primitive.

```rust
RadialProgress { value: 70.0, aria_label: "Upload progress", "70%" }

RadialProgress {
    value: 45.0,
    class: "text-primary",
    style: "--size: 7rem; --thickness: 0.5rem;",
    aria_label: "Upload progress",
    "45%"
}
```

`value` is required and uses a fixed range from `0` through `100`. Finite values below zero clamp
to `0`, and values above 100 clamp to `100`. Negative and positive infinity clamp to the
corresponding endpoint; `NaN` normalizes to `0`. The normalized value is the one exposed as
`aria-valuenow`, `data-value`, the Primitive's `--progress-value` percentage, and daisyUI's unitless
`--value`.

Caller children render inside the ring and supply its visible label. A progressbar is named by the
author rather than by its contents, so the caller must still provide `aria_label` or
`aria_labelledby`.

## State bridging

Neither Tier 1 nor Tier 2 applies. daisyUI does not select a progress state from ARIA and has no
modifier class for the number. Instead, its ring and head read a unitless CSS custom property,
`--value`. `RadialProgress` normalizes the typed prop once, gives that number and a fixed maximum of
100 to the progress Primitive, and writes the same number to `--value`. The Primitive remains the
owner of `role="progressbar"`, `aria-valuemin`, `aria-valuemax`, `aria-valuenow`, and its data
attributes. Caller attributes with those names are discarded so the Primitive's final attribute
spread cannot replace the fixed range or normalized value; unrelated caller attributes still
compose normally.

The Primitive also calculates `--progress-value`, but writes it as an inline `style` before that
attribute spread. Supplying the radial `--value` through the spread would therefore replace the
percentage declaration wholesale. The composed style preserves the equivalent percentage from the
same normalized number first, then the caller's declarations, then `--value`; it does not normalize
or calculate a second value.

The Primitive intentionally renders a `div[role=progressbar]`, not a native `progress`. This is the
shape daisyUI documents for radial progress: caller content can sit inside the ring, and its
`::before` and `::after` can draw consistently in Firefox as well as Chromium and WebKit. The
linear `Progress` Component is separate because it has a track and an indicator child; this
Component has one element whose pseudo-elements are the ring and its head.

## Customization

Colour remains `currentColor`. Use ordinary text-colour utilities such as `text-primary`; there is
no colour Axis to duplicate them. daisyUI's default `--size` is `5rem`, and its default
`--thickness` is one tenth of that size. Callers customize either property through inline style or
their own utilities rather than through arbitrary-string props:

```rust
RadialProgress {
    value: 70.0,
    class: "text-success",
    style: "--size: 8rem; --thickness: 0.75rem;",
    "70%"
}
```

Inline style receives special composition because a normal attribute merge would replace the
whole declaration block. The Primitive's normalized `--progress-value` is retained first, caller
declarations follow, and the normalized `--value` is appended as an important declaration.
`--size`, `--thickness`, colour, and unrelated declarations therefore survive, while even a
caller-supplied important `--value` cannot silently erase the typed `value` prop.

daisyUI transitions the ring's derived `--radialprogress` and the head's `transform` over 300ms
with a linear timing function. The Registry adds no motion. Reactive changes to `value` update the
one shared custom property and let daisyUI perform both transitions; reduced-motion behaviour is
therefore whatever the consuming app and daisyUI provide.

## Deviations

ADR-0012 explicitly omitted `radial-progress`: feeding the Primitive's percentage-valued
`--progress-value` into only part of daisyUI's dial would move the ring while leaving its head
behind. This Component supersedes only that omission. Its fixed 0-through-100 contract makes the
normalized number itself the unitless `--value`, which daisyUI uses for both pseudo-elements, so
the ring, head, and ARIA advance together. ADR-0012 remains the decision for how the separate,
linear `Progress` Component draws its fill.

The only deviation from daisyUI's raw example is ownership of progress semantics: daisyUI asks the
caller to repeat `--value`, `aria-valuenow`, and `role`; this Component has the Primitive generate
the semantics from the typed value instead.

## daisyUI classes deliberately not used

- Colour utilities are not built into the Component. The ring already uses `currentColor`, so the
  caller's text-colour utilities compose through `class`.
- There are no size or thickness classes for radial progress. `--size` and `--thickness` remain
  caller CSS customizations.
- The linear `progress` class is not used. It styles a different track-and-fill shape and belongs
  to the separate `Progress` Component.
