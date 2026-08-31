# Breadcrumbs

An accessible daisyUI-styled breadcrumb trail with semantic navigation and ordered-list markup.

```rust
Breadcrumbs { aria_label: "Breadcrumb",
    BreadcrumbsList {
        BreadcrumbsItem { a { href: "/", "Home" } }
        BreadcrumbsItem { a { href: "/components", "Components" } }
        BreadcrumbsItem { "Breadcrumbs" }
    }
}
```

This is a Presentational component and wraps no Primitive. `Breadcrumbs` renders
`nav.breadcrumbs`, `BreadcrumbsList` renders its direct `ol`, and each `BreadcrumbsItem` renders a
direct `li`. Links are ordinary caller-owned anchors rather than a `BreadcrumbsLink` part, so they
can be native links, router links, or any other appropriate caller content without adding a router
or cross-Component dependency.

All three parts accept global HTML attributes. `BreadcrumbsList` also accepts native `ol`
attributes such as `start` and `reversed`, and `BreadcrumbsItem` accepts the native `li` `value`
attribute.

The caller must label every breadcrumb navigation landmark with `aria_label` or
`aria_labelledby`. Ancestor pages are links. The current page can be plain text, in which case
`aria-current` is optional; if the current page is also a link, put `aria_current: "page"` on that
link. The Component does not infer a current item or add accessibility attributes to caller
content.

daisyUI generates separators with a `::before` pseudo-element on every `li` after the first. Do
not put separator characters in the markup. Links, current-page text, and any wrapper used to group
an icon with text stay direct children of `BreadcrumbsItem`, where daisyUI's direct-child selector
supplies spacing, hover underlining, and the focus-visible outline. An icon grouped with link text
remains inside that direct link child.

## State bridging

There is no state to bridge because the component has no state; neither Tier applies. A breadcrumb
trail adds no focus management, keyboard interaction, current-item state, or ARIA wiring. Native
links retain their browser behaviour, and daisyUI styles their native `:focus-visible` state.

`dioxus-primitives` is still declared as a dependency, as every Component declares it, because
`merge_attributes` lives there.

## Axes

There are no Axes. daisyUI exposes only the structural `breadcrumbs` class; width, text size,
colour, and other presentation remain caller classes. Caller classes concatenate on all three
parts, and caller attributes pass through to their semantic elements.

## Deviations

daisyUI documents `div.breadcrumbs > ul > li`. This Component deliberately renders
`nav.breadcrumbs > ol > li`. daisyUI's stylesheet applies the same rules to a direct `ol` as to a
direct `ul`, while the `nav` landmark and ordered list express the breadcrumb pattern's navigation
and hierarchy semantics without additional roles. The caller remains responsible for the
landmark's accessible name.

## daisyUI classes deliberately not used

- `link` and its modifiers - breadcrumb links are direct caller content, and the `breadcrumbs`
  selectors already provide their hover and focus-visible treatment.
- `menu` - a breadcrumb trail is an ordered hierarchy, not an interactive menu.
- Width and typography utilities from daisyUI's examples - callers add these through `class`; a
  constrained `Breadcrumbs` scrolls horizontally using daisyUI's own overflow rule.
