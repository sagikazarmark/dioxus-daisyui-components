# Schemaform DaisyUI

A `schemaform-dioxus` renderer package that presents a form with the
`dioxus-daisyui-components` registry and daisyUI classes: a control renderer for
every control kind, a structure bundle for homogeneous arrays and the form
shell, and a finding presenter for the summary and node-local findings.

[Live examples](https://daisyui-components.dioxus.cc/components/schemaform_daisyui) ·
[their sources](docs/examples/)

It fills three seams:

- A **control renderer** for every control kind: strings, numbers, and integers
  with `Field`, `FieldLabel`, `Input`, and `FieldDescription`; booleans with a
  native checkbox or the registry's `Checkbox`; choices with `NativeSelect`,
  `RadioGroup`, or `Select`; multiple choices as native checkbox groups;
  constants as read-only output; and presence
  operations as its `Button`.
- A **structure bundle** with a `CollectionRenderer` that presents homogeneous
  arrays as a fieldset of item cards with joined square action buttons, and a
  `ShellRenderer` that lays the form out with a mode-aware submit button.
- A **finding presenter** that frames the form-wide summary as an alert of
  focus-to-target links and renders node-local findings as a stack.

`SchemaformDaisyui` binds a form through all three at once; `controls()`,
`structure()`, and `findings()` let a host compose them with its own slots.

Only the structural nodes no seam exists for yet — layouts, groups, and tabs —
still come from the adapter's built-in renderer. schemaform's demo styles those
through the adapter's `schemaform-*` class hooks with daisyUI classes in its own
stylesheet (`demo/src/forms.css`); that theme is the demo's, not this
component's, and shrinks as further structure seams ship.

## Supported targets

Browser CSR is supported; desktop/WebView support is pending manual verification.
Both targets share one code path. The `schemaform` and
`schemaform-dioxus` 0.4.1 requirements in the manifest and root `Cargo.toml`
include the platform-neutral DOM bridge from
[schemaform#29](https://github.com/sagikazarmark/schemaform/issues/29): rejected
writes resynchronise the DOM and focus moves through `document::eval`. The
registry's widgets use that same Dioxus API or `MountedData` on both targets.

Browser behavior is checked automatically. Desktop evidence is manual and
WebView-specific; the [desktop checklist](../../../docs/desktop-smoke.md)
includes this page's rejected-write resync, blocked-submission focus and array
mutation focus. Its first interactive run is pending. SSR and hydration remain
out of scope for this renderer.

The host renderer must provide a Dioxus `Document`. A web host that disables
`dioxus-web` default features must re-enable `document`; its Content-Security-Policy
must allow `unsafe-eval` for the bridge's focus and resync scripts. Focus settles
asynchronously over a few tasks rather than before the event handler returns.

## Layout

The directory is a `dx components` member like every other component here, only
larger, so the line between what an install copies and what stays behind is
worth drawing:

- `component.json` declares the registry components it is built on (`field`,
  `input`, `checkbox`, `native_select`, `radio_group`, `select`, `button`), each
  pinned to one revision of this registry so an install's parts share one
  contract: one `Binding` and one `FieldMetaValues`. Within the registry the
  package compiles against those components as they are in the tree; the pin
  governs what `dx components add` fetches. The Cargo entries name
  `dioxus-field` at the version every field-aware manifest here declares, the
  primitive at the revision they all pin, and the `schemaform` and
  `schemaform-dioxus` release that ships the headless edit hooks and the
  structure seams; the registry's own `Cargo.toml` requires the same.
- `mod.rs`, `component.rs`, `appearance.rs`, `mapping.rs`, `parts.rs`,
  `text.rs`, `boolean.rs`, `choice.rs`, `multiple_choice.rs`, `constant.rs`, `collection.rs`,
  `shell.rs`, and `findings.rs` are what an install copies. None of them
  contains test code. `docs/` holds the page's examples and, with this file and
  the manifest, is excluded from installs.
- The tests are split the way the registry splits every form control's. The
  binding tests in `tests/conformance/schemaform_daisyui.rs` drive the
  `dioxus-field` bindings through a capturing renderer without rendering
  markup. Everything observable in markup is a Playwright spec against the
  page's examples, `tests/browser/schemaform_daisyui.spec.ts`; the registry
  accepts no render-to-string tests (ADR-0007). The adapter's own contract
  tests stayed with schemaform, in its demo's `tests/schemaform_daisyui/`,
  where they exercise `schemaform-dioxus` through that copy of this package as
  a real consumer.
- CI installs every component of the registry, this one included, into a fresh
  application with `dx components add` and checks that it compiles, so the
  manifest's pins and crate requirements are exercised on every change.

## What it renders

| Control kind | Widget |
| --- | --- |
| string, number, integer | `Input` (`type="password"` when write-only) |
| string with `format` | `email` / `idn-email` → `type="email"`; `uri`, `uri-reference`, `iri`, `iri-reference` → `url`; `date` → `date`; `date-time` → `datetime-local`; `time` → `time`; unknown or absent formats → `text`. Write-only takes precedence; numbers and integers stay `text`. |
| boolean, not nullable | native `input type="checkbox"` with daisyUI's `checkbox` class, driven by `use_boolean_edit` |
| boolean, nullable | registry `Checkbox`; JSON null is the indeterminate state |
| boolean, write-only | `NativeSelect<bool>` over the localized false/true labels, resting on the replacement placeholder |
| choice | `NativeSelect<ChoiceIdentity>` by default |
| choice with `"widget": "daisyui:radio"` | `RadioGroup` with one `RadioItem` per option |
| choice with `"widget": "daisyui:select"` | the compound `Select` with one `SelectOption` per option |
| multiple choice (`uniqueItems` array of finite choices) | a `Field` containing a `fieldset` and one native `checkbox` per option, driven by `use_multiple_choice_edit` |
| constant | read-only `output` from presentation and facets |
| any read-only node | read-only `output` inside the same `Field` |

The null option of a nullable choice is an ordinary option in all three choice
widgets; selecting it sets null. A non-nullable boolean keeps the built-in's
native semantics on purpose: a native checkbox is what browsers, assistive
technology, and the existing tests expect. The nullable checkbox reaches null
through the set-null presence affordance, since a click from either boolean
state yields a boolean; the `CheckboxState` mapping lives in this component
only, never in the published crates.

Multiple-choice checkboxes carry `MultipleChoiceEdit::option_element_id`, so rejected
toggles resynchronise the correct native element. Membership always follows the node;
the first toggle can create an absent array. Array presence actions and the incompatible
member readout remain available. Read-only arrays show selected labels as output;
write-only arrays show the adapter's status with disabled, unchecked options.
Required array presence is indicated by `(required)` in the group legend, never
by `required` or `aria-required` on individual options. A required array may be
empty; `minItems` independently constrains how many options must be selected.
`format` is presentation-only: schemaform does not validate the annotation.

Write-only booleans and choices never echo their value: the widget rests on the
replacement placeholder, the edit hook puts it back there after every write,
and the label is the localized replacement label. A constant is never an
editable widget with a metadata-only field context; it is output, with the
presence affordances that materialize or remove it.

Every editable control shares one frame: the registry `Field` over the
control's binding and metadata, the label (above the widget, or beside a
checkable one in a `FieldRow`), the widget, the incompatible-value readout, and
the supplements under it — help, the findings that are not field errors, the
error region, and the presence affordances. The readout is
`NodePresentation::incompatible_value`, shown for every kind: an input holding
a number where the schema wants a string, a checkbox holding a string, and a
select holding a value no option carries all tell the user what the replace
affordance beside them discards. A widget symbol on a non-choice control
changes nothing: the renderer dispatches on the control kind first.

## What it renders around the controls

| Node | Presentation |
| --- | --- |
| homogeneous array | `fieldset` carrying the adapter's element id, focusable for the container presence operations; `fieldset-legend`; help; the incompatible-value readout and container presence `Button`s; a dashed empty state while there are no items; the adapter-keyed item hosts in a grid; the append affordance as an outline `Button`; the adapter's live region; the local findings |
| array item | `card card-border card-sm` with `role="group"`, labelled by a title reading the item noun and its position; the insert, move up, move down, and remove affordances as a `join` of `btn-square` icon buttons whose `aria-label` is the affordance's positional accessible name and whose `title` is its label; the item's controls follow |
| form shell, gated mode | a `grid gap-4` of the adapter's summary region, the body, and the submit affordance as `btn btn-primary` of `type="submit"` |
| form shell, advisory mode | `btn btn-warning` with the adapter's localized affordance label; delivers data and findings through `on_advisory_submit`, without blocking or moving focus |
| finding summary | nothing while empty; otherwise `alert alert-soft`, `alert-error` while any finding blocks submission and `alert-warning` otherwise, with one `link` button per finding that reveals and focuses its target |
| node-local findings | one `p` per finding in `text-error` or `text-warning` |

In advisory mode, the summary and local findings use `alert-warning` and
`text-warning`, including findings that would block gated submission. Field findings
are descriptions instead of error-region entries. The adapter's validation facts
(`aria-invalid` and `data-blocking`) remain intact. `DaisyuiShell` provides a reactive
`AdvisoryPresentation` context from the submit affordance, so mode changes update
both presenters and Field supplements. A host composing these presenters with its
own shell can provide `AdvisoryPresentation(ReadSignal<bool>)` around its body and
summary; without it the presenters use gated presentation.

Every button carries its affordance id, so focus after a move lands on the same
button in the moved row; every finding element carries its stable id, so ids the
adapter hands out in `aria-describedby` resolve. Identity, keying, focus after a
mutation, and announcements stay the adapter's: the renderers place what they
are handed and never compose affordances themselves.

A built-in fixed object rendered as an array item keeps whatever frame the
host's theme gives `.schemaform-group`; the collection knows nothing of it. A
theme that frames groups is the one to flatten them inside a card, as the
demo's `forms.css` does under `[data-schemaform-daisyui="collection-item"]`.

## What it maps

`schemaform-dioxus` owns the correctness-critical editing behaviour through
`use_text_edit`, `use_boolean_edit`, `use_choice_edit`, and `use_multiple_choice_edit`; this component only
maps those onto the `dioxus-field` convention the registry speaks:

| schemaform-dioxus | dioxus-field |
| --- | --- |
| `TextEdit::value` / `input` / `blur` | `Binding<String>` read / write / focus exit |
| `BooleanEdit::checked` / `set` / `blur` | `Binding<Option<bool>>` read / write / focus exit |
| `BooleanEdit` with null ↔ `CheckboxState::Indeterminate` | `Binding<CheckboxState>` |
| `ChoiceEdit::selected` / `select` / `blur` | `Binding<Option<ChoiceIdentity>>` read / write / focus exit |
| `ChoiceEdit` with the identity as its `as_str` form | `Binding<String>` for `RadioGroup` |
| `MultipleChoiceEdit::selected` / `toggle` / `blur` | native checkbox membership / input / blur, with a metadata-only `Field` |
| — | `Binding::commit` is a no-op for every kind: the core applies each edit as it happens |
| `NodePresentation::element_id` / `ControlFacets::name` | `FieldMetaValues::id` / `name` |
| `ControlFacets::{required, disabled, touched, dirty}` | the same flags |
| `NodePresentation::invalid` | `FieldMetaValues::invalid = Some(..)` |
| blocking parse, validation, and external findings | `FieldMetaValues::errors`, and the error region |
| `NodeProjection::display_text()` | the text of a read-only or constant `output` |

Every binding's identity is its edit's hook-stable handles (the value signal
and the callbacks), so bindings built on later renders compare equal and the
registry widgets neither re-render per edit nor re-register focus. A native
`option`'s value string (`NativeSelectOption::form_value`), and a
`RadioItem`'s value, is the option's opaque `ChoiceIdentity` string: the same
string the edit hook writes back into the widget after a rejected write.

Findings are split the way the registry's `Field` addresses them. The findings
that block submission and concern the entered value — parse blockers,
validation findings, blocking external findings — are field errors: their text
goes into `FieldMetaValues::errors`, and they are rendered in the **error
region**, the element the control references through `aria-errormessage`, a
polite live region under the id `{element_id}-errors`. The component renders
that region itself rather than through the registry's `FieldError`, registering
the same id with the field's metadata, so that each finding inside it is an
element carrying the adapter's stable id. Every other finding — help text,
capability findings, advisory external findings — is a `FieldDescription`
carrying its stable id, which the control references through
`aria-describedby`. A blocking capability finding therefore still marks the
field invalid but is read as a description, since it is about what the form can
present rather than what the user typed.

Two registry behaviours to know about: the compound `Select` and the
`RadioGroup` report focus exit one task after they lose focus, so touched state
lands one task later than with the native select; and the compound `Select`
renders no hidden form participant, so a `daisyui:select` choice carries the
control binding as the trigger's `name` but does not take part in native form
submission. The native select and the radio group's hidden radios do.

## State bridging

The component itself has no state to bridge: the core owns every piece of form
state, and nothing here holds any of its own. The mapping module turns the
adapter's edit hooks into controlled `dioxus-field` bindings — read is the
hook's value signal, write is the hook's callback — that the registry widgets
consume as they would any host binding. The widgets never see form data, and a
write the core rejects is undone by the hook resynchronising the widget's DOM
state, not by state kept in this component.

## Axes

- `appearance: Appearance`: `Default` emits the Tailwind utilities the package
  lays itself out with (gaps, borders, widths, the semantic text colours);
  `None` emits none of them. The daisyUI component classes — `fieldset`,
  `card`, `btn`, `join`, `alert`, `link`, `input`, `checkbox`, `select` — and
  the `sr-only` that keeps a hidden label accessible render under both values,
  since a caller's utilities override component classes cleanly and only the
  package's own utilities would tie with theirs. The same elements, ids, and
  markers render under both values.

The axis is fixed when a form is bound, like the renderers: it is a prop of
`SchemaformDaisyui`, an argument of `configuration_with`, `controls_with`,
`structure_with`, and `findings_with`, and a builder method on
`DaisyuiControlRenderer`, `DaisyuiCollection`, `DaisyuiShell`, and
`DaisyuiFindings`. `Appearance::ALL` lists both values. The registry parts the
controls compose keep their own appearance axes at their defaults.

## Usage

The whole presentation, bound once when the component mounts:

```rust
use crate::components::schemaform_daisyui::SchemaformDaisyui;

rsx! {
    SchemaformDaisyui {
        form,
        on_submit: move |snapshot| { /* ... */ },
        on_error: move |error| { /* ... */ },
    }
}
```

For advisory submission, pass `submission_mode: SubmissionMode::Advisory` and
`on_advisory_submit`. That callback receives `schemaform::AdvisorySubmission`, never
a validated `SubmissionSnapshot`; the required `on_submit` callback is unused in
advisory mode. Both props forward to `SchemaForm`, and the mode can change after mount.

The seams composed by hand, when one of them should differ:

```rust
use schemaform_dioxus::RenderConfiguration;

use crate::components::schemaform_daisyui;

let bound = RenderConfiguration::builder()
    .controls(schemaform_daisyui::controls())
    .structure(schemaform_daisyui::structure())
    .summary_presenter(schemaform_daisyui::findings())
    .local_presenter(schemaform_daisyui::findings())
    .build()
    .bind(&form)?;
```

`schemaform_daisyui::configuration()` is that composition, ready to bind. The
seams are independent: a host that keeps the built-in controls can still adopt
`structure()` and the summary presenter, as the demo's other gallery pages do.
Structure renderers are fixed when a form is bound; changing the bundle means
rebinding.

`controls()` starts from `ControlRegistry::with_builtins()`, registers the
renderer through a matcher above the built-in priority that accepts every
definition node the adapter derives a control kind from, and registers one
renderer each for the exact widget symbols `daisyui:radio` and `daisyui:select`
(`RADIO_WIDGET` and `SELECT_WIDGET`). `structure()` is
`StructureRenderers::default()` with the collection and shell slots replaced,
so any slot this component does not implement stays the built-in. `findings()`
is one presenter for both presenter slots; it tells the summary from a local
collection by `FindingCollectionContext::is_summary`. The widget symbols are
named on a UI schema control:

```json
{
  "type": "control",
  "value": {
    "binding": { "origin": "root", "pointer": "/billing_cycle" },
    "widget": "daisyui:radio"
  }
}
```

The control dispatcher handles every kind in schemaform-dioxus 0.4.1 explicitly.
Its upstream `ControlKind` is still `#[non_exhaustive]`, so stable Rust requires
an unknown-kind guard: that guard fails visibly instead of silently rendering an
unstyled built-in. Compile-time exhaustiveness across future kinds requires an
upstream API change.

## Deviations

Where this component departs from the registry's conventions, and why:

- **It is a renderer package, not one widget.** It exports trait
  implementations and factory functions alongside its `#[component]`, and it
  spans several files (`mapping.rs`, `parts.rs`, one file per control kind, the
  collection, the shell, the presenter) rather than one `component.rs`.
- **Fixed prose.** The empty state's text, "Nothing here yet.", is a fixed
  English string: the localized item noun the context carries is a name, not a
  word a sentence can be built around. An item card's title is the item noun
  followed by its position (`Tags item 2`), a fixed word order.
  The multiple-choice legend's `(required)` indicator is also fixed English;
  the adapter currently supplies no localized required-field indicator.
- **Its own ids.** Beside the ids the adapter reserves, the component derives
  `{element_id}-label`, `{element_id}-errors`, `{element_id}-incompatible`,
  `{element_id}-legend`, and `{row_id}-title` from the adapter's ids, and a
  radio item's id from the control's element id and the option's identity.
  Multiple-choice option ids come directly from the edit hook.
- **Error colour contrast is the theme's.** Findings, the error region, and the
  summary use daisyUI's semantic `text-error`, `text-warning`, `alert-error`,
  and `alert-warning`, as the registry's `FieldError` does. On daisyUI's stock
  light theme, error and warning text do not reach WCAG AA contrast; the
  Preview overrides `--color-error`, `--color-warning` and their content colours
  on this Component page under the light theme. Axe audits each new Example's
  rendered form in light and dark, including submitted advisory findings. The
  component ships no colour of its own (ADR-0002), so a host that wants AA on
  the stock theme overrides the same variable, or renders findings through a
  presenter of its own.

## daisyUI classes deliberately not used

- `validator` and `validator-hint`: they style a control from the browser's
  constraint-validation pseudo-classes. Validity here comes from the core's
  findings, not from native constraints, so they would disagree with
  `aria-invalid` and duplicate the error region.
- `tooltip` on the item action buttons: their full name is already exposed
  through `aria-label`, and the visible label through `title`; a tooltip would
  add a third, hover-only copy.
- `badge` or `status` for findings: findings are text the control references
  through `aria-describedby` and `aria-errormessage`; a badge would not be
  read as the description of anything.
