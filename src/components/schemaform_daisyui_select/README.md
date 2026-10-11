# Schemaform DaisyUI Select

A `schemaform_daisyui` add-on that registers the `daisyui:select` widget
symbol: a UI schema control naming it renders its choice as the registry's
compound `Select`, a trigger and a dropdown listbox, rather than the native
select `schemaform_daisyui` renders by default.

[Live examples](https://daisyui-components.dioxus.cc/components/schemaform_daisyui_select) ·
[their sources](docs/examples/)

Install it beside `schemaform_daisyui` only when a UI schema names the symbol;
the compound select is the heavier widget (ADR-0032), so a form that never asks
for it should not carry it. Without it, such a UI schema fails to bind with the
adapter's missing widget finding, which `SchemaformDaisyui` raises as a render
error. With it, add `SelectAddon` to `SchemaformDaisyui`'s `addons`, or apply
`Addon::controls` to a registry composed by hand. `SELECT_WIDGET` is the
symbol.

## What it renders

Every option, the null option of a nullable choice included, is a
`SelectOption` over the option's opaque `ChoiceIdentity` whose text is the
localized label; the trigger shows that text for the selected option and the
placeholder otherwise. A write-only control shows its replacement placeholder.
The trigger carries the control's element id and name.

The control sits in `schemaform_daisyui`'s frame — label, findings, error
region, presence affordances — at the form's density and with its button
classes, since the add-on's renderer is the base renderer with the choice
widget swapped. A widget symbol on a non-choice control changes nothing: that
control renders as the base renders it. `SelectControl` is the control as a
component, for a host whose own control renderer presents a choice this way.

## State bridging

The component has no state to bridge. It uses `schemaform_daisyui`'s
`use_choice_binding`, the same `Binding<Option<ChoiceIdentity>>` the native
select speaks.

## Deviations

- **No native form participation.** The registry's compound `Select` renders no
  hidden form participant, so a `daisyui:select` choice carries the control
  binding as the trigger's `name` but does not take part in native form
  submission. The native select and the radio group's hidden radios do.
- **Late touched state.** The compound `Select` reports focus exit one task
  after the trigger loses focus, so touched state lands one task later than
  with the native select.
- **It is an add-on.** It builds on `schemaform_daisyui`'s control frame,
  which the base shares as crate-visible items rather than public API
  (ADR-0035).

## daisyUI classes deliberately not used

None beyond the registry `Select`'s own: the control is that component inside
the base's field frame.
