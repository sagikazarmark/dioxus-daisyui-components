#!/usr/bin/env bash
#
# Seam 2: the Preview in a real browser.
#
# Builds the Preview and drives it with playwright: computed styles for every
# value of every Axis, a caller's classes surviving the merge, focus and ARIA
# behaviour on three engines, and one screenshot per Component page per theme.
#
# The screenshots have no stored baselines. A run builds the Preview twice
# (once at the base commit and once at the change), renders the first to produce
# the baselines and the second to compare against them. What a screenshot is
# measured against is therefore the Preview as it was immediately before the
# change, rendered minutes earlier by the same engine, and an intentional
# change to how something renders is a diff to read rather than a directory of
# images to renew. See docs/adr/0007-acceptance-testing-is-fully-automated.md.
#
# Everything below the builds runs inside a pinned container. Both renders
# happening in the same container is what makes them comparable at all; running
# the behavioural specs in there too costs nothing and keeps one command for
# the whole seam.
#
# The builds stay outside it. What the container pins is what renders; what
# gets rendered is pinned by lockfiles instead: the CLI's own tailwind, the
# daisyUI release package-lock.json names, and the revision every
# component pins the primitive to, none of which the host can vary.
#
# Arguments are passed through to playwright. Two options are read here first:
#
#     ./scripts/check-browser.sh --base <ref>   # what to compare against
#     ./scripts/check-browser.sh --reuse        # keep the baselines on disk
#
# `--reuse` skips the base build and renders nothing but the change, comparing
# against whatever is already in tests/browser/screenshots. It is how CI reuses
# a cached recording, and how a second local run after a failed one is quick.

set -euo pipefail

registry_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

# The playwright version this pins has to stay in step with the one
# tests/browser/package.json declares: the image ships the browsers that
# release was tested against, and playwright refuses to drive browsers it did
# not install.
image="mcr.microsoft.com/playwright:v1.62.1-noble"

# The architecture is pinned along with the image. The two renders are only
# comparable to each other, but the behavioural specs still want a fixed
# engine, and CI's runners are amd64.
platform="linux/amd64"

cd "$registry_root"

base_ref="${BASE_REF:-}"
reuse="${REUSE_BASELINES:-}"
playwright_args=()

while [[ $# -gt 0 ]]; do
    case "$1" in
        --base)
            base_ref="${2:?--base needs a ref}"
            shift 2
            ;;
        --reuse)
            reuse=1
            shift
            ;;
        *)
            playwright_args+=("$1")
            shift
            ;;
    esac
done

# A named ref that is not in this checkout is not an argument to argue with:
# a force-push and a repository's first push both hand CI a base commit that no
# longer resolves, so it falls back to what an unnamed run would have used.
if [[ -n "$base_ref" ]] && ! git rev-parse --verify --quiet "$base_ref^{commit}" >/dev/null; then
    echo "==> $base_ref is not in this checkout; measuring against the default instead"
    base_ref=""
fi

# What the change is measured against, when nothing named it.
#
# A dirty tree is being compared against the commit it is sitting on, which is
# the inner loop for a styling change: edit, run, read the diff. A clean tree is
# compared against where it left `main`, so that a branch is measured against
# what it branched from rather than against its own first commit, and `main`
# itself against the commit before it, because a branch point of itself would
# compare the build against nothing.
if [[ -z "$reuse" && -z "$base_ref" ]]; then
    # `--verify --quiet` because a plain rev-parse of a branch that is not here
    # prints the name it could not resolve to stdout and would be read as a ref.
    main_sha=$(git rev-parse --verify --quiet main || true)

    if ! git diff --quiet HEAD --; then
        base_ref=HEAD
    elif [[ -n "$main_sha" && "$(git rev-parse HEAD)" != "$main_sha" ]]; then
        base_ref=$(git merge-base main HEAD)
    else
        base_ref=$(git rev-parse HEAD~1)
    fi
fi

target_root="${CARGO_TARGET_DIR:-$registry_root/target}"
head_target_root="$target_root/render-head-target"
base_target_root="$target_root/render-base-target"
head_dist="$head_target_root/dx/preview/release/web/public"
base_head_dist="$base_target_root/dx/preview/release/web/public"
base_tree="$registry_root/target/render-base"

# Base and change use separate target directories. A shared Cargo target can
# consider the just-built base package authoritative for the change when the
# detached worktree's source timestamps are newer, serving the wrong Preview
# and producing a false green rendering comparison.
base_dist="$registry_root/target/render-base-dist"
screenshots="$registry_root/tests/browser/screenshots"

# The container sees the registry at a fixed path, so the dists it is told to
# serve are named relative to that rather than to wherever this is checked out.
# Only what is under the registry root is mounted, so a target directory moved
# outside it is caught here rather than as an empty page in every screenshot.
container_dist() {
    case "$1" in
        "$registry_root"/*) echo "/registry${1#"$registry_root"}" ;;
        *) echo "the preview builds to $1, which is outside the mounted repository; unset CARGO_TARGET_DIR or point it inside it" >&2; return 1 ;;
    esac
}

# Resolved here rather than inside the docker invocation, because a command
# substitution that fails in the middle of an argument list is not a failure
# `set -e` can see, and the run would go on to serve nothing.
head_dist_in_container=$(container_dist "$head_dist")

build_preview() {
    dx build --package dioxus-daisyui-components --bin preview --features preview --platform web --release
}

if [[ -n "$reuse" ]]; then
    echo "==> Reusing the baselines already in tests/browser/screenshots"
    record_dist=""
else
    echo "==> Building the preview at $base_ref"

    discard_base_tree() {
        git worktree remove --force "$base_tree" 2>/dev/null || true
        rm -rf "$base_tree"
        git worktree prune
    }

    # On a trap rather than at the end, because a run whose rendering moved
    # exits at the suite, which is the ordinary outcome of changing how
    # something looks, and no reason to leave a built checkout of the base
    # commit behind.
    trap discard_base_tree EXIT

    # Discarded on the way in as well, so that a run killed before its trap
    # could fire leaves nothing for the next one to build on top of, and so
    # that a registration whose directory is gone cannot refuse the add.
    discard_base_tree
    git worktree add --detach "$base_tree" "$base_ref" >/dev/null

    # Laid down before the build so that what the build wrote can be told from
    # what a previous one left. See the check below it.
    stamp="$base_target_root/.render-base-stamp"
    mkdir -p "$(dirname "$stamp")"
    : > "$stamp"

    # Its own npm install, because the base commit may pin a different daisyUI,
    # and what tailwind scans is the tree it is run from.
    (
        cd "$base_tree"
        export CARGO_TARGET_DIR="$base_target_root"
        npm ci --no-audit --no-fund
        build_preview
    )

    # The dist location is an assumption about the Dioxus CLI rather than
    # something this script controls. Catch a change there instead of copying a
    # previous build and producing a green run that tested nothing.
    if [[ -z "$(find "$base_head_dist" -newer "$stamp" -print -quit 2>/dev/null)" ]]; then
        echo "the base build did not write $base_head_dist; the Dioxus CLI no longer follows CARGO_TARGET_DIR" >&2
        exit 1
    fi

    rm -rf "$base_dist"
    cp -R "$base_head_dist" "$base_dist"

    # A baseline left by an earlier run is a page from a different base commit,
    # which is exactly the image nobody meant to compare against.
    rm -rf "$screenshots"
    record_dist=$(container_dist "$base_dist")
fi

echo "==> Installing daisyUI"
npm ci

# The container has no rust toolchain in it, so the previews are built out here
# and only served in there. Release rather than debug, because a debug wasm
# build is slow enough to make every spec look flaky.
echo "==> Building the preview"
(
    export CARGO_TARGET_DIR="$head_target_root"
    build_preview
)

mkdir -p "$screenshots"

echo "==> Running the browser suite in $image"
docker run --rm --init \
    --platform "$platform" \
    --ipc=host \
    --volume "$registry_root:/registry" \
    --workdir /registry/tests/browser \
    --user "$(id -u):$(id -g)" \
    --env HOME=/tmp \
    --env CI \
    --env PINNED_CONTAINER=1 \
    --env RECORD_DIST="$record_dist" \
    --env HEAD_DIST="$head_dist_in_container" \
    "$image" \
    sh -c '
        set -eu
        npm ci --no-audit --no-fund

        if [ -n "$RECORD_DIST" ]; then
            echo "--> Recording the baselines from the base commit"
            RECORD_BASELINES=1 DIST="$RECORD_DIST" \
                npx playwright test --project rendering --update-snapshots
        fi

        echo "--> Driving the change"
        DIST="$HEAD_DIST" npx playwright test "$@"
    ' playwright "${playwright_args[@]+"${playwright_args[@]}"}"

echo "==> Seam 2 passed"
