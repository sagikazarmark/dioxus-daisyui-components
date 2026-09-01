# Adornment slots switch the input's own structure

daisyUI 5 draws an adorned text field as a wrapper carrying the `input` class around the real
`input` and its neighbouring spans, with the box — width, border, join radii, `:focus-within`
styling — on the wrapper. The Registry had no way to spell that shape: nesting `Input` inside a
hand-written `.input` wrapper emits a second `input` class and doubles the box, so callers
reinvented it with `join`, differently each time. `Input` now takes `prefix` and `suffix`
`Element` props, forwarded by `InputField`; when either is set the Component emits
`span.input > (slot span, input, slot span)` instead of the bare element. The caller-facing
contract lives in the Component README; this ADR records the pattern and the mechanisms.

Scoped precisely, two firsts: these are the Registry's first plain `Element` slot props on a
control (`items: Vec<Element>` and `render_toast` exist elsewhere), and its first prop-presence
switch of a control's *own* emitted structure (`description: Option<String>` on the sugar already
toggles a separate part).

## Slot props rather than compound parts

An `InputAdornment` part family would move the Binding, metadata, and focus resolution that today
live in one component into a context shared across parts, for content that has no behaviour at
all: adornments are inert by contract (the README records why an interactive one would mis-report
Focus Exit). Closed `Element` props keep the resolution in one place and keep the surface closed.
This is not the "slots" CONTEXT.md lists under *Avoid* for Compound parts — that entry names an
alternative vocabulary for parts, and the Legal-collapse test still passes here because no caller
markup sits between Registry elements: the slots are inside the control's own box, and their
content is handed over whole.

## Adornedness is presence, and presence is render-stable

Adorned means `prefix.is_some() || suffix.is_some()`: an empty `Some(rsx! {})` selects the
wrapper arm deliberately, and an implementer must not "optimize" it back to the bare arm. The
empty stable slot is the supported shape for conditional adornments, because flipping the option
flips the tree shape and remounts the native input: caret, IME, and selection state are lost, the
mounted-control handle goes stale until the next `onmounted`, and — since browsers fire no
`focusout` when the focused node is removed — that session's Focus Exit is silently never
reported. Both slot spans render whenever the arm is adorned, so any content change within `Some`
diffs in place; a conformance case pins that no remount occurs, alongside the adorned
exit-ordering case. The slot spans carry `empty:hidden` so an empty slot does not hold open
daisyUI's `.5rem` flex gap; the utility only matches a slot with nothing in it, so rendering
content is what defeats it (the axis-that-emits-nothing shape of ADR-0004 has nothing left to
switch off).

## The focus-forward mitigation

The Focus Exit reporter is the native input's sync `onfocusout`, and it ignores `relatedTarget`,
so a click on the wrapper's padding or an adornment would blur the input and fire Focus Exit
mid-edit. `MouseData` in dioxus-html 0.7.10 exposes no event-target accessor, so a single wrapper
handler cannot tell a padding click from one bubbling out of the input; the mitigation is
therefore three-part:

- the registry-emitted slot spans prevent the default on `mousedown`;
- the inner input's own `mousedown` stops propagation, so the wrapper handler only ever sees
  padding (and slot) clicks;
- the wrapper's `mousedown` prevents the default and requests focus on the native input through
  the same mounted handle the Field focus request uses.

Two recorded costs: the input's `mousedown` becomes invisible to callers' Dioxus ancestors, and
adornment text is unselectable — with the wrapper's `cursor: text` over it a known cosmetic
dishonesty. The conformance harness dispatches events directly at the control and moves no real
focus, so it cannot observe any of this by construction; the mitigation is covered by a browser
spec (type, click the suffix and the padding, assert the Focus Exit counter holds and the caret
survives).

## A second attributes surface

The wrapper owns the box, so the class split owes the caller a way to reach it (the Select
README's rule); box-scoped utilities on the transparent inner input are silent no-ops. The
surface is `wrapper_attributes: Vec<Attribute>` — a full attributes list rather than a
`wrapper_class: Option<String>`, because widening a string prop later is a breaking change to
copied source. It is the Registry's first *second* attributes surface on one component. The
Select precedent is narrower: `SelectListSize` is a typed axis the split owed, one class on one
element; the input's wrapper is a general box, so it is owed a general surface. Dioxus permits
one `#[props(extends)]` sink per component, so the prop cannot auto-extend; the documented shape
is the explicit `attributes!(span { .. })` invocation, its `class` concatenating with the
wrapper's own exactly as caller `class` does on the input, and the prop carries a `///` comment
(the policy test exempts only `attributes` and `children`).

## Consequences

- Axis classes and the derived `input-error` relocate to the wrapper when adorned, with the
  ADR-0005 precedent that a class goes on the element whose layout it drives; caller
  `attributes`, the Binding contract, and Field metadata stay on the native input, so no
  `dioxus-field` change was needed and the conformance level is unchanged.
- The conformance suite pins observable exit ordering rather than which element reports it. If an
  adornment ever becomes legitimately interactive, the recorded extension path is a
  wrapper-scoped containment exit — reporting from the wrapper's `focusout` when `relatedTarget`
  leaves the wrapper — not a relaxation of the ordering.
- `Textarea` inherits the identical wrapper pattern (`.textarea textarea`,
  `.textarea:focus-within` exist in the pinned CSS) as a follow-up; `Select`'s trigger is already
  the flex `.select` box and needs none of this.
- Floating labels do not compose with the wrapper (`.floating-label>span` captures it as the
  caption); the README records the combination as unsupported.
