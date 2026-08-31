import { expect, test } from "@playwright/test";

import { computedStyle, expectGrows, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "table" });
});

test.describe("native structure", () => {
  test("every part preserves its native table element and valid ordering", async ({ page }) => {
    const table = page.locator("#semantic-table");

    await expect(table).toHaveRole("table");
    await expect(table).toHaveAccessibleName("Quarterly subscriptions by plan");
    expect(await table.evaluate((node) => node.tagName)).toBe("TABLE");
    expect(await table.evaluate((node) => [...node.children].map((child) => child.tagName))).toEqual([
      "CAPTION",
      "THEAD",
      "TBODY",
      "TFOOT",
    ]);

    expect(await page.locator("#sales-caption").evaluate((node) => node.tagName)).toBe("CAPTION");
    expect(await page.locator("#semantic-head").evaluate((node) => node.tagName)).toBe("THEAD");
    expect(await page.locator("#semantic-body").evaluate((node) => node.tagName)).toBe("TBODY");
    expect(await page.locator("#semantic-foot").evaluate((node) => node.tagName)).toBe("TFOOT");
    expect(await page.locator("#heading-row").evaluate((node) => node.tagName)).toBe("TR");
    expect(await page.locator("#plan-heading").evaluate((node) => node.tagName)).toBe("TH");
    expect(await page.locator("#starter-accounts").evaluate((node) => node.tagName)).toBe("TD");
  });

  test("native semantic attributes and caption content survive forwarding", async ({ page }) => {
    await expect(page.locator("#sales-caption")).toHaveText("Quarterly subscriptions by plan");
    await expect(page.locator("#plan-heading")).toHaveAttribute("scope", "col");
    await expect(page.locator("#plan-heading")).toHaveAttribute("rowspan", "2");
    await expect(page.locator("#quarter-heading")).toHaveAttribute("scope", "colgroup");
    await expect(page.locator("#quarter-heading")).toHaveAttribute("colspan", "2");
    await expect(page.locator("#starter-heading")).toHaveAttribute("scope", "row");
    await expect(page.locator("#starter-accounts")).toHaveAttribute(
      "headers",
      "starter-heading accounts-heading",
    );
    await expect(page.locator("#total-cell")).toHaveAttribute(
      "headers",
      "total-heading accounts-heading revenue-heading",
    );
    await expect(page.locator("#total-cell")).toHaveAttribute("colspan", "2");
  });

  test("caller classes and global attributes reach every Compound part", async ({ page }) => {
    const expected = [
      ["#semantic-table", "table", "rounded-none"],
      ["#sales-caption", "caption-top", "font-semibold"],
      ["#semantic-head", "bg-base-200/50"],
      ["#semantic-body", "text-base-content"],
      ["#semantic-foot", "bg-base-200/50"],
      ["#active-row", "bg-base-200"],
      ["#plan-heading", "whitespace-nowrap"],
      ["#starter-accounts", "font-medium"],
    ];

    for (const [selector, ...classes] of expected) {
      const actual = ((await page.locator(selector).getAttribute("class")) ?? "").split(/\s+/);
      for (const className of classes) expect(actual).toContain(className);
    }

    await expect(page.locator("#semantic-table")).toHaveAttribute("data-owner", "caller");
    await expect(page.locator("#semantic-table")).toHaveCSS("border-radius", "0px");
    await expect(page.locator("#active-row")).not.toHaveCSS(
      "background-color",
      "rgba(0, 0, 0, 0)",
    );
  });

  test("horizontal overflow belongs to the caller's wrapper", async ({ page }) => {
    const wrapper = page.locator("#caller-overflow-wrapper");
    const table = page.locator("#wide-table");

    await expect(wrapper).toHaveClass(/\boverflow-x-auto\b/);
    await expect(table).toHaveClass(/\btable\b/);
    await expect(table).not.toHaveClass(/\boverflow-x-auto\b/);
    expect(await table.evaluate((node) => node.parentElement?.id)).toBe("caller-overflow-wrapper");

    const dimensions = await wrapper.evaluate((node) => ({
      client: node.clientWidth,
      scroll: node.scrollWidth,
    }));
    expect(dimensions.scroll).toBeGreaterThan(dimensions.client);
  });
});

test.describe("axes", () => {
  test("every size increases cell spacing and the medium default emits no modifier", async ({
    page,
  }) => {
    const tables = page.locator('[data-axis="size"] > table');
    await expect(tables).toHaveCount(5);

    const padding = await tables.evaluateAll((nodes) =>
      nodes.map((node) =>
        parseFloat(getComputedStyle(node.querySelector("tbody td") as Element).paddingTop),
      ),
    );
    expectGrows(padding, "size");

    const classes = await tables.evaluateAll((nodes) =>
      nodes.map((node) => [...node.classList].filter((name) => name.startsWith("table-"))),
    );
    expect(classes).toEqual([
      ["table-xs"],
      ["table-sm"],
      [],
      ["table-lg"],
      ["table-xl"],
    ]);
  });

  test("the zebra Axis independently stripes even body rows", async ({ page }) => {
    const tables = page.locator('[data-axis="zebra"] > table');
    await expect(tables).toHaveCount(2);

    const classes = await tables.evaluateAll((nodes) => nodes.map((node) => [...node.classList]));
    expect(classes[0]).not.toContain("table-zebra");
    expect(classes[1]).toContain("table-zebra");

    const evenRows = tables.locator("tbody tr:nth-child(2)");
    expectVaries(await computedStyle(evenRows, "background-color"), "zebra");
  });

  test("the row-pinning Axis independently sticks the header and footer", async ({ page }) => {
    const wrappers = page.locator('[data-axis="row-pinning"] > div');
    await expect(wrappers).toHaveCount(2);

    const tables = wrappers.locator("table");
    await expect(tables.first()).not.toHaveClass(/\btable-pin-rows\b/);
    await expect(tables.last()).toHaveClass(/\btable-pin-rows\b/);

    expectVaries(await computedStyle(wrappers.locator("thead"), "position"), "row-pinning header");
    expectVaries(await computedStyle(wrappers.locator("tfoot"), "position"), "row-pinning footer");
    await expect(wrappers.last().locator("thead")).toHaveCSS("position", "sticky");
    await expect(wrappers.last().locator("tfoot")).toHaveCSS("position", "sticky");
  });

  test("the column-pinning Axis independently sticks header cells", async ({ page }) => {
    const wrappers = page.locator('[data-axis="column-pinning"] > div');
    await expect(wrappers).toHaveCount(2);

    const tables = wrappers.locator("table");
    await expect(tables.first()).not.toHaveClass(/\btable-pin-cols\b/);
    await expect(tables.last()).toHaveClass(/\btable-pin-cols\b/);

    const rowHeaders = wrappers.locator("tbody tr:first-child th");
    expectVaries(await computedStyle(rowHeaders, "position"), "column-pinning");
    await expect(wrappers.last().locator("tbody tr:first-child th")).toHaveCSS(
      "position",
      "sticky",
    );
  });

  test("row and column pinning apply together", async ({ page }) => {
    const wrapper = page.locator("#combined-pinning-wrapper");
    const table = wrapper.locator("table");
    const classes = ((await table.getAttribute("class")) ?? "").split(/\s+/);

    expect(classes).toContain("table-pin-rows");
    expect(classes).toContain("table-pin-cols");
    await expect(table.locator("thead")).toHaveCSS("position", "sticky");
    await expect(table.locator("tfoot")).toHaveCSS("position", "sticky");
    await expect(table.locator("tbody tr:first-child th")).toHaveCSS("position", "sticky");

    const dimensions = await wrapper.evaluate((node) => ({
      horizontal: node.scrollWidth > node.clientWidth,
      vertical: node.scrollHeight > node.clientHeight,
    }));
    expect(dimensions).toEqual({ horizontal: true, vertical: true });
  });
});
