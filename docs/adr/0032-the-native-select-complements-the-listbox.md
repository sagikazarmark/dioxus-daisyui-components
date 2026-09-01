# The native select complements the listbox

`NativeSelect` renders a real `<select>` carrying daisyUI's `select` class, and wraps no
Primitive. It is the second select in the registry, beside the primitives-backed listbox
`Select`, and the split is deliberate: the listbox earns its weight where the popup is the point
(styled rows, custom row content, typeahead across groups), while the native variant is for the
select fields most forms are made of. In mockbank's dioform spike, swapping a native `select`
for the registry listbox moved the release wasm by ~101KB gzipped: the primitive's popup, roving
focus, typeahead and focus trap, reached for the first time. daisyUI's own `select` is written
for the native element — `.select:open`, `.select select`, `option`, `optgroup`,
`.select[multiple]` and `::picker(select)` all name it — so four of the listbox README's
recorded deviations exist only because no native select was on offer.

The component is a Native control under ADR-0019 and field-aware under ADR-0028, in the Input
mold: `FieldSurface::NATIVE` applies fully, because `required`, `disabled`, validity and `name`
all have native spellings on a real `<select>`, unlike the listbox's `button` trigger. Per the
Input/InputField precedent, the closed `NativeSelectField` composition ships in the same
component, which is why the manifest pins the `field` Component.

## A separate installable unit

The native select is its own Component directory and manifest rather than a variant inside
`select`. The point of the component is its weight: installing and rendering it must pull no
`dioxus-primitives` select machinery into the binary, and a shared manifest would drag the
listbox's primitive dependency along with it. The name borrows the **Native control** vocabulary
`CONTEXT.md` already defines, and leaves the published listbox's name alone.

## The component owns the value mapping, positionally

A native `<select>` speaks option-value strings, and the Binding contract is
`Binding<Option<T>>` — what selection producers now emit. The closed `options` prop of
`NativeSelectOption<T>` values is where the two meet: each option's value string is its position
in the list, mapped back by first match, so `T` needs no `Display` or `FromStr` bound and
duplicate values resolve deterministically. `PartialEq` is used only to mark the selected
option.

A positional string is the wrong thing for a form to submit, so an option may carry an explicit
`form_value` string that replaces its index — form participation is a headline reason this
component exists, and submitting an opaque index would undercut it. Without one, the submitted
value is the index, and the README records it. The emitted strings are required unique and
non-empty, asserted at render: the browser reports a pick as the option's value string and
nothing else, so a `form_value` colliding with another option's index, or with the placeholder's
reserved empty string, would be ambiguous in the one place the component hears about a pick.

The component drives the browser's selection through both the `value` property on the `select`
and the `selected` attribute on the matching option. Neither alone is safe: the property write
selects nothing if it lands before the options exist, and the content attribute goes inert once
the user has picked (the dirty-value flag). The browser specs pin both windows.

## `None` is the placeholder, and never a write

A `None` value selects daisyUI's placeholder pattern: a disabled first `option` with an empty
value string, rendered whenever the `placeholder` prop is set and still listed after a choice is
made, exactly as daisyUI documents it. The option is disabled, and the empty string parses to no
option, so a `None` write never arrives from the control — which composes with a producer's
required-path refusal policy rather than depending on it. A select whose value is `None` with
neither `placeholder` nor `default_value` displays its first option while the Binding holds
`None`; that is the native element's own rule, documented rather than papered over.

## Groups and multiple are out of scope

The `options` prop is flat: a children-based `option`/`optgroup` form would hand the value
strings back to the caller and reopen the mapping the closed prop exists to own. Groups can
arrive later as a nested options shape without breaking the flat form. Multi-select is
single-select by construction of the value contract; daisyUI's `.select[multiple]` rule now has
the native element it needs, but a multiple variant is a different contract
(`Binding<Vec<T>>`) and a different visual shape, so it is a follow-up rather than a mode. Both
absences are recorded in the README's listbox comparison, which is where a caller who needs them
is pointed back to the listbox.

## Consequences

- The registry has two selects, split by what the popup is for; each README points at the other.
- The **Native control** entry in `CONTEXT.md` gains the native select; the reasoning is
  ADR-0019's, arrived at for the one daisyUI component whose class was written for a native
  element all along.
- Four listbox deviations do not apply here: the caret flips from `.select:open`, the browser
  owns picker focus end to end, the placeholder is a real disabled option, and the selected
  value submits natively. Multi-select moves from impossible to unshipped.
- The first generic Field Composition sugar: `NativeSelectField<T>` forwards `options` and the
  `Binding<Option<T>>` surface, so it is generic where `InputField` and `TextareaField` are not.
- `NativeSelectOption<T>` is the registry's first locally-defined data-carrying prop struct.
  Its fields are private behind a constructor and builder methods, so it can grow without
  breaking callers.
- The appearance axis exposes `select-ghost`, following the Input and Textarea mold; the listbox
  declined the same axis, and the two selects diverge there because this one is built like the
  Native controls.
