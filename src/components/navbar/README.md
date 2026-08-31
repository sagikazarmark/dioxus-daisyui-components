# Navbar

A navigation bar styled with daisyUI's `navbar`, `btn`, `dropdown` and `menu` classes, wrapping
the `dioxus-primitives` navbar.

```rust
Navbar { aria_label: "Primary",
    NavbarStart {
        NavbarNav { index: 0usize,
            NavbarTrigger { "Products" }
            NavbarContent {
                NavbarItem { index: 0usize, value: "new", to: "/new", "New" }
                NavbarItem { index: 1usize, value: "popular", to: "/popular", "Popular" }
            }
        }
    }
    NavbarCenter {
        NavbarItem { index: 1usize, value: "home", to: "/", class: "btn btn-ghost", "Home" }
    }
    NavbarEnd { "Account" }
}
```

`NavbarStart`, `NavbarCenter` and `NavbarEnd` are separate parts because daisyUI writes them as
sibling regions and callers may use any subset. Dropdowns and direct links share the root `index`
order; items inside a dropdown have their own order.

## State bridging

**Tier 2** marks a disabled popup item, and **Bridged utilities** mark an open trigger.

The primitive reports `data-state` on `NavbarNav`, but has no controlled open prop or callback.
The trigger ring is therefore a variant of that attribute through the wrapper's `group` class.
State is neither recomputed nor owned by the registry. Popup visibility is daisyUI's own
`dropdown-hover` and `:focus-within` behavior.

A disabled item is Tier 2 on its presentational `li`: daisyUI's `menu-disabled` class supplies the
muted, pointer-inert state that the primitive's `data-disabled` does not match.

## Axes

- `color: NavbarTriggerColor` on a trigger: the button colour family.
- `size: NavbarTriggerSize` on a trigger: `btn-xs` through `btn-xl`.
- `open_appearance: NavbarTriggerOpenAppearance` on a trigger: the Bridged open ring, or nothing.
- `size: NavbarMenuSize` on a popup: `menu-xs` through `menu-xl`.
- `appearance: NavbarContentAppearance` on a popup: its box utilities, or nothing.

The trigger and menu class strings are duplicated rather than depending on other registry
components. Both appearance axes have a `None` value that emits no utilities, so callers replace
the open marker or popup paint without competing with registry utilities (ADR-0004). Every enum
exposes `ALL` for the Preview and browser tests.

## Deviations

**Triggers are buttons, not daisyUI menu items.** This is the same primitive tree and selector
constraint ADR-0018 records for the menubar, so navbar triggers carry `btn` rather than repeating
that reasoning here.

**The dropdown is split.** ADR-0005's structure is used again: `dropdown-content` and the box are
on the primitive's content element, `menu` is on a presentational `ul` inside it, and every popup
item is wrapped in a presentational `li`. Direct bar links are not list items. The primitive keeps
the `menu` and `menuitem` roles.

**Open visibility does not use `dropdown-open`.** daisyUI makes the first child carrying
`tabindex` ignore pointer events while that modifier is present. The primitive always puts
`tabindex` on the trigger, so using the modifier would let a trigger open its menu but prevent the
second pointer press from closing it. The primitive opens on hover and keeps keyboard focus inside
an open nav, so daisyUI's `dropdown-hover` and `:focus-within` selectors reveal the content without
another utility bridge.

**The popup does not collide-detect.** This matches daisyUI's dropdown: a caller needing viewport
collision handling supplies a floating-element solution.

## daisyUI classes deliberately not used

- `menu-horizontal` and `menu-vertical` on the bar: the bar has daisyUI's purpose-built `navbar`.
- `menu-dropdown`, `menu-dropdown-toggle`, and `menu-dropdown-show`: CSS-only submenu state would
  duplicate the primitive's state.
- `dropdown-open`: it disables pointer events on the primitive's focusable trigger, as above.
- `btn-active`: the primitive does not expose open state to Rust; the trigger uses a Bridged
  utility instead.
- `menu-active`: navbar links are destinations, not a selection model. A caller may use the
  primitive's `active_class` when route activity should be shown.
- Responsive variants of the axis classes: callers add those through `class`.
