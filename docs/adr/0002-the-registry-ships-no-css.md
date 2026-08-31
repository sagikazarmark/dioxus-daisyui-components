# The registry ships no CSS

Components emit daisyUI class names and Tailwind utilities for presentation. They ship no
stylesheet, in any form, ever.

Behavior-critical structural styles may be inline when no native attribute expresses the required
behavior and relying on generated utilities could expose an otherwise hidden form participant.
Such styles carry no theme or design values. The Switch's registry-owned native checkbox follows
the pinned Primitive's zero-sized inline hiding so its duplicate form machinery cannot become
visible before or without Tailwind discovery.

The original design allowed a per-component "shim" stylesheet as a last resort for state that
daisyUI expresses in CSS but the primitives expose only as `data-*` attributes. That escape
hatch does not work: `dx` invokes the tailwindcss binary with a single input
(`<crate-root>/tailwind.css`), so a stylesheet colocated with a component is never processed
and its `@apply` never expands. Every workaround either adds a per-component manual setup step
or hardcodes values daisyUI owns.

Where a primitive's state cannot reach daisyUI's selectors, the resolution is Rust-side: lift
the state into the styled wrapper so the daisyUI modifier class can be emitted from Rust, or
drop the daisyUI *component* class for that element and style it with utilities.

## Consequences

- Installing a component requires no setup beyond a Tailwind rebuild, unconditionally.
- Some daisyUI component classes are deliberately unused where their CSS assumes a DOM shape
  the primitive does not produce. Each such omission is recorded in the component's `README.md`.
- A styled wrapper may own controlled/uncontrolled state plumbing that would otherwise be left
  entirely to the primitive, in order to see the state it needs to emit a modifier class.
