# Checkbox

A checkbox styled with daisyUI's `checkbox` classes, wrapping a temporary local copy of the
`dioxus-primitives` checkbox.

```rust
Checkbox {
    color: CheckboxColor::Primary,
    size: CheckboxSize::Lg,
    default_value: CheckboxState::Checked,
    on_change: move |state| {},
    on_commit: move |()| {},
    aria_label: "Accept the terms",
}
```

`CheckboxField` is the closed happy-path composition over the separate Field parts:

```rust
CheckboxField {
    context,
    label: "Accept the terms",
    description: "Required before creating an account.",
}
```

It renders `Field`, a `FieldRow` holding `Checkbox` and `FieldLabel` in that order, the optional
`FieldDescription`, and an always-mounted `FieldError`. It has no children slot. The surrounding
Field keeps its default grid layout, and the row lays the checkbox out beside its label, which is
how a checkbox reads rather than as a control stacked under a caption. Use `Field` plus its
Compound parts for another custom layout, or when content or attributes must land between those
elements. Generated part and control ids provide the accessible relationships without id props on
`CheckboxField`.

The lower-level `Checkbox` also renders no children: daisyUI draws the mark itself, so there is
nothing to put inside. Outside `CheckboxField`, a label is the caller's, next to the checkbox rather
than in it, and the checkbox is named by `aria_label` or by `aria_labelledby`.

The primitive's state and form props are all exposed. `value` is the optionally controlled
`CheckboxState`, `default_value` seeds uncontrolled state, and `form_value` is the string a
checked control submits alongside `name`. `required` and `disabled` retain their primitive
meanings. `CheckboxState` is re-exported, so a caller needs one import. The tri-state surface stays
available independently from the ordinary two-state Field binding.

`binding`, `bool_binding`, and `meta` accept explicit `dioxus-field` values. `binding` carries
`CheckboxState`; `bool_binding` carries `bool`, mapping `false` to unchecked and `true` to checked
in both directions. Without an explicit value binding, the Component resolves either supported
type from Field Context and then falls back to standalone `CheckboxState`. Field metadata supplies
the control's id, name, required and disabled state, ARIA relationships, and `data-*` state.
Existing explicit props and caller attributes win over those metadata values.

`CheckboxField` forwards the complete control surface to its `Checkbox`: the colour and size Axes;
both binding forms, metadata, value, initial state, required, disabled, name, submitted value,
change, commit, and Focus Exit props; and global attributes. Its `class`, ARIA attributes, and data
attributes therefore land on the checkbox button, not the surrounding Field. `field_appearance`,
`row_appearance`, `description_appearance`, and `error_appearance` forward the corresponding
Field-part Axes. Styling another part or choosing a different arrangement is a signal to use the
Compound parts directly.

## Conformance

This control is **field-aware**. Binding Resolution uses this precedence:

1. explicit `binding: Binding<CheckboxState>`;
2. explicit `bool_binding: Binding<bool>`;
3. a compatible `Binding<CheckboxState>` or `Binding<bool>` from Field Context;
4. standalone `CheckboxState` seeded by `default_value`.

Only one Value Binding exists in Field Context, so its two supported types have no precedence over
one another. An explicit `binding` deliberately remains authoritative when both explicit forms are
supplied, preserving the existing tri-state API. `CheckboxField` forwards both explicit forms and
also supplies its `context` to the composed Checkbox.

A Field Context whose Value Binding is of any other type is a producer error, not a runtime state:
the checkbox panics at first render naming both types rather than falling back to standalone state
and rendering a control the form never reaches.

The lower-level trio remains available: a `Some` value in
`value: ReadSignal<Option<CheckboxState>>` overrides the rendered Binding value, `on_change`
observes each user toggle, and `on_commit` observes every completed toggle. Writes and commits
still reach the resolved Binding. A checkbox toggle is one complete interaction unit, so change
and commit have the same cadence. Global attributes continue to spread onto the Primitive's
checkbox button.

User toggles write `ChangeOrigin::User` through either binding form. Application writes through a
`Binding<bool>` retain their supplied origin and reactively update the checkbox. Commit and Focus
Exit delegate to that same resolved Binding without changing their ordering or independence.

The complete logical focus scope is the visible Primitive button; the hidden native form
participant is not focusable. A toggle Commits immediately while that button retains focus and does
not imply Focus Exit. Leaving the button later calls the resolved Binding's Focus Exit capability
and then the optional dependency-free `on_focus_exit` prop, once each. Focus Exit remains
independent from Commit and does not itself infer touched, blurred, or validation semantics.

## State bridging

**Tier 1** on the checked state: the primitive sets `aria-checked` to `true`, `false` or
`mixed`, and daisyUI's rule is `.checkbox:checked, .checkbox[aria-checked=true]`. The ARIA
attribute daisyUI already matches is exactly there, so there is nothing to bridge.

**Tier 2** on producer-defined invalidity: when the colour Axis is omitted and Field metadata is
invalid, the Component emits `checkbox-error`. Passing any colour value explicitly, including
`CheckboxColor::Default`, wins over metadata. The Registry does not compute invalidity.

The disabled state needs no bridging either, though it is not Tier 1: that tier is about ARIA
attributes, and this is a native one. The primitive sets `disabled` on the `button` it renders
and daisyUI's rule is `.checkbox:disabled`, which matches it directly.

The checked and disabled states remain on the element daisyUI selects, in attributes daisyUI
already reads. Only producer-defined invalidity needs a modifier class, and per ADR-0002 the
Component ships no CSS of its own.

## Axes

- `color: Option<CheckboxColor>`: `checkbox-neutral`, `checkbox-primary`, `checkbox-secondary`,
  `checkbox-accent`, `checkbox-info`, `checkbox-success`, `checkbox-warning`, `checkbox-error`;
  omission permits invalid Field metadata to emit `checkbox-error`.
- `size: CheckboxSize`: `checkbox-xs`, `checkbox-sm`, `checkbox-lg`, `checkbox-xl`.

An explicit `CheckboxColor::Default` emits nothing and is daisyUI's uncoloured checkbox, which is a
distinct look from `checkbox-neutral`. `CheckboxSize::Default` renders at the same size as
daisyUI's explicit `checkbox-md`, so that class is not emitted.

Each enum exposes `ALL`, listing every value of the axis. The preview iterates it to render
every axis value and the browser specs assert computed styles over the same rendered set.

The installable Checkbox Component revision-pins the Field Component it composes, so installing
Checkbox also installs the matching Field parts required by `CheckboxField`.

## Deviations

**The Primitive is copied locally until its form name can be absent.** The pinned Primitive gives
its synchronized native checkbox a required `String` name, which renders `name=""` for an unnamed
Component. This Component carries a narrow copy of the Primitive from revision
`bf007c15d0cf4d04d3181cc46cf12325aa773955` and changes that input to accept an optional name. The
copy keeps the Primitive's controlled state, keyboard, focus, ARIA, and form synchronization and
omits only its unused indicator part. It can be removed when the pinned Primitive represents an
absent name itself.

**The Primitive's prop names are adapted to the Binding trio.** Its `checked` prop is exposed as
`value`, its submitted `value` is exposed as `form_value`, and its `on_checked_change` callback is
exposed as `on_change`. These are deliberate breaking renames for the Registry's trio contract;
the wrapped Primitive still receives the corresponding native props.

**Field focus registration is forwarded through the Primitive's attributes.** The forwarded
`onmounted` listener captures the actual button and `use_focus_registration` calls `set_focus` on
that handle. Dioxus 0.7 invokes only one same-name listener on an element, so the wrapper also
repeats the Primitive's post-toggle focus call through this handle, preserving its macOS pointer
focus workaround while making Field focus requests reach the same control.

**The indicator part is omitted.** `CheckboxIndicator` carries no behaviour (it reads the
checkbox's state off a context and renders its children only when checked) and daisyUI draws
the mark itself, from a `::before` on the checkbox element. Rendering the indicator would put an
empty `span` inside a small padded box, which is a decorative hazard with no upside. A caller
who wants a mark of their own uses the primitive directly, where the indicator is still there.

**The indeterminate state has no daisyUI styling.** `CheckboxState::Indeterminate` is accepted
and passed through, and the primitive announces it correctly as `aria-checked="mixed"`, but
daisyUI keys its dash on `.checkbox:indeterminate`, a native pseudo-class that only a real
`input` element can be in. The primitive renders a `button`, so the rule can never match, and
an indeterminate checkbox renders as an unchecked one. Bridging it would take CSS, which
ADR-0002 rules out. Recorded as a gap in the PRD rather than worked around.

The primitive itself does keep an `input` in the DOM, mirrored to the checkbox's state so that
forms submit, but it is `aria-hidden`, visually hidden, and not the element daisyUI's classes
are on, so its `:indeterminate` state styles nothing.

## daisyUI classes deliberately not used

- `checkbox-md`: the default size emits nothing, and daisyUI renders an unclassed checkbox at
  exactly that size.
- `label` and `fieldset-label`: daisyUI's wrappers for laying a control out next to its text.
  That markup is the caller's, not this component's, and emitting it here would put an element
  between the caller and the checkbox for nothing.
- `validator`: pinned daisyUI can read Field metadata's `aria-invalid`, but this Component already
  emits its explicit `checkbox-error` modifier from the same state. Adding `validator` would
  duplicate that Axis and also opt into browser-owned valid paint.
- `validator-hint`: its visibility requires a sibling after an element carrying `validator`.
  `FieldError` instead registers `aria-errormessage`, renders a polite live region, and follows
  producer metadata without that adjacency and class coupling.
