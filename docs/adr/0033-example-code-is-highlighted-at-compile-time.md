# Example code is highlighted at compile time on one fixed surface

The code tab of every Example is syntax-highlighted. The highlighting is worked out when the
Component's `docs/mod.rs` is compiled, by the pinned documentation tooling's `syntax-highlighting`
feature (ADR-0027), over the very `include_str!` text the Example is compiled from (ADR-0009); the
Preview renders the result through `dioxus-code`. No parser ships to the browser and nothing runs
at page load.

The tokens are one palette, GitHub Dark, on one surface the Registry owns, under every daisyUI
theme. The installation page's plain blocks sit on the same surface, so the site has one kind of
code block rather than two.

## Why one fixed surface

The rest of the Preview is themed by the component it documents (ADR-0020), and a code block
that followed suit was the obvious first choice. Two things argued against it.

A palette is drawn against a canvas. Light tokens on a dark `base-200` are unreadable and the
reverse is garish, so following the theme means at least two palettes and a way to pick between
them. daisyUI says which of its thirty-five themes are dark only through the `color-scheme`
property, which no selector or container query can read; the pick would have to be made in Rust
from the route's theme, with a light-or-dark flag per theme in `Theme::ALL` mirroring daisyUI's
own. That is a copy of daisyUI kept beside it, of exactly the kind ADR-0020 removed.

The tooling's own default chrome made the same call, for both of its themes, and fixing the
surface with the palette is what lets the palette contribute token colours alone: `dioxus-code`'s
canvas is painted over by the panel's `bg-code-surface`, so a palette change is one constant and
a surface change another.

## Consequences

- **Every Preview build compiles C for `wasm32-unknown-unknown`.** `dioxus-code` depends on
  `arborium`, whose tree-sitter runtime and bundled sysroot are compiled by `cc` for every
  target regardless of features. `cc` reaches for `clang` on a wasm target when no `CC_*`
  variable names another compiler. `devenv.nix` names Nix's unwrapped clang through
  `CC_wasm32_unknown_unknown`, because the wrapped one adds host flags that a wasm target
  rejects; the Dagger `dioxus` container installs Debian's `clang` through a `distro` overlay,
  which targets wasm out of the box. `scripts/check-browser.sh` builds on the host, so it needs
  the devenv. A Component consumer is unaffected: the feature lives behind `preview`.
- `dioxus-code` is a direct, `preview`-gated dependency, because the facade re-exports the
  highlighted-source types but not the `Code` component or its palettes. Cargo unifies it with
  the facade's own `dioxus-code`; a version the facade cannot share fails loudly at the type.
- `dioxus-code` mounts its own stylesheets: a `.dxc` base rule and a class-scoped palette. Both
  are unlayered and arrive after the Registry's stylesheet, so no Tailwind utility can override
  them; `style.css` carries one unlayered, more specific `.example-code .dxc` rule that makes the
  block transparent and sets it at the installation page's measurements.
- The installation page's blocks are not highlighted. Its Rust fragments would need
  `dioxus-code` as a macro dependency for one `code_str!` each, and its shell and CSS would need
  `lang-*` features that ship a runtime parser to the browser. Plain text on the shared surface is
  the zero-cost path.
- The tab is now labelled `Code`, as the tooling's default chrome labels its own, rather than
  `RSX`: the file it prints is an ordinary Rust module with imports and a doc comment, not only
  an `rsx!` body. The one browser spec that opens the tab by name (`drawer`) follows the label.
- The DOM protocol is unchanged: `data-example` and `data-example-content` keep their promises,
  and the highlighted markup (`pre.dxc > code > span.a-*`) is presentation outside it.
