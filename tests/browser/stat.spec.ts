import { expect, type Page, test } from "@playwright/test";

import { axis, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "stat" });
});

test.describe("styling", () => {
  test("every direction changes the grid flow explicitly", async ({ page }) => {
    const flows = await axis(page, "direction").evaluateAll((stats) =>
      stats.map((item) => getComputedStyle(item).gridAutoFlow),
    );

    expect(flows).toEqual(["column", "row"]);
  });

  test("every direction puts the divider on the corresponding edge", async ({ page }) => {
    const dividers = axis(page, "direction").locator(":scope > .stat:first-child");
    const borders = await dividers.evaluateAll((stats) =>
      stats.map((stat) => {
        const style = getComputedStyle(stat);
        return [style.borderInlineEndStyle, style.borderBlockEndStyle];
      }),
    );

    expect(borders).toEqual([
      ["dashed", "none"],
      ["none", "dashed"],
    ]);
  });

  test("the compound parts remain direct grid children", async ({ page }) => {
    const stats = axis(page, "direction");

    await expect(stats).toHaveCount(2);
    await expect(stats.locator(":scope > .stat")).toHaveCount(4);
    await expect(stats.locator(":scope > .stat > .stat-title")).toHaveCount(4);
    await expect(stats.locator(":scope > .stat > .stat-value")).toHaveCount(4);
    await expect(stats.locator(":scope > .stat > .stat-desc")).toHaveCount(4);
    await expect(stats.locator(":scope > .stat > .stat-figure")).toHaveCount(2);
    await expect(stats.locator(":scope > .stat > .stat-actions")).toHaveCount(2);
  });

  test("every part occupies daisyUI's grid placement", async ({ page }) => {
    await expectGridPlacement(page, "#placement-title", "1");
    await expectGridPlacement(page, "#placement-value", "1");
    await expectGridPlacement(page, "#placement-description", "1");
    await expectGridPlacement(page, "#placement-actions", "1");
    await expectGridPlacement(page, "#placement-figure", "2", "1", "span 3");
  });

  test("a caller's classes and attributes join every part's own", async ({ page }) => {
    await expectPart(page, "#caller-stats", "Caller stats", "stats", "stats-vertical", "rounded-none");
    await expectPart(page, "#caller-stat", "Caller stat", "stat", "px-2");
    await expectPart(page, "#caller-title", "Caller title", "stat-title", "italic");
    await expectPart(page, "#caller-value", "Caller value", "stat-value", "text-primary");
    await expectPart(
      page,
      "#caller-description",
      "Caller description",
      "stat-desc",
      "uppercase",
    );
    await expectPart(page, "#caller-figure", "Caller figure", "stat-figure", "text-secondary");
    await expectPart(
      page,
      "#caller-actions",
      "Caller actions",
      "stat-actions",
      "justify-self-end",
    );
    await expect(page.locator("#caller-stats")).toHaveCSS("border-radius", "0px");
  });

  test("the default emits the explicit horizontal modifier", async ({ page }) => {
    const classes = ((await page.locator("#default-stats").getAttribute("class")) ?? "")
      .split(/\s+/)
      .filter(Boolean);

    expect(classes).toEqual(["stats", "stats-horizontal"]);
  });
});

async function expectGridPlacement(
  page: Page,
  selector: string,
  columnStart: string,
  rowStart = "auto",
  rowEnd = "auto",
): Promise<void> {
  const placement = await page.locator(selector).evaluate((part) => {
    const style = getComputedStyle(part);
    return [style.gridColumnStart, style.gridRowStart, style.gridRowEnd];
  });

  expect(placement).toEqual([columnStart, rowStart, rowEnd]);
}

async function expectPart(
  page: Page,
  selector: string,
  title: string,
  ...expectedClasses: string[]
): Promise<void> {
  const part = page.locator(selector);
  const classes = ((await part.getAttribute("class")) ?? "").split(/\s+/);

  for (const expectedClass of expectedClasses) expect(classes).toContain(expectedClass);
  await expect(part).toHaveAttribute("title", title);
}
