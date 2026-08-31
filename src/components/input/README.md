# Input

A native text field styled with daisyUI's `input` classes. It renders a real `input`, accepts
native input attributes, and wraps no Primitive.

[Live examples](https://daisyui-components.dioxus.cc/components/input) ·
[their sources](docs/examples/)

`InputField` is the closed happy-path composition over the separate Field parts.

It renders `Field`, `FieldLabel`, `Input`, the optional `FieldDescription`, and an always-mounted
`FieldError`, in that order. It has no children slot. Use `Field` plus its Compound parts when
content or attributes must land between those elements. Generated part ids provide the accessible
relationships without id props on `InputField`.

Classes passed by the caller concatenate with the Component's own. Every other attribute the
caller passes overrides the Component's, so `type`, `name`, `placeholder`, `required`, `disabled`,
form attributes, and ARIA or data attributes reach the real control. `value`, `on_change`, and
`on_commit` are explicit props because they form the control's Binding contract and Dioxus'
extended attributes do not include event handlers.

`InputField` forwards the complete control surface to its `Input`: the colour, size, and appearance
Axes; binding, metadata, value, change, commit, required, and disabled props; and global/native
input attributes. Its `class` and every native attribute therefore land on the input, not the
surrounding Field. `field_appearance`, `description_appearance`, and `error_appearance` forward the
corresponding Field-part Axes. Styling a different part is another signal to use the parts directly.

`binding` and `meta` accept explicit `dioxus-field` values. Without them, the Component resolves
Field Context and then standalone state. Field metadata supplies `id`, `name`, required and
disabled state, ARIA relationships, and `data-*` state; caller attributes override the matching
metadata attributes.

## Conformance

This control is **field-aware**. An explicit `binding: Binding<String>` wins over Field Context;
without either, the control owns standalone state. The lower-level trio remains available:
`value: Option<ReadSignal<String>>` can override the rendered value, `on_change` observes each
value produced by native input, and `on_commit` observes the native `change` event. Writes and
commits still reach the resolved Binding. The browser fires `change` on blur after editing and when
an Enter-driven interaction commits, so that event defines the end of one interaction unit.

The complete logical focus scope is the native input itself. Commit and Focus Exit stay independent:
the native `change` event is the only thing that commits, so leaving an unchanged session reports
Focus Exit alone rather than synthesizing a Commit, which would run Commit validation over a value
the reader never edited. Typing alone only writes. Each actual exit reports once, calling the
resolved Binding's Focus Exit capability and then the optional dependency-free `on_focus_exit` prop.
Focus Exit does not itself infer touched, blurred, or validation semantics.

The resolved Field focus request calls `set_focus` on the native input handle captured at mount.
Global and native input attributes continue to spread onto that same control.

## State bridging

**Tier 2** on producer-defined invalidity: when the colour Axis is omitted and Field metadata is
invalid, the Component emits `input-error`. Passing any colour value explicitly, including
`InputColor::Default`, wins over metadata. The Registry does not compute invalidity.

There is no Primitive state to bridge. As ADR-0019 records for Native controls, the browser
supplies focus, the caret, selection, editing, autofill, constraint validation, disabled behavior,
and form participation on the same real element that daisyUI styles. Disabled needs no Tier or
modifier class: the explicit or metadata-derived native attribute reaches the input, and daisyUI
reads it directly with `.input:is(:disabled,[disabled])`.

## Axes

- `color: Option<InputColor>` - the eight named daisyUI colours, plus an explicit default that emits
  nothing; omission permits invalid Field metadata to emit `input-error`.
- `size: InputSize` - `xs`, `sm`, default, `lg`, and `xl`; the default emits nothing.
- `appearance: InputAppearance` - daisyUI's default bordered input or `input-ghost`; the default
  emits nothing.

Each enum exposes `ALL`, listing every value of its Axis. The Preview iterates those lists and the
browser specs assert that every rendered value differs. The class strings are complete literals
in this Component's own file, even where the combobox and date pickers emit the same strings, so
Tailwind's scanner can see them when this Component is installed alone.

The installable Input Component revision-pins the Field Component it composes, so installing Input
also installs the matching Field parts required by `InputField`.

`floating-label` remains an appearance of `Label`, not `Input`. The class styles an ancestor label,
expects that label to contain a `span`, and reads the placeholder and focus state of the input
inside it. `Input` neither owns nor can enforce that surrounding tree, so a caller nests it inside
`Label { appearance: LabelAppearance::Floating, ... }` and supplies a placeholder.

## Deviations

**No Primitive.** This is a Native control under ADR-0019. daisyUI's rules name states and
pseudo-elements of a real `input` - including `:disabled`, `:focus-within`, `::placeholder`, and
the validation pseudo-classes used by `validator` - so replacing it with a button-based Primitive
would make those rules match nothing. The browser already supplies the behavior a text field
needs.

**The raw `oninput` passthrough was replaced by typed `on_change`.** The Registry is at `0.0.0`, so
the trio pilot takes the breaking migration rather than retaining two callbacks for the same native
event. Callers receive the resulting `String` directly; `on_commit` separately reports the native
`change` event.

## daisyUI classes deliberately not used

- `input-md` - the default size emits nothing, and daisyUI renders an unmodified input at that
  size.
- `floating-label` - it belongs to the surrounding `Label`, whose appearance Axis already emits
  it.
- `validator` - pinned daisyUI can read Field metadata's `aria-invalid`, but this Component already
  emits its explicit `input-error` modifier from the same state. Adding `validator` would duplicate
  that Axis and also opt into browser-owned valid paint.
- `validator-hint` - its visibility requires a sibling after an element carrying `validator`.
  `FieldError` instead registers `aria-errormessage`, renders a polite live region, and follows
  producer metadata without that adjacency and class coupling.
- Responsive prefixes (`sm:input-lg` and the rest) - a caller reaches them through `class`, which
  concatenates.
