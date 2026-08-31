# The drawer projects dialog state into a checkbox

The drawer keeps daisyUI's hidden checkbox as styling machinery, but not as a control. `Drawer`
owns open state by ADR-0006's controlled/uncontrolled pattern and projects that state into an
inert `.drawer-toggle`. The checkbox is disabled, outside the tab order and hidden from the
accessibility tree; `DrawerTrigger` is the real button and changes the same state that controls
the dialog Primitive.

Neither existing state-bridging tier can replace the checkbox. daisyUI has no transient-open
modifier class to emit at Tier 2, and its selectors do not match the Primitive's `data-state` or
ARIA at Tier 1. They require `.drawer-toggle:checked ~ .drawer-side`, so the projected checkbox,
the always-mounted `.drawer-content` and the dialog Primitive's `.drawer-side` root remain ordered
siblings under `.drawer`.

The dialog Primitive wraps the side rather than the whole drawer. Its root animation-mounts only
the side, while the drawer grid and page content remain mounted when closed. Inside that root,
`.drawer-overlay` and `DrawerPanel` are direct children. `DrawerPanel` renders the Primitive's
`DialogContent` as the direct non-overlay child, so daisyUI's `.drawer-side >
:not(.drawer-overlay)` slide rule and the Primitive's focus trap, naming and outside-dismiss
boundary all reach the same panel. The overlay is outside the panel and therefore dismisses
through the Primitive.

## Consequences

- The checkbox never owns or independently changes state. One Registry-owned `DrawerPhase` signal
  drives trigger ARIA, the controlled dialog Primitive and the checkbox projection.
- `DrawerPhase::Opening` lasts until the dialog side has painted after mounting. It counts as open
  for ARIA and the Primitive but leaves the checkbox unchecked, so the side first paints in
  daisyUI's unchecked position. Advancing the same signal to `Open` checks the projection and runs
  daisyUI's own transition.
- The Primitive's modal trap is withheld during `Opening`, while the panel is still hidden. It
  activates from the same phase change that checks the projection, after the panel can receive
  focus. `Closing` unchecks the projection while keeping the Primitive root mounted until the
  unchecked state has painted; `Closed` then lets the Primitive remove its own trap, restore focus
  and unmount.
- Escape, overlay dismissal, focus trapping, focus restoration, title and description wiring, and
  animation-aware unmounting remain the Primitive's behavior.
- Pointer dismissal can let the browser's default focus step run after the Primitive removes its
  trap. `Drawer` remembers the mounted trigger and reinforces restoration on the next task after
  its phase reaches `Closed`; trapping and dismissal remain the Primitive's.
- `drawer-end` is a placement Axis. `drawer-open` is not open state: it creates an always-visible,
  sticky grid column and can make visual, modal and ARIA states disagree, so the Registry does not
  expose or support it, including responsive forms such as `lg:drawer-open`.
- daisyUI's focus forwarding rule for `label.drawer-button` does not apply. `DrawerTrigger` is a
  real button with its own native focus ring rather than a label for the inert checkbox.
- The composition ships no CSS; it depends only on daisyUI's existing sibling and direct-child
  selectors.
