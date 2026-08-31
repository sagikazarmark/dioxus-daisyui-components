# OTP

A native one-time-code field styled with daisyUI's `otp` classes. It renders daisyUI's required
`label`, one empty `span` per visual character box, and one real `input` containing the complete
code.

[Live examples](https://daisyui-components.dioxus.cc/components/otp) ·
[their sources](docs/examples/)

`OtpField` is the closed happy-path composition over the separate Field parts.

It renders `Field`, `FieldLabel`, `Otp`, the optional `FieldDescription`, and an always-mounted
`FieldError`, in that order. It has no children slot. Use `Field` plus its Compound parts when
content or attributes must land between those elements. Generated part ids provide the accessible
relationships without id props on `OtpField`.

`length` defaults to four and accepts one through eight, the range daisyUI positions in its CSS.
It drives the span count, `maxlength`, and numeric validation pattern together. The input always
uses `type="text"`, `inputmode="numeric"`, and `autocomplete="one-time-code"`; those attributes
are the markup that makes this an OTP field and cannot be contradicted by caller attributes.
`required`, `disabled`, and `name` fall back to Field metadata when omitted. A standalone OTP is
optional and unnamed unless those props are passed explicitly.

Classes passed by the caller concatenate on the outer label, which is the element daisyUI styles.
Other global attributes, including `id`, ARIA attributes, and data attributes, land on the real
input they identify and describe. `name`, `required`, and `disabled` are explicit props. `value`,
`on_change`, and `on_commit` preserve the control's Binding trio; `on_focus_exit` separately
reports departure. Raw native callbacks are not exposed.

`OtpField` forwards the complete control surface to its `Otp`: the colour, size, and appearance
Axes; length, binding, metadata, value, required, name, disabled, change, commit, and Focus Exit
props; and global attributes. Its `class` therefore lands on the styled OTP label while every other
global attribute lands on the native input, preserving `Otp`'s split. `field_appearance`,
`description_appearance`, and `error_appearance` forward the corresponding Field-part Axes.
Styling a different part is another signal to use the parts directly.

`binding` and `meta` accept explicit `dioxus-field` values. Without them, the Component resolves
Field Context and then standalone state. Field metadata supplies `id`, `name`, required and
disabled state, ARIA relationships, and `data-*` state to the inner input. Caller attributes
override matching metadata attributes without moving their classes off the styled label.

## Conformance

This control is **field-aware**. An explicit `binding: Binding<String>` wins over Field Context;
without either, the control owns standalone state. The lower-level trio remains available:
`value: Option<ReadSignal<String>>` can override the rendered value, and `on_change` observes each
value produced by native input. Writes and commits still reach the resolved Binding.

A value commits as soon as it consists of exactly `length` ASCII digits. The native `change` event
also commits an edited incomplete value, normally on blur, so the interaction still ends when a
user leaves before completing the code. When a full-length input event already committed, its
following `change` event is suppressed so one completed code ends one interaction unit.

Focus Exit is independent of Commit. Its complete logical scope is the styled OTP label subtree,
which contains the one native input today. One bubbling `focusout` handler on that label calls the
resolved Binding's `focus_exit()` and then the optional direct `on_focus_exit` prop exactly once.
A completed code can therefore Commit while focus remains in the input, and its later departure is
a separate Focus Exit; an unchanged departure reports Focus Exit without manufacturing Commit.
The Component does not infer touched, blurred, or validation state from either event.

The resolved Field focus request calls `set_focus` on the inner native input handle captured at
mount. Field metadata and global attributes spread onto that same input; only the merged class is
moved to the label daisyUI styles.

## State bridging

**Tier 2** on producer-defined invalidity: when the colour Axis is omitted and Field metadata is
invalid, the Component emits `otp-error` on the label. Passing any colour value explicitly,
including `OtpColor::Default`, wins over metadata. The Registry does not compute invalidity.

There is no Primitive state to bridge. Typing, Backspace, paste, selection, autofill, constraint
validation, disabled behaviour, and form participation all belong to the one native input. daisyUI
uses that input's `:focus` and `:valid` state, plus the label's `:focus-within` state, to paint the
sibling spans. Explicit or metadata-derived disabled state reaches the native input directly.

No Registry-owned advance, retreat, paste distribution, or focus behavior is needed: the visual
boxes are not separate controls. The complete code is edited in one input and daisyUI projects its
characters over the spans with letter spacing.

## Axes

- `color: Option<OtpColor>` - the eight named daisyUI colours, plus an explicit default that emits
  nothing; omission permits invalid Field metadata to emit `otp-error`.
- `size: OtpSize` - `xs`, `sm`, default, `lg`, and `xl`; the default emits nothing.
- `appearance: OtpAppearance` - separate boxes or `otp-joined`; the default emits nothing.

Each enum exposes `ALL`, listing every value of its Axis. The Preview iterates those lists and the
browser specs assert that every rendered value differs. Every class string is a complete literal
in this Component's own file so Tailwind's scanner can see it when the Component is installed
alone.

The installable OTP Component revision-pins the Field Component it composes, so installing OTP also
installs the matching Field parts required by `OtpField`.

## Deviations

**No Primitive.** This is a Native control under ADR-0019. The issue that introduced it described
one input per box, but daisyUI 5.7.17's actual selectors require presentational spans followed by a
single input. Following that markup both lets daisyUI paint the boxes and leaves all editing
behaviour with the browser, so the Registry does not establish an exception for writing behaviour.

The Component owns the complete invariant tree rather than exposing its spans as Compound parts.
They carry no caller content or behaviour, and changing their count independently from the input's
length would break daisyUI's projection.

**`required`, `disabled`, and `name` are declared props rather than scanned attributes.** Input
reads an explicit `required` or `disabled` out of its extended attribute list because it extends
the full `input` attribute set. This Component extends only `GlobalAttributes`, which carry no
form-participation attributes, so there is no list to scan; the three follow Checkbox's shape as
explicit props with Field-metadata fallback.

**The raw `oninput` passthrough was replaced by typed `on_change`.** The Registry is at `0.0.0`, so
the trio migration takes the breaking change rather than retaining two callbacks for the same
native event. Callers receive the resulting `String` directly; `on_commit` separately reports a
complete code or a native change that ends an incomplete interaction.

## daisyUI classes deliberately not used

- `otp-md` - the default size emits nothing, and daisyUI renders an unmodified OTP field at that
  size.
- `validator` - pinned daisyUI can read Field metadata's `aria-invalid`, but this Component already
  emits its explicit `otp-error` modifier from the same state. Adding `validator` would duplicate
  that Axis and also opt into browser-owned valid paint.
- `validator-hint` - its visibility requires a sibling after an element carrying `validator`.
  `FieldError` instead registers `aria-errormessage`, renders a polite live region, and follows
  producer metadata without that adjacency and class coupling.
- Responsive prefixes (`sm:otp-lg` and the rest) - a caller reaches them through `class`, which
  concatenates on the label.
