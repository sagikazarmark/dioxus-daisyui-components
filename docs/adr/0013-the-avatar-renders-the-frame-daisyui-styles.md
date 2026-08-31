# The avatar renders the frame daisyUI styles

`AvatarFrame` is a plain `div` the registry renders between the primitive's root and whatever
goes inside it. The primitive has no part there and needs none; daisyUI does.

Every rule that makes an avatar look like an avatar is written against that element. `.avatar`
itself only positions: it is an inline flex box with a `::before` for the status dot. The square
aspect ratio and the clipping are `.avatar > div`, the image fill is `.avatar img`, and the size
and the corner radius are Tailwind utilities on the same `div` in every one of daisyUI's own
examples. An image dropped straight into the primitive's root is styled by none of it: it is
not clipped, has no shape, and sizes itself.

The element could have been left to the caller to write, since it carries no behaviour. It is
rendered here instead for the reason the dropdown menu renders its list (ADR-0005): the markup
daisyUI's selectors need is not the markup the primitive produces, and closing that gap is what
this registry is for. A caller who had to remember a bare `div` in the middle would be doing the
registry's job, and getting a subtly unstyled avatar when they forgot.

## Consequences

- The parts are `AvatarRoot` and `AvatarFrame`, with a collapsed `Avatar` over the two. The
  collapse is legal: daisyUI puts nothing between them and no caller content goes there.
- Caller attributes on the collapsed component land on the **frame**, because the size, the
  shape and any ring are written there on both sides of the selector.
- That hides one thing worth reaching, so one attribute is routed back: the primitive gives the
  root `role="img"`, whose name must come from the author rather than from its content, so the
  collapsed component takes a `label` prop that lands on the root as `aria-label`. An avatar
  named by another element on the page uses the parts.
- The frame's size and shape are defeatable utilities (ADR-0004), since daisyUI has no class for
  either.
- `avatar-group` is not part of this component. It is a layout for a set of avatars (a
  negatively spaced flex row with a ring on each) and belongs to whatever renders the set.
