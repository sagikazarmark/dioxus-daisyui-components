import { expect, type Locator, type Page, test } from "@playwright/test";

import { axis, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "popover" });
});

test.describe("styling", () => {
  // A placement is a box in a different place, so what varies is where the
  // panel landed rather than a property on it. The panels are the second child
  // of each rendered popover, and every one of them is held open.
  test("every placement puts the panel somewhere of its own", async ({ page }) => {
    expectVaries(await boxes(panels(page, "side")), "side");
  });

  test("every alignment puts the panel somewhere of its own", async ({ page }) => {
    expectVaries(await boxes(panels(page, "align")), "align");
  });

  test("the panel's appearance axis decides whether it draws a box", async ({ page }) => {
    expectVaries(
      await panels(page, "appearance").evaluateAll((nodes) =>
        nodes.map((node) => {
          const style = getComputedStyle(node);
          return [style.backgroundColor, style.padding, style.borderRadius].join(" ");
        }),
      ),
      "appearance",
    );
  });

  test("a caller's classes join the component's own", async ({ page }) => {
    const root = page.locator("#caller-attributes");
    const classes = ((await root.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, including the class it emits for the open state,
    // on the root.
    expect(classes).toContain("dropdown");
    expect(classes).toContain("dropdown-bottom");
    expect(classes).toContain("dropdown-start");
    expect(classes).toContain("dropdown-open");

    // The panel keeps daisyUI's positioning class and takes the caller's width
    // beside the box utilities this component emits.
    const panel = page.locator("#caller-panel");
    const panelClasses = ((await panel.getAttribute("class")) ?? "").split(/\s+/);
    expect(panelClasses).toContain("dropdown-content");
    expect(panelClasses).toContain("bg-base-100");
    expect(panelClasses).toContain("w-56");
    await expect(panel).toHaveCSS("width", "224px");

    // And the box utilities switch off, which is how a caller wins a tie
    // against a utility rather than out-ranking it (ADR-0004); the positioning
    // is daisyUI's and stays either way.
    const repainted = page.locator("#caller-repainted");
    expect(((await repainted.getAttribute("class")) ?? "").split(/\s+/)).toContain(
      "dropdown-content",
    );
    await expect(repainted).toHaveCSS("border-radius", "0px");
    await expect(repainted).toHaveCSS("position", "absolute");
  });
});

test.describe("behaviour", () => {
  const changes = (page: Page) => page.getByTestId("changes");
  const state = (page: Page) => page.getByTestId("state");

  // Tier 2, observed from the outside: the panel is readable while the popover
  // is open and not while it is shut. Which class does it is deliberately not
  // asserted; that is composition, and ADR-0007 keeps these specs off it.
  test("the trigger opens the panel and closes it again", async ({ page }) => {
    const trigger = page.getByRole("button", { name: "Filters" });
    const panel = page.getByText("A popover holds whatever you write");

    await expect(panel).toHaveCount(0);

    await trigger.click();
    await expect(panel).toBeVisible();

    await trigger.click();
    await expect(panel).toHaveCount(0);
  });

  test("Escape and a click outside both dismiss the panel", async ({ page }) => {
    const trigger = page.getByRole("button", { name: "Filters" });
    const panel = page.getByText("A popover holds whatever you write");

    await trigger.click();
    await expect(panel).toBeVisible();

    await page.keyboard.press("Escape");
    await expect(panel).toHaveCount(0);

    await trigger.click();
    await expect(panel).toBeVisible();

    // Somewhere that is neither the trigger nor the panel: the heading of the
    // page itself.
    await page.getByRole("heading", { name: "Popover", exact: true }).click();
    await expect(panel).toHaveCount(0);
  });

  test("the panel is a dialog named by its trigger", async ({ page }) => {
    await page.getByRole("button", { name: "Filters" }).click();

    const panel = page.locator("#overview [role='dialog']");
    await expect(panel).toBeVisible();

    // The relationship a screen reader announces the panel by, which is the
    // primitive's and which the classes this component emits leave intact.
    const labelledby = await panel.getAttribute("aria-labelledby");
    expect(labelledby, "the panel is named by nothing").toBeTruthy();
    await expect(page.locator(`#${labelledby}`)).toHaveText("Filters");
  });

  test("a modal popover traps the keyboard and a non-modal one does not", async ({ page }) => {
    await page.getByRole("button", { name: "Modal", exact: true }).click();
    await page.locator("#modal-last").focus();

    // Tabbing off the last control inside a modal panel comes back round to the
    // panel rather than walking on into the page behind it.
    await page.keyboard.press("Tab");
    await expect(page.locator("#outside")).not.toBeFocused();
    await expect(page.locator("#modal [role='dialog'] :focus")).toBeAttached();

    await page.keyboard.press("Escape");

    // The other answer: the keyboard walks straight out of a panel that is not
    // modal, which is what a panel beside a page a reader is still using wants.
    await page.getByRole("button", { name: "Not modal", exact: true }).click();
    await page.locator("#modeless-only").focus();

    await page.keyboard.press("Tab");
    await expect(page.locator("#outside")).toBeFocused();
  });

  test("a controlled popover answers to its caller rather than to itself", async ({ page }) => {
    const trigger = page.locator("#controlled-trigger");

    await expect(state(page)).toHaveText("closed");

    await trigger.click();
    await expect(state(page)).toHaveText("open");
    await expect(changes(page)).toHaveText("1");

    // A button inside the panel closes it, which is the half an uncontrolled
    // popover cannot do: the state travels out through the callback and back in
    // through the open prop.
    await page.locator("#controlled-confirm").click();
    await expect(state(page)).toHaveText("closed");
    await expect(page.locator("#controlled-confirm")).toHaveCount(0);
  });
});

/** The panels of one axis' rendered set, in the order the variant list is in. */
function panels(page: Page, name: string): Locator {
  return axis(page, name).locator("> [role='dialog']");
}

/**
 * Where each of them landed *against its own trigger*, as a string so that a
 * pair can be compared.
 *
 * Relative rather than absolute, because every popover in a rendered set sits
 * somewhere else on the page already; absolute positions would differ under an
 * axis that did nothing at all.
 */
function boxes(panels: Locator): Promise<string[]> {
  return panels.evaluateAll((nodes) =>
    nodes.map((node) => {
      const panel = node.getBoundingClientRect();
      const root = (node.parentElement as HTMLElement).getBoundingClientRect();
      return `${Math.round(panel.left - root.left)}x${Math.round(panel.top - root.top)}`;
    }),
  );
}
