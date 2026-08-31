# Documentation tooling is external and pinned

Generic documentation tooling lives in
[`sagikazarmark/dioxus-registry-preview`](https://github.com/sagikazarmark/dioxus-registry-preview).
This Registry depends on the published
[`dioxus-registry-preview`](https://crates.io/crates/dioxus-registry-preview) `0.1.0` facade for the
Rust side, and pins revision `f669725acef7e8bd0cdb6ad35478186e01429e32` for the matching generic
Playwright helper, which is not published to a package registry. That revision retains the released
Rust source and corrects the helper's package version to `0.1.0`. The repository's
[consumer contract](https://github.com/sagikazarmark/dioxus-registry-preview/blob/f669725acef7e8bd0cdb6ad35478186e01429e32/docs/adr/0001-versioned-consumer-contract.md)
is authoritative for:

- the generated pieces and default assembly of `component!`;
- stable group IDs and page-catalog descriptors;
- generic DOM protocol markers and browser-helper behavior;
- invocation-relative discovery and adapter conventions;
- facade-owned isolated default chrome;
- package and diagnostic versioning; and
- Dioxus and Rust compatibility.

**The `component_pages!` invocation is the site configuration.** Its manifest path, ordered
group-ID mapping, and default Component are read both by macro expansion and by full-site
validation, so generation and validation cannot disagree about them. The separate
`registry-docs.json` schema the facade previously discovered from is gone, and the Registry
repository behind generated install commands comes from `package.repository` in `Cargo.toml`.

**Facade model structs and policy enums are non-exhaustive.** Authored descriptors are built
through `PageDescriptor::new`, generated documentation is destructured with `..`, and matches on
placement and policy enums carry a wildcard arm. The Preview refuses a navigation placement it has
no protocol marker for rather than writing out one of the two it does.

This Registry retains every Consumer-owned input and policy: Component documentation source,
macro invocations, the Preview and routes, the branded shell, `ThemeController`-based switching,
`ExampleSection` and `ReadmeSection`, the installation page, daisyUI browser assertions, screenshot
baselines, and Rust policy tests for README sections and Tier vocabulary. A Component change remains
atomic here and needs no tooling-repository edit.

## Registry Protocol Extensions

`data-axis` marks a page-unique row whose direct non-`aria-hidden` children are one rendered
Component per Axis value, in that Axis's `ALL` order, with no other visible direct children.

`data-axis-triggers` marks a page-unique row used when Axis values cannot be rendered side by side.
Its direct non-`aria-hidden` children are one trigger per Axis value in `ALL` order; each trigger
reveals or dispatches its corresponding value.

These markers and assertions about daisyUI computed styles are Registry acceptance vocabulary, not
part of the generic DOM protocol. Custom pages and Examples preserve them whenever they render an
Axis.

## Deferred

Props/public-API extraction and search, sitemap, or link-checking artifacts remain deferred. The
facade now derives Registry facts and an installation page from the catalog invocation, but only
through its own `chrome::App`; this Registry keeps its branded shell and hand-written installation
page, so `APP_CATALOG` goes unused here. Adopting generated installation content is a separate
decision. Future generic artifacts must derive from the external core model rather than introduce
another membership list.
