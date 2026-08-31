# Toggle

A button that stays pressed, styled with daisyUI's `btn` classes, wrapping the
`dioxus-primitives` toggle.

[Live examples](https://daisyui-components.dioxus.cc/components/toggle) ·
[their sources](docs/examples/)

One toggle is one button. A row of them that behave as a set (one pressed at a time, or several,
joined into a single shape) is the toggle group instead; this is the case where the control
stands on its own, next to whatever else the caller wrote.

## State bridging

**Tier 2** on the pressed state. daisyUI's pressed button is `btn-active`, and it matches no ARIA
attribute at all: `aria-pressed`, which the primitive sets on this very element, appears nowhere
in daisyUI's stylesheet, not once in a megabyte of it. So the class is emitted from Rust as a
complete literal.

The state is **lifted** (ADR-0006), the way the toggle group's pressed set is. This component
seeds a signal from `default_pressed`, always hands the primitive a controlled value, and
intercepts the change callback, so a controlled caller and an uncontrolled one both work and this
component is the only writer. Without the lift there would be nothing to emit the class from: the
primitive keeps its state internally when nobody controls it, and tells the outside about it only
through the callback and through `aria-pressed`.

The disabled state is not bridged and does not need to be: the primitive sets the `disabled`
attribute on the `button`, and daisyUI's rule is `.btn:disabled`. It is a native attribute rather
than an ARIA one, so it is not Tier 1 either; there is nothing to bridge.

## Axes

- `color: ToggleColor`: `btn-neutral`, `btn-primary`, `btn-secondary`, `btn-accent`, `btn-info`,
  `btn-success`, `btn-warning`, `btn-error`.
- `size: ToggleSize`: `btn-xs`, `btn-sm`, `btn-lg`, `btn-xl`.

Both are the button component's axes, with the class strings duplicated rather than depended on:
cross-Component dependencies are for public composition, while the Tailwind contract requires
every one of these classes to be a literal in the file that emits it.

`ToggleColor::Default` emits nothing, which is daisyUI's uncoloured button. `ToggleSize::Default`
emits nothing and renders at the same size as `btn-md`.

Both enums expose `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

`btn-active` is not an axis. It is what this component emits for the pressed state, and a caller
who set it themselves would be lying about the state to daisyUI while `aria-pressed` said
otherwise.

## Deviations

**A pressed toggle is a coloured button, not a filled one.** daisyUI's `btn-active` is what its
own documentation calls the "active" look of a button, which is the same paint a button gets while
the pointer is held down. On an uncoloured toggle that is a grey fill, and on a coloured one it is
a darker shade of the colour. It is the only pressed look daisyUI has for a button, and it is what
the toggle group emits too, so the two components read the same way when a caller uses both.

**The toggle takes focus on click.** That is the primitive's, not this component's: Safari and
Firefox on macOS do not focus a `button` on click, and the primitive sets focus explicitly to
match what the other primitive libraries do. It matters here because daisyUI's focus ring and its
`btn-active` fill are then both visible at once on a freshly clicked toggle, which is deliberate:
the fill says what the state is and the ring says where the keyboard is.

## daisyUI classes deliberately not used

- `swap`, `swap-on`, `swap-off`, `swap-active`, `swap-rotate`, `swap-flip`: daisyUI's other
  answer for a control with two states, which swaps one child for another as the state changes.
  It is not exposed because it is markup rather than paint: it wants a `swap-on` child and a
  `swap-off` child that the caller writes, and this component would then be emitting `swap-active`
  for a structure it cannot see. A caller who wants it has the state in their own hands (they can
  control `pressed`) and passes `swap-active` through `class`, which concatenates.
- `btn-outline`, `btn-soft`, `btn-dash`, `btn-ghost`, `btn-link`, `btn-wide`, `btn-block`,
  `btn-square`, `btn-circle`: the button component's other looks, not exposed here for the same
  reason they are not exposed there: a caller reaches them through `class`, which concatenates.
  `btn-square` and `btn-circle` in particular are what an icon toggle wants.
- `btn-disabled`: daisyUI's class for a button that only looks disabled. The primitive sets the
  native attribute, which daisyUI styles and which also makes the button inert; the class would
  say the same thing to the eye and nothing to anybody else.
- The responsive prefixes daisyUI generates for the colour and size classes (`sm:btn-lg` and the
  rest): a caller reaches those through `class`, which concatenates.
