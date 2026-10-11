import { expect, type Locator, type Page, test } from "@playwright/test";

import { openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "schemaform_daisyui_radio" });
});

function example(page: Page, slug: string): Locator {
  return page.locator(`[data-example="${slug}"]`);
}

test("the radio widget selects an option per item, the null option included", async ({ page }) => {
  const scope = example(page, "overview");
  const yearly = scope.getByRole("radio", { name: "yearly", exact: true });
  const monthly = scope.getByRole("radio", { name: "monthly", exact: true });
  await expect(yearly).toHaveAttribute("aria-checked", "true");

  await monthly.click();
  await expect(monthly).toHaveAttribute("aria-checked", "true");
  await expect(yearly).toHaveAttribute("aria-checked", "false");
});

test("a radio field sits in the base frame, at the form's density and button classes", async ({ page }) => {
  const scope = example(page, "overview");
  const field = scope.locator('[data-schemaform-daisyui="choice"]').first();
  await expect(field).toHaveAttribute("data-schemaform-density", "compact");

  // Compact puts the presence operations in the label row, in the host's class.
  const presence = field.locator('[data-schemaform-field-header] [data-schemaform-button="presence"]').first();
  await expect(presence).toHaveClass(/\bbtn-ghost\b/);
  await expect(presence).not.toHaveClass(/\bbtn-sm\b/);
});
