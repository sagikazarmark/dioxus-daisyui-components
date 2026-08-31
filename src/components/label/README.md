# Label

A caption naming a form control, styled with daisyUI's label classes, wrapping the
`dioxus-primitives` label.

```rust
Label { html_for: "collector", "Ship logs to the collector" }
Switch { id: "collector" }
```

`html_for` is required, because the association is the whole reason to reach for a component here
rather than write `label` by hand: clicking the caption moves focus to the control, and a screen
reader announces the control by the caption's text.

## State bridging

There is **no state to bridge**. The primitive renders one `label` element with a `for`
attribute, and nothing else (no ARIA state, no keyboard handling, no open state) so neither tier
applies. Everything this component does is put a class on that element.

The one case that looks like bridging is not. daisyUI restyles a label that sits inside its
`input` or `select` group, through `.label:is(.input>*,.select>*)`, and it matches the ancestor
itself: the class this component already emits is all daisyUI needs, and emitting a second one
would be the registry repeating a rule daisyUI wrote.

## Axes

- `appearance: LabelAppearance`: `label`, legacy `fieldset-label`, `floating-label`, or nothing.

The three classes are values of one axis rather than three components, because they are three
classes for the same element and no two of them go together. `Label` is the current caption beside
a control, including inside a Fieldset. `Fieldset` retains daisyUI's deprecated `fieldset-label`
class for existing callers. `Floating` is the odd one: it styles a label that *wraps* its control
and carries a `span` of its own, which daisyUI shrinks and lifts as the field fills.

`LabelAppearance::None` is the only value that emits nothing, which makes this the inverted axis
shape ADR-0004 describes rather than the usual one. There is no such thing as an unclassed daisyUI
label, so a `Default` arm that emitted nothing would stand for a fourth look rather than for
daisyUI's own.

The enum exposes `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

## Deviations

**The deprecated Fieldset appearance remains available.** Pinned daisyUI keeps `fieldset-label` in
its CSS to avoid a breaking change but documents `label` for new fieldsets. The Registry likewise
retains `LabelAppearance::Fieldset` as a legacy Axis value while its Fieldset Examples use the
default `Label` appearance.

**A floating label wants a particular tree, and the caller writes it.** daisyUI's
`.floating-label` styles a `span` inside the label and reads the placeholder state of the control
beside it, so the control has to be *inside* the label rather than pointed at from it. This
component still emits `for`, which is harmless and keeps the association explicit, but the markup
inside is the caller's: a `span` and a control, in that order. Nothing here can enforce that, so it
is written down instead, as it is for every other component in the registry whose daisyUI classes
constrain what goes inside them.

**The control is never rendered here.** `FileInput`, `Input` and `Textarea` are separate Native controls, and a
floating label wraps one after the `span` that daisyUI lifts. The label neither owns nor configures
that control; callers can instead name any other registry control or native form control.

## daisyUI classes deliberately not used

- `fieldset`, `fieldset-legend`: the native group and its heading are owned by the separate
  Fieldset Component; Label remains caller content inside it.
- `file-input`, `input`, `select`, `textarea`, `validator`, `validator-hint`: classes for the
  controls or sibling hint a label names, not classes on the label itself. The registry's separate
  `FileInput`, `Input` and `Textarea` Native controls own `file-input`, `input` and `textarea`
  respectively.
- The responsive prefixes daisyUI generates for `label` (`sm:label` and the rest): a caller
  reaches those through `class`, which concatenates.
