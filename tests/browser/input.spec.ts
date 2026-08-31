import { expect, test } from "@playwright/test";

import {
  axis,
  expectAxisGrows,
  expectAxisVaries,
  expectVaries,
  openPreview,
} from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "input" });
});

test("renders a real input with daisyUI's base class", async ({ page }) => {
  const input = page.locator("#default-input");

  await expect(input).toHaveJSProperty("tagName", "INPUT");
  await expect(input).toHaveClass(/\binput\b/);
});

test("defaults emit no modifier classes", async ({ page }) => {
  const classes = ((await page.locator("#default-input").getAttribute("class")) ?? "")
    .split(/\s+/)
    .filter(Boolean);

  expect(classes).toEqual(["input"]);
});

test("every colour renders an input differently", async ({ page }) => {
  await expectAxisVaries(page, "color", "border-top-color");
});

test("every size renders an input at a size of its own", async ({ page }) => {
  await expectAxisGrows(page, "size", "height");
});

test("every appearance renders an input differently", async ({ page }) => {
  const rendered = await axis(page, "appearance").evaluateAll((inputs) =>
    inputs.map((input) => {
      const style = getComputedStyle(input);
      return [style.backgroundColor, style.borderColor, style.boxShadow].join(" ");
    }),
  );

  expectVaries(rendered, "appearance");
});

test("caller classes join the component's and native attributes override", async ({ page }) => {
  const input = page.locator("#caller-attributes");
  const classes = ((await input.getAttribute("class")) ?? "").split(/\s+/);

  expect(classes).toContain("input");
  expect(classes).toContain("input-primary");
  expect(classes).toContain("input-lg");
  expect(classes).toContain("input-ghost");
  expect(classes).toContain("rounded-none");
  await expect(input).toHaveCSS("border-radius", "0px");
  await expect(input).toHaveAttribute("type", "email");
  await expect(input).toHaveAttribute("name", "contact");
  await expect(input).toHaveAttribute("placeholder", "maintainer@example.com");
  await expect(input).toHaveJSProperty("required", true);
});

test("participates in its native form", async ({ page }) => {
  const form = page.locator("#input-form");
  const input = page.locator("#form-input");

  expect(await input.evaluate((node) => (node as HTMLInputElement).form?.id)).toBe("input-form");
  expect(await input.evaluate((node) => (node as HTMLInputElement).checkValidity())).toBe(false);

  await input.fill("dioxus-daisyui-components");
  await expect(page.locator("#form-value")).toHaveText("dioxus-daisyui-components");
  await expect(page.getByTestId("input-commits")).toHaveText("0");

  await input.blur();
  await expect(page.getByTestId("input-commits")).toHaveText("1");

  await input.fill("dioxus-field");
  await input.press("Enter");
  await expect(page.getByTestId("input-commits")).toHaveText("2");
  await expect(page.getByTestId("input-submit-commits")).toHaveText("2");

  expect(
    await form.evaluate((node) =>
      Object.fromEntries(new FormData(node as HTMLFormElement).entries()),
    ),
  ).toEqual({ component: "dioxus-field" });
});

test("changed and unchanged focus sessions commit and focus-exit exactly once", async ({ page }) => {
  const input = page.locator("#form-input");
  const commits = page.getByTestId("input-commits");
  const focusExits = page.getByTestId("input-focus-exits");

  await input.fill("changed");
  await expect(commits).toHaveText("0");
  await expect(focusExits).toHaveText("0");

  await input.blur();
  await expect(commits).toHaveText("1");
  await expect(focusExits).toHaveText("1");

  await input.focus();
  await input.blur();
  await expect(commits).toHaveText("2");
  await expect(focusExits).toHaveText("2");
});

test("the native disabled attribute is the state daisyUI styles", async ({ page }) => {
  const enabled = page.locator("#enabled-input");
  const disabled = page.locator("#disabled-input");

  await expect(disabled).toBeDisabled();
  expect(await disabled.getAttribute("class")).toBe(await enabled.getAttribute("class"));
  await expect(enabled).toHaveCSS("cursor", "text");
  await expect(disabled).toHaveCSS("cursor", "not-allowed");
  expect(await disabled.evaluate((node) => (node as HTMLInputElement).disabled)).toBe(true);

  await disabled.focus();
  await expect(disabled).not.toBeFocused();
});

test("InputField composes its label, input, and always-mounted error", async ({ page }) => {
  const input = page.locator("#field-aware-input");

  const labelId = await input.getAttribute("aria-labelledby");
  expect(labelId).toBeTruthy();
  const label = page.locator(`#${labelId}`);
  await expect(label).toHaveText("Account");
  await expect(label).toHaveAttribute("for", "field-aware-input");
  await expect(input).toHaveAttribute("aria-describedby", /^\S+$/);
  const errorId = await input.getAttribute("aria-describedby");
  const error = page.locator(`#${errorId}`);
  await expect(error).toHaveAttribute("aria-live", "polite");
  await expect(error).toBeEmpty();

  await expect(input).toHaveClass(/\binput-sm\b/);
  await expect(input).toHaveClass(/\binput-ghost\b/);
  await expect(input.locator("xpath=..")).toHaveCSS("display", "block");
  await expect(input).toHaveAttribute("name", "account");
  await expect(input).toHaveAttribute("type", "email");
  await expect(input).toHaveAttribute("placeholder", "Account name");
  await expect(input).toHaveAttribute("required", "true");
  await expect(input).toHaveAttribute("aria-invalid", "true");
  await expect(input).toHaveClass(/\binput-error\b/);

  await input.fill("registry");
  await expect(page.getByTestId("field-aware-input-value")).toHaveText(
    "Current value: registry",
  );

  await page.locator("#focus-field-aware-input").click();
  await expect(input).toBeFocused();
});
