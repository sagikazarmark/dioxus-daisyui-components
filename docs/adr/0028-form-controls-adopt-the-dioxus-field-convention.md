# Form controls adopt the dioxus-field convention

The Registry adopts the
[`dioxus-field`](https://github.com/sagikazarmark/dioxus-field) `0.6.0` convention for connecting
form-library-owned values and metadata to form controls. A field-shaped control becomes
**trio-conformant** by exposing the dependency-free `value` / `on_change` / `on_commit` prop trio
and preserving attribute spread. Where the additional integration pays for itself, it becomes
**field-aware** by resolving the Field Context through `use_binding`, `use_field_meta`, and
`use_focus_registration`, and reports logical Focus Exit through the resolved Binding. A control
that supports more than one Binding value type uses `FieldContext::try_resolve` in place of the
single-type `use_binding` hook while preserving the same precedence.

The two levels keep the lower boundary useful without forcing every control or consumer to depend
on the crate. The trio gives a Binding a common widget-facing shape: a reactive value, user writes,
and the control-defined end of an interaction unit. Field awareness adds resolution from explicit
props, Field Context, or standalone state and connects Field metadata, focus requests, and Focus
Exit. Commit and Focus Exit remain independent: Focus Exit means focus left the widget's complete
logical scope, including owned popup content, and never infers touched, blurred, validation, or
other form state. The crate's conformance probes test the same contract across Native controls and
Primitive-backed controls without making the Registry depend on a form library. Each control's
README declares its Conformance level rather than leaving consumers to infer it from the props or
dependencies.

## Controls consume the context directly

Field-aware controls resolve `dioxus_field::FieldContext` themselves. That remains the integration
mechanism: no wrapper owns a second binding, metadata, or focus path. The `field` Component provides
the styled Compound parts `Field`, `FieldLabel`, `FieldDescription`, and `FieldError` over the
crate's headless parts. The shared context type lives in a published external crate, so a control
and a producer do not need the `field` Component merely to integrate.

Per-control `XyzField` Components may additionally provide **Composition sugar**. Each is thin
`rsx!` over `Field`, its parts, and the existing field-aware control; it has no children slot and
forwards the control's props and attributes. Anything outside that closed prop surface is a signal
to use `Field` plus its Compound parts. This creates one convenient composition, not a second
control implementation or a second Field mechanism.

Because this sugar actually composes published Registry Components, its control Component declares
a revision-pinned dependency on `field`. Cross-Component dependencies remain inappropriate for
sharing class literals or implementation fragments; they are used here for public composition and
the pin keeps an installed wrapper and its parts on a known-compatible contract.

The existing `Label` Component does not change. Its required `html_for` remains the standalone
association contract. `FieldLabel` is the in-Field part whose association comes from Field
metadata; it repeats the applicable styling locally rather than depending on `Label` merely to
share a class literal, keeping the Field Component independently installable.

## Producer state is bridged, not computed

The Registry computes no validity. Field metadata carries producer-defined flags and pre-rendered
errors; controls and Field parts only project those facts into ARIA, `data-*` attributes, and
daisyUI paint. This extends ADR-0023's boundary between Registry presentation and application
behaviour to field integration. It does not test or weaken that boundary: deciding whether a value
is valid, deriving errors, and formatting their text remain outside the Registry.

Invalid paint follows explicit-wins precedence. An explicitly passed colour Axis always wins. When
the caller passes no colour, `meta.invalid()` emits the control's daisyUI error modifier such as
`input-error` or `checkbox-error` as Tier 2 State bridging. This mirrors the convention's own
explicit prop, then Field Context, then standalone-state resolution order while retaining the
caller's direct control over presentation.

The Registry continues not to use daisyUI's `validator` or `validator-hint` classes. The pinned
`validator` can paint invalidity from `aria-invalid` as well as browser-owned `:user-invalid`, but
field-aware controls already emit their own explicit error modifier from metadata. Adding
`validator` would duplicate that control-owned Axis and also opt into browser-owned valid paint.
`validator-hint` additionally requires a sibling after an element carrying `validator`, coupling
error presentation to a particular control and adjacency. `FieldError` instead follows producer
metadata for visibility, renders a polite live region, and registers an error id. Controls whose
surface supports validity expose its first id through `aria-errormessage`; every control includes
all mounted error ids in `aria-describedby` while invalid. Error-colour utilities use a Defeatable
utility Axis under ADR-0004. Affected Component READMEs use this rationale in their "daisyUI
classes deliberately not used" sections as those controls adopt the convention.

## Explicit ids remain at the boundary

The convention generates stable fallback ids for controls and Field parts so its ARIA references
never dangle. Producers may still supply `FieldMetaValues.id`, and Registry callers can supply
explicit label, description, and error ids when something outside the Field must reference those
elements. When both Field metadata and a control prop provide an id, the explicit control prop
wins. This follows the declared-id pattern already used by `Select`: an id that participates in
external coordination is a named prop with defined precedence, not an incidental value left to
attribute merge order.

## Consequences

- Form controls acquire one documented convention incrementally, beginning with trio conformance
  for `Input` and `Checkbox`; adopting this ADR changes no Component code by itself.
- Field-aware controls depend directly on `dioxus-field`, while trio-only controls retain the
  dependency-free prop contract.
- Closed `XyzField` Composition sugar may bundle a field-aware control with the published Field
  parts without becoming a second control implementation.
- A Component that provides this sugar pins its dependency on the Registry's `field` Component;
  ordinary Field-aware controls still need only `dioxus-field`.
- Explicit colour and id props remain authoritative; metadata fills their absence and supplies
  accessibility and state attributes.
- The Registry paints producer-owned invalidity and errors but never becomes their source.
