# Native Select

A native single-choice select styled with daisyUI's `select` class. It renders a real `<select>`
with `option` children, accepts native select attributes, and wraps no Primitive. The browser
draws the popup daisyUI's class styles, which is the element that class was written for.

[Live examples](https://daisyui-components.dioxus.cc/components/native_select) ·
[their sources](docs/examples/)

This component complements the [listbox `Select`](../select/), which earns its weight where the
popup is the point: styled rows, custom row content, typeahead across groups. The native variant
is for the select fields most forms are made of, at native-control weight — installing it pulls
no `dioxus-primitives` select machinery into the binary.

`NativeSelectField` is the closed happy-path composition over the separate Field parts. It
renders `Field`, `FieldLabel`, `NativeSelect`, the optional `FieldDescription`, and an
always-mounted `FieldError`, in that order. It has no children slot. Use `Field` plus its
Compound parts when content or attributes must land between those elements.

## The value mapping

A native `<select>` speaks option-value strings, so this component owns the mapping between the
typed value and those strings. The `options` prop is a closed list of `NativeSelectOption<T>`
values, each carrying the typed value and its visible label; the selected option is found by
`PartialEq`, first match winning. The string an option speaks — and the string a native form
submits — is its **position in the list**, unless `NativeSelectOption::form_value` replaces it
with an explicit string. `T` therefore needs no `Display` or `FromStr`, duplicate values resolve
deterministically, and a form that needs a meaningful submitted value names one per option.

A `None` value selects the placeholder: daisyUI's native pattern of a disabled first `option`,
rendered whenever the `placeholder` prop is set and still listed after a choice is made. Because
that option is disabled — and its empty value string parses to nothing — a `None` write never
arrives from the control. Pass `placeholder` or `default_value` whenever the value can start as
`None`: a native select given neither displays its first option while the Binding still holds
`None`, which is the element's own behaviour.

Classes passed by the caller concatenate with the Component's own. Every other attribute the
caller passes overrides the Component's, so `name`, `required`, `disabled`, form attributes, and
ARIA or data attributes reach the real control. `value`, `on_change`, and `on_commit` are
explicit props because they form the control's Binding contract and Dioxus' extended attributes
do not include event handlers.

## Conformance

This control is **field-aware**. An explicit `binding: Binding<Option<T>>` wins over Field
Context; without either, the control owns standalone state seeded by `default_value`. The
lower-level trio remains available: `value: Option<ReadSignal<Option<T>>>` can override the
rendered Binding value, `on_change` observes each successful option selection, and `on_commit`
observes that selection as a completed interaction unit. A native select fires `input` and
`change` together on each pick, so change and Commit have the same cadence — the same boundary
the listbox `Select` records.

Focus Exit is independent, and the complete logical focus scope is the native select itself: the
browser owns picker focus end to end, so opening and closing the native popup never reports an
exit. One confirmed departure calls `Binding::focus_exit()` and then the optional
dependency-free `on_focus_exit` prop. Neither Commit nor Focus Exit infers form touched,
blurred, validation, or other form-library semantics.

Field metadata lands on the real `<select>` through the full native surface: `id`, `name`,
native `required` and `disabled`, `aria-invalid`, `aria-errormessage`, `aria-describedby`, and
the `data-*` state — everything the listbox's `button` trigger cannot carry. The resolved Field
focus request calls `set_focus` on the select handle captured at mount. Explicit `required`,
`disabled`, and caller attributes override matching metadata attributes while classes
concatenate.

## State bridging

**Tier 2** on producer-defined invalidity: when the colour Axis is omitted and Field metadata is
invalid, the Component emits `select-error`. Passing any colour value explicitly, including
`NativeSelectColor::Default`, wins over metadata. The Registry does not compute invalidity.

There is no Primitive state to bridge. As ADR-0019 records for Native controls, the browser
supplies focus, the picker, keyboard navigation, constraint validation, disabled behaviour, and
form participation on the same real element that daisyUI styles. The states the listbox had to
work around are all Tier 1 here: the caret flips while the picker shows (`.select:open`), the
placeholder fades as a real disabled option, and the disabled select is read directly from the
native attribute by `.select:is(:disabled,[disabled])`.

## Axes

- `color: Option<NativeSelectColor>`: `select-neutral`, `select-primary`, `select-secondary`,
  `select-accent`, `select-info`, `select-success`, `select-warning`, `select-error`; omission
  permits invalid Field metadata to emit `select-error`.
- `size: NativeSelectSize`: `select-xs`, `select-sm`, `select-lg`, `select-xl`.
- `appearance: NativeSelectAppearance`: `select-ghost`.

`NativeSelectColor::Default` and `NativeSelectSize::Default` emit nothing, which renders at the
same appearance as daisyUI's explicit `select-md` and unclassed select.

The appearance axis follows the Input and Textarea mold, where ghost is the one daisyUI style
alongside the semantic colours. The listbox `Select` declined the same axis; the two selects
diverge here because this one follows the Native controls it is built like.

Each enum exposes `ALL`, listing every value of the axis. The preview iterates it and the
browser specs read the same rendered set.

## Deviations

**No Primitive.** This is a Native control under ADR-0019. daisyUI's select rules name native
states and elements — `.select:open`, `.select select`, `option`, `optgroup`,
`.select[multiple]`, `::picker(select)` — so replacing the element with a button-based Primitive
would make those rules match nothing, which is what the listbox `Select`'s Deviations record.
The browser already supplies the behaviour a select field needs.

**What the listbox has that this does not.** Option groups: the closed `options` prop is flat,
and a children-based `optgroup` form would hand the option-value strings back to the caller,
reopening the mapping the closed prop exists to own. Custom row content and styled rows: an
`option`'s rendering belongs to the browser. Typeahead across groups, a controlled open state,
and popup placement axes: the popup is the browser's, so there is nothing to place or hold open.
A form whose select needs any of these uses the listbox `Select` and carries the value in its
own state.

**Multi-select is unshipped, no longer impossible.** The value contract is `Binding<Option<T>>`,
single-select by construction. daisyUI's `.select[multiple]` rule now has the native element it
needs, but a multiple variant is a different contract (`Binding<Vec<T>>`) and a different visual
shape — daisyUI drops the caret and renders a scrolling list — so it is a follow-up rather than
a mode of this component.

**The submitted form value defaults to the option's position.** The positional mapping is what
frees `T` from string bounds, so a form that submits without an explicit
`NativeSelectOption::form_value` submits an index. Name a form value on each option when the
submitted string must mean something.

**A select whose value is `None` with no placeholder displays its first option.** The browser
requires a native select to display something; only a disabled placeholder option can render
"nothing chosen". The component documents the rule rather than inventing a hidden option the
caller did not write.

## daisyUI classes deliberately not used

- `select-md` - the default size emits nothing, and daisyUI renders an unmodified select at that
  size.
- `.select[multiple]` and its scrolling-list shape - multi-select is out of scope for the
  single-select value contract, recorded under Deviations.
- The `dropdown` and `menu` families the listbox `Select` borrows - the browser draws this
  popup, so there is no box or row to style.
- `validator` - pinned daisyUI can read Field metadata's `aria-invalid`, but this Component
  already emits its explicit `select-error` modifier from the same state. Adding `validator`
  would duplicate that Axis and also opt into browser-owned valid paint.
- `validator-hint` - its visibility requires a sibling after an element carrying `validator`.
  `FieldError` instead registers `aria-errormessage`, renders a polite live region, and follows
  producer metadata without that adjacency and class coupling.
- Responsive prefixes (`sm:select-lg` and the rest) - a caller reaches them through `class`,
  which concatenates.
