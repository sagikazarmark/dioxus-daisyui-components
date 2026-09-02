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

test("a changed session commits once; an unchanged one focus-exits without committing", async ({
  page,
}) => {
  const input = page.locator("#form-input");
  const commits = page.getByTestId("input-commits");
  const focusExits = page.getByTestId("input-focus-exits");

  await input.fill("changed");
  await expect(commits).toHaveText("0");
  await expect(focusExits).toHaveText("0");

  await input.blur();
  await expect(commits).toHaveText("1");
  await expect(focusExits).toHaveText("1");

  // Commit and Focus Exit are independent (ADR-0028): leaving an unchanged
  // control reports that focus left, and must not Commit a value no
  // interaction produced.
  await input.focus();
  await input.blur();
  await expect(focusExits).toHaveText("2");
  await expect(commits).toHaveText("1");
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

test("adornments render inside the span.input wrapper with the input as a direct child", async ({ page }) => {
  const input = page.locator("#suffix-input");
  const wrapper = input.locator("xpath=..");

  await expect(wrapper).toHaveJSProperty("tagName", "SPAN");
  await expect(wrapper).toHaveClass(/\binput\b/);
  await expect(wrapper).toContainText("EUR");

  const inputClasses = ((await input.getAttribute("class")) ?? "").split(/\s+/);
  expect(inputClasses).not.toContain("input");
  expect(inputClasses).toContain("tabular-nums");

  const bothWrapper = page.locator("#both-input").locator("xpath=..");
  await expect(bothWrapper.locator("> span").first()).toHaveText("$");
  await expect(bothWrapper.locator("> span").last()).toHaveText(".00");
});

test("every colour renders an adorned wrapper differently", async ({ page }) => {
  await expectAxisVaries(page, "adorned-color", "border-top-color");
});

test("every size renders an adorned wrapper at a size of its own", async ({ page }) => {
  await expectAxisGrows(page, "adorned-size", "height");
});

test("focus lands on the wrapper's focus-within styling, not the inner input", async ({ page }) => {
  const input = page.locator("#suffix-input");
  const wrapper = input.locator("xpath=..");

  const blurred = await wrapper.evaluate((node) => getComputedStyle(node).outlineWidth);
  await input.focus();
  await expect(input).toBeFocused();
  const focused = await wrapper.evaluate((node) => getComputedStyle(node).outlineWidth);
  expect(focused).not.toBe(blurred);
  await expect(input).toHaveCSS("outline-style", "none");
});

test("the disabled adorned wrapper paints through the nested-input rule", async ({ page }) => {
  const input = page.locator("#disabled-adorned-input");
  const wrapper = input.locator("xpath=..");

  await expect(input).toBeDisabled();
  await expect(wrapper).toHaveCSS("cursor", "not-allowed");
});

test("clicking the adornment or the wrapper padding does not exit the focus session", async ({ page }) => {
  const input = page.locator("#amount-input");
  const wrapper = input.locator("xpath=..");
  const focusExits = page.getByTestId("amount-focus-exits");

  await input.click();
  await input.pressSequentially("12");
  await expect(focusExits).toHaveText("0");

  await wrapper.locator("> span").last().click();
  await expect(input).toBeFocused();
  await expect(focusExits).toHaveText("0");
  expect(await input.evaluate((node) => (node as HTMLInputElement).selectionStart)).toBe(2);

  await wrapper.click({ position: { x: 4, y: 4 } });
  await expect(input).toBeFocused();
  await expect(focusExits).toHaveText("0");
  expect(await input.evaluate((node) => (node as HTMLInputElement).selectionStart)).toBe(2);

  await input.blur();
  await expect(focusExits).toHaveText("1");
  await expect(page.getByTestId("amount-commits")).toHaveText("1");
});

test("wrapper attributes reach the wrapper's own box", async ({ page }) => {
  const wrapper = page.locator("#amount-input").locator("xpath=..");

  await expect(wrapper).toHaveClass(/\bw-full\b/);
  const [wrapperWidth, parentWidth] = await wrapper.evaluate((node) => {
    const parent = node.parentElement as HTMLElement;
    const style = getComputedStyle(parent);
    return [
      node.getBoundingClientRect().width,
      parent.clientWidth - parseFloat(style.paddingLeft) - parseFloat(style.paddingRight),
    ];
  });
  expect(wrapperWidth).toBeCloseTo(parentWidth, 0);
});

test("a conditional adornment keeps a hidden slot and never remounts the input", async ({ page }) => {
  const input = page.locator("#slot-input");
  const wrapper = input.locator("xpath=..");
  const slot = wrapper.locator("> span").last();

  await expect(slot).toBeHidden();
  await input.evaluate((node) => {
    (node as HTMLElement).dataset.mounted = "kept";
  });

  await page.locator("#toggle-slot").click();
  await expect(slot).toBeVisible();
  await expect(slot).toContainText("verified");
  await expect(input).toHaveAttribute("data-mounted", "kept");
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
