# The preview is themed by the component it documents

The preview carries no `data-theme` anywhere. Its theme comes from the header's theme controller,
through daisyUI's own rule:

```css
:root:has(input.theme-controller[value=dark]:checked) { … }
```

and daisyUI's base layer paints the document root from it. The registry's own component is what
themes the site that documents it.

Until now the shell pinned the theme on a wrapper `div` below the root, from the `?theme=` in the
URL. That worked, and it masked the mechanism completely: a `[data-theme]` on an element wins for
everything inside it, so the controller's `:has()` rule landed on a root nobody could see. The
switcher in the header had to navigate to change anything, and the component was a bystander in
the one app whose job is to demonstrate it. ADR-0019 recorded that as a cost. This ADR pays it
back.

## What made it possible

**No control on a component page may name a theme the preview offers.** The theme controller's
own page renders more than fifty controllers, many of them checked, because that is what it takes
to show every value of every axis. If any of them named a theme the preview switches between, it
would re-declare that theme on `:root`, from inside a documentation page, and inside a screenshot
baseline.

The preview offers every theme daisyUI ships, so the examples name themes daisyUI does *not*:
`parchment`, `midnight`, `seafoam`. daisyUI emits its `:has()` rule once per theme it emits, so a
value it never emitted matches nothing at all. The controls check, paint, take focus and report
changes exactly as they would in an app that defined those themes; they simply theme nothing
here. It is the component's own documented no-op, put to work.

A browser spec holds the line: every controller on the page is read, and none of their values may
be a theme the switcher offers. An example added later that reaches for `dark` fails that spec
rather than a baseline three commits on.

## Determinism

ADR-0007 rests on an address rendering the same page every time. It still does: on a load of
`/components/theme_controller?theme=dark`, exactly one controller in the document names a theme
daisyUI emitted (the header's), so `:root` takes that theme and no other. Nothing depends on the
order daisyUI wrote its themes out in, which is what would decide it if two of them were checked
at once.

A screenshot is taken per page per **baseline** theme rather than per theme. Thirty-five themes
across twenty-seven pages is nine hundred and forty-five images, a quarter of a gigabyte of them,
to review a colour change in. Three entries in `Theme::ALL` mark the sweep's baselines and say why
they are enough, the switcher marks them where the harness reads its theme list, and a spec holds
the marks to being a subset of what the menu offers.

The URL is still where the theme is recorded. A pick has already repainted the page by the time it
navigates; the navigation buys an address that still names what is on screen. A theme arriving
from anywhere else (the back button, a typed URL, a link) is met by rebuilding the switcher's
controls from the address, which is what repaints the page.

## Consequences

- The browser tests read the theme off the checked control rather than off an attribute, because
  that is where the theme now is. The pinned external DOM protocol and `openPreview` helper assert
  it as part of the address taking.
- The viewport follows the theme, not just the page. daisyUI declares `color-scheme` alongside a
  theme's colours, and it was landing on a `div`: the document root kept the default one, so a
  dark page had a light scrollbar and a light overscroll area. Both are the root's to draw, and
  the root is now themed.
- The shell's wrapper is gone, colour classes and all; daisyUI's base paints `:root` from
  `--color-base-100`, so the page needs nothing of its own.
- A theme the preview offers must still be enabled in `tailwind.css` and named in `Theme`.
  The new half of that rule is the inverse one above: a theme an example names must *not* be.
- The switcher offers every theme under one radio name, and nothing else in the header touches the
  theme. A light–dark switch beside it was tried and dropped: it could only be navigation, since
  on `dracula` the one checked radio is dracula's and a light or dark *control* painted as though
  it were checked would claim a state it does not have. The component's `README.md` records the
  general rule (a theme belongs to one control), which is what leaves the menu as the only one.
- The menu closes on the pointer that picked rather than on the change. Walking a radio group with
  the arrow keys changes the theme at every step, and fires a `click` as it goes, which is why
  that event cannot tell a walk from a decision, so a menu that closed on either would end the
  walk at its first press. Escape is what the keyboard closes it with.
