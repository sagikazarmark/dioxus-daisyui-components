# Switch

A switch styled with daisyUI's `toggle` classes, wrapping the `dioxus-primitives` switch.

[Live examples](https://daisyui-components.dioxus.cc/components/switch) ·
[their sources](docs/examples/)

`SwitchField` is the closed happy-path composition over the separate Field parts.

It renders `Field`, a `FieldRow` holding `Switch` and `FieldLabel` in that order, the optional
`FieldDescription`, and an always-mounted `FieldError`. It has no children slot. The surrounding
Field keeps its default grid layout, and the row lays the switch out beside its label, which is how
a switch reads rather than as a control stacked under a caption. Use `Field` plus its Compound
parts for another custom layout, or when content or attributes must land between those elements.
Generated part and control ids provide the accessible relationships without id props on
`SwitchField`.

The lower-level `Switch` also renders no children: daisyUI draws the knob itself, so there is
nothing to put inside. Outside `SwitchField`, a label is the caller's, next to the switch rather than
in it, and the switch is named by `aria_label` or by `aria_labelledby`.

The primitive's state props are exposed. `value` is the optionally controlled `bool`,
`default_value` seeds uncontrolled state, and `form_value` is the string a switched-on control
submits alongside `name`. `required` and `disabled` apply to the native form participant.
That registry-owned participant is hidden with structural inline styles rather than Tailwind
utilities, so it cannot become visible when dependency source is absent from Tailwind's scan.

`binding` and `meta` accept explicit `dioxus-field` values. Without them, the Component resolves
Field Context and then standalone state. Field metadata supplies the control's id, name, required
and disabled state, ARIA relationships, and `data-*` state. Existing explicit props and caller
attributes win over those metadata values.

`SwitchField` forwards the complete control surface to its `Switch`: the colour Axis; binding,
metadata, value, initial state, required, disabled, name, submitted value, change, and commit props;
and global attributes. Its `class`, ARIA attributes, and data attributes therefore land on the
switch button, not the surrounding Field. `field_appearance`, `row_appearance`,
`description_appearance`, and `error_appearance` forward the corresponding Field-part Axes. Styling
another part or choosing a different arrangement is a signal to use the Compound parts directly.

## Conformance

This control is **field-aware**. An explicit `binding: Binding<bool>` wins over Field Context;
without either, the control owns standalone state seeded by `default_value`. The lower-level trio
remains available: a `Some` value in `value: ReadSignal<Option<bool>>` overrides the rendered
Binding value, `on_change` observes each user toggle, and `on_commit` observes every completed
toggle. Writes and commits still reach the resolved Binding. A switch toggle is one complete
interaction unit, so change and commit have the same cadence. Global attributes continue to spread
onto the Primitive's switch button.

The complete logical focus scope is the visible Primitive button; its hidden native form
participants are not focusable. A toggle Commits immediately while that button retains focus and
does not imply Focus Exit. Leaving the button later calls the resolved Binding's Focus Exit
capability and then the optional dependency-free `on_focus_exit` prop, once each. Focus Exit remains
independent from Commit and does not itself infer touched, blurred, or validation semantics.

## State bridging

**Tier 1** on the checked state: the primitive sets `aria-checked` on the `button` it renders,
and daisyUI's rule is `.toggle:checked, .toggle[aria-checked=true], .toggle:has(>input:checked)`.
The attribute daisyUI already matches is exactly there, so there is nothing to bridge, and the
colour axis needs no bridging either, since every colour rule is written the same way
(`.toggle-primary:checked, .toggle-primary[aria-checked=true]`).

The disabled state needs no bridging and is not Tier 1: that tier is about ARIA attributes, and
this is a native one. The primitive sets `disabled` on the `button` and daisyUI's rule is
`.toggle:disabled`.

**Tier 2** on producer-defined invalidity: when the colour Axis is omitted and Field metadata is
invalid, the Component emits `toggle-error`. Passing any colour value explicitly, including
`SwitchColor::Default`, wins over metadata. The Registry does not compute invalidity.

The checked and disabled states remain on the element daisyUI selects, in attributes daisyUI
already reads. Only producer-defined invalidity needs a modifier class, and per ADR-0002 the
Component ships no CSS of its own.

## Axes

- `color: Option<SwitchColor>`: `toggle-neutral`, `toggle-primary`, `toggle-secondary`,
  `toggle-accent`, `toggle-info`, `toggle-success`, `toggle-warning`, `toggle-error`; omission
  permits invalid Field metadata to emit `toggle-error`.

An explicit `SwitchColor::Default` emits nothing, which is daisyUI's uncoloured toggle, a distinct
look from `toggle-neutral`.

The enum exposes `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

The installable Switch Component revision-pins the Field Component it composes, so installing
Switch also installs the matching Field parts required by `SwitchField`.

**There is no size axis.** That is a gap rather than an omission, and the next section is what
it costs.

## Deviations

**The Primitive's prop names are adapted to the Binding trio.** Its `checked` prop is exposed as
`value`, its submitted `value` is exposed as `form_value`, and its `on_checked_change` callback is
exposed as `on_change`. These are deliberate breaking renames for the Registry's trio contract;
the wrapped Primitive still receives the corresponding state props.

**The native form participant is repeated to preserve `required`.** The pinned Primitive renders a
hidden checkbox for `name` and `form_value`, but does not put its declared `required` prop on that
input. The Registry passes an empty name to that input and renders a synchronized hidden checkbox
with `name`, `form_value`, `required`, and `disabled`, matching the Checkbox Component's native form
behavior without submitting the value twice. Its zero-sized inline hiding matches the Primitive's
own hidden checkbox because invisibility is part of the control's behavior, not optional
presentation. It does not add a shipped stylesheet or theme values.

Cargo-library consumers must configure the stable Tailwind `@source` described in the root README
for ordinary Registry utilities and responsive variants. The native form participant remains
hidden if that configuration is missing, but other utility-backed presentation is not guaranteed.

**Field focus registration is forwarded through the Primitive's attributes.** The forwarded
`onmounted` listener captures the actual button and `use_focus_registration` calls `set_focus` on
that handle. Dioxus 0.7 invokes only one same-name listener on an element, so the wrapper also
repeats the Primitive's post-toggle focus call through this handle, preserving its macOS pointer
focus workaround while making Field focus requests reach the same control.

**daisyUI's size classes cannot reach this element.** Every one of them is written

```css
.toggle-sm[type=checkbox], .toggle-sm:has([type=checkbox]) { --size: … }
```

and the primitive renders a `button` with `role="switch"`, with the hidden input that makes the
switch submittable as its *sibling* rather than as a descendant. The first arm needs the element
to be a checkbox and the second needs it to contain one, so neither can ever match, and emitting
`toggle-lg` here would put a class in the markup that changes nothing. The size axis is left
unexposed rather than shipped inert. Recorded as ADR-0010, which the radio group shares for a
milder version of the same thing, and which the checkbox is genuinely clear of, since daisyUI
writes its sizes as plain class selectors.

Moving the classes to a wrapper that *does* contain the input was the alternative, and it costs
more than it buys: `.toggle:has(>input:checked)` would keep the checked look and the sizes would
start working, but every colour rule and the disabled rule are written against the element
carrying `aria-checked` and `disabled`, which is the button inside. A sized switch with no
colour and no disabled look is a worse switch than an unsized one.

A caller who needs another size sets daisyUI's own custom property through a utility, which
concatenates onto the element the classes are already on:

```rust
Switch { class: "[--size:calc(var(--size-selector,0.25rem)*8)]" }
```

That is the caller's decision to take on a daisyUI internal, deliberately not one this registry
takes on their behalf: `--size` and its multipliers are daisyUI's to change, and a component
that hardcoded them would go quietly wrong on a release that did.

**The thumb part is omitted, and so are children.** `SwitchThumb` carries no behaviour (it
renders a `span`) and daisyUI draws the knob from a `::before` on the toggle itself. What
daisyUI does put inside a toggle is a pair of marks either side of the knob, shown and hidden by

```css
.toggle:has(:checked) > :nth-child(2) { opacity: 0 }
.toggle:has(:checked) > :nth-child(3) { opacity: 1 }
```

which is the size gate again in another guise: `:has(:checked)` looks for a checked *descendant*,
and the only checked thing here is the primitive's hidden input, which is a sibling. Marks
written inside this component would therefore sit at whatever the off state draws them at and
never swap, so children are not accepted at all rather than accepted and ignored. A caller who
wants either uses the primitive directly.

## daisyUI classes deliberately not used

- `toggle-xs`, `toggle-sm`, `toggle-md`, `toggle-lg`, `toggle-xl`: the size axis above. Not a
  matter of taste: they cannot match this markup.
- `label` and `fieldset-label`: daisyUI's wrappers for laying a control out next to its text.
  That markup is the caller's, and emitting it here would put an element between the caller and
  the switch for nothing.
- `validator`: pinned daisyUI can read Field metadata's `aria-invalid`, but this Component already
  emits its explicit `toggle-error` modifier from the same state. Adding `validator` would
  duplicate that Axis and also opt into browser-owned valid paint.
- `validator-hint`: its visibility requires a sibling after an element carrying `validator`.
  `FieldError` instead registers `aria-errormessage`, renders a polite live region, and follows
  producer metadata without that adjacency and class coupling.
- The responsive prefixes daisyUI generates for the colour classes (`sm:toggle-primary` and the
  rest): a caller reaches them through `class`, which concatenates.
