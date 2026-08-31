# File input

A native file picker styled with daisyUI's `file-input` classes. It renders a real
`input type="file"` and wraps no Primitive.

```rust
FileInput {
    accept: "image/png,image/jpeg",
    multiple: true,
    name: "attachments",
    color: FileInputColor::Primary,
    onchange: move |event| {
        // Read the browser-owned FileList from the form event.
    },
}
```

Classes passed by the caller concatenate with the Component's own. The input type is fixed because
the browser's file picker and selected `FileList` belong to a real file input and cannot be moved to
another element. `accept` and `multiple` are explicit file configuration props. `name`, `form`,
`required`, and `disabled` expose the native control behavior without making the defining `type`
overridable. `onchange` is explicit because Dioxus' extended attributes do not include event
handlers.

## State bridging

There is **no state to bridge**, and there is no Primitive to bridge it from. As ADR-0019 records
for Native controls, the browser supplies focus, the file-picking dialogue, the selected
`FileList`, disabled behavior, and form participation on the same real element that daisyUI styles.

## Axes

- `color: FileInputColor` - the eight named daisyUI colours, plus a default that emits nothing.
- `size: FileInputSize` - `xs`, `sm`, default, `lg`, and `xl`; the default emits nothing.
- `appearance: FileInputAppearance` - daisyUI's default bordered file input or
  `file-input-ghost`; the default emits nothing.

Each enum exposes `ALL`, listing every value of its Axis. The Preview iterates those lists and the
browser specs assert that every rendered value differs. The class strings are complete literals in
this Component's own file so Tailwind's scanner can see them when this Component is installed
alone.

## Deviations

**No Primitive.** This is a Native control under ADR-0019. The file-picking dialogue can only be
opened from a user gesture on a real file input, and its selected `FileList` is readable only from
that input. The browser already supplies the behavior, so a button-based Primitive could not
usefully replace it.

**The input type is fixed.** Unlike a general text input, a file picker stops being this Component
if its type changes. The Component therefore exposes global attributes rather than all input
attributes, and writes `type="file"` itself.

## daisyUI classes deliberately not used

- The inner button is not a Compound part. The browser generates it as the
  `::file-selector-button` pseudo-element, which is not a child a caller can reach with attributes
  or classes. daisyUI styles it through `.file-input::file-selector-button`; this Component cannot
  expose it for separate caller styling.
- `file-input-md` - the default size emits nothing, and daisyUI renders an unmodified file input at
  that size.
- Responsive prefixes (`sm:file-input-lg` and the rest) - a caller reaches them through `class`,
  which concatenates.
