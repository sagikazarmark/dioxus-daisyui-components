# The combobox's highlight cannot be `:focus-visible`

The select gets daisyUI's keyboard highlight for nothing. `.menu` writes its highlight against
`:focus-visible` as well as against its own `.menu-focus`, and the select's primitive moves real
DOM focus onto the option the arrow keys are on, so the option that is focused is the option that
is painted, with no class emitted from Rust and nothing recomputed.

A combobox cannot do that, and the reason is the pattern rather than the primitive. The field is a
text input: everything typed has to keep going into it while the list is walked, so focus never
leaves it. The option the arrow keys are on is named by `aria-activedescendant` on the field, which
is how ARIA says "the keyboard is here" without moving focus there. The primitive reports the same
thing on the option itself as `data-highlighted`.

daisyUI matches neither. `aria-activedescendant` appears nowhere in its stylesheet, and
`data-highlighted` is not a name daisyUI knows.

## The resolution

`ComboboxOption` emits daisyUI's own `menu-focus` as a **Bridged utility**: a Tailwind variant of
the attribute the primitive already sets:

```
data-[highlighted=true]:menu-focus
```

Nothing is recomputed in Rust, and the primitive stays the only owner of which option is
highlighted. That is what tells this apart from the Lifted state ADR-0006 describes and from the
Mirrored state of ADR-0011: both of those exist so that a *component* can emit a class from state
it keeps, and there is no state kept here at all.

It is the fourth Bridged utility in the registry, after the slider's disabled muting, the tag's
selection ring and the menu bar trigger's open ring, and the first written as a variant that emits
a daisyUI component class rather than a Tailwind one. That is deliberate: daisyUI has the class for
this state, it simply has no selector that reaches it. Whether a variant of a daisyUI class
survives Tailwind's scan and applies is not something a class name in a source file can promise, so
the browser suite asserts the highlight paints rather than asserting the class is present.

## Consequences

- The highlight is an axis, `ComboboxOptionAppearance`, because it is a utility this component
  emits and ADR-0004 requires those to be defeatable. Switched off, an option is painted only on
  hover and when chosen.
- The hover highlight is unaffected and needs nothing: `.menu`'s hover rule is a plain `:hover`,
  which is the pointer rather than the keyboard.
- A screen reader hears the same thing either way. `aria-activedescendant` is what announces the
  option to a screen reader, and it is the primitive's; the class is paint for people who can see
  it.
- If a later daisyUI release matches `aria-activedescendant` or a `data-*` attribute of its own,
  this becomes Tier 1 and the axis goes away.
