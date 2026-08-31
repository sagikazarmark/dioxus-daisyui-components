import { expect, test } from "@playwright/test";

import { axis, expectAxisVaries, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "alert" });
});

test.describe("styling", () => {
  test("every colour fills an alert differently", async ({ page }) => {
    await expectAxisVaries(page, "color", "background-color");
  });

  test("every appearance renders an alert differently", async ({ page }) => {
    const rendered = await axis(page, "appearance").evaluateAll((alerts) =>
      alerts.map((alert) => {
        const style = getComputedStyle(alert);
        return [style.backgroundColor, style.borderColor, style.borderStyle, style.color].join(" ");
      }),
    );

    expectVaries(rendered, "appearance");
  });

  test("every direction arranges an alert differently", async ({ page }) => {
    await expectAxisVaries(page, "direction", "grid-auto-flow");
  });

  test("a caller's classes join the alert's own", async ({ page }) => {
    const alert = page.locator("#caller-alert");
    const classes = ((await alert.getAttribute("class")) ?? "").split(/\s+/);

    expect(classes).toContain("alert");
    expect(classes).toContain("alert-success");
    expect(classes).toContain("alert-outline");
    expect(classes).toContain("alert-vertical");
    expect(classes).toContain("rounded-none");
    await expect(alert).toHaveCSS("border-radius", "0px");
  });

  test("defaults emit only the explicit direction modifier", async ({ page }) => {
    const classes = ((await page.locator("#default-alert").getAttribute("class")) ?? "")
      .split(/\s+/)
      .filter(Boolean);

    expect(classes).toEqual(["alert", "alert-horizontal"]);
  });
});

test.describe("accessibility", () => {
  test("an alert has no live-region role unless the caller asks for one", async ({ page }) => {
    await expect(page.locator("#default-alert")).not.toHaveAttribute("role", "alert");
    await expect(page.locator("#caller-alert")).toHaveAttribute("role", "alert");
  });
});
