# Agents

## Agent skills

### Issue tracker

Issues and PRDs live as GitHub issues on `sagikazarmark/dioxus-daisyui-components`, managed via the `gh` CLI. See `docs/agents/issue-tracker.md`.

### Triage labels

The five canonical triage roles, each label string equal to its name. See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: `CONTEXT.md` + `docs/adr/` at the repo root. See `docs/agents/domain.md`.

### Documentation tooling

Generic documentation discovery, macros, validation, isolated default chrome, and Playwright helpers live in [`sagikazarmark/dioxus-registry-preview`](https://github.com/sagikazarmark/dioxus-registry-preview). This Registry pins the `dioxus-registry-preview` crates.io release in `Cargo.toml` and the matching Playwright helper revision in `tests/browser/package.json`; generic tooling changes belong in that repository.
