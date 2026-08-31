# Styled wrappers own open and value state

Where a component needs to emit a daisyUI modifier class that depends on primitive state, the
registry's wrapper owns that state: it seeds a signal from the primitive's `default_*` prop
when the controlled prop is absent, always passes a controlled value down, and intercepts the
primitive's `on_*_change` callback. Where child components also need to see the state, the
wrapper provides its own context alongside the primitive's.

This is forced by two facts. daisyUI hides `.modal` and `.dropdown-content` unless a modifier
class is present, and matches no ARIA or `data-*` attribute that the primitives set, so the
class must come from Rust (ADR-0002 rules out CSS). And the primitives' contexts are private:
`DropdownMenuContext` and `SelectContext` are not `pub`, so a wrapper cannot read the state
from inside. `DialogCtx` is public, but the class belongs on `DialogRoot`'s own element, which
cannot consume the context it provides.

Applies to: dialog (`open` → `modal-open`), dropdown menu (`open` → `dropdown-open`), select
(`open` → `dropdown-open`, and `value` → `menu-active` on the selected option, since `.menu`
matches no ARIA attribute at all), and drawer (`open` → an inert checkbox's checked state, because
there is no transient-open class to emit). ADR-0024 records that projection exception.

## Consequences

- §6b lists controlled/uncontrolled plumbing as a reason to keep a primitive; the registry does
  a slice of it anyway. This is accepted because it is the one piece of primitive behaviour
  that is cheap to get right, unlike focus trapping, typeahead, or dismissal layers.
- Exit animations work, because the modifier class is removed while the element is still
  mounted, which is what the primitives' animation-aware unmounting waits on.
- Lifted state must stay in sync with the primitive's; the wrapper is the only writer.
- The tooltip and the alert dialog were added later and lift their open state the same way, for
  the same reason: `.tooltip-content` and `.modal` are both hidden until a modifier class says
  otherwise. The tooltip is the one case where daisyUI would have revealed the element anyway
  for a pointer or a keyboard, but not for a caller holding it open, which is what the lift is
  there for.
- Where a primitive offers no controlled prop to lift, the wrapper mirrors instead. ADR-0011
  records that shape and the accordion is the one component in it.
