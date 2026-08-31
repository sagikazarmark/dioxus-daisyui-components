# Tabs interleaves triggers and panels in one container

The tabs component puts `.tabs` and `role="tablist"` on the outer element, does not expose the
primitive's `TabList` part, and renders triggers and panels interleaved as direct children.
This reproduces daisyUI's canonical tabs markup exactly.

daisyUI shows a panel only via `.tab:is(.tab-active,[aria-selected=true],…) + .tab-content`,
an adjacent-sibling rule, and gives `.tab-content` `display:none` in its base rule. It also
keys `tabs-lift` and `tabs-box` panel borders on `:has(>.tab-content)`. The primitive's default
tree puts triggers inside `TabList` and panels outside it, so no trigger is ever adjacent to a
panel and every panel would be permanently invisible. `TabList` turns out to be a bare
`<div role="tablist">` with no context, state, or keyboard handling (triggers consume the
context provided by `Tabs` and register focus by explicit index), so removing it costs nothing
behaviourally.

## Consequences

- `role="tablist"` owns tabpanels as well as tabs, which ARIA 1.2 disallows. Inactive panels
  carry both `hidden` and `display:none` and so are absent from the accessibility tree, leaving
  one stray child. Keyboard navigation, focus order, `aria-selected`, `aria-controls`, and
  panel announcement are unaffected. This is the markup daisyUI itself documents.
- Callers who need a strictly conforming tablist use `dioxus_primitives::tabs` directly and
  style the panels themselves.
- Establishes the general rule that a primitive part may be omitted when it carries no
  behaviour and its element blocks a daisyUI selector.
