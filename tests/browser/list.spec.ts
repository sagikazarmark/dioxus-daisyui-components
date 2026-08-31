import { expect, type Page, test } from "@playwright/test";

import { axis, computedStyle, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "list" });
});

test.describe("styling", () => {
  test("the list, rows, and columns carry daisyUI's structural classes", async ({ page }) => {
    await expectPartClasses(page, "#default-list", "list");
    await expectPartClasses(page, "#default-row", "list-row");
    await expectPartClasses(page, "#default-grow-column", "list-col-grow");
    await expectPartClasses(page, "#default-wrap-column", "list-col-wrap");
  });

  test("every grow value allocates the row differently", async ({ page }) => {
    expectVaries(await computedStyle(axis(page, "grow"), "grid-template-columns"), "grow");
  });

  test("every wrap value places the column on a different row", async ({ page }) => {
    const columns = axis(page, "wrap").locator(":scope > div:first-child");
    expectVaries(await computedStyle(columns, "grid-row-start"), "wrap");
  });

  test("defaults emit no column modifier classes", async ({ page }) => {
    const defaultGrow = axis(page, "grow").first().locator(":scope > div:first-child");
    const defaultWrap = axis(page, "wrap").first().locator(":scope > div:first-child");

    expect(((await defaultGrow.getAttribute("class")) ?? "").trim()).toBe("");
    expect(((await defaultWrap.getAttribute("class")) ?? "").trim()).toBe("");
  });

  test("a caller's classes join every part's own", async ({ page }) => {
    await expectPartClasses(page, "#caller-list", "list", "rounded-none");
    await expectPartClasses(page, "#caller-row", "list-row", "p-2");
    await expectPartClasses(
      page,
      "#caller-column",
      "list-col-grow",
      "list-col-wrap",
      "italic",
    );
  });
});

async function expectPartClasses(
  page: Page,
  selector: string,
  ...expectedClasses: string[]
): Promise<void> {
  const classes = ((await page.locator(selector).getAttribute("class")) ?? "").split(/\s+/);
  for (const expectedClass of expectedClasses) expect(classes).toContain(expectedClass);
}
