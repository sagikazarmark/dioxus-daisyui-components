import { expect, type Page, test } from "@playwright/test";

import {
  axisTriggers,
  computedStyle,
  example,
  expectVaries,
  openPreview,
} from "./preview";

declare global {
  interface Window {
    /** Transition properties recorded by the drawer motion test. */
    drawerMotion?: string[];
    /** The panel border recorded when its close transition starts. */
    drawerClosingBorder?: string;
    /** Side visibility whenever focus enters a drawer panel. */
    drawerFocusVisibility?: string[];
  }
}

test.beforeEach(async ({ page }, testInfo) => {
  const motion = testInfo.title.startsWith("animates in and out")
    ? "running"
    : "still";
  await openPreview(page, { component: "drawer" }, motion);
  await dismissPlacementShowcase(page);
});

test("keeps the page mounted around the projected toggle and dialog side", async ({
  page,
}) => {
  const showcase = example(page, "overview");
  const drawer = showcase.locator(".drawer");
  const children = drawer.locator(":scope > *");

  await expect(children.nth(0)).toHaveClass(/(^|\s)drawer-toggle(\s|$)/);
  await expect(children.nth(0)).toBeDisabled();
  await expect(children.nth(0)).toHaveAttribute("aria-hidden", "true");
  await expect(children.nth(1)).toHaveClass(/(^|\s)drawer-content(\s|$)/);
  await expect(children.nth(1)).toBeAttached();

  const trigger = showcase.getByRole("button", { name: "Open navigation" });
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await expect(showcase.locator(".drawer-side")).toHaveCount(0);

  await trigger.click();

  await expect(children.nth(0)).toBeChecked();
  await expect(children.nth(1)).toBeAttached();
  await expect(children.nth(2)).toHaveClass(/(^|\s)drawer-side(\s|$)/);
  await expect(trigger).toHaveAttribute("aria-expanded", "true");
});

test("uses direct overlay and panel children and announces the panel", async ({
  page,
}) => {
  const showcase = example(page, "overview");
  const trigger = showcase.getByRole("button", { name: "Open navigation" });
  await trigger.click();

  const side = showcase.locator(".drawer-side");
  const children = side.locator(":scope > *");
  await expect(children).toHaveCount(2);
  await expect(children.nth(0)).toHaveClass(/(^|\s)drawer-overlay(\s|$)/);

  const panel = children.nth(1);
  await expect(panel).toHaveRole("dialog");
  await expect(panel).toHaveAttribute("aria-modal", "true");
  await expect(panel).toHaveAccessibleName("Navigation");
  await expect(panel).toHaveAccessibleDescription(
    "Choose a section of the application.",
  );

  const controls = await trigger.getAttribute("aria-controls");
  expect(controls, "the trigger points at no side").toBeTruthy();
  await expect(side).toHaveAttribute("id", controls as string);
});

test("an uncontrolled drawer traps focus, restores it after Escape, and dismisses by overlay", async ({
  page,
}) => {
  const showcase = example(page, "overview");
  const trigger = showcase.getByRole("button", { name: "Open navigation" });
  const content = showcase.locator(".drawer-content");

  await recordDrawerFocusVisibility(page);
  await trigger.focus();
  await page.keyboard.press("Enter");

  const dashboard = showcase.getByRole("link", { name: "Dashboard" });
  const settings = showcase.getByRole("link", { name: "Settings" });
  await expect(dashboard).toBeFocused();
  expect(await page.evaluate(() => window.drawerFocusVisibility)).toEqual([
    "visible",
  ]);
  await page.keyboard.press("Tab");
  await expect(settings).toBeFocused();
  await page.keyboard.press("Tab");
  await expect(dashboard).toBeFocused();

  await page.keyboard.press("Escape");
  await expect(showcase.locator(".drawer-side")).toHaveCount(0);
  await expect(content).toBeAttached();
  await expect(trigger).toBeFocused();

  await page.keyboard.press("Enter");
  await expect(dashboard).toBeFocused();
  await showcase
    .locator(".drawer-overlay")
    .click({ position: { x: 700, y: 100 } });
  await expect(showcase.locator(".drawer-side")).toHaveCount(0);
  await expect(trigger).toBeFocused();
});

test("a controlled drawer opens and dismisses through its caller", async ({
  page,
}) => {
  const controlled = example(page, "controlled");
  const trigger = controlled.getByRole("button", {
    name: "Open account drawer",
  });

  await expect(controlled.getByTestId("controlled-state")).toHaveText("closed");
  await expect(controlled.getByTestId("controlled-changes")).toHaveText("0");
  await trigger.click();
  await expect(controlled.getByTestId("controlled-state")).toHaveText("open");
  await expect(controlled.getByTestId("controlled-changes")).toHaveText("1");
  await expect(
    controlled.getByRole("button", { name: "Close account drawer" }),
  ).toBeFocused();

  await page.keyboard.press("Escape");
  await expect(controlled.getByTestId("controlled-state")).toHaveText("closed");
  await expect(controlled.getByTestId("controlled-changes")).toHaveText("2");
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await expect(trigger).toBeFocused();
});

test("caller classes and attributes reach every exposed element", async ({
  page,
}) => {
  const controlled = example(page, "controlled");
  const trigger = controlled.locator("#controlled-trigger");
  await trigger.click();

  const expected = [
    ["#controlled-drawer", "root", "rounded-none"],
    ["#controlled-content", "content", "p-1"],
    ["#controlled-trigger", "trigger", "btn-primary"],
    ["#controlled-side", "side", "z-20"],
    ["#controlled-overlay", "overlay", "bg-secondary"],
    ["#controlled-panel", "panel", "w-72"],
    ["#controlled-title", "title", "text-2xl"],
    ["#controlled-description", "description", "italic"],
  ] as const;

  for (const [selector, part, className] of expected) {
    const element = controlled.locator(selector);
    await expect(element).toHaveAttribute("data-part", part);
    await expect(element).toHaveClass(new RegExp(`(^|\\s)${className}(\\s|$)`));
  }

  await expect(trigger).toHaveAttribute("aria-controls", "controlled-side");
});

test("every placement enters from a different inline edge", async ({
  page,
}) => {
  const placements = example(page, "placements");
  const triggers = axisTriggers(page, "placement");
  const grids: string[] = [];

  expect(await triggers.count(), "the placement axis rendered nothing").toBe(2);

  for (let index = 0; index < 2; index++) {
    const trigger = triggers.nth(index);
    await trigger.click();

    const panel = placements.locator("#placement-panel");
    await expect(panel).toBeVisible();
    const [grid] = await computedStyle(
      placements.locator(".drawer"),
      "grid-auto-columns",
    );
    grids.push(grid);

    await page.keyboard.press("Escape");
    await expect(panel).toHaveCount(0);
  }

  expectVaries(grids, "placement");
});

test.describe("motion and projected state variants", () => {
  test("animates in and out while open and close variants follow the checkbox", async ({
    page,
  }) => {
    await recordMotion(page);

    const trigger = axisTriggers(page, "placement").first();
    await trigger.click();
    const panel = page.locator("#placement-panel");
    await expect
      .poll(() => recordedMotion(page), {
        message: "nothing transitioned on the way in",
      })
      .toContain("translate");
    const [openBorder] = await computedStyle(panel, "border-color");

    await forgetMotion(page);
    await page.keyboard.press("Escape");

    await expect
      .poll(() => recordedMotion(page), {
        message: "nothing transitioned on the way out",
      })
      .toContain("translate");
    await expect
      .poll(() => page.evaluate(() => window.drawerClosingBorder), {
        message: "the close variant was never observable during exit",
      })
      .not.toBeUndefined();
    const closingBorder = await page.evaluate(() => window.drawerClosingBorder);
    expect(closingBorder).not.toBe(openBorder);
    await expect(panel).toHaveCount(0);
  });
});

async function recordMotion(page: Page): Promise<void> {
  await page.evaluate(() => {
    window.drawerMotion = [];
    window.drawerClosingBorder = undefined;
    document.addEventListener(
      "transitionrun",
      (event) => {
        if (
          event.target instanceof Element &&
          event.target.closest(".drawer-side")
        ) {
          window.drawerMotion?.push(event.propertyName);
          const side = event.target.closest(".drawer-side");
          const drawer = side?.parentElement;
          const toggle = drawer?.querySelector(".drawer-toggle");
          const panel = side?.querySelector("#placement-panel");
          if (toggle instanceof HTMLInputElement && !toggle.checked && panel) {
            window.drawerClosingBorder = getComputedStyle(panel).borderColor;
          }
        }
      },
      true,
    );
  });
}

function recordedMotion(page: Page): Promise<string[]> {
  return page.evaluate(() => window.drawerMotion ?? []);
}

async function forgetMotion(page: Page): Promise<void> {
  await page.evaluate(() => window.drawerMotion?.splice(0));
}

/** Records whether a panel is visible at each focus entry. */
async function recordDrawerFocusVisibility(page: Page): Promise<void> {
  await page.evaluate(() => {
    window.drawerFocusVisibility = [];
    document.addEventListener(
      "focusin",
      (event) => {
        if (!(event.target instanceof Element)) return;
        const side = event.target.closest(".drawer-side");
        if (side)
          window.drawerFocusVisibility?.push(getComputedStyle(side).visibility);
      },
      true,
    );
  });
}

/** Unmounts the two non-modal drawers that the page holds open for its baseline. */
async function dismissPlacementShowcase(page: Page): Promise<void> {
  const panels = page.locator("[data-placement-showcase]");
  if ((await panels.count()) > 0) {
    const showcase = example(page, "showcase");
    const toggles = showcase.locator(".drawer-toggle");
    await expect(toggles).toHaveCount(2);
    await expect(toggles.nth(0)).toBeChecked();
    await expect(toggles.nth(1)).toBeChecked();
    await expect(panels.nth(0)).toBeVisible();
    await expect(panels.nth(1)).toBeVisible();
    const sideWidths = await showcase
      .locator(".drawer-side")
      .evaluateAll((sides) =>
        sides.map((side) => side.getBoundingClientRect().width),
      );
    expect(
      Math.min(...sideWidths),
      "a placement showcase side is clipped",
    ).toBeGreaterThan(400);
    await page
      .locator('[data-example="showcase"]')
      .getByRole("tab", { name: "RSX" })
      .click();
    await expect(panels).toHaveCount(0);
  }
}
