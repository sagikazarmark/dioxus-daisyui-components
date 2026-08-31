import { existsSync } from "node:fs";

import { expect, type Page, test } from "@playwright/test";

import { addresses, openPreview } from "./preview";

// Rendering has no stored baselines. `./scripts/check-browser.sh` renders the
// base commit's preview to produce them and then renders the change's against
// them, both in the pinned container within the one run, so what a screenshot
// is compared against was taken minutes earlier on the same engine, fonts and
// rasteriser rather than by whoever last renewed an image. These still refuse
// to run outside that script, because a run that skipped the recording pass
// would compare a build against nothing at all.
test.skip(
  !process.env.PINNED_CONTAINER,
  "rendering runs in the pinned container: ./scripts/check-browser.sh",
);

// The recording pass, which is the base commit's build being turned into the
// baselines. It records what is there rather than asserting anything about it.
const recording = !!process.env.RECORD_BASELINES;

test("every component page renders unchanged under every theme", async ({ page }) => {
  // One image per component page per theme. The page already renders every
  // value of every axis, so a single image covers them all and a failing
  // baseline's diff localises the change.
  for (const address of await addresses(page)) {
    const name = `${address.component}-${address.theme}.png`;

    // A page the change adds has nothing at the base commit to be compared
    // against, and that is the normal shape of a new component rather than a
    // failure. Without this the assertion would fail the run and write the
    // missing image, which the retry would then pass; a new component would
    // look like a flake.
    if (!recording && !existsSync(test.info().snapshotPath(name, { kind: "screenshot" }))) {
      test.info().annotations.push({ type: "new page", description: name });
      continue;
    }

    await openPreview(page, address);

    if (address.component === "loading") {
      await freezeLoadingMasks(page);
    }

    // Soft, so that one theme having moved does not hide the rest: a run that
    // fails reports every page whose rendering the change moved, not the first
    // one it reached.
    await expect.soft(page).toHaveScreenshot(name, { fullPage: true });
  }
});

/**
 * Removes SMIL motion from daisyUI's already-computed loading masks.
 *
 * Playwright can quiesce CSS and Web Animations, but an SVG used as a CSS image
 * is a separate document and its SMIL timeline keeps running. Re-encoding that
 * same SVG without its animation elements preserves each mask's static shape
 * while allowing the rendering assertion to obtain two identical frames.
 */
async function freezeLoadingMasks(page: Page): Promise<void> {
  await page.locator(".loading").evaluateAll((loaders) => {
    for (const loader of loaders) {
      const mask = getComputedStyle(loader).maskImage;
      const start = mask.indexOf("data:image/svg+xml,");
      if (start === -1) continue;

      const dataUrl = mask.slice(start).replace(/["']?\)$/, "");
      const comma = dataUrl.indexOf(",");
      const svg = decodeURIComponent(dataUrl.slice(comma + 1)).replace(
        /<animate(?:Transform)?\b[^>]*\/>/g,
        "",
      );
      const frozen = `url("data:image/svg+xml,${encodeURIComponent(svg)}")`;
      const style = (loader as HTMLElement).style;
      style.setProperty("mask-image", frozen, "important");
      style.setProperty("-webkit-mask-image", frozen, "important");
    }
  });
}
