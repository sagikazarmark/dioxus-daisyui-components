import { defineConfig, devices } from "@playwright/test";

/**
 * Seam 2: the preview in a real browser.
 *
 * See docs/adr/0007-acceptance-testing-is-fully-automated.md. There are two ways
 * in, and both pin the same playwright release, so both drive the browsers that
 * release was tested against:
 *
 * `./scripts/check-browser.sh` builds the preview twice and runs the whole
 * suite, rendering specs included, inside the pinned container. It is the only
 * entry point they run under, because it is the one that records the baselines
 * they are compared against.
 *
 * `dagger check playwright:test` runs the behavioural specs against the service
 * the preview module builds and serves, with no dist and no container of this
 * repository's own. It records no baselines, so the rendering specs skip
 * themselves there.
 */

// Where the preview under test is. The `playwright` Dagger module binds the
// preview module's service into the test container and names it here, in which
// case the suite drives that build and starts nothing itself; otherwise
// `./scripts/check-browser.sh` has built a dist for the webServer block below
// to serve.
const port = 8080;
const wired = process.env.PLAYWRIGHT_BASE_URL;
const baseURL = wired ?? `http://127.0.0.1:${port}`;

// The rendering specs, which are the only ones that do not run on every
// engine. ADR-0007 records why screenshots are page-level and single-engine.
const rendering = "**/rendering.spec.ts";

export default defineConfig({
  testDir: ".",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  // The HTML report is what `dagger api call playwright report -o <dir>` pulls
  // out of a failed run, and what a local run leaves behind to open; the list
  // reporter is what either one reads while it goes.
  reporter: [["list"], ["html", { open: "never" }]],
  timeout: 60 * 1000,
  expect: { timeout: 15 * 1000 },

  use: {
    baseURL,
    trace: "on-first-retry",
  },

  // Baselines are not committed. They are rendered from the base commit at the
  // start of every run and land here, which is why this directory is ignored:
  // what a run compares against is the preview as it was before the change,
  // rather than an image somebody remembered to renew.
  snapshotPathTemplate: "{testDir}/screenshots/{arg}{ext}",

  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
      testIgnore: rendering,
    },
    {
      name: "firefox",
      use: { ...devices["Desktop Firefox"] },
      testIgnore: rendering,
    },
    {
      name: "webkit",
      use: { ...devices["Desktop Safari"] },
      testIgnore: rendering,
    },
    {
      name: "rendering",
      use: { ...devices["Desktop Chrome"] },
      testMatch: rendering,
      // One test takes one screenshot per component page per theme, and the
      // registry keeps growing, so this one is budgeted by the size of the
      // registry rather than by the minute every other spec gets.
      timeout: 5 * 60 * 1000,
    },
  ],

  // A run serves two different builds in turn (the base commit's, to record
  // the baselines, and then the change's, to compare against them) so which
  // build is on the port is the whole meaning of a pass. An existing server is
  // never reused: attaching to the previous pass's would compare a build
  // against itself. A wired service is the one exception, and not one of these
  // two builds at all: the preview module built it and Dagger is running it, so
  // starting a second server here would serve a dist nobody built.
  webServer: wired
    ? undefined
    : {
        command: "node ./serve.mjs",
        env: { PORT: String(port), ...(process.env.DIST ? { DIST: process.env.DIST } : {}) },
        url: baseURL,
        reuseExistingServer: false,
        stdout: "pipe",
      },
});
