# Presentational components belong in the registry

The Registry includes daisyUI components that carry no behaviour and therefore wrap no Primitive.
They are **Presentational components**: installable APIs for daisyUI's classes and markup, with no
state, focus handling, keyboard interaction or ARIA wiring of their own.

Stopping at Primitive-backed behaviour would make a small coherent registry, but not the most
useful one. The copy-in model also solves a visual integration problem: it puts complete daisyUI
class literals where Tailwind scans them, exposes daisyUI's styling axes as typed props, preserves
the Registry's attribute-merging convention, and gives repeated daisyUI structures one documented
Rust API. Those benefits still exist when there is no behaviour to bridge. Admitting this category
does not commit the Registry to wrapping every daisyUI class; Components are still added when an
application needs them, and individual candidates may be rejected on their own merits.

This is not the exception ADR-0019 records. A **Native control** wraps no Primitive because the
browser supplies the behaviour daisyUI expects on a real form element. A Presentational component
has no substitute behaviour provider: it has no behaviour at all. A component that implements
state, focus or keyboard interaction in Registry code is neither category and does not follow from
this decision.

## Structure follows daisyUI

Primitive-backed Compound parts mirror the Primitive's composition because those boundaries carry
behaviour. With no Primitive tree to preserve, a Presentational component's Compound parts instead
mirror daisyUI's documented markup. A part boundary is justified where daisyUI gives an element
its own structural class or role and the caller needs to supply that element's content or
attributes. An internal wrapper that offers neither is rendered by the nearest part rather than
exposed merely because it exists in an example.

This makes daisyUI's markup, not an imagined behaviour abstraction, the authority for components
such as cards, stats, steps and timelines. It also means their documentation must explain any
departure from that markup just as a Primitive-backed Component explains a departure from its
Primitive.

## Axes and state bridging

Axes are unchanged. Each independent daisyUI styling dimension is a separate prop, each class is
a complete literal, and each axis enum exposes `ALL` for the Preview. Any axis that emits Tailwind
utilities follows ADR-0004 and has a value that emits nothing, so those utilities are Defeatable.

The `## State bridging` section in a Presentational component's `README.md` says **There is no state
to bridge because the component has no state; neither Tier applies.** This is the intended meaning
of the Registry policy test's allowance for the phrase `no state to bridge`, not a loophole
in the lint. A prop that chooses a class or writes a value is an Axis, not state bridging.

Presentational components continue to declare `dioxus-primitives` identically to every other
Component because the shared attribute-merging helper is part of the Component API convention.

## The acceptance test is narrower

Both acceptance seams still apply. Seam 1 installs and compiles the Component. At Seam 2, the
browser can assert computed styles for every Axis, caller attributes and classes surviving the
merge, and the page-level screenshot under each baseline theme.

There are deliberately no component-specific keyboard, focus or ARIA assertions: no behaviour
exists to drive. Computed styles prove that the class literals survived Tailwind and apply, while
the screenshot covers the complete daisyUI structure, but neither proves accessibility behaviour
the way a Primitive-backed Component's browser spec does. That reduction is accepted as a
consequence of the category rather than filled with tests of implementation details.

## Consequences

- The Registry covers useful daisyUI-only structures without pretending that they wrap behaviour.
- `CONTEXT.md` names Presentational component alongside Native control and distinguishes the two.
- Compound-part reviews for these Components cite daisyUI's markup rather than a Primitive tree.
- The badge, card, alert and list extractions tracked in issues #41 through #44 are Components and
  may proceed; their existing classes remain duplicated rather than becoming cross-Component
  dependencies.
- The presentational candidates tracked in issue #50 may be considered individually; this ADR puts
  the category in scope but does not accept every candidate automatically.
- The interactive candidates in issue #49 still need behaviour from a suitable Primitive or Native
  control. This decision does not permit the Registry to implement missing behaviour itself.
