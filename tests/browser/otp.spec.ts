import { expect, test } from "@playwright/test";

import { axis, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "otp" });
});

test("renders daisyUI's visual boxes over one native one-time-code input", async ({ page }) => {
  const otp = page.locator("#default-otp").locator("..");
  const input = otp.locator("input");

  await expect(otp).toHaveJSProperty("tagName", "LABEL");
  await expect(otp).toHaveClass(/\botp\b/);
  await expect(otp.locator(":scope > span")).toHaveCount(4);
  await expect(input).toHaveCount(1);
  await expect(input).toHaveAttribute("type", "text");
  await expect(input).toHaveAttribute("autocomplete", "one-time-code");
  await expect(input).toHaveAttribute("inputmode", "numeric");
  await expect(input).toHaveAttribute("maxlength", "4");
  await expect(input).toHaveAttribute("pattern", "[0-9]{4}");
  await expect(input).toHaveJSProperty("required", false);
});

test("defaults emit no modifier classes", async ({ page }) => {
  const classes = ((await page.locator("#default-otp").locator("..").getAttribute("class")) ?? "")
    .split(/\s+/)
    .filter(Boolean);

  expect(classes).toEqual(["otp"]);
});

test("every colour renders the boxes differently", async ({ page }) => {
  const rendered = await axis(page, "color").evaluateAll((labels) =>
    labels.map((label) => getComputedStyle(label.querySelector("span")!).borderTopColor),
  );

  expectVaries(rendered, "color");
});

test("every size renders the boxes at a size of its own", async ({ page }) => {
  const rendered = await axis(page, "size").evaluateAll((labels) =>
    labels.map((label) => parseFloat(getComputedStyle(label.querySelector("span")!).height)),
  );

  expectVaries(rendered, "size");
  expect(rendered).toEqual([...rendered].sort((a, b) => a - b));
});

test("every appearance renders the boxes differently", async ({ page }) => {
  const rendered = await axis(page, "appearance").evaluateAll((labels) =>
    labels.map((label) => getComputedStyle(label).getPropertyValue("--otp-gap")),
  );

  expectVaries(rendered, "appearance");
});

test("caller classes style the row while defining attributes stay fixed", async ({
  page,
}) => {
  const input = page.locator("#caller-attributes");
  const otp = input.locator("..");
  const classes = ((await otp.getAttribute("class")) ?? "").split(/\s+/);

  expect(classes).toContain("otp");
  expect(classes).toContain("otp-primary");
  expect(classes).toContain("otp-lg");
  expect(classes).toContain("otp-joined");
  expect(classes).toContain("rounded-none");
  await expect(otp).toHaveCSS("border-radius", "0px");
  await expect(input).toHaveAttribute("aria-label", "Eight-digit code");
  await expect(input).toHaveAttribute("inputmode", "numeric");
  await expect(otp.locator(":scope > span")).toHaveCount(8);
  await expect(input).toHaveAttribute("maxlength", "8");
  await expect(input).toHaveAttribute("pattern", "[0-9]{8}");
});

test("the shortest supported field keeps its markup and validation in step", async ({ page }) => {
  const input = page.locator("#shortest-otp");

  await expect(input.locator("..").locator(":scope > span")).toHaveCount(1);
  await expect(input).toHaveAttribute("maxlength", "1");
  await expect(input).toHaveAttribute("pattern", "[0-9]{1}");
});

test("native editing keeps Commit deduped and reports departure separately", async ({ page }) => {
  const form = page.locator("#otp-form");
  const input = page.locator("#form-otp");
  const focusExits = page.getByTestId("otp-focus-exits");

  expect(await input.evaluate((node) => (node as HTMLInputElement).checkValidity())).toBe(false);

  await input.pressSequentially("123");
  await expect(page.getByTestId("otp-commits")).toHaveText("0");
  await input.press("4");
  await expect(page.locator("#otp-value")).toHaveText("1234");
  await expect(page.getByTestId("otp-commits")).toHaveText("1");
  await expect(focusExits).toHaveText("0");
  await input.blur();
  await expect(page.getByTestId("otp-commits")).toHaveText("1");
  await expect(focusExits).toHaveText("1");

  await input.focus();
  await input.press("Backspace");
  await expect(input).toHaveValue("123");
  await input.blur();
  await expect(page.getByTestId("otp-commits")).toHaveText("2");
  await expect(focusExits).toHaveText("2");

  await input.fill("6543");
  await expect(page.getByTestId("otp-commits")).toHaveText("3");
  await input.press("ControlOrMeta+A");
  await input.press("ControlOrMeta+C");
  await input.fill("");
  await input.press("ControlOrMeta+V");
  await expect(input).toHaveValue("6543");
  await expect(page.getByTestId("otp-commits")).toHaveText("4");
  await input.blur();
  await expect(page.getByTestId("otp-commits")).toHaveText("4");
  await expect(focusExits).toHaveText("3");

  await input.focus();
  await input.blur();
  await expect(page.getByTestId("otp-commits")).toHaveText("4");
  await expect(focusExits).toHaveText("4");

  expect(await input.evaluate((node) => (node as HTMLInputElement).checkValidity())).toBe(true);
  expect(
    await form.evaluate((node) =>
      Object.fromEntries(new FormData(node as HTMLFormElement).entries()),
    ),
  ).toEqual({ code: "6543" });
});

test("the native disabled attribute prevents focus without changing the axes", async ({ page }) => {
  const enabled = page.locator("#enabled-otp");
  const disabled = page.locator("#disabled-otp");

  await expect(disabled).toBeDisabled();
  expect(await disabled.locator("..").getAttribute("class")).toBe(
    await enabled.locator("..").getAttribute("class"),
  );
  await disabled.focus();
  await expect(disabled).not.toBeFocused();
});

test("OtpField composes its parts while preserving OTP attribute routing", async ({ page }) => {
  const input = page.locator("#field-aware-otp");
  const otp = input.locator("..");
  const field = otp.locator("..");

  const labelId = await input.getAttribute("aria-labelledby");
  expect(labelId).toBeTruthy();
  const label = page.locator(`[id="${labelId}"]`);
  await expect(label).toHaveText("Verification code");
  await expect(label).toHaveAttribute("for", "field-aware-otp");

  const describedBy = (await input.getAttribute("aria-describedby"))?.split(/\s+/) ?? [];
  expect(describedBy).toHaveLength(2);
  const errorId = await input.getAttribute("aria-errormessage");
  expect(errorId).toBeTruthy();
  expect(describedBy).toContain(errorId);
  const descriptionId = describedBy.find((id) => id !== errorId);
  expect(descriptionId).toBeTruthy();
  const description = page.locator(`[id="${descriptionId}"]`);
  const error = page.locator(`[id="${errorId}"]`);
  await expect(description).toHaveText("Use the code from your authenticator app.");
  await expect(error).toHaveAttribute("aria-live", "polite");
  await expect(error).toContainText("Enter the six-digit verification code.");

  await expect(otp).toHaveClass(/\botp-sm\b/);
  await expect(otp).toHaveClass(/\botp-joined\b/);
  await expect(otp).toHaveClass(/\brounded-none\b/);
  await expect(otp.locator(":scope > span")).toHaveCount(6);
  await expect(field).toHaveCSS("display", "block");
  await expect(description).toHaveCSS("white-space", "nowrap");
  await expect(error).not.toHaveClass(/\btext-error\b/);
  const [fieldColor, errorColor] = await field.evaluate((fieldElement) => {
    const errorElement = fieldElement.querySelector('[aria-live="polite"]');
    if (!(errorElement instanceof HTMLElement)) {
      throw new Error("OtpField has no error region");
    }
    return [getComputedStyle(fieldElement).color, getComputedStyle(errorElement).color];
  });
  expect(errorColor).toBe(fieldColor);

  await expect(input).toHaveAttribute("name", "verification_code");
  await expect(input).toHaveAttribute("required", "true");
  await expect(input).toHaveAttribute("maxlength", "6");
  await expect(input).toHaveAttribute("pattern", "[0-9]{6}");
  await expect(input).toHaveAttribute("aria-label", "Authenticator verification code");
  await expect(input).toHaveAttribute("data-otp-field", "forwarded");
  await expect(input).toHaveAttribute("aria-invalid", "true");
  await expect(input).toHaveAttribute("aria-describedby", describedBy.join(" "));
  await expect(input).toHaveAttribute("aria-errormessage", errorId!);
  await expect(input).not.toHaveClass(/\botp-error\b/);
  await expect(otp).toHaveClass(/\botp-error\b/);

  await input.fill("123456");
  await expect(page.getByTestId("field-aware-otp-value")).toHaveText("Current value: 123456");

  await page.locator("#focus-field-aware-otp").click();
  await expect(input).toBeFocused();
});
