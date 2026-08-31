import { expect, test } from "@playwright/test";

import {
  axis,
  expectAxisGrows,
  expectAxisVaries,
  expectVaries,
  openPreview,
} from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "badge" });
});

test.describe("styling", () => {
  test("every colour fills a badge differently", async ({ page }) => {
    await expectAxisVaries(page, "color", "background-color");
  });

  test("every size renders a badge at a size of its own", async ({ page }) => {
    await expectAxisGrows(page, "size", "height");
  });

  test("every appearance renders a badge differently", async ({ page }) => {
    const rendered = await axis(page, "appearance").evaluateAll((badges) =>
      badges.map((badge) => {
        const style = getComputedStyle(badge);
        return [
          style.backgroundColor,
          style.borderColor,
          style.borderStyle,
          style.color,
        ].join(" ");
      }),
    );

    expectVaries(rendered, "appearance");
  });

  test("a caller's classes join the badge's own", async ({ page }) => {
    const badge = page.locator("#caller-attributes");
    const classes = ((await badge.getAttribute("class")) ?? "").split(/\s+/);

    expect(classes).toContain("badge");
    expect(classes).toContain("badge-primary");
    expect(classes).toContain("badge-outline");
    expect(classes).toContain("rounded-none");
    await expect(badge).toHaveCSS("border-radius", "0px");
  });

  test("defaults emit no modifier classes", async ({ page }) => {
    const classes = ((await page.locator("#default-badge").getAttribute("class")) ?? "")
      .split(/\s+/)
      .filter(Boolean);

    expect(classes).toEqual(["badge"]);
  });
});
