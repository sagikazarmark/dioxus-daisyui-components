# Fieldset

A native form group and legend, styled with daisyUI's `fieldset` and `fieldset-legend` classes.
Labels, controls, descriptions, and actions remain caller content.

[Live examples](https://daisyui-components.dioxus.cc/components/fieldset) ·
[their sources](docs/examples/)

`Fieldset` renders a real `fieldset`, and `FieldsetLegend` renders a real `legend`. Use the
standalone Label Component when a control needs an explicitly associated caption. Its required
`html_for` becomes the label's `for` attribute, so a control with the matching id is announced by
and operated through that caption.

Keep `FieldsetLegend` as the first direct child of `Fieldset`. HTML gives only that first direct
legend the group's caption semantics. It is also the boundary of the native disabled exception:
when `Fieldset::disabled` is true, descendant controls are disabled except for controls inside the
first legend.

`form` associates the fieldset itself with a form and `name` names it in that form's controls
collection. The association does not propagate to descendant controls outside that form; those
controls still need their own `form` attribute. A fieldset is not itself submitted as a name/value
entry.

## State bridging

There is **no state to bridge** and no Registry state is added. The browser owns the fieldset's
accessible group name, form owner, disabled descendants, and first-legend exception on the native
elements. The Registry neither mirrors nor lifts any of that state; neither Tier applies.

## Axes

There are no styling Axes. Both Compound parts always emit their one daisyUI class. Width, border,
background, padding, and radius are caller-owned utilities passed through `class`, where they
concatenate with `fieldset`.

## Deviations

**Labels remain separate Components.** The pinned daisyUI documentation lists `label` as a
Component rather than a Fieldset part and demonstrates it as caller content inside a fieldset.
Fieldset therefore does not duplicate the standalone Label interface or install it implicitly;
callers may compose Label, Field parts, native labels, or controls without labels as appropriate.

**The deprecated fieldset label class is not emitted.** The pinned daisyUI 5.7.17 CSS still defines
`fieldset-label`, but marks it deprecated in favour of `label`, matching the documentation. The
standalone Label Component remains the owner of both label appearances.

**First-child placement remains composition.** A Compound part cannot inspect or reorder its
siblings, so `FieldsetLegend` cannot enforce that it is first. The Preview keeps it as the first
direct child in every Example, and callers must preserve that order to retain native legend and
disabled-exception semantics.

## daisyUI classes deliberately not used

- `label` - owned by the standalone Label Component and composed as caller content.
- `fieldset-label` - deprecated by the pinned daisyUI CSS in favour of `label`.
- `input`, `select`, `checkbox`, and other control classes - the fieldset groups caller-provided
  controls without owning or configuring them.
- `bg-base-*`, `border-*`, `rounded-*`, width, and padding utilities - these are box design choices,
  not Axes or defaults owned by the Registry.
- Responsive-prefixed fieldset classes - callers add them through `class`, which concatenates.
