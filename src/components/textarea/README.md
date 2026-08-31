# Textarea

A native multi-line text field styled with daisyUI's `textarea` classes. It renders a real
`textarea`, accepts native textarea attributes, and wraps no Primitive.

[Live examples](https://daisyui-components.dioxus.cc/components/textarea) ·
[their sources](docs/examples/)

`TextareaField` is the closed happy-path composition over the separate Field parts.

It renders `Field`, `FieldLabel`, `Textarea`, the optional `FieldDescription`, and an always-mounted
`FieldError`, in that order. It has no children slot. Use `Field` plus its Compound parts when
content or attributes must land between those elements. Generated part ids provide the accessible
relationships without id props on `TextareaField`.

Classes passed by the caller concatenate with the Component's own. Every other attribute the
caller passes overrides the Component's, so `name`, `placeholder`, `required`, `disabled`, form
attributes, and ARIA or data attributes reach the real control. `value`, `on_change`, and
`on_commit` are explicit props because they form the control's Binding contract and Dioxus'
extended attributes do not include event handlers.

`TextareaField` forwards the complete control surface to its `Textarea`: the colour, size, and
appearance Axes; binding, metadata, value, change, commit, required, and disabled props; and
global/native textarea attributes. Its `class`, `rows`, and every other native attribute therefore
land on the textarea, not the surrounding Field. `field_appearance`, `description_appearance`, and
`error_appearance` forward the corresponding Field-part Axes. Styling a different part is another
signal to use the parts directly.

`binding` and `meta` accept explicit `dioxus-field` values. Without them, the Component resolves
Field Context and then standalone state. Field metadata supplies `id`, `name`, required and
disabled state, ARIA relationships, and `data-*` state; caller attributes override the matching
metadata attributes.

## Conformance

This control is **field-aware**. An explicit `binding: Binding<String>` wins over Field Context;
without either, the control owns standalone state. The lower-level trio remains available:
`value: Option<ReadSignal<String>>` can override the rendered value, `on_change` observes each
value produced by native input, and `on_commit` observes the native `change` event. Writes and
commits still reach the resolved Binding. The browser fires `change` on blur after editing, so that
event defines the end of one interaction unit; Enter inserts a newline rather than committing.

The complete logical focus scope is the native textarea itself. Commit and Focus Exit stay
independent: the native `change` event is the only thing that commits, so leaving an unchanged
session reports Focus Exit alone rather than synthesizing a Commit, which would run Commit
validation over a value the reader never edited. Typing alone only writes. Each actual exit reports
once, calling the resolved Binding's Focus Exit capability and then the optional dependency-free
`on_focus_exit` prop. Focus Exit does not itself infer touched, blurred, or validation semantics.

The resolved Field focus request calls `set_focus` on the native textarea handle captured at mount.
Global and native textarea attributes continue to spread onto that same control.

## State bridging

**Tier 2** on producer-defined invalidity: when the colour Axis is omitted and Field metadata is
invalid, the Component emits `textarea-error`. Passing any colour value explicitly, including
`TextareaColor::Default`, wins over metadata. The Registry does not compute invalidity.

There is no Primitive state to bridge. As ADR-0019 records for Native controls, the browser
supplies focus, the caret, selection, editing, constraint validation, disabled behavior, and form
participation on the same real element that daisyUI styles. Disabled needs no Tier or modifier
class: the explicit or metadata-derived native attribute reaches the textarea, and daisyUI reads
it directly with `.textarea:is(:disabled,[disabled])`.

## Axes

- `color: Option<TextareaColor>` - the eight named daisyUI colours, plus an explicit default that
  emits nothing; omission permits invalid Field metadata to emit `textarea-error`.
- `size: TextareaSize` - `xs`, `sm`, default, `lg`, and `xl`; the default emits nothing.
- `appearance: TextareaAppearance` - daisyUI's default bordered textarea or `textarea-ghost`; the
  default emits nothing.

Each enum exposes `ALL`, listing every value of its Axis. The Preview iterates those lists and the
browser specs assert that every rendered value differs. The class strings are complete literals in
this Component's own file so Tailwind's scanner can see them when this Component is installed
alone.

The installable Textarea Component revision-pins the Field Component it composes, so installing
Textarea also installs the matching Field parts required by `TextareaField`.

## Rows and resizing

`rows` is exposed as the native textarea prop through `extends = textarea`. It is not a styling
Axis: the browser uses it to establish the control's intrinsic row count, while daisyUI applies its
own minimum height.

daisyUI does not set a resize policy, so the Component does not either. Callers choose one with
their own utility classes, such as `resize-none`, `resize-y`, or `resize-x`; those classes join the
Component's own.

## Deviations

**No Primitive.** This is a Native control under ADR-0019. daisyUI's rules name states and
pseudo-elements of a real `textarea`, including `:disabled`, `:focus-within`, and `::placeholder`.
The browser already supplies the behavior a multi-line text field needs.

**The raw `oninput` passthrough was replaced by typed `on_change`.** The Registry is at `0.0.0`, so
the field convention takes the breaking migration rather than retaining two callbacks for the same
native event. Callers receive the resulting `String` directly; `on_commit` separately reports the
native `change` event.

## daisyUI classes deliberately not used

- `textarea-md` - the default size emits nothing, and daisyUI renders an unmodified textarea at
  that size.
- `floating-label` - it belongs to the surrounding `Label`, not to the textarea itself.
- `validator` - pinned daisyUI can read Field metadata's `aria-invalid`, but this Component already
  emits its explicit `textarea-error` modifier from the same state. Adding `validator` would
  duplicate that Axis and also opt into browser-owned valid paint.
- `validator-hint` - its visibility requires a sibling after an element carrying `validator`.
  `FieldError` instead registers `aria-errormessage`, renders a polite live region, and follows
  producer metadata without that adjacency and class coupling.
- Responsive prefixes (`sm:textarea-lg` and the rest) - a caller reaches them through `class`,
  which concatenates.
