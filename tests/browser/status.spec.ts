import { expect, test } from "@playwright/test";

import {
  axis,
  expectAxisGrows,
  expectAxisVaries,
  openPreview,
} from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "status" });
});

test.describe("styling", () => {
  test("every colour paints the status differently", async ({ page }) => {
    await expectAxisVaries(page, "color", "background-color");
  });

  test("every size has larger square dimensions than the one before it", async ({
    page,
  }) => {
    await expectAxisGrows(page, "size", "width");
    await expectAxisGrows(page, "size", "height");

    const dimensions = await axis(page, "size").evaluateAll((statuses) =>
      statuses.map((status) => {
        const style = getComputedStyle(status);
        return [style.width, style.height];
      }),
    );
    expect(dimensions.every(([width, height]) => width === height)).toBe(true);
  });

  test("the empty span omits defaults and preserves caller attributes and classes", async ({
    page,
  }) => {
    const defaultStatus = page.locator("#default-status");
    const caller = page.locator("#caller-attributes");

    expect(await defaultStatus.evaluate((node) => node.tagName)).toBe("SPAN");
    expect(await defaultStatus.evaluate((node) => node.childNodes.length)).toBe(
      0,
    );
    expect(
      ((await defaultStatus.getAttribute("class")) ?? "")
        .split(/\s+/)
        .filter(Boolean),
    ).toEqual(["status"]);

    expect(await caller.evaluate((node) => node.tagName)).toBe("SPAN");
    expect(await caller.evaluate((node) => node.childNodes.length)).toBe(0);
    const classes = ((await caller.getAttribute("class")) ?? "").split(/\s+/);
    expect(classes).toContain("status");
    expect(classes).toContain("status-primary");
    expect(classes).toContain("status-lg");
    expect(classes).toContain("rounded-none");
    await expect(caller).toHaveAttribute("data-owner", "caller");
    await expect(caller).toHaveAttribute("role", "img");
    await expect(caller).toHaveAttribute("aria-label", "Caller-owned status");
    await expect(caller).toHaveCSS("border-radius", "0px");
  });

  test("caller ping and bounce utilities join the status classes", async ({
    page,
  }) => {
    const ping = page.locator("#caller-ping");
    const bounce = page.locator("#caller-bounce");

    await expect(ping).toHaveClass(/\bstatus-error\b/);
    await expect(ping).toHaveClass(/\banimate-ping\b/);
    await expect(ping).toHaveCSS("animation-name", "ping");
    await expect(bounce).toHaveClass(/\bstatus-info\b/);
    await expect(bounce).toHaveClass(/\banimate-bounce\b/);
    await expect(bounce).toHaveCSS("animation-name", "bounce");
  });
});
