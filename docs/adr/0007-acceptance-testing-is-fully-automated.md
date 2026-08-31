# Acceptance testing is fully automated

Every acceptance criterion is checked in CI rather than by eye, across exactly two application
seams. Reusable cross-registry contracts may additionally have focused Rust conformance tests;
ADR-0028 introduces that genre for the field convention.

**Seam 1: installing a component.** `dx components add` runs against the registry by path into
a checked-in scratch Dioxus app, and the result is compiled. This is the user-facing entry
point and the only place the manifests are observable at all: `exclude` targets are
canonicalized and hard-error when the named file does not exist, and nothing below this seam
can see a wrong member path, a malformed dependency entry, or a missed module declaration.

**Seam 2: the preview in a real browser.** Playwright drives the built preview and covers
everything else:

- **Cargo-library source discovery**: the Preview binary imports the root library through its
  package name. Its Tailwind input disables automatic discovery, then scans Preview chrome and
  `src/components` as separate explicit sources. This exercises the stable `@source` contract used
  by an application that consumes the root library from a path, submodule, or vendored dependency.

- **Class literals**: each axis enum exposes `pub const ALL`; the preview iterates it to render
  every axis value, and the specs assert *computed styles* on those elements. This is stronger
  than searching the generated stylesheet for expected strings: a match there proves the literal
  survived Tailwind's scan, whereas a computed style proves it survived and applies.
- **Attribute merging**: the rendered element's class list is asserted to contain both the
  registry's classes and the caller's.
- **Behaviour**: focus, `aria-*`, and keyboard sequences, on chromium, firefox and webkit.
- **Rendering**: screenshots, one per component page per theme, chromium only.

The behavioural specs earn their place because the registry deliberately rearranges the DOM
around the primitives (ADR-0003, ADR-0005), and they are what proves the behaviour the
primitives were kept for still works. Upstream's `playwright/` specs are behavioural only and
are a direct template.

Render-to-string tests were considered and rejected. Everything they would assert is observable
in the browser, and they would add a seam below the application boundary, testing component
functions rather than behaviour.

A lint additionally asserts each component has a `README.md` recording its tier. That observes
the repository rather than the feature, so it is not counted as a seam.

Screenshots are page-level and single-browser: the preview page already renders every axis
value, so one image per theme covers them all, and a failing baseline's diff image localises
the change. Per-variant baselines would multiply the images tenfold for signal the diff already
carries; adding firefox and webkit would triple them for signal about browsers rather than
about the class mapping.

**Baselines are rendered, not stored.** A run builds the preview twice (at the base commit and
at the change), records the first and compares the second against it. Committed baselines were
tried first and rejected on what they cost the repository: a hundred and sixty-eight images, and
a daisyUI or Tailwind bump legitimately rewriting all of them at forty megabytes a time, which
in five days made screenshots ninety-eight percent of the git history. Rendering them instead
also removes the renewal commit as a category (an intentional change to how something renders
is a diff to read, not a directory to refresh and review) and removes the class of failure
where a baseline was renewed without anyone looking at what moved.

What this gives up is a fixed point. The comparison answers *did this change move the rendering*
rather than *is the rendering right*, so it cannot see drift that arrived one imperceptible
commit at a time, and it cannot see rendering that was already wrong when it was first recorded.
Both were already true of a committed baseline that a renewal commit rubber-stamped; neither is
what the seam is for, which is catching a class mapping that stopped applying.

## Consequences

- The preview must take component and theme as URL parameters so Playwright can navigate
  deterministically.
- Both renders happen in the same pinned container within the one run, which is what makes them
  comparable, rather than a container pinned so that images taken months apart on different
  machines still agree.
- The rendering specs are driven only by `./scripts/check-browser.sh`, which is what performs the
  recording pass; a run that reached them any other way would compare a build against nothing.
- A page the change adds has nothing at the base commit to be measured against, so the rendering
  spec records it as new and asserts nothing about it.
- CI needs the full history to build the base commit, and caches each recording under that
  commit so that pull requests onto an unchanged `main` share one.
- Playwright injects a stylesheet at test time to quiesce animations. This is test-only and
  never shipped, so it does not contradict ADR-0002.
- Tests live outside component directories, so no component's `exclude` list has to name them,
  which matters because a missing `exclude` target fails the install.
- Rust integration tests are reserved for reusable cross-registry conformance contracts. Their
  adapters mount the real Component and drive its normal interaction path rather than reproducing
  resolution or Component logic in test code. The inner loop for styling and ordinary Component
  behaviour remains the Preview itself.
