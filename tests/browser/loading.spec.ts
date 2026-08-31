import { expect, test } from "@playwright/test";

import { axis, computedStyle, expectAxisGrows, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "loading" });
});

test.describe("styling", () => {
  test("every animation uses a distinct mask", async ({ page }) => {
    expectVaries(await computedStyle(axis(page, "animation"), "mask-image"), "animation");
  });

  test("every size is larger than the one before it", async ({ page }) => {
    await expectAxisGrows(page, "size", "width");
  });

  test("a caller's text colour paints the indicator", async ({ page }) => {
    const colors = await page.locator("#caller-attributes").evaluate((node) => {
      const style = getComputedStyle(node);
      return { foreground: style.color, indicator: style.backgroundColor };
    });

    expect(colors.indicator).toBe(colors.foreground);
  });

  test("the empty span omits default modifiers and preserves caller attributes", async ({ page }) => {
    const defaultLoading = page.locator("#default-loading");
    const caller = page.locator("#caller-attributes");

    expect(await defaultLoading.evaluate((node) => node.tagName)).toBe("SPAN");
    expect(await defaultLoading.evaluate((node) => node.childNodes.length)).toBe(0);
    expect(((await defaultLoading.getAttribute("class")) ?? "").split(/\s+/).filter(Boolean)).toEqual([
      "loading",
    ]);

    const classes = ((await caller.getAttribute("class")) ?? "").split(/\s+/);
    expect(classes).toContain("loading");
    expect(classes).toContain("loading-bars");
    expect(classes).toContain("loading-lg");
    expect(classes).toContain("text-secondary");
    await expect(caller).toHaveAttribute("data-owner", "caller");
  });
});
