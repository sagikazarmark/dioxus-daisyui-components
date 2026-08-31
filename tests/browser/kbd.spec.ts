import { expect, test } from "@playwright/test";

import { expectAxisGrows, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "kbd" });
});

test("renders a native noninteractive kbd with no default size modifier", async ({ page }) => {
  const key = page.locator("#default-kbd");

  await expect(key).toHaveJSProperty("tagName", "KBD");
  await expect(key).toHaveJSProperty("tabIndex", -1);
  await expect(key).not.toHaveAttribute("role", /.+/);
  expect(
    ((await key.getAttribute("class")) ?? "").split(/\s+/).filter(Boolean),
  ).toEqual(["kbd"]);
});

test("every size renders a key at a size of its own", async ({ page }) => {
  await expectAxisGrows(page, "size", "height");
});

test("caller classes and attributes survive merging", async ({ page }) => {
  const key = page.locator("#caller-attributes");
  const classes = ((await key.getAttribute("class")) ?? "").split(/\s+/);

  expect(classes).toContain("kbd");
  expect(classes).toContain("kbd-lg");
  expect(classes).toContain("rounded-none");
  expect(classes).toContain("text-primary");
  await expect(key).toHaveCSS("border-radius", "0px");
  await expect(key).toHaveAttribute("data-owner", "caller");
  await expect(key).toHaveAttribute("aria-label", "Command key");
});
