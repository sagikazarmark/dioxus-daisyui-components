import { expect, test } from "@playwright/test";

import { openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "skeleton" });
});

test.describe("styling", () => {
  test("the block form is a rounded animated gradient", async ({ page }) => {
    const block = page.locator("#rectangular-placeholder");

    await expect(block).not.toHaveCSS("background-image", "none");
    await expect(block).not.toHaveCSS("border-radius", "0px");
    await expect(block).toHaveCSS("animation-name", "skeleton");
    await expect(block).toHaveCSS("width", "256px");
    await expect(block).toHaveCSS("height", "128px");
  });

  test("the text form clips the gradient to its real text", async ({ page }) => {
    const text = page.locator("#text-placeholder");

    await expect(text).toHaveText("Preparing your account details");
    await expect(text).toHaveCSS("background-clip", "text");
    await expect(text).toHaveCSS("color", "rgba(0, 0, 0, 0)");
    await expect(text).not.toHaveCSS("background-image", "none");
  });

  test("both forms preserve caller classes and attributes", async ({ page }) => {
    const block = page.locator("#caller-block");
    const text = page.locator("#caller-text");

    await expect(block).toHaveClass(/\bskeleton\b/);
    await expect(block).toHaveClass(/\bw-32\b/);
    await expect(block).toHaveAttribute("data-owner", "caller");
    await expect(block).toHaveCSS("width", "128px");
    await expect(block).toHaveCSS("height", "48px");
    await expect(block).toHaveCSS("border-radius", "0px");

    await expect(text).toHaveClass(/\bskeleton\b/);
    await expect(text).toHaveClass(/\bskeleton-text\b/);
    await expect(text).toHaveClass(/\btext-xl\b/);
    await expect(text).toHaveAttribute("data-owner", "caller");
    await expect(text).toHaveText("Caller text survives");
  });
});
