# Field

Styled Compound parts over `dioxus-field` for connecting one form control to its Binding, Field
metadata, label, description, errors, and focus behaviour.

[Live examples](https://daisyui-components.dioxus.cc/components/field) ·
[their sources](docs/examples/)

`Field` accepts a `FieldContext`, `Binding`, or `Signal` and provides the resulting context to its
children. A field-aware control such as `Input` or `Checkbox` resolves its Binding and metadata
from that context. `FieldLabel` resolves its native `for` from the metadata's control id and
registers its own generated id for `aria-labelledby`; it deliberately has no `html_for` prop.
`FieldDescription` and `FieldError` also generate stable ids by default. Callers can still supply
explicit ids when another element must reference those parts directly. The convention generates a
stable fallback control id when metadata declares none.

Errors are already formatted by the metadata producer before they reach this Component.
`FieldError` takes no children. Its polite live region stays mounted so later errors are announced
reliably; while invalid it renders one element per producer error, and while valid it stays empty.
Its id joins `aria-describedby` while invalid, whose support is consistent across control roles.
On controls whose declared surface supports validity, the first error id is also the control's
`aria-errormessage`.

`FieldRow` is the row a checkable control shares with its label:

```rust
Field { context,
    FieldRow {
        Switch {}
        FieldLabel { "Product updates" }
    }
    FieldDescription { "A short email when a release ships." }
    FieldError {}
}
```

A checkbox or a switch reads as its label's companion rather than as a control stacked under a
caption, and neither can be nested inside `FieldLabel`, because the Primitive renders a `button`
and a `label` does not activate one. The part is plain layout: it resolves nothing, so metadata
still reaches the control and the label from the surrounding `Field`. Controls that fill a line of
their own, such as `Input` or `Textarea`, keep their label above them and need no row.

The row's default emits no `justify-*`, so the settings-row arrangement is a caller utility on top
of it rather than a second layout Axis. Put the label first and push the control to the row's end:

```rust
FieldRow { class: "justify-between",
    FieldLabel { class: "whitespace-normal", "Product updates" }
    Switch {}
}
```

Sibling Fields of the same width line their controls up in a column this way, because each control
sits at its own Field's end. Fields of differing widths do not, and aligning across them is a grid
the page owns rather than something a row can see. The added `whitespace-normal` is worth having
whenever a label may be long: daisyUI's `label` class sets `white-space: nowrap` for short
captions, and a pushed-apart row gives a long one room to overflow instead of wrap.

The order of the two children is the caller's, and it is DOM order rather than paint: reversing it
with `flex-row-reverse` or `order` would leave the label and the control read in one sequence and
seen in another. `SwitchField` and `CheckboxField` render the control first and expose no
arrangement prop, because Composition sugar covers the happy path and a different arrangement is
the signal to compose these parts directly.

`FieldDescription` registers its resolved id for `aria-describedby` for its mounted lifetime.
It is supporting prose rather than a short control caption, so it wraps by default and can shrink
inside a constrained grid track. A long description therefore does not widen a full-width control
or overlap an adjacent Field.

## State bridging

There is **no state to bridge** to a daisyUI selector in these wrappers, so neither Tier applies.
The `dioxus-field` parts project producer-defined required, disabled, invalid, touched, and dirty
metadata into `data-*` attributes. They also own `for`, `aria-describedby`, `aria-errormessage`,
`aria-live`, and error visibility. The Registry does not recompute or mirror those facts.

Field-aware controls bridge their own invalid paint separately. For example, an `Input` with no
explicit colour emits `input-error` from Field invalidity as Tier 2 State bridging. That control
behaviour is not repeated by `Field`, whose caller content may be any control.

## Axes

- `appearance: FieldAppearance` on `Field` - `grid min-w-0 gap-2`, or no layout utilities.
- `appearance: FieldRowAppearance` on `FieldRow` - `flex min-w-0 items-center gap-2`, or no layout
  utilities.
- `appearance: FieldDescriptionAppearance` on `FieldDescription` - `min-w-0 whitespace-normal`,
  or no wrapping utilities.
- `appearance: FieldErrorAppearance` on `FieldError` - `text-error`, or no colour utility.

The default arms emit Tailwind utilities because daisyUI has no independently usable class for
these jobs. Every `None` value emits nothing, so callers replace the defaults without competing
with another utility at equal specificity and cascade layer (ADR-0004). Each enum exposes `ALL`;
the Preview renders every value and the browser specs compare their computed styles.

`FieldLabel` and `FieldDescription` always emit daisyUI's `label` class. The description's default
appearance adds `min-w-0 whitespace-normal`, overriding the class's short-caption `white-space:
nowrap` default so explanatory prose wraps. Its `None` appearance emits neither utility, allowing a
caller to replace both without a cascade-order tie. Caller classes concatenate with every Compound
part's own classes, and other caller attributes override them.

Cargo-library consumers must expose `src/components` at a stable path to the consuming Tailwind
build; the root README documents the required `@source` configuration.

## Deviations

**The label class is intentionally duplicated.** The standalone Label Component already emits
`label`, but it requires `html_for`, and the in-Field association has a different source:
`FieldLabel` repeats the complete class literal locally while `dioxus-field::Label` derives `for`
from `FieldMeta::id`. A dependency is for composing a published Component, not for sharing one
class literal, so the Component remains independently installable.

**Descriptions undo the label class's no-wrap rule.** daisyUI's `label` class is designed for short
captions and sets `white-space: nowrap`. `FieldDescription` can contain normal explanatory prose,
so its default adds shrink-safe wrapping utilities. `Field` is also shrinkable under its default
layout, preventing either part from contributing an oversized minimum to a parent grid track.

**There is no generic collapsed Field.** Caller content belongs between the label, control,
description, and error Compound parts. Field-aware controls may offer closed per-control
Composition sugar such as `InputField`; those functions compose this Component and their existing
control without adding children or duplicating either implementation. Anything beyond their props
drops back to these parts.

**Explicit ids stay at the producer boundary.** Field metadata can declare the control id, and
callers can declare label, description, and error ids when another element must reference them.
The convention otherwise supplies stable fallbacks so references never dangle; an explicit id
still wins.

## daisyUI classes deliberately not used

- `validator` - pinned daisyUI can paint invalidity from `aria-invalid` as well as browser
  `:user-invalid`, but this Component cannot add a class to the caller-owned control. Field-aware
  controls already emit their own explicit error modifier, and adding validator paint here would
  duplicate that control-owned Axis while also opting into browser-owned valid paint.
- `validator-hint` - its visibility depends on being a sibling after an element carrying
  `validator`. `FieldError` instead works with any field-aware control, registers
  `aria-errormessage`, renders a polite live region, and follows producer metadata directly. Using
  the class would add adjacency and control-class coupling while duplicating visibility the
  headless part already owns.
- `fieldset`, `fieldset-legend` - a Field connects one control to metadata; it is not a native group
  of controls. The separate Fieldset Component owns that markup.
- `fieldset-label` - deprecated by the pinned daisyUI CSS in favour of the `label` class this
  Component uses.
- `input`, `checkbox`, and other control classes - controls remain caller content and own their own
  daisyUI classes and Axes.
