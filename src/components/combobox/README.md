# Combobox

A field that filters a list as it is typed into, styled with daisyUI's `input`, `dropdown` and
`menu` classes, wrapping the `dioxus-primitives` combobox. The filtering, the typeahead-free
arrow-key navigation, the `combobox` and `listbox` roles, the `aria-activedescendant`,
`aria-expanded` and `aria-controls` wiring and the dismissal on Escape and on a blur are all the
primitive's rather than reimplemented here.

```rust
rsx! {
    Combobox::<String> {
        on_change: move |value| tracing::info!("chose {value:?}"),
        on_commit: move |()| tracing::info!("selection committed"),
        on_focus_exit: move |()| tracing::info!("focus left the combobox"),

        ComboboxInput { color: ComboboxColor::Primary, placeholder: "Pick a fruit" }
        ComboboxList {
            ComboboxOption::<String> { value: "orange".to_string(), index: 0usize, "Orange" }
            ComboboxOption::<String> { value: "lemon".to_string(), index: 1usize, "Lemon" }
            ComboboxOption::<String> {
                value: "currant".to_string(),
                index: 2usize,
                disabled: true,
                "Currant"
            }
            ComboboxEmpty { "No fruit by that name" }
        }
    }
}
```

An option's `index` is its position in the keyboard navigation order, and it is explicit because
the primitive's focus collection is ordered by it rather than by the DOM, which is what makes the
list-item wrappers below free, and what keeps the order steady while a query hides some of the
options.

This is the select's structure with one part swapped. daisyUI has no combobox and no styled
listbox at all, so the popup is borrowed from daisyUI's `dropdown` and `menu` exactly as the
select's is (ADR-0005). What is not borrowed is the field: a select's field is a `button` wearing
`select`, and this one is the `input` that daisyUI's `input` class was written for.

## Composition

The compound parts are `Combobox` (the dropdown), `ComboboxInput` (the field), `ComboboxList` (the
box, with the menu inside it), `ComboboxOption` (one row) and `ComboboxEmpty` (the line shown when
the query keeps no option). There is no collapsed component: the field and the list are two
elements a caller writes between, so there is nothing to collapse.

`binding` and `meta` accept explicit `dioxus-field` values on `Combobox`. Without them, the
Component resolves Field Context and then standalone state. The Binding owns the selected
`Option<T>` only; open state and query remain separate combobox state. Field metadata supplies the
input's id, name, required and disabled state, ARIA relationships, and `data-*` state. Explicit
root state props, the input's explicit `id`, and caller attributes win over metadata.

**The popup's classes split across two elements** (ADR-0005). `dropdown-content` plus the box's
utilities go on the primitive's list `div`, `menu` goes on a `ul` rendered inside it, and each row
is wrapped in an `li`:

```html
<div class="dropdown dropdown-bottom dropdown-start dropdown-open">
  <input class="input" role="combobox" aria-expanded="true" aria-activedescendant="…">
  <div class="dropdown-content bg-base-100 rounded-box shadow-sm" role="listbox">
    <ul role="none" class="menu w-full">
      <li role="none" class="empty:hidden"><div role="option" class="…">Orange</div></li>
    </ul>
  </div>
</div>
```

**A list item hides itself when it is empty.** An option the query drops renders nothing at all
(the primitive returns an empty element rather than a hidden one), which would leave an empty `li`
behind, and daisyUI draws `li:empty` as a divider rule. So each wrapper carries `empty:hidden`, and
a list filtered down to one option shows one row rather than one row and four rules.

**The list is emitted whether the popup is open or not.** While the popup is closed the primitive
renders its children where they stand rather than inside its own element, so that every option can
register the text the field displays when it is the chosen one. The list is hidden instead, by a
utility that fires exactly when it is not inside the box: `[:not(.dropdown-content)>&]:hidden`.

## State bridging

**Tier 2** on the open state and on the chosen option, and neither is cosmetic.

- The open state is **lifted** (ADR-0006). daisyUI hides `.dropdown-content` outright unless
  `dropdown-open` is on the element above it, and matches no attribute the primitive sets. This
  component seeds a signal from `default_open`, always hands the primitive a controlled value and
  intercepts the callback.
- The chosen option is **lifted** for the same shape of reason: daisyUI marks the chosen row of a
  menu with `menu-active`, and `.menu` matches no ARIA attribute at all: not `aria-selected`,
  which the primitive does set on the option, not anything else. The primitive's context is
  private, so an option cannot read the value from it; `Combobox` provides it alongside and
  `ComboboxOption` compares its own value against it.
- The disabled option is Tier 2 as well, on the wrapper: daisyUI mutes a disabled row through
  `.menu-disabled` on the list item, and the primitive reports the state as `aria-disabled` and
  `data-disabled`.

**The highlighted option is neither tier.** It is a **Bridged utility**,
`data-[highlighted=true]:menu-focus`, and ADR-0021 records why: focus stays in the field while the
list is walked, so daisyUI's own `:focus-visible` rule (the one that gives the select its
highlight for free) never matches here.

**The query is not bridged and not lifted.** No class depends on what has been typed, so the query
is passed straight through to the primitive, which stays its only owner.

**The disabled field is not bridged either.** The primitive puts the native `disabled` attribute on
the `input` from either the explicit root prop or Field metadata, and daisyUI's rule is
`.input:is(:disabled,[disabled])`. It is a native attribute rather than an ARIA one, so it is not
Tier 1; there is nothing to bridge.

**Tier 2** on producer-defined invalidity: when the input's colour Axis is omitted and Field
metadata is invalid, `ComboboxInput` emits `input-error`. Passing any colour explicitly, including
`ComboboxColor::Default`, wins over metadata. The Registry does not compute invalidity.

## Conformance

This control is **field-aware**. An explicit `binding: Binding<Option<T>>` wins over Field Context;
without either, `Combobox` owns standalone selection state seeded by `default_value`. The
lower-level trio remains available: `value: Option<ReadSignal<Option<T>>>` can override the
rendered Binding value, `on_change` observes each successful option selection, and `on_commit`
observes that selection as a completed interaction unit. Writes and commits still reach the
resolved Binding. A successful keyboard or pointer option selection is the exact Commit boundary,
so change and Commit have the same cadence. Typing a query, opening, closing, and Escape do not
Commit. The query is not part of the Binding contract and continues through its dedicated props.

Focus Exit is independent. The complete logical focus scope is the outer `Combobox`, including its
input, popup listbox, and every option. Keyboard highlighting and selection keep DOM focus in the
input; pointer selection also retains input focus. Movement anywhere inside that complete scope
reports nothing. One confirmed departure calls `Binding::focus_exit()` and then the optional
dependency-free `on_focus_exit` prop; selection, Escape, popup open changes, closing, and an
individual child blur do not imply Focus Exit. Neither Commit nor Focus Exit infers form touched,
blurred, validation, or other form-library semantics.

Field focus requests call `set_focus` on the primitive-rendered input handle captured at mount.
Root attributes continue to spread onto the dropdown element; input metadata and caller input
attributes spread onto the actual control.

## Axes

- `color: Option<ComboboxColor>` on the field: `input-neutral`, `input-primary`,
  `input-secondary`, `input-accent`, `input-info`, `input-success`, `input-warning`, `input-error`;
  omission permits invalid Field metadata to emit `input-error`.
- `size: ComboboxSize` on the field: `input-xs`, `input-sm`, `input-lg`, `input-xl`.
- `side: ComboboxSide` on the root: `dropdown-top`, `dropdown-bottom`, `dropdown-left`,
  `dropdown-right`.
- `align: ComboboxAlign` on the root: `dropdown-start`, `dropdown-center`, `dropdown-end`.
- `size: ComboboxListSize` on the list: `menu-xs`, `menu-sm`, `menu-lg`, `menu-xl`.
- `appearance: ComboboxListAppearance` on the list: the box's fill, corners and shadow, or
  nothing.
- `appearance: ComboboxOptionAppearance` on an option: the keyboard highlight, or nothing.

The colour and size class strings are the input's, duplicated rather than depended on:
cross-Component dependencies are for public composition, while the Tailwind contract requires every
one of these classes to be a literal in the file that emits it.

An explicit `ComboboxColor::Default` and `ComboboxSize::Default` emit nothing, which is daisyUI's
own uncoloured field at its own default size. Omitting colour is distinct: invalid Field metadata
can then emit `input-error`. The side and align axes emit a class for every value,
including their defaults, for the reason ADR-0008 records. The two appearance axes are the inverted
shape ADR-0004 describes: their `Default` emits and their `None` emits nothing, because what they
carry is utilities this component emits rather than daisyUI component classes.

Every enum exposes `ALL`, listing every value of the axis. The preview iterates it to render every
axis value and the browser specs assert computed styles over the same rendered set.

## Deviations

**The Primitive's value callback is adapted to the Binding trio.** Its `on_value_change` callback
is exposed as `on_change`, and each successful option selection also fires `on_commit`. This is a
deliberate breaking rename at Registry version `0.0.0`; the wrapped Primitive still receives its
native callback.

**Field metadata belongs to the input even though selection belongs to the root.** `Combobox`
resolves the Binding and metadata because it owns selected and disabled behavior, then makes the
resolved metadata available to `ComboboxInput`, which is the actual control and focus target. The
query is deliberately not a second field value.

**Native form participation therefore describes the input, not the selected Binding value.** A
metadata or explicit `name` and `required` land on the native input with the other Field attributes,
so browser submission and required validation see the input's displayed query text. The selected
`Option<T>` remains the Field-shaped value and must be submitted through the producer that owns its
Binding; this Component cannot serialize a generic `T` into a hidden native control.

**There is no typeahead, and there does not need to be.** The select has one because its field
cannot be typed into; here the typing *is* the interaction, and what it does is filter. The filter
is a prop, defaulting to the primitive's own case-insensitive substring match.

**The chosen value and the query are different things, and the field shows each in turn.** With the
popup open the field shows what has been typed; closed, it shows the chosen option's text. That is
the primitive's, and it is what makes a combobox recover from a query that chose nothing.

**Reopening the popup therefore shows an empty field, whatever is chosen.** Every path that opens
one empties the query first, and the field shows the query while it is open. The value survives:
the chosen option keeps `menu-active`, and the field fills back in on close. But for as long as the
popup is open the field reads as though nothing was ever chosen, and since choosing a different
option means opening the popup, a reader sees their choice thrown away each time they go to change
it. Nothing here can reach it (the primitive keeps `ComboboxContext` to itself, so the field can
only be wrapped, not composed) and it is filed upstream as
[DioxusLabs/dioxus-components#294](https://github.com/DioxusLabs/dioxus-components/issues/294).
**The preview does not list this component while that stands**, though its page is still at
`/components/combobox` and the browser suite still drives it.

**Escape closes the popup while focus remains in the field.** It does not Commit a selection or
report Focus Exit; a later departure from the complete combobox scope is separate.

**A combobox is single-select.** The primitive offers no multiple-selection combobox, so neither
does this.

## daisyUI classes deliberately not used

- `select`, `select-*`: daisyUI's field for a list of options. It paints a caret into padding it
  reserves at the inline end, which is the wrong shape for a field that is typed into: the caret
  would sit where the text is being written. The select component uses it; this one uses `input`.
- `menu-title` on anything but the empty line: daisyUI's heading inside a menu. The primitive has
  no group part for a combobox, so there is nothing to label; the class is used only for the line
  that says nothing matched, which is what keeps it from being padded and highlighted like a row.
- `menu-dropdown`, `menu-dropdown-toggle`, `menu-dropdown-show`: daisyUI's collapsible submenu. A
  combobox's list is one flat set of options the query filters, and a submenu would be a second
  place for options to hide.
- `dropdown-hover`, `dropdown-open` as an axis, `dropdown-close`: daisyUI's other ways of deciding
  whether a dropdown is open. The open state is the primitive's here, and `dropdown-open` is
  emitted from it rather than offered as something a caller sets.
- `input-ghost`: daisyUI's borderless field. Not exposed for the reason the button's other looks
  are not: a caller reaches it through `class`, which concatenates.
- `validator`: pinned daisyUI can read Field metadata's `aria-invalid`, but this Component already
  emits its explicit `input-error` modifier from the same state. Adding `validator` would duplicate
  that Axis and also opt into browser-owned valid paint for the query input rather than the selected
  value.
- `validator-hint`: its visibility requires a sibling after an element carrying `validator`.
  `FieldError` instead registers `aria-errormessage`, renders a polite live region, and follows
  producer metadata without that adjacency and class coupling.
- The responsive prefixes daisyUI generates for these classes (`sm:input-lg` and the rest): a
  caller reaches those through `class`, which concatenates.
