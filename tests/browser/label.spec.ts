import { expect, test } from "@playwright/test";

import { axis, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "label" });
});

test.describe("styling", () => {
  // daisyUI expresses its three labels across more than one property (one
  // mutes the text, one lays the element out, one makes it a positioning
  // context for the caption it lifts) so this reads the set for itself and
  // then says what every other axis says about it: every value renders
  // differently from every other.
  test("every appearance renders a label of its own", async ({ page }) => {
    const rendered = await axis(page, "appearance").evaluateAll((nodes) =>
      nodes.map((node) => {
        const style = getComputedStyle(node);
        return [style.display, style.color, style.position, style.whiteSpace].join(" ");
      }),
    );

    expectVaries(rendered, "appearance");
  });

  test("a caller's classes join the component's own", async ({ page }) => {
    const label = page.locator("#caller-attributes");
    const classes = ((await label.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, and then the caller's, on one element.
    expect(classes).toContain("label");
    expect(classes).toContain("text-error");
    expect(classes).toContain("font-semibold");

    // And the caller's applies, rather than merely surviving the merge, over
    // the muted colour daisyUI's own class sets, which the caller wins on
    // cascade layers rather than on specificity (ADR-0004).
    const [muted, repainted] = await page
      .locator("#caller-none, #caller-attributes")
      .evaluateAll((nodes) => nodes.map((node) => getComputedStyle(node).color));
    expect(repainted, "the caller's colour did not reach the label").not.toBe(muted);
  });

  // The axis value that emits nothing, which is what a caller switches to when
  // they want the element and not daisyUI's look for it.
  test("the appearance axis can emit no class at all", async ({ page }) => {
    const classes = ((await page.locator("#caller-none").getAttribute("class")) ?? "").split(/\s+/);

    expect(classes).not.toContain("label");
    expect(classes).not.toContain("fieldset-label");
    expect(classes).not.toContain("floating-label");
    expect(classes).toContain("uppercase");
  });
});

test.describe("behaviour", () => {
  test("a caption names the control it points at", async ({ page }) => {
    const label = page.getByText("Project name");
    const control = page.locator("#overview-project");

    // The association a screen reader announces the control by, which is the
    // whole of what the primitive adds: the caption points at the control's id,
    // and the control is named by the caption's text.
    await expect(label).toHaveAttribute("for", "overview-project");
    await expect(control).toHaveAccessibleName("Project name");
  });

  test("clicking a caption focuses the control it names", async ({ page }) => {
    const control = page.locator("#overview-project");

    await expect(control).not.toBeFocused();

    await page.getByText("Project name").click();
    await expect(control).toBeFocused();
  });

  // The registry's own controls are `button` elements the primitives render,
  // which are labelable, so a caption reaches them exactly as it reaches an
  // input, and operating the control through its caption works.
  test("clicking a caption operates the registry control it names", async ({ page }) => {
    const control = page.locator("#controls-watch");

    await expect(control).not.toBeChecked();

    await page.getByText("Rebuild on change").click();
    await expect(control).toBeChecked();
  });
});
