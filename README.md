# DaisyUI Components for Dioxus

A `dx components` registry of Dioxus components that apply [daisyUI][daisyui] class names to
[`dioxus-primitives`][primitives] behaviour, native browser behaviour, or presentational markup.

A component is copied into your own source tree, where it emits daisyUI class names and nothing
else. Your app supplies Tailwind and daisyUI; the registry ships no CSS, defines no theme, and
adds no styling layer of its own, so your `[data-theme]` restyles everything at once.

[primitives]: https://github.com/DioxusLabs/components
[daisyui]: https://daisyui.com

## Setup

`dx` runs its own pinned Tailwind (v4.1.5) over a single stylesheet at your crate root, and that
file is the only input it processes. Everything below assumes that file is `tailwind.css`.

### 1. Load Tailwind and daisyUI

Install daisyUI from npm next to your crate:

```shell
npm install daisyui@5
```

And write `tailwind.css` at your crate root:

```css
@import "tailwindcss";
@plugin "daisyui";
```

<details>
<summary>Without a package manager</summary>

daisyUI ships a self-contained plugin bundle for exactly this, so a project that will not add
npm can vendor it instead. Download `daisyui.js` from the [daisyUI release][releases] into your
crate (plus `daisyui-theme.js` if you define custom themes) and point `@plugin` at it by
relative path:

```css
@import "tailwindcss";
@plugin "./daisyui.js";
```

The npm route is the recommended one: vendoring makes a daisyUI upgrade a manual download
rather than a lockfile bump, and nothing records which version you are on.

[releases]: https://github.com/saadeghi/daisyui/releases
</details>

### 2. Bundle the generated stylesheet

`dx` writes the compiled CSS next to your assets; bundle it from your app:

```rust
const TAILWIND: Asset = asset!("/assets/tailwind.css");

rsx! {
    document::Stylesheet { href: TAILWIND }
}
```

### 3. Install a component

```shell
dx components add button --path /path/to/dioxus-daisyui-components
```

The first install creates `src/components/` and prints the one manual step it leaves to you:
add `mod components;` to your `main.rs`. Cargo dependencies are added for you.

Then rebuild. `dx serve` and `dx build` run Tailwind over your stylesheet, which is where the
component's class names are picked up. There are no other setup steps.

> The registry is published as a git registry (`dx components add button --git <url>`) once the
> dialog component lands. Until then it installs by local path.

### Cargo library consumption

The root library can also be consumed through Cargo, but Tailwind does not scan Cargo dependency
checkouts automatically. The consuming application must make the dependency available at a stable,
application-controlled path (for example as a path dependency, Git submodule, or under `vendor/`
with `cargo vendor`) and add its Component source to `tailwind.css`:

```css
@import "tailwindcss";
@source "./vendor/dioxus-daisyui-components/src/components";
@plugin "daisyui";
```

The path is relative to `tailwind.css`; adjust it to the chosen stable location. Pointing Tailwind
at Cargo's generated Git checkout is not a portable contract because that path includes
Cargo-managed repository and revision hashes. A manual utility safelist is possible but not
recommended: it must be updated whenever a Component adds or changes utility-backed defaults.

daisyUI component classes and application-owned utilities still come from the consuming build, and
the Registry ships no stylesheet. Behavior-critical structural styles, such as hiding the Switch's
additional native form participant, do not rely on Tailwind discovery.

## Usage

Every daisyUI styling axis is its own prop, so they combine freely the way daisyUI's classes do:

```rust
use crate::components::button::{Button, ButtonColor, ButtonSize};

rsx! {
    Button {
        color: ButtonColor::Primary,
        size: ButtonSize::Lg,
        onclick: move |_| tracing::info!("clicked"),
        "Save"
    }
}
```

Your own classes concatenate with the component's, and your own attributes override it:

```rust
rsx! {
    Button { class: "w-full", id: "save", "Save" }
}
```

Each component has a `README.md` in this repository recording how it bridges state to daisyUI's
selectors, or why there is no state to bridge, and which daisyUI classes it deliberately does not
use. GitHub renders that documentation when browsing the component's directory.

## Development

Install daisyUI, then serve the preview:

```shell
npm install
dx serve --package dioxus-daisyui-components --bin preview --features preview --platform web
```

The preview is a documentation site: a sidebar of component pages, a theme switcher in the
header, and each page a list of examples shown as a preview and as the code behind it.

The switcher is the registry's own theme controller over every theme daisyUI ships, and it is what
themes the site: daisyUI reads the checked control from the document root, so the preview is themed
by the component it documents. See
[ADR-0020](docs/adr/0020-the-preview-is-themed-by-the-component-it-documents.md).

It is addressable: the path names the page and `?theme=` names the daisyUI theme to show it
under: `http://localhost:8080/components/button?theme=dark`. Both are what the sidebar and
the header's theme switcher navigate between, and a theme that is absent or unrecognised falls
back to `light`. `Theme` lists every theme daisyUI ships and `tailwind.css` asks for
`themes: all`; the two have to stay in step.

Three entries in `Theme::ALL` are marked as screenshot baselines rather than all thirty-five:
three themes per page rather than thirty-five, which keeps a run's rendering pass to minutes
and its diffs readable.

An example is one file under `src/components/<component>/docs/examples/`, holding one component.
The page renders that component and prints that file: the code tab is `include_str!` of the
same source, so a snippet cannot drift from what it documents and one that stops compiling
fails the build. See
[ADR-0009](docs/adr/0009-an-example-is-a-file-that-is-rendered-and-printed.md). A new example
is a new file and an entry in the Component's `docs/mod.rs`; the documentation macro declares
the module, includes its source, and writes its `ExampleSection` into the generated page. The root
Registry manifest generates the Preview's Component page list, so adding a member there also adds
its page to the browser-test sweep.

Because the example is where the code lives, a component's `README.md` does not restate it. Each
one opens on its introduction and then links to its Preview page and its `docs/examples/`
directory, and the prose below is narrative: composition, state bridging, deviations, and the
daisyUI classes deliberately left alone. Prose still quotes a fragment where one illustrates the
argument around it. The policy test fails a README that opens on a synopsis instead. See
[ADR-0030](docs/adr/0030-a-readme-links-to-its-examples-rather-than-restating-them.md).

An example that renders a theme controller names a theme daisyUI does **not** ship: `parchment`,
`midnight`, `seafoam`. Since the preview is themed by that component, and offers every theme
daisyUI has, one naming `dark` would re-theme the site from inside the page documenting it, and
inside a screenshot baseline; `tests/browser/theme_controller.spec.ts` fails the build if one
does.

`dx` runs its pinned Tailwind (v4.1.5) over `tailwind.css` and writes
`assets/tailwind.css`, which `asset!` then bundles. That output is generated and
git-ignored, so a clean checkout has to go through `dx` at least once before plain `cargo`
commands that enable the Preview can resolve the asset.

The repository root is one unpublished Cargo package. Its library exports every component from
`src/components/`, while its `preview` binary is gated behind the `preview` feature and re-exports
that library module so example imports match consumer imports. Components remain authored at the
exact path `dx` installs them to, and the Preview compiles those same modules. Colocated examples
are authoring inputs only: component manifests exclude them from installs and the Cargo package
include list excludes them from the library package. See
[ADR-0025](docs/adr/0025-the-registry-is-a-root-library-with-a-gated-preview.md).

Three checks run in CI and are runnable locally:

```shell
dagger check preview:install                # install every component into a scratch app and compile it
./scripts/check-browser.sh                  # drive the built preview in a browser
dagger check preview:test                   # check documentation policy, primitive pins, and the field convention
```

The install seam runs entirely in Dagger, against the toolchain the Registry itself builds with.
`preview:install:closure` installs each dependency-bearing component on its own and compiles the
closure it pulls in; `preview:install:registry` lists the Registry, installs all of it, and compiles
the result for the host and for `wasm32-unknown-unknown`. Both install into `tests/fixtures/install`,
a checked-in scratch app rather than one scaffolded per run, because `dx new` drives its template
through an interactive prompt and refuses to run without a TTY.

The Preview's `component_pages!` invocation in `src/preview/pages/mod.rs` is the site configuration.
It names the root Registry manifest, stable Component group IDs, and default Component module. The
`registry_policy` test loads that invocation through the facade's validation library, adds this
Registry's required README sections and state-bridging vocabulary, and checks the Registry-owned
primitive dependency declaration. Generic layout, manifest, source, README, Example, group, and
page validation comes from
[`dioxus-registry-preview`](https://crates.io/crates/dioxus-registry-preview) 0.1.0.

`check-browser.sh` builds the preview and runs `tests/browser` against it with Playwright:
computed styles for every value of every axis, a caller's classes surviving the merge, and
focus and ARIA behaviour on chromium, firefox and webkit, plus one screenshot per component
page per theme. Everything below the builds runs in a pinned container, so Docker has to be
running.

The screenshots have no stored baselines. A run builds the preview twice (at the base commit
and at the change), records the first and compares the second against it, so what a page is
measured against is how it looked immediately before the change rather than an image somebody
remembered to renew. There is nothing to update: a rendering the change moved is a diff to
read, left in `tests/browser/test-results`.

By default the base is the commit a dirty tree sits on, and otherwise where the branch left
`main`. Name another, or reuse the recording already on disk after a failed run:

```shell
./scripts/check-browser.sh --base <ref>
./scripts/check-browser.sh --reuse
```

The behavioural half of that suite has a second entry point that runs entirely in Dagger,
against the Preview the `preview` module serves rather than against a dist built on the host:

```shell
dagger check playwright:test                       # the behavioural specs, against the served preview
dagger api call playwright report -o ./report      # the HTML report, after a run that failed
```

Nothing of this repository's own is needed to run it: `dagger.toml` wires `preview:service` into
the [`playwright`](https://github.com/dagger/playwright) module, which derives its image from the
`@playwright/test` version `tests/browser/package-lock.json` pins, so both entry points drive the
browsers that release was tested against. The rendering specs are not reached this way. They are
only meaningful once the base commit's build has been recorded, which is `check-browser.sh`'s job,
so they skip themselves here.

Browser tests live outside component directories. Colocated examples are excluded explicitly by
every component manifest, and a missing exclusion target fails the install.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
