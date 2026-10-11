# Schemaform DaisyUI Radio

A `schemaform_daisyui` add-on that registers the `daisyui:radio` widget symbol:
a UI schema control naming it renders its choice as the registry's
`RadioGroup`, one `RadioItem` per option.

[Live examples](https://daisyui-components.dioxus.cc/components/schemaform_daisyui_radio) ·
[their sources](docs/examples/)

Install it beside `schemaform_daisyui` only when a UI schema names the symbol.
Without it, such a UI schema fails to bind with the adapter's missing widget
finding, which `SchemaformDaisyui` raises as a render error. With it, add
`RadioAddon` to `SchemaformDaisyui`'s `addons`, or apply `Addon::controls` to a
registry composed by hand. `RADIO_WIDGET` is the symbol:

```json
{
  "type": "control",
  "value": {
    "binding": { "origin": "root", "pointer": "/billing_cycle" },
    "widget": "daisyui:radio"
  }
}
```

## What it renders

Every option, the null option of a nullable choice included, is a `RadioItem`
whose value is the option's opaque `ChoiceIdentity` string, labelled by the
text beside it; selecting the null option sets null. The group root carries the
control's element id and ARIA state; the hidden form participants the registry
renders carry its name, so the radio group takes part in native form
submission. Nothing is checked while nothing is selected, which is also a
write-only control's resting state after every write.

The control sits in `schemaform_daisyui`'s frame — label, findings, error
region, presence affordances — at the form's density and with its button
classes, since the add-on's renderer is the base renderer with the choice
widget swapped. A widget symbol on a non-choice control changes nothing: that
control renders as the base renders it. `RadioGroupControl` is the control as
a component, for a host whose own control renderer presents a choice this way.

The registry's `RadioGroup` reports focus exit one task after it loses focus,
so touched state lands one task later than with the native select.

## State bridging

The component has no state to bridge. `use_radio_binding` maps
`schemaform-dioxus`'s `ChoiceEdit` onto the `Binding<String>` the registry's
`RadioGroup` speaks: the selected option's identity string, or the empty string
while nothing is selected; a write selects the option carrying the string.

## Deviations

- **Its own ids.** A radio item's id is derived from the control's element id
  and the option's identity, and its label's from the item's.
- **It is an add-on.** It builds on `schemaform_daisyui`'s control frame and
  mapping, which the base shares as crate-visible items rather than
  public API (ADR-0035).

## daisyUI classes deliberately not used

- `join` around the items: the options are a group of labelled radios, laid
  out as rows; a join would fuse them into one segmented control without the
  labels beside them.
