# Drawer

A page drawer styled with daisyUI's `drawer` classes and backed by the `dioxus-primitives`
dialog. The page stays mounted while the side traps focus, dismisses on Escape or the overlay,
and returns focus to the trigger.

[Live examples](https://daisyui-components.dioxus.cc/components/drawer) ·
[their sources](docs/examples/)

## Composition

The compound parts are `Drawer`, `DrawerContent`, `DrawerTrigger`, `DrawerSide`,
`DrawerOverlay`, `DrawerPanel`, `DrawerTitle` and `DrawerDescription`.

`Drawer` always renders the inert `.drawer-toggle` first. The caller then writes
`DrawerContent` followed by `DrawerSide`, leaving the checkbox, content and side as ordered direct
siblings. `DrawerSide` renders the dialog Primitive's root as `.drawer-side`; `DrawerOverlay` and
`DrawerPanel` must be its direct children. `DrawerPanel` is the Primitive's `DialogContent`, not a
wrapper around it, so daisyUI slides the same element that traps focus.

`DrawerOverlay` is outside `DrawerPanel`, so the Primitive receives its `pointerdown` as an outside
interaction and dismisses the side. The Primitive owns the trap and its cleanup; `Drawer` remembers
the mounted trigger and refocuses it after the phase reaches `Closed`, covering the browser's later
pointer focus step that can otherwise undo the Primitive's restoration.

An explicit `id` on `DrawerSide` is shared with `DrawerTrigger`'s `aria-controls`. Other caller
classes concatenate with each part's own class and other attributes override the defaults.
`DrawerSide` traps focus by default; its `is_modal` prop passes through the Primitive for a side
that is deliberately non-modal.

## State bridging

Neither **Tier 1** nor **Tier 2** can express daisyUI's open selector. daisyUI does not match the
Primitive's ARIA or `data-state`, and it has no transient-open modifier class: it only reveals the
side through `.drawer-toggle:checked ~ .drawer-side`.

`Drawer` therefore lifts open state by ADR-0006's controlled/uncontrolled pattern and projects it
into a disabled, untabbable, ARIA-hidden `.drawer-toggle`. The checkbox is styling machinery, not
a second control. One Registry-owned phase signal drives its checked state, `DrawerTrigger`'s
`aria-expanded`, and the controlled dialog Primitive. Its `Opening` phase is logically open but
leaves the projection unchecked until the mounted side has painted; it is animation sequencing,
not a second state owner. Changes from Escape or outside dismissal travel back through that one
callback.

The dialog's modal trap is withheld only during `Opening`, while daisyUI still hides the panel. It
activates when that same phase advances to `Open`, so the visible panel receives focus. On close it
remains the Primitive that removes the trap and restores focus. `Closing` unchecks the projection
first but keeps the dialog root mounted until that style has painted; advancing the same signal to
`Closed` then lets the Primitive unmount after daisyUI's exit transition.

The root and `DrawerContent` stay mounted while closed. Only the dialog-backed `DrawerSide` is
animation-mounted. daisyUI defines no insertion starting style for a cold-mounted drawer side, so
the checked projection waits until the Primitive's mounted side has painted. The side first
paints in daisyUI's unchecked position, then the checkbox checks and daisyUI runs its own entry
transition. Closing unchecks immediately, and the Primitive keeps the side mounted through
daisyUI's exit transition.

## Axes

`placement: DrawerPlacement` is `Start` (no class) or `End` (`drawer-end`). The edges follow the
document's writing direction. The enum exposes `ALL`, and the Preview renders every value from it.

## Deviations

The hidden checkbox does not accept input. daisyUI's own drawer makes it the control and uses a
label as the trigger; this Component uses the dialog Primitive for behavior and a real button for
the trigger, while retaining the checkbox only because daisyUI's sibling selector requires it.

The dialog Primitive unmounts `.drawer-side` after its exit animation, while daisyUI's CSS-only
drawer leaves the hidden side in the document. The `.drawer` root and `.drawer-content` are outside
the Primitive and remain mounted.

daisyUI's `.drawer-toggle:focus-visible ~ .drawer-content label.drawer-button` rule does not apply:
the inert checkbox never receives focus and `DrawerTrigger` is a button, not its label. The button
keeps daisyUI's own `.btn:focus-visible` treatment instead.

## daisyUI classes deliberately not used

- `drawer-open` is an always-visible sticky grid column, not transient open state. It and responsive
  forms such as `lg:drawer-open` can make visual, modal and ARIA states disagree at a breakpoint,
  so they are unsupported rather than exposed as an Axis.
- `is-drawer-open` and `is-drawer-close` remain available to callers as Tailwind variants against
  the projected checkbox state; they are variants rather than classes emitted by a part.
