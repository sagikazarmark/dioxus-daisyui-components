# Tabs

Tabs styled with daisyUI's `tabs` classes, wrapping the `dioxus-primitives` tabs. Arrow-key
navigation, roving focus, the `tab` and `tabpanel` roles and the `aria-selected` and
`aria-controls` wiring are the primitive's rather than reimplemented here, and the panels show
and hide through daisyUI's own rules rather than through anything this component emits.

[Live examples](https://daisyui-components.dioxus.cc/components/tabs) ·
[their sources](docs/examples/)

**A trigger and its panel are written as a pair, in order.** That is not a style preference:
daisyUI reveals a panel only from the tab immediately before it, so a panel written anywhere
else stays invisible. `index` orders keyboard navigation and pairs a trigger with the panel it
points `aria-controls` at, and `value` is what the pair is activated by; both have to match
across the pair.

## Composition

The compound parts are `Tabs`, `TabTrigger` and `TabContent`. There is no collapsed component:
the parts interleave, so there is nothing to collapse them into.

**The primitive's `TabList` part is an omitted part** (ADR-0003). The `tabs` class and the
`tablist` role are on `Tabs` itself, and triggers and panels are its direct children.

This is forced by daisyUI's selectors, all four of which reach across the tree the primitive
would otherwise build:

- `.tab:is(.tabs > .tab)` styles a tab only where it is a **direct child** of the tabs element.
- `.tab:is(.tab-active, [aria-selected=true], …) + .tab-content` reveals a panel only from the
  **adjacent** active tab, and `.tab-content` is `display: none` in daisyUI's base rule, so a
  panel that rule never reaches is permanently invisible.
- `.tabs-lift > .tab-content` and `.tabs-box > .tab-content` key the joined borders on the panel
  being a direct child too.
- `.tabs-lift:has(> .tab-content) > .tab:first-child` squares the first tab off only when the
  panels are siblings of the tabs.

In the primitive's default tree the triggers sit inside `TabList` and the panels sit outside it,
so no trigger is ever adjacent to a panel and not one of those rules matches.

Omitting the part costs nothing behaviourally. `TabList` is a bare `<div role="tablist">` with no
context, no state and no keyboard handling: triggers consume the context `Tabs` provides, and
they register with the focus collection by explicit `index` rather than by DOM position, which
is exactly why a panel can sit between two triggers without disturbing the order they are
navigated in.

`id` is a prop of its own on both `TabTrigger` and `TabContent` rather than an attribute that
falls through, because the primitive takes one as a prop and puts it on the element itself. For
a panel it also generates one when the caller passes none, and points the trigger's
`aria-controls` at it; an id arriving as an attribute would replace the one the trigger is still
naming.

## State bridging

**Tier 1** on the active tab, and nothing to emit at all.

daisyUI's rule is `.tab:is(.tab-active, [aria-selected=true], [aria-current=true],
[aria-current=page])` and the primitive sets `aria-selected` on the trigger it renders. The
attribute daisyUI already matches is exactly there, so no modifier class is emitted for the
active tab and no state is lifted; the active tab and the inactive ones carry byte-identical
class attributes.

The disabled state needs no bridging either, though it is not Tier 1: that tier is about ARIA
attributes, and this is a native one. The primitive sets `disabled` on the `button` it renders,
and daisyUI's rule is `.tab[disabled]`, which matches it directly.

Showing and hiding a panel is not bridged either, and this is the part worth being precise
about: it is daisyUI's adjacent-sibling rule that switches `display` on, and the structural
rearrangement above is what makes that rule reach. The primitive independently marks an inactive
panel `hidden` and renders no children into it, so an inactive panel is hidden twice over, but
the visible one is visible because of daisyUI, not in spite of it.

## Axes

- `appearance: TabsAppearance`: `tabs-border`, `tabs-lift`, `tabs-box`, on the tabs element.
- `size: TabsSize`: `tabs-xs`, `tabs-sm`, `tabs-lg`, `tabs-xl`, on the tabs element.
- `appearance: TabContentAppearance` on `TabContent`: `border-base-300 bg-base-100 p-6`.

Both of the first two default to a value that emits nothing. `TabsAppearance::Default` is
daisyUI's plain row of tabs, where the active one is told apart by its text colour alone;
`TabsSize::Default` renders at the same size as daisyUI's explicit `tabs-md`, so that class is
not emitted.

The panel's axis inverts the usual convention: its default value *emits* utilities and its
`None` value emits nothing. daisyUI's `tab-content` sets a border width but leaves the colour
transparent and paints no background, so the joined look of the lifted and boxed appearances
needs a surface that daisyUI has no class for; the utilities it uses in its own examples are
emitted here instead. What a caller would otherwise have to override is then a Tailwind utility
this component emitted, and two utilities only tie, with the tie settled by generated-stylesheet
order rather than by the class attribute. Switching ours off is how a caller wins it (ADR-0004).
It is not named `style`, which collides with the global HTML attribute.

Each enum exposes `ALL`, listing every value of the axis. The preview iterates it to render
every axis value and the browser specs assert computed styles over the same rendered set.

## Deviations

**The tablist owns the panels, which ARIA disallows.** This is the sharpest trade-off in the
registry and is recorded here in full rather than buried.

`role="tablist"` permits only `tab` children. Reproducing daisyUI's markup puts the panels in
there as well, so a strict validator will flag every set of tabs this component renders.

What that costs in practice is one stray child of the tablist. An inactive panel carries both
the primitive's `hidden` attribute and daisyUI's `display: none`, so it is absent from the
accessibility tree altogether; the active panel is the one exception, and it is announced
correctly: the trigger points `aria-controls` at it, it carries `role="tabpanel"`, and it takes
focus in sequence after the tab bar. Keyboard navigation, focus order, `aria-selected`,
`aria-controls` and panel announcement are all unaffected, and this is the markup daisyUI itself
documents and ships.

**The escape hatch is the primitive.** A caller who needs a strictly conforming tablist uses
`dioxus_primitives::tabs` directly (`TabList` is still there) and styles the panels
themselves, which means giving up daisyUI's `tab-content` along with the `tabs-lift` and
`tabs-box` looks that depend on it. There is no third option: making the tree conform and
keeping daisyUI's appearance would take CSS, which ADR-0002 rules out.

**Vertical tabs are not supported in this version.** The primitive reports its orientation and
navigates by the arrow keys that match it, and daisyUI has no vertical tabs at all: its tabs
element is a horizontal flex row in every appearance. So the orientation is pinned horizontal
here rather than passed through: a vertical set would announce and navigate one way while
rendering the other. A caller who needs one uses the primitive directly.

**A tab is a `button`, not an anchor or a radio input.** That is the primitive's markup, and
daisyUI's rule has no element-type selector, so `tab` styles it as it stands. The radio-input
form of daisyUI's tabs (where the state lives in a checked input and needs no script) is the
one this component replaces outright, since it is exactly the behaviour the primitive is here
for.

## daisyUI classes deliberately not used

- `tab-active`: the whole of Tier 1 here. daisyUI's active rule matches the `aria-selected`
  attribute the primitive already sets, so emitting the class as well would make two ways to
  express one state, and the class would be the one that could go out of step.
- `tab-disabled`: daisyUI's own rule already matches the `disabled` attribute the primitive
  sets on the tab, so the class would be a second expression of the same state.
- `tabs-top` and `tabs-bottom`: daisyUI's placement modifiers, which move the panel above or
  below the tab bar. That is a placement axis rather than an appearance one and is not exposed
  in this version; `tabs-top` is also what an unmodified set already renders as.
- The radio-input tabs markup (`tab` on an `input[type=radio]`, with `aria-label` drawn as the
  tab's text): the CSS-only tabs, replaced outright by the primitive's state.
- The responsive prefixes daisyUI generates for all of the above (`sm:tabs`, `md:tab-content`,
  and so on): a caller reaches them through `class`, which concatenates.
