# The registry lives inside the preview app

**Superseded by [ADR-0025](0025-the-registry-is-a-root-library-with-a-gated-preview.md).**

Components are authored at `preview/src/components/<name>/` and the root `component.json`
lists those paths as members, rather than keeping a sibling `components/` directory that the
preview app consumes. `dx components add` copies a component into the consumer's
`src/components/<name>/`, so authoring it at that same path means any crate-root-absolute
path literal a component needs (`asset!`, `#[css_module]`) is valid in both the registry and
the consumer without rewriting. It also makes the preview app compile the exact bytes that
get copied out, so "it renders in the preview" is direct evidence rather than a proxy.

This matches how the upstream `DioxusLabs/components` registry is laid out.

## Consequences

- No separate Tailwind `@source` glob for the preview app; the components are already inside
  the preview crate's own source tree.
- The repository root reads as a Dioxus app rather than as a registry; `members` entries are
  long paths.
