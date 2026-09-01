import { expect, test } from "@playwright/test";

import {
  axis,
  expectAxisGrows,
  expectAxisVaries,
  expectVaries,
  openPreview,
} from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "native_select" });
});

test("renders a real select with daisyUI's base class", async ({ page }) => {
  const select = page.locator("#default-native-select");

  await expect(select).toHaveJSProperty("tagName", "SELECT");
  await expect(select).toHaveClass(/\bselect\b/);
});

test("defaults emit no modifier classes", async ({ page }) => {
  const classes = ((await page.locator("#default-native-select").getAttribute("class")) ?? "")
    .split(/\s+/)
    .filter(Boolean);

  expect(classes).toEqual(["select"]);
});

test("daisyUI draws the caret on the native element", async ({ page }) => {
  const select = page.locator("#default-native-select");

  const [backgroundImage, paddingStart, paddingEnd] = await select.evaluate((node) => {
    const style = getComputedStyle(node);
    return [style.backgroundImage, style.paddingInlineStart, style.paddingInlineEnd];
  });
  expect(backgroundImage).not.toBe("none");
  expect(parseFloat(paddingEnd)).toBeGreaterThan(parseFloat(paddingStart));
});

test("every colour renders a select differently", async ({ page }) => {
  await expectAxisVaries(page, "color", "border-top-color");
});

test("every size renders a select at a size of its own", async ({ page }) => {
  await expectAxisGrows(page, "size", "height");
});

test("every appearance renders a select differently", async ({ page }) => {
  const rendered = await axis(page, "appearance").evaluateAll((selects) =>
    selects.map((select) => {
      const style = getComputedStyle(select);
      return [style.backgroundColor, style.borderColor, style.boxShadow].join(" ");
    }),
  );

  expectVaries(rendered, "appearance");
});

test("caller classes join the component's and native attributes override", async ({ page }) => {
  const select = page.locator("#caller-attributes");
  const classes = ((await select.getAttribute("class")) ?? "").split(/\s+/);

  expect(classes).toContain("select");
  expect(classes).toContain("select-primary");
  expect(classes).toContain("select-lg");
  expect(classes).toContain("select-ghost");
  expect(classes).toContain("w-64");
  await expect(select).toHaveAttribute("name", "flavor");
  await expect(select).toHaveJSProperty("required", true);
});

test("a preselected default renders as the browser's selection on mount", async ({ page }) => {
  // Guards the creation-ordering half of the dual value/selected strategy: the
  // select's value property only takes if its options exist when it is set.
  await expect(page.locator("#controlled-native-select")).toHaveValue("1");
  await expect(page.locator("#placeholder-chosen")).toHaveValue("1");
});

test("a controlled select follows external writes after a user pick", async ({ page }) => {
  const select = page.locator("#controlled-native-select");

  await select.selectOption("0");
  await expect(select).toHaveValue("0");
  await expect(page.getByTestId("controlled-native-select-value")).toHaveText('Some("orange")');
  await expect(page.getByTestId("native-select-changes")).toHaveText("1");
  await expect(page.getByTestId("native-select-commits")).toHaveText("1");

  // Guards the dirty-value-flag half: after a user pick, only the value
  // property write can still move the browser's selection.
  await page.locator("#select-cherry-externally").click();
  await expect(select).toHaveValue("2");
  await expect(page.getByTestId("controlled-native-select-value")).toHaveText('Some("cherry")');
});

test("the placeholder is a disabled first option selected while the value is None", async ({ page }) => {
  const empty = page.locator("#placeholder-empty");
  const placeholder = empty.locator("option").first();

  await expect(empty).toHaveValue("");
  await expect(placeholder).toHaveText("Pick a flavor");
  await expect(placeholder).toHaveJSProperty("disabled", true);
  expect(await empty.evaluate((node) => (node as HTMLSelectElement).selectedIndex)).toBe(0);

  const chosen = page.locator("#placeholder-chosen");
  await expect(chosen.locator("option").first()).toHaveJSProperty("disabled", true);
  await expect(chosen.locator("option").first()).toHaveText("Pick a flavor");
});

test("participates in its native form", async ({ page }) => {
  const form = page.locator("#native-select-form");
  const select = page.locator("#form-native-select");

  expect(await select.evaluate((node) => (node as HTMLSelectElement).form?.id)).toBe(
    "native-select-form",
  );
  expect(await select.evaluate((node) => (node as HTMLSelectElement).checkValidity())).toBe(false);

  await select.selectOption("pending");
  await expect(page.locator("#native-select-form-value")).toHaveText("Some(Pending)");
  await expect(page.getByTestId("form-native-select-commits")).toHaveText("1");
  expect(await select.evaluate((node) => (node as HTMLSelectElement).checkValidity())).toBe(true);

  await form.evaluate((node) => (node as HTMLFormElement).requestSubmit());
  await expect(page.getByTestId("form-native-select-submit-commits")).toHaveText("1");

  expect(
    await form.evaluate((node) =>
      Object.fromEntries(new FormData(node as HTMLFormElement).entries()),
    ),
  ).toEqual({ status: "pending", flavor: "1" });
});

test("changed and unchanged focus sessions commit and focus-exit exactly once", async ({ page }) => {
  const select = page.locator("#form-native-select");
  const commits = page.getByTestId("form-native-select-commits");
  const focusExits = page.getByTestId("form-native-select-focus-exits");

  await select.focus();
  await select.selectOption("active");
  await expect(commits).toHaveText("1");
  await expect(focusExits).toHaveText("0");

  await select.blur();
  await expect(commits).toHaveText("1");
  await expect(focusExits).toHaveText("1");

  await select.focus();
  await select.blur();
  await expect(commits).toHaveText("1");
  await expect(focusExits).toHaveText("2");
});

test("the native disabled attribute is the state daisyUI styles", async ({ page }) => {
  const enabled = page.locator("#enabled-native-select");
  const disabled = page.locator("#disabled-native-select");

  await expect(disabled).toBeDisabled();
  expect(await disabled.getAttribute("class")).toBe(await enabled.getAttribute("class"));
  expect(await disabled.evaluate((node) => (node as HTMLSelectElement).disabled)).toBe(true);

  await disabled.focus();
  await expect(disabled).not.toBeFocused();
});

test("NativeSelectField composes its label, select, and always-mounted error", async ({ page }) => {
  const select = page.locator("#field-aware-native-select");

  const labelId = await select.getAttribute("aria-labelledby");
  expect(labelId).toBeTruthy();
  const label = page.locator(`#${labelId}`);
  await expect(label).toHaveText("Flavor");
  await expect(label).toHaveAttribute("for", "field-aware-native-select");
  await expect(select).toHaveAttribute("aria-describedby", /^\S+$/);
  const errorId = await select.getAttribute("aria-describedby");
  const error = page.locator(`#${errorId}`);
  await expect(error).toHaveAttribute("aria-live", "polite");
  await expect(error).toBeEmpty();

  await expect(select).toHaveClass(/\bselect-sm\b/);
  await expect(select).toHaveAttribute("name", "flavor");
  await expect(select).toHaveAttribute("required", "true");
  await expect(select).toHaveAttribute("aria-invalid", "true");
  await expect(select).toHaveClass(/\bselect-error\b/);

  await select.selectOption("1");
  await expect(page.getByTestId("field-aware-native-select-value")).toHaveText(
    "Current value: Some(Lemon)",
  );

  await page.locator("#focus-field-aware-native-select").click();
  await expect(select).toBeFocused();
});
