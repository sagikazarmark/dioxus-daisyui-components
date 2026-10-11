# Attribute names both sets define resolve to the element

A Component that forwards to a native element extends two attribute sets: `GlobalAttributes`,
for `class`, `id`, `role`, `aria-*` and the rest, and the element's own, for `disabled`,
`placeholder`, `type`. `#[props(extends = X)]` implements dioxus-html's `XExtension` trait for the
props builder, and `rsx!` turns a named attribute into a call of the method that trait carries.
Where both traits define the same name, the call is ambiguous and the caller gets E0034. In
dioxus-html 0.7.10 that is `autofocus` on `button`, `input`, `select` and `textarea`;
`spellcheck` on `input` and `textarea`; `tabindex` on `input`; `size` on `input` and `select`;
and `alt`, `width` and `height` on `input` and `img`, which the global set carries as style
attributes. A host writing `Input { autofocus: true }` could only fall back to the string key
`"autofocus": "true"`, which loses the type check and is found by nobody (#34).

Extending only the element set is not a way out: an element's extension trait carries its own
attributes and nothing else, so `class` and `id` would stop being nameable. The conflict is
upstream's (DioxusLabs/dioxus#5271), and its maintainers note it wants something like deref
specialisation to settle in the builder. Inside `rsx!` element syntax the element's attribute
already wins.

## The Component carries an inherent method per colliding name

An inherent method takes precedence over trait methods in Rust's method resolution, so each
affected Component adds an `impl` on its generated `<Name>PropsBuilder` with one method per
colliding name. Each method forwards to the element's extension trait, which resolves the name
to the element's attribute, as `input { autofocus }` does. The value type stays
`impl IntoAttributeValue`, so `spellcheck: false` and `spellcheck: "false"` both work, as they do
on the element. A name the Component already declares as a prop (`size` on Input and
NativeSelect, `alt` on AvatarImage) is shadowed by that prop and gets no method.

Typed props were the alternative. They would invent value types and doc comments for twenty-odd
attributes that ADR-0029 says a Native control spells as the element does, and would not go away
when upstream resolves the conflict.

## Consequences

- The `impl` names the macro-generated builder type and its generic shape (the field-state tuple,
  then the Component's own generics). Both are dioxus 0.7 implementation details; the comment on
  every block points at #5271 so the blocks are removed together once upstream resolves it.
- Each block is `#[allow(dead_code)]`: inside a host's private `components` module, an unused
  inherent `pub fn` warns where an unused trait method never did. Blocks hold only the forwarding
  methods, so the allow hides nothing else.
- `tests/registry_policy.rs` carries the collision table for dioxus-html 0.7.10 and fails on a
  Component that extends both sets without a method for each colliding name it does not declare
  as a prop. `tests/conformance/attribute_names.rs` names every one on every Component, and
  mounts the form controls to show the attribute reaches the native element, through the
  Composition sugar to the inner control. The table is recomputed when dioxus-html moves.
