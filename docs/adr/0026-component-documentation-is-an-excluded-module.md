# Component documentation is an excluded module

The repository is a Cargo workspace whose only member is the root Registry and Preview package.
Convention-driven documentation lives in the external
[`dioxus-registry-preview`](https://github.com/sagikazarmark/dioxus-registry-preview) repository.
The root package depends on the published `dioxus-registry-preview` facade. Runtime code depends on
it only through the `preview` feature, Registry policy tests use its validation exports, and the
generic Playwright helper pins a matching repository revision. Extraction changed dependency
locations without changing Component authoring.

Each Component keeps installable Rust at its root and all Preview-only Rust and Examples in an
excluded `docs/` directory:

```text
src/components/<name>/
├── README.md
├── component.json
├── component.rs
├── mod.rs
└── docs/
    ├── mod.rs
    └── examples/
```

`README.md` is the Component's narrative documentation. GitHub renders it at the Component root,
and the macro makes both its source and rendered body available to the Preview. A generated page
is examples-only by default, matching the Preview's existing pages; a documentation module opts
into rendering the README with `render_readme: true`. When rendered, its first H1 and introductory
paragraph are omitted because the Preview already gets the heading and catalog description from
the README metadata and manifest.

`docs/mod.rs` invokes `dioxus_registry_preview::component!` under the external tooling's
[consumer contract](https://github.com/sagikazarmark/dioxus-registry-preview/blob/f11ad519a9c80f606de5c0c5ba414af8a293c61e/docs/adr/0001-versioned-consumer-contract.md).
The invocation declares the
Preview-owned group and listing policy, plus the ordered Examples. By convention the macro reads
`../component.json`, `../component.rs`, `../README.md`, and `examples/<module>.rs`; only an unusual
layout declares a different `component_root`. An Example's slug defaults to its Rust module name,
its title defaults to that name in sentence case, and either may be declared separately. The macro
parses the Component source before generating a page, so malformed source or a source file without
a public `#[component]` function fails at the authoring seam. It also emits `include_str!`
references to every input, making Cargo rebuild when one changes.

The generated default `DocumentationPage` composes literal Dioxus calls through named generated
sections, preserving ADR-0009. The Preview's conventional `crate::example::ExampleSection` is the
visual adapter and may be overridden, so the documentation packages do not depend back on the
Registry. Component `mod.rs` does not declare `docs`, and the manifest excludes `README.md` and
`docs`, so neither the macro nor its generated documentation reaches a consumer of
`dx components add`.

The Preview invokes `dioxus_registry_preview::component_pages!` over the root Registry manifest. Its
`members` list generates the documentation module declarations, address type, metadata dispatch,
literal page calls and string conversions. Adding a Component to the Registry therefore adds its
page to the Preview and browser-test sweep without maintaining a second membership list. Groups
remain in Component documentation because they are Preview presentation rather than Registry
metadata.

Button was the tracer for this layout; every Component now uses it. This decision supersedes
ADR-0025's statements that the root is not a workspace and that Components use the old authoring
layout.

## Consequences

- One macro invocation replaces a handwritten Component page, example module declarations, source
  constants, and repeated manifest metadata.
- The root Registry manifest is also the Preview's Component page list.
- A Component README is useful on GitHub without becoming part of an installed Component.
- Documentation dependencies are paid only by the Preview feature.
- The framework initially derives structure and compiling examples; semantic rationale remains
  authored Markdown, and richer public-API extraction can deepen the same macro later.
- The external tooling's use of `proc_macro::Span::local_file` establishes its Rust 1.88 minimum;
  this Registry inherits that floor when the Preview feature is enabled.
