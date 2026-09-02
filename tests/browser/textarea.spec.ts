import { expect, test } from "@playwright/test";

import {
  axis,
  expectAxisGrows,
  expectAxisVaries,
  expectVaries,
  openPreview,
} from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "textarea" });
});

test("renders a real textarea with daisyUI's base class", async ({ page }) => {
  const textarea = page.locator("#default-textarea");

  await expect(textarea).toHaveJSProperty("tagName", "TEXTAREA");
  await expect(textarea).toHaveClass(/\btextarea\b/);
});

test("defaults emit no modifier classes", async ({ page }) => {
  const classes = ((await page.locator("#default-textarea").getAttribute("class")) ?? "")
    .split(/\s+/)
    .filter(Boolean);

  expect(classes).toEqual(["textarea"]);
});

test("every colour renders a textarea differently", async ({ page }) => {
  await expectAxisVaries(page, "color", "border-top-color");
});

test("every size renders a textarea at a size of its own", async ({ page }) => {
  await expectAxisGrows(page, "size", "font-size");
});

test("every appearance renders a textarea differently", async ({ page }) => {
  const rendered = await axis(page, "appearance").evaluateAll((textareas) =>
    textareas.map((textarea) => {
      const style = getComputedStyle(textarea);
      return [style.backgroundColor, style.borderColor, style.boxShadow].join(" ");
    }),
  );

  expectVaries(rendered, "appearance");
});

test("caller classes join the component's and native attributes override", async ({ page }) => {
  const textarea = page.locator("#caller-attributes");
  const classes = ((await textarea.getAttribute("class")) ?? "").split(/\s+/);

  expect(classes).toContain("textarea");
  expect(classes).toContain("textarea-primary");
  expect(classes).toContain("textarea-lg");
  expect(classes).toContain("textarea-ghost");
  expect(classes).toContain("resize-none");
  await expect(textarea).toHaveCSS("resize", "none");
  await expect(textarea).toHaveAttribute("name", "notes");
  await expect(textarea).toHaveAttribute("placeholder", "Implementation notes");
  await expect(textarea).toHaveAttribute("rows", "6");
  await expect(textarea).toHaveJSProperty("required", true);
});

test("participates in its native form", async ({ page }) => {
  const form = page.locator("#textarea-form");
  const textarea = page.locator("#form-textarea");

  expect(await textarea.evaluate((node) => (node as HTMLTextAreaElement).form?.id)).toBe(
    "textarea-form",
  );
  expect(await textarea.evaluate((node) => (node as HTMLTextAreaElement).checkValidity())).toBe(
    false,
  );

  await textarea.fill("first line");
  await textarea.press("Enter");
  await textarea.pressSequentially("second line");
  await expect(page.locator("#form-value")).toHaveText("first line\nsecond line");
  await expect(page.getByTestId("textarea-commits")).toHaveText("0");
  await expect(page.getByTestId("textarea-focus-exits")).toHaveText("0");

  await textarea.blur();
  await expect(page.getByTestId("textarea-commits")).toHaveText("1");
  await expect(page.getByTestId("textarea-focus-exits")).toHaveText("1");

  // Commit and Focus Exit are independent (ADR-0028): leaving an unchanged
  // control reports that focus left, and must not Commit a value no
  // interaction produced.
  await textarea.focus();
  await textarea.blur();
  await expect(page.getByTestId("textarea-focus-exits")).toHaveText("2");
  await expect(page.getByTestId("textarea-commits")).toHaveText("1");

  expect(
    await form.evaluate((node) =>
      Object.fromEntries(new FormData(node as HTMLFormElement).entries()),
    ),
  ).toEqual({ notes: "first line\nsecond line" });
});

test("the native disabled attribute is the state daisyUI styles", async ({ page }) => {
  const enabled = page.locator("#enabled-textarea");
  const disabled = page.locator("#disabled-textarea");

  await expect(disabled).toBeDisabled();
  expect(await disabled.getAttribute("class")).toBe(await enabled.getAttribute("class"));
  expect(await enabled.evaluate((node) => getComputedStyle(node).cursor)).not.toBe("not-allowed");
  await expect(disabled).toHaveCSS("cursor", "not-allowed");
  expect(await disabled.evaluate((node) => (node as HTMLTextAreaElement).disabled)).toBe(true);

  await disabled.focus();
  await expect(disabled).not.toBeFocused();
});

test("TextareaField composes its parts and forwards textarea attributes", async ({
  page,
}) => {
  const textarea = page.locator("#field-aware-textarea");

  const labelId = await textarea.getAttribute("aria-labelledby");
  expect(labelId).toBeTruthy();
  const label = page.locator(`#${labelId}`);
  await expect(label).toHaveText("Implementation notes");
  await expect(label).toHaveAttribute("for", "field-aware-textarea");
  const describedBy = (await textarea.getAttribute("aria-describedby"))?.split(/\s+/) ?? [];
  expect(describedBy).toHaveLength(2);
  const description = page.locator(`#${describedBy[0]}`);
  await expect(description).toHaveText(
    "Include constraints and important tradeoffs.",
  );
  const error = page.locator(`#${describedBy[1]}`);
  await expect(error).toHaveAttribute("aria-live", "polite");
  await expect(error).toContainText("Add implementation notes.");

  await expect(textarea).toHaveClass(/\btextarea-sm\b/);
  await expect(textarea).toHaveClass(/\btextarea-ghost\b/);
  await expect(description).toHaveCSS("white-space", "nowrap");
  await expect(error).not.toHaveClass(/\btext-error\b/);
  const [fieldColor, errorColor] = await textarea.locator("xpath=..").evaluate((field) => {
    const error = field.querySelector('[aria-live="polite"]');
    if (!(error instanceof HTMLElement)) {
      throw new Error("TextareaField has no error region");
    }
    return [getComputedStyle(field).color, getComputedStyle(error).color];
  });
  expect(errorColor).toBe(fieldColor);

  await expect(textarea).toHaveAttribute("name", "notes");
  await expect(textarea).toHaveAttribute("placeholder", "Describe the implementation");
  await expect(textarea).toHaveAttribute("rows", "5");
  await expect(textarea).toHaveAttribute("required", "true");
  await expect(textarea).toHaveAttribute("aria-invalid", "true");
  await expect(textarea).toHaveAttribute(
    "aria-describedby",
    /^\S+ \S+$/,
  );
  await expect(textarea).toHaveAttribute("aria-errormessage", /^\S+$/);
  await expect(textarea).toHaveClass(/\btextarea-error\b/);

  await textarea.fill("Use the native change event.");
  await expect(page.getByTestId("field-aware-textarea-value")).toHaveText(
    "Current value: Use the native change event.",
  );

  await page.locator("#focus-field-aware-textarea").click();
  await expect(textarea).toBeFocused();
});
