import { expect, type Page, test } from "@playwright/test";

import { computedStyle, expectAxisGrows, expectAxisVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "checkbox" });
});

test.describe("styling", () => {
  // The colour row is rendered checked, which is the state daisyUI expresses
  // colour in: `--input-color` is what a colour class sets, and the checked
  // rule is what fills the box with it.
  test("every colour applies a colour of its own", async ({ page }) => {
    await expectAxisVaries(page, "color", "background-color");
  });

  test("every size applies a size of its own", async ({ page }) => {
    await expectAxisGrows(page, "size", "height");
  });

  test("a caller's classes join the component's own", async ({ page }) => {
    const checkbox = page.locator("#caller-attributes");
    const classes = ((await checkbox.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, and then the caller's, on one element.
    expect(classes).toContain("checkbox");
    expect(classes).toContain("checkbox-primary");
    expect(classes).toContain("rounded-none");

    // And the caller's applies, rather than merely surviving the merge, over
    // a corner radius daisyUI's own class sets, which the caller wins on
    // cascade layers rather than on specificity (ADR-0004).
    await expect(checkbox).toHaveCSS("border-radius", "0px");
  });

  // This is the whole of tier 1, observed: daisyUI's checked rule matches the
  // ARIA attribute the primitive already sets, so the checked and the
  // unchecked checkbox differ in how they render while carrying the very same
  // classes. A component that had reached for a modifier class instead would
  // pass the first half and fail the second.
  test("checking styles the checkbox without a class of its own", async ({ page }) => {
    const unchecked = checkbox(page, "Unchecked");
    const checked = checkbox(page, "Checked");

    await expect(unchecked).not.toBeChecked();
    await expect(checked).toBeChecked();

    expect(await checked.getAttribute("class")).toBe(await unchecked.getAttribute("class"));

    // The mark, which daisyUI draws in a pseudo-element of the checkbox itself
    // and reveals by raising its opacity. It is read rather than the
    // background because an uncoloured checkbox fills with nothing when
    // checked; the colour axis covers the fill, and this covers the tick.
    const [mark, checkedMark] = (
      await computedStyle(page.locator("#unchecked, #checked"), "opacity", "::before")
    ).map(parseFloat);

    expect(checkedMark, "checking did not bring out the mark").toBeGreaterThan(mark);
  });
});

test.describe("behaviour", () => {
  const changes = (page: Page) => page.getByTestId("changes");
  const commits = (page: Page) => page.getByTestId("checkbox-commits");
  const focusExits = (page: Page) => page.getByTestId("checkbox-focus-exits");

  test("a checkbox takes keyboard focus and toggles from it", async ({ page }) => {
    const toggle = checkbox(page, "Toggle");

    await expect(toggle).not.toBeChecked();
    await expect(changes(page)).toHaveText("0");
    await expect(commits(page)).toHaveText("0");
    await expect(focusExits(page)).toHaveText("0");

    // Focus is arrived at by tabbing off the last checkbox that takes it,
    // rather than set on this one outright: `locator.press` would focus it
    // first, which makes any assertion about focus say nothing.
    await checkbox(page, "Indeterminate").focus();
    await page.keyboard.press("Tab");
    await expect(toggle).toBeFocused();

    // ARIA says the space bar is what changes a checkbox, and the primitive
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

    // The checkbox is controlled by the page, so a second press says the state
    // travelled out through the change callback and back in through the value
    // prop rather than the primitive keeping a copy of its own.
    await page.keyboard.press(" ");
    await expect(toggle).not.toBeChecked();
    await expect(changes(page)).toHaveText("2");
    await expect(commits(page)).toHaveText("2");
    await expect(focusExits(page)).toHaveText("0");

    await page.keyboard.press("Tab");
    await expect(focusExits(page)).toHaveText("1");
  });

  test("a disabled checkbox is announced disabled, takes no focus, and does not toggle", async ({
    page,
  }) => {
    const disabled = checkbox(page, "Disabled");

    // The accessibility tree as a screen reader reads it: a checkbox, named by
    // its label, announced disabled and unchecked. daisyUI draws the disabled
    // look from that state rather than from a class, so this is both halves of
    // it.
    await expect(disabled).toHaveRole("checkbox");
    await expect(disabled).toBeDisabled();
    await expect(disabled).not.toBeChecked();

    // Tab from the checkbox before it and focus lands past both disabled ones,
    // so they are passed over rather than merely refusing focus.
    await checkbox(page, "Indeterminate").focus();
    await page.keyboard.press("Tab");

    await expect(disabled).not.toBeFocused();
    await expect(checkbox(page, "Disabled checked")).not.toBeFocused();
    await expect(checkbox(page, "Toggle")).toBeFocused();

    await disabled.click({ force: true });
    await expect(disabled).not.toBeChecked();
    await expect(page.getByTestId("disabled-bool-value")).toHaveText("false");
  });

  test("an unnamed checkbox omits name from its form participant", async ({ page }) => {
    const formControl = page.locator('#unchecked + input[type="checkbox"]');

    await expect(formControl).toBeAttached();
    await expect(formControl).not.toHaveAttribute("name");
  });

  test("indeterminate state reaches ARIA and the native form participant", async ({ page }) => {
    const control = checkbox(page, "Indeterminate");
    const formControl = page.locator('#indeterminate + input[type="checkbox"]');

    await expect(control).toHaveAttribute("aria-checked", "mixed");
    await expect
      .poll(() =>
        formControl.evaluate((node) => {
          const input = node as HTMLInputElement;
          return { checked: input.checked, indeterminate: input.indeterminate };
        }),
      )
      .toEqual({ checked: true, indeterminate: true });
  });
});

test("CheckboxField composes generated parts and preserves field behaviour", async ({ page }) => {
  const fieldCheckbox = checkbox(page, "Accept the terms");
  const form = page.locator("#field-aware-checkbox-form");
  const field = fieldCheckbox.locator("xpath=..");
  const formControl = form.locator('input[type="checkbox"][name="terms"]');
  const controlId = await fieldCheckbox.getAttribute("id");
  const labelId = await fieldCheckbox.getAttribute("aria-labelledby");
  const errorId = await fieldCheckbox.getAttribute("aria-describedby");

  expect(controlId).toMatch(/^dxf-field-/);
  expect(labelId).toMatch(/^dxf-label-/);
  expect(errorId).toMatch(/^dxf-error-/);

  const label = page.locator(`[id="${labelId}"]`);
  const error = page.locator(`[id="${errorId}"]`);
  await expect(label).toHaveText("Accept the terms");
  await expect(label).toHaveAttribute("for", controlId!);
  await expect(fieldCheckbox).toHaveAccessibleName("Accept the terms");
  await expect(error).toHaveAttribute("aria-live", "polite");
  await expect(error).toBeEmpty();
  await expect(field).toHaveCSS("display", "grid");
  await expect(page.getByTestId("field-aware-checkbox-value")).toHaveText(
    "Current state: Unchecked",
  );
  await expect(page.getByTestId("field-aware-checkbox-origin")).toHaveText("Last origin: none");
  await expect(page.getByTestId("field-aware-checkbox-commits")).toHaveText("Commits: 0");
  await expect(page.getByTestId("field-aware-checkbox-focus-exits")).toHaveText("Focus exits: 0");

  await expect(fieldCheckbox).toHaveAttribute("name", "terms");
  await expect(fieldCheckbox).not.toHaveAttribute("required");
  await expect(fieldCheckbox).toHaveAttribute("aria-required", "true");
  await expect(fieldCheckbox).toHaveAttribute("data-required", "true");
  await expect(fieldCheckbox).toHaveAttribute("aria-invalid", "true");
  await expect(fieldCheckbox).toHaveAttribute("aria-errormessage", errorId!);
  await expect(fieldCheckbox).toHaveClass(/\bcheckbox-error\b/);
  await expect(formControl).toHaveAttribute("required", "true");
  await expect(formControl).toHaveAttribute("aria-hidden", "true");
  await expect(formControl).toHaveAttribute("tabindex", "-1");
  await expect(formControl).toBeAttached();
  expect(
    await form.evaluate((node) =>
      Object.fromEntries(new FormData(node as HTMLFormElement).entries()),
    ),
  ).toEqual({});

  await label.click();
  await expect(fieldCheckbox).toBeChecked();
  await expect(page.getByTestId("field-aware-checkbox-value")).toHaveText(
    "Current state: Checked",
  );
  await expect(page.getByTestId("field-aware-checkbox-origin")).toHaveText("Last origin: User");
  await expect(page.getByTestId("field-aware-checkbox-commits")).toHaveText("Commits: 1");
  await expect(page.getByTestId("field-aware-checkbox-focus-exits")).toHaveText("Focus exits: 0");
  await expect(formControl).toBeChecked();
  expect(
    await form.evaluate((node) =>
      Object.fromEntries(new FormData(node as HTMLFormElement).entries()),
    ),
  ).toEqual({ terms: "on" });

  await page.locator("#focus-field-aware-checkbox").click();
  await expect(fieldCheckbox).not.toBeChecked();
  await expect(fieldCheckbox).toBeFocused();
  await expect(page.getByTestId("field-aware-checkbox-value")).toHaveText(
    "Current state: Unchecked",
  );
  await expect(page.getByTestId("field-aware-checkbox-origin")).toHaveText(
    "Last origin: Programmatic",
  );
  await expect(page.getByTestId("field-aware-checkbox-commits")).toHaveText("Commits: 1");
  await expect(page.getByTestId("field-aware-checkbox-focus-exits")).toHaveText("Focus exits: 1");
});

/** One of the checkboxes the page renders, by the name it is announced under. */
function checkbox(page: Page, name: string) {
  return page.getByRole("checkbox", { name, exact: true });
}
