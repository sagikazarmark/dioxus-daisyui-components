import { expect, type Page, test } from "@playwright/test";

import { computedStyle, example, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "breadcrumbs" });
});

test.describe("structure", () => {
  test("renders nav.breadcrumbs > ol > li with direct caller content", async ({ page }) => {
    const nav = overview(page);
    const list = nav.locator(":scope > ol");
    const items = list.locator(":scope > li");

    expect(await nav.evaluate((node) => node.tagName)).toBe("NAV");
    await expect(nav).toHaveClass(/(^|\s)breadcrumbs(\s|$)/);
    await expect(nav.locator(":scope > *")).toHaveCount(1);
    await expect(list).toHaveCount(1);
    await expect(items).toHaveCount(3);

    await expect(items.nth(0).locator(":scope > a")).toHaveText("Components");
    await expect(items.nth(1).locator(":scope > a")).toHaveText("Navigation");
    await expect(items.nth(2).locator(":scope > *")).toHaveCount(0);
    await expect(items.nth(2)).toHaveText("Breadcrumbs");
  });

  test("keeps caller icons inside direct item children", async ({ page }) => {
    const items = page.locator("#icon-breadcrumbs > ol > li");
    const icons = items.locator("svg");

    await expect(items).toHaveCount(3);
    await expect(items.nth(0).locator(":scope > a > svg")).toHaveCount(1);
    await expect(items.nth(1).locator(":scope > a > svg")).toHaveCount(1);
    await expect(items.nth(2).locator(":scope > span > svg")).toHaveCount(1);
    await expect(icons).toHaveCount(3);
    expect(
      await icons.evaluateAll((nodes) =>
        nodes.map((node) => node.getAttribute("aria-hidden")),
      ),
    ).toEqual(["true", "true", "true"]);
  });
});

test.describe("styling", () => {
  test("daisyUI generates separators between direct list items", async ({ page }) => {
    const items = page.locator("#overview-list > li");
    const separated = page.locator("#overview-list > li:not(:first-child)");

    expect(await computedStyle(items.first(), "content", "::before")).toEqual(["none"]);

    for (const content of await computedStyle(separated, "content", "::before")) {
      expect(content, "a later item has no generated separator").not.toBe("none");
    }
    for (const width of await computedStyle(separated, "width", "::before")) {
      expect(parseFloat(width), "a generated separator has no width").toBeGreaterThan(0);
    }
    for (const border of await computedStyle(separated, "border-top-style", "::before")) {
      expect(border, "a generated separator has no border").toBe("solid");
    }
  });

  test("a long constrained trail scrolls horizontally", async ({ page }) => {
    const trail = page.locator("#overflow-trail");

    await expect(trail).toHaveCSS("overflow-x", "auto");
    const dimensions = await trail.evaluate((node) => ({
      clientWidth: node.clientWidth,
      scrollWidth: node.scrollWidth,
    }));
    expect(dimensions.scrollWidth, "the long trail does not overflow").toBeGreaterThan(
      dimensions.clientWidth,
    );

    await trail.evaluate((node) => {
      node.scrollLeft = node.scrollWidth;
    });
    expect(await trail.evaluate((node) => node.scrollLeft)).toBeGreaterThan(0);
  });

  test("caller classes and global and native attributes survive on every part", async ({
    page,
  }) => {
    const root = page.locator("#caller-breadcrumbs");
    const list = page.locator("#caller-list");
    const item = page.locator("#caller-item");

    await expect(root).toHaveClass(/(^|\s)breadcrumbs(\s|$)/);
    await expect(root).toHaveClass(/(^|\s)rounded-box(\s|$)/);
    await expect(root).toHaveAttribute("data-owner", "root");
    await expect(list).toHaveClass(/(^|\s)text-sm(\s|$)/);
    await expect(list).toHaveAttribute("data-owner", "list");
    await expect(list).toHaveAttribute("start", "4");
    await expect(item).toHaveClass(/(^|\s)font-semibold(\s|$)/);
    await expect(item).toHaveAttribute("data-owner", "item");
    await expect(item).toHaveAttribute("value", "5");
  });
});

test.describe("accessibility", () => {
  test("the preview names its breadcrumb landmarks", async ({ page }) => {
    await expect(page.locator("nav.breadcrumbs")).toHaveCount(4);
    for (const name of ["Breadcrumb", "File location", "Deep location", "Account location"]) {
      await expect(page.getByRole("navigation", { name, exact: true })).toHaveCount(1);
    }
  });

  test("a non-linked current page stays text and a linked one declares aria-current", async ({
    page,
  }) => {
    const currentText = page.locator("#overview-list > li").last();

    await expect(currentText.getByRole("link")).toHaveCount(0);
    await expect(currentText).toHaveText("Breadcrumbs");
    await expect(page.getByRole("link", { name: "Profile" })).toHaveAttribute(
      "aria-current",
      "page",
    );
  });

  test("keyboard-focused links receive daisyUI's focus-visible outline", async ({ page }) => {
    const panel = example(page, "overview");
    const links = overview(page).getByRole("link");

    await panel.focus();
    for (let index = 0; index < (await links.count()); index++) {
      await page.keyboard.press("Tab");
      const link = links.nth(index);
      await expect(link).toBeFocused();
      await expect(link).toHaveCSS("outline-style", "solid");
      await expect(link).toHaveCSS("outline-width", "2px");
    }
  });
});

function overview(page: Page) {
  return page.locator("#overview-breadcrumbs");
}
