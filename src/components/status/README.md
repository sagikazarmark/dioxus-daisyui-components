# Status

A small daisyUI indicator for visually showing an element's current status.

[Live examples](https://daisyui-components.dioxus.cc/components/status) ·
[their sources](docs/examples/)

`Status` renders an empty `span.status`. Colour and size are independent typed Axes. The Component
does not accept children because daisyUI's status icon is empty; visible meaning belongs beside it,
not inside it.

The Component assigns no unconditional role, label or live-region behaviour. A dot beside visible
text is decorative and can receive caller-supplied `aria-hidden="true"`. A standalone icon can
instead receive the caller's appropriate `role` and `aria-label`. Whether a changing status should
be announced depends on the surrounding application, so callers put live-region semantics on the
region whose change should be announced.

## State bridging

There is no state to bridge because the component has no state; neither Tier applies. A colour
prop chooses presentation rather than carrying an application status, and the Component does not
lift, mirror or derive the caller's online, offline, success or error state. This is ADR-0023's
Presentational component decision.

`dioxus-primitives` is still declared as a dependency, as every Component declares it, because
`merge_attributes` lives there.

## Axes

- `color: StatusColor` - `status-neutral`, `status-primary`, `status-secondary`, `status-accent`,
  `status-info`, `status-success`, `status-warning`, `status-error`.
- `size: StatusSize` - `status-xs`, `status-sm`, `status-lg`, `status-xl`.

Both Axes are non-exhaustive and expose `ALL` for the Preview. `StatusColor::Default` emits no
modifier and remains distinct from `StatusColor::Neutral`. `StatusSize::Default` also emits no
modifier because bare `status` is already medium-sized. Every modifier is a complete literal in
this Component's source so Tailwind can discover it when the Component is installed alone.

## Motion

Ping and bounce are caller-owned Tailwind utilities, not daisyUI Status Axes or Registry state.
Callers add `animate-ping` to one status and compose a static status beneath it, or add
`animate-bounce` directly to one status. The Component does not select an animation, track whether
one is running, or impose a reduced-motion policy. Callers that need one can use Tailwind's
`motion-safe:` or `motion-reduce:` variants with their chosen animation utility.

## Deviations

None from daisyUI: the Component reproduces its documented empty `span.status`, colour modifiers
and size modifiers. It wraps no Primitive because a visual status dot has no behaviour for one to
provide.

## daisyUI classes deliberately not used

- `status-md` - bare `status` has the identical medium dimensions, so the default size emits
  nothing.
- Responsive-prefixed status classes - callers add them through `class`, which concatenates.
- `animate-ping` and `animate-bounce` - these are Tailwind utilities callers compose, not daisyUI
  Status classes or Axes.
