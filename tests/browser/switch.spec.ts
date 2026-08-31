import { expect, type Page, test } from "@playwright/test";

import { computedStyle, expectAxisVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "switch" });
});

test.describe("styling", () => {
  // The colour row is rendered on, which is the state daisyUI expresses a
  // toggle's colour in: every colour rule is written against the checked
  // element, so a row of switches that were off would all read the same.
  //
  // What is read is the toggle's own `color`, which is where a colour class
  // lands: a checked toggle paints its track `base-100` whatever the colour
  // says, and the knob is filled with `currentColor` from a pseudo-element.
  // Reading the knob instead would say nothing on webkit, which resolves
  // `currentColor` in a pseudo-element's background to the same value for
  // every one of them.
  test("every colour applies a colour of its own", async ({ page }) => {
    await expectAxisVaries(page, "color", "color");
  });

  test("a caller's classes join the component's own", async ({ page }) => {
    const toggle = page.locator("#caller-attributes");
    const classes = ((await toggle.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, and then the caller's, on one element.
    expect(classes).toContain("toggle");
    expect(classes).toContain("toggle-primary");
    expect(classes).toContain("rounded-none");

    // And the caller's applies, rather than merely surviving the merge, over
    // a corner radius daisyUI's own class sets, which the caller wins on
    // cascade layers rather than on specificity (ADR-0004).
    await expect(toggle).toHaveCSS("border-radius", "0px");
  });

  // The size gap, from the other side: this component exposes no size axis
  // because daisyUI's size classes cannot match a `button`, and what it
  // documents in their place is a caller setting daisyUI's own custom property
  // through a utility. This asserts the documented escape hatch works, so that
  // the advice cannot rot silently.
  test("a caller sizes the switch through daisyUI's own custom property", async ({ page }) => {
    const [normal, sized] = (
      await computedStyle(page.locator("#caller-attributes, #caller-size"), "height")
    ).map(parseFloat);

    expect(sized, "the caller's size utility did not reach the toggle").toBeGreaterThan(normal);
  });

  test("the registry form participant stays hidden without generated utilities", async ({ page }) => {
    const formControl = page
      .locator("#field-aware-switch-form")
      .locator('input[type="checkbox"][name="telemetry"]');

    await expect(formControl).toHaveCount(1);
    await page.locator('link[rel="stylesheet"], style').evaluateAll((stylesheets) => {
      for (const stylesheet of stylesheets) stylesheet.remove();
    });

    const hiding = await formControl.evaluate((node) => {
      const style = getComputedStyle(node);
      const bounds = node.getBoundingClientRect();
      return {
        position: style.position,
        pointerEvents: style.pointerEvents,
        opacity: style.opacity,
        margin: style.margin,
        width: bounds.width,
        height: bounds.height,
      };
    });

    expect(hiding).toEqual({
      position: "absolute",
      pointerEvents: "none",
      opacity: "0",
      margin: "0px",
      width: 0,
      height: 0,
    });
    await expect(formControl).not.toBeVisible();
  });
});

test.describe("behaviour", () => {
  const changes = (page: Page) => page.getByTestId("changes");
  const commits = (page: Page) => page.getByTestId("switch-commits");
  const focusExits = (page: Page) => page.getByTestId("switch-focus-exits");

  test("an unresolved name is absent from the registry form participant", async ({ page }) => {
    const formControl = page.locator("#off + input + input");

    await expect(formControl).toHaveAttribute("type", "checkbox");
    await expect(formControl).not.toHaveAttribute("name");
  });

  // Tier 1, observed: daisyUI's checked rule matches the ARIA attribute the
  // primitive already sets, so a switch that is on and one that is off differ
  // in how they render while carrying the very same classes. A component that
  // had reached for a modifier class instead would pass the first half and
  // fail the second.
  test("turning a switch on styles it without a class of its own", async ({ page }) => {
    const off = switchControl(page, "Off");
    const on = switchControl(page, "On");

    await expect(off).not.toBeChecked();
    await expect(on).toBeChecked();

    expect(await on.getAttribute("class")).toBe(await off.getAttribute("class"));

    // daisyUI slides the knob across by moving the toggle's own background
    // position, and fills the track with the colour, so the two states differ
    // in what the element paints rather than in what it is called.
    const [offPaint, onPaint] = await computedStyle(page.locator("#off, #on"), "background-color");
    expect(onPaint, "turning the switch on repainted nothing").not.toBe(offPaint);
  });

  test("a switch takes keyboard focus and toggles from it", async ({ page }) => {
    const toggle = switchControl(page, "Toggle");

    await expect(toggle).not.toBeChecked();
    await expect(changes(page)).toHaveText("0");
    await expect(commits(page)).toHaveText("0");
    await expect(focusExits(page)).toHaveText("0");

    // Focus is arrived at by tabbing off the last switch that takes it, rather
    // than set on this one outright: `locator.press` would focus it first,
    // which makes any assertion about focus say nothing.
    await switchControl(page, "On").focus();
    await page.keyboard.press("Tab");
    await expect(toggle).toBeFocused();

    // ARIA says the space bar is what changes a switch, and the primitive
    // takes it at its word: enter is swallowed rather than treated as a second
    // way to toggle.
    await page.keyboard.press("Enter");
    await expect(toggle).not.toBeChecked();
    await expect(changes(page)).toHaveText("0");
    await expect(commits(page)).toHaveText("0");

    await page.keyboard.press(" ");
    await expect(toggle).toBeChecked();
    await expect(changes(page)).toHaveText("1");
    await expect(commits(page)).toHaveText("1");
    await expect(focusExits(page)).toHaveText("0");

    // The switch is controlled by the page, so a second press says the state
    // travelled out through the change callback and back in through the
    // value prop rather than the primitive keeping a copy of its own.
    await page.keyboard.press(" ");
    await expect(toggle).not.toBeChecked();
    await expect(changes(page)).toHaveText("2");
    await expect(commits(page)).toHaveText("2");
    await expect(focusExits(page)).toHaveText("0");

    await page.keyboard.press("Tab");
    await expect(focusExits(page)).toHaveText("1");
  });

  test("a disabled switch is announced disabled, takes no focus, and does not toggle", async ({
    page,
  }) => {
    const disabled = switchControl(page, "Disabled");

    // The accessibility tree as a screen reader reads it: a switch, named by
    // its label, announced disabled and off. daisyUI draws the disabled look
    // from that state rather than from a class, so this is both halves of it.
    await expect(disabled).toHaveRole("switch");
    await expect(disabled).toBeDisabled();
    await expect(disabled).not.toBeChecked();

    // Tab from the switch before it and focus lands past both disabled ones,
    // so they are passed over rather than merely refusing focus.
    await switchControl(page, "On").focus();
    await page.keyboard.press("Tab");

    await expect(disabled).not.toBeFocused();
    await expect(switchControl(page, "Disabled on")).not.toBeFocused();
    await expect(switchControl(page, "Toggle")).toBeFocused();

    await disabled.click({ force: true });
    await expect(disabled).not.toBeChecked();
  });
});

test("SwitchField composes generated parts and preserves field behaviour", async ({ page }) => {
  const fieldSwitch = switchControl(page, "Anonymous telemetry");
  const form = page.locator("#field-aware-switch-form");
  const field = fieldSwitch.locator("xpath=..");
  const formControl = form.locator('input[type="checkbox"][name="telemetry"]');
  const controlId = await fieldSwitch.getAttribute("id");
  const labelId = await fieldSwitch.getAttribute("aria-labelledby");
  const describedBy = (await fieldSwitch.getAttribute("aria-describedby"))?.split(/\s+/) ?? [];

  expect(controlId).toMatch(/^dxf-field-/);
  expect(labelId).toMatch(/^dxf-label-/);
  expect(describedBy).toHaveLength(2);
  expect(describedBy[0]).toMatch(/^dxf-description-/);
  expect(describedBy[1]).toMatch(/^dxf-error-/);

  const label = page.locator(`[id="${labelId}"]`);
  const description = page.locator(`[id="${describedBy[0]}"]`);
  const error = page.locator(`[id="${describedBy[1]}"]`);
  await expect(label).toHaveText("Anonymous telemetry");
  await expect(label).toHaveAttribute("for", controlId!);
  await expect(fieldSwitch).toHaveAccessibleName("Anonymous telemetry");
  await expect(description).toHaveText(
    "Send anonymous usage data to help improve the product.",
  );
  await expect(error).toHaveAttribute("aria-live", "polite");
  await expect(error).toContainText("Choose whether to send anonymous telemetry.");
  await expect(field).toHaveCSS("display", "grid");

  await expect(fieldSwitch).toHaveAttribute("name", "telemetry");
  await expect(fieldSwitch).not.toHaveAttribute("required");
  await expect(fieldSwitch).toHaveAttribute("aria-required", "true");
  await expect(fieldSwitch).toHaveAttribute("data-required", "true");
  await expect(fieldSwitch).toHaveAttribute("aria-invalid", "true");
  await expect(fieldSwitch).toHaveAttribute(
    "aria-describedby",
    `${describedBy[0]} ${describedBy[1]}`,
  );
  await expect(fieldSwitch).toHaveAttribute("aria-errormessage", describedBy[1]);
  await expect(fieldSwitch).toHaveClass(/\btoggle-error\b/);
  await expect(formControl).toHaveAttribute("required", "true");
  await expect(formControl).toHaveAttribute("aria-hidden", "true");
  await expect(formControl).toHaveAttribute("tabindex", "-1");
  await expect(formControl).not.toBeChecked();
  await expect(formControl).toBeAttached();
  expect(await formControl.evaluate((node) => (node as HTMLInputElement).checkValidity())).toBe(
    false,
  );
  expect(
    await form.evaluate((node) =>
      Object.fromEntries(new FormData(node as HTMLFormElement).entries()),
    ),
  ).toEqual({});

  await label.click();
  await expect(fieldSwitch).toBeChecked();
  await expect(page.getByTestId("field-aware-switch-value")).toHaveText("Current state: true");
  await expect(formControl).toBeChecked();
  expect(await formControl.evaluate((node) => (node as HTMLInputElement).checkValidity())).toBe(
    true,
  );
  expect(
    await form.evaluate((node) =>
      Object.fromEntries(new FormData(node as HTMLFormElement).entries()),
    ),
  ).toEqual({ telemetry: "on" });

  await page.locator("#focus-field-aware-switch").click();
  await expect(fieldSwitch).toBeFocused();
});

/** One of the switches the page renders, by the name it is announced under. */
function switchControl(page: Page, name: string) {
  return page.getByRole("switch", { name, exact: true });
}
