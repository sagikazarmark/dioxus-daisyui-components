# Dropdown placement and alignment always emit a class

The dropdown menu's `placement` and `alignment` axes emit a daisyUI class for every value,
their default values included. Every other axis in the registry has a `Default` arm that emits
nothing, standing for daisyUI's unclassed element.

An unclassed `.dropdown-content` is `position: absolute` with no offsets set, so it lands at its
static position, immediately after whatever precedes it in flow, aligned to the box's inline
start. With the markup this component renders today that is the same place `dropdown-bottom
dropdown-start` puts it, because the trigger is the only thing written before the content. It
stops being the same place as soon as a caller writes anything else inside `DropdownMenu`, and
it is not the same thing even now: `dropdown-bottom` also gives the menu a transform origin, so
the two grow from different edges as they open.

Emitting the class says where the menu goes in terms of the element rather than in terms of the
flow, which is what a caller passing `placement` is asking for. It also keeps the axis honest:
every value of it is a class that can be shown to apply, where a value that emitted nothing
could only be shown to render like a value that does.

Nothing is lost. daisyUI's placement classes carry no colour, size or paint; the only thing a
caller could want from an unclassed dropdown is flow placement, which is the less predictable
of the two.

## Consequences

- The two axes compose the way daisyUI's classes do (an alignment on a vertical placement moves
  the menu across the trigger, on a horizontal one along it), and every combination is a pair of
  classes daisyUI wrote rules for.
- This is a third shape for an axis, alongside "default emits nothing" and the inverted
  appearance axes of ADR-0004. Which one an axis takes is a question about the daisyUI classes
  behind it rather than a convention to apply evenly.
- The select component borrows this dropdown's structure, and inherits the decision with it.
- The tooltip arrives at the same shape for a different reason, which is worth keeping distinct:
  daisyUI's base rule places its bubble exactly where `tooltip-top` does, so the class looks
  redundant, until the tail, a pseudo-element the base rule gives no position at all and only a
  placement or alignment class puts anywhere. Both components emit for every value; only this
  one does it because of flow.
