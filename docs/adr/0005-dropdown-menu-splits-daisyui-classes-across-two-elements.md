# Dropdown menu splits daisyUI's classes across two elements

daisyUI puts `dropdown-content` and `menu` on a single `<ul>`. The registry splits them:
`dropdown-content` plus the box utilities go on the primitive's content `<div>`, and `menu`
goes on a `<ul role="none">` rendered inside it, with each item wrapped in `<li role="none">`.

Three constraints force at least two elements. `DropdownMenuContent` renders a hardcoded
`<div>` and, unlike `DropdownMenuTrigger`, has no `as` prop. `DropdownMenuContext` is private,
so a replacement content component could never reach the open state, trigger id, or focus
collection. And every visual rule daisyUI's `.menu` applies to items is of the form
`.menu :where(li:not(.menu-title)>…)`, so literal `li` elements are required. Given two
elements are unavoidable, each class goes on the element whose layout it was written to drive.

The `role="none"` wrappers exist because the primitive renders `role="listbox"` on the content
and `role="option"` on items; interposing plain `ul`/`li` would break ARIA's listbox-to-option
ownership. Wrapping is behaviourally free because items register with the focus collection by
explicit index, not DOM position.

The primitive's `listbox`/`option` roles are left alone rather than rewritten to
`menu`/`menuitem`, despite the component's name. Changing them would also mean overriding
`aria-haspopup` on the trigger and diverging from the roles the primitive's keyboard handling
was built around; if upstream corrects them, the registry inherits the fix.

## Consequences

- The box utilities and `.menu`'s own defaults overlap (`width:fit-content`, `padding:.5rem`),
  so daisyUI's example class list is split deliberately rather than copied.
- The same borrowed-dropdown structure applies to `select`.
