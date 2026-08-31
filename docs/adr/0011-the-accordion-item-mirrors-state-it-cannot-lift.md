# The accordion item mirrors state it cannot lift

`AccordionItem` seeds a signal from `default_open` and updates it from the primitive's
`on_change` callback, and passes no controlled value back down. The primitive stays the only
writer, and the styled wrapper only reads. This is the inverse of the lift ADR-0006 describes
and is forced by the same kind of fact.

daisyUI reveals a panel through `.collapse-open` on the item and matches nothing the primitive
sets, so the class has to be emitted from Rust and the item has to know whether it is open,
which is the situation ADR-0006 answers with a lift. A lift is not available here. The
primitive's `AccordionItem` has no controlled `open` prop: the accordion **root** owns which
items are open, because it enforces `allow_multiple_open` and `collapsible` across all of them.
An item that owned its own open state could disagree with the set about it: two items both
believing they are open under a root that allows one.

So the state travels the other way. The primitive fires `on_change` from an effect on every
change, including the one `default_open` causes on mount, and the wrapper writes what it is told
into a signal of its own. Seeding that signal from the same `default_open` the primitive is
seeded from keeps the class right on the first render rather than one callback later.

## Consequences

- There are now two shapes for emitting a modifier class from primitive state: **lifted**, where
  the wrapper owns the state and the primitive is controlled (dialog, dropdown menu, select,
  tooltip, alert dialog), and **mirrored**, where the primitive owns it and the wrapper follows
  (accordion). Which one applies is a question about the primitive's props rather than a
  convention to apply evenly; a component whose primitive offers a controlled prop should lift.
- The closing animation still plays, for the same reason it does under a lift: the class comes
  off while the panel is still in the document, which is what the primitive's animation-aware
  unmounting waits on.
- A caller's own `on_change` still fires, because the wrapper intercepts the callback and calls
  it rather than replacing it.
- The mirror is one render behind nothing, but it is a second copy of state. If the primitive
  ever stops firing `on_change` for a transition, the class stops following it, which is worth
  knowing when the pinned revision moves, and is what the browser specs assert by opening and
  closing rather than only by opening.
