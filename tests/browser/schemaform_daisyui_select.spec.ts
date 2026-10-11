import { expect, type Locator, type Page, test } from "@playwright/test";

import { openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "schemaform_daisyui_select" });
});

function example(page: Page, slug: string): Locator {
  return page.locator(`[data-example="${slug}"]`);
}

test("the compound select shows the chosen option on its trigger", async ({ page }) => {
  const scope = example(page, "overview");
  const region = scope.locator('button[name="/region"]');
  await expect(region).toContainText("eu");

  await region.click();
  await page.getByRole("option", { name: "us", exact: true }).click();
  await expect(region).toContainText("us");
  await expect(region).toHaveAttribute("aria-expanded", "false");
});
