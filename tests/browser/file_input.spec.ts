import { expect, test } from "@playwright/test";

import {
  axis,
  expectAxisGrows,
  expectAxisVaries,
  expectVaries,
  openPreview,
} from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "file_input" });
});

test("renders a real file input with daisyUI's base class", async ({ page }) => {
  const input = page.locator("#default-fileinput");

  await expect(input).toHaveJSProperty("tagName", "INPUT");
  await expect(input).toHaveAttribute("type", "file");
  await expect(input).toHaveClass(/\bfile-input\b/);
});

test("defaults emit no modifier classes", async ({ page }) => {
  const classes = ((await page.locator("#default-fileinput").getAttribute("class")) ?? "")
    .split(/\s+/)
    .filter(Boolean);

  expect(classes).toEqual(["file-input"]);
});

test("every colour renders a file input differently", async ({ page }) => {
  await expectAxisVaries(page, "color", "border-top-color");
});

test("every size renders a file input at a size of its own", async ({ page }) => {
  await expectAxisGrows(page, "size", "height");
});

test("every appearance renders a file input differently", async ({ page }) => {
  const rendered = await axis(page, "appearance").evaluateAll((inputs) =>
    inputs.map((input) => {
      const style = getComputedStyle(input);
      return [style.backgroundColor, style.borderColor, style.boxShadow].join(" ");
    }),
  );

  expectVaries(rendered, "appearance");
});

test("caller classes join the component's and file configuration reaches the input", async ({
  page,
}) => {
  const input = page.locator("#configured-fileinput");
  const classes = ((await input.getAttribute("class")) ?? "").split(/\s+/);

  expect(classes).toContain("file-input");
  expect(classes).toContain("file-input-primary");
  expect(classes).toContain("file-input-lg");
  expect(classes).toContain("file-input-ghost");
  expect(classes).toContain("rounded-none");
  await expect(input).toHaveCSS("border-radius", "0px");
  await expect(input).toHaveAttribute("accept", "image/png,image/jpeg");
  await expect(input).toHaveJSProperty("multiple", true);
});

test("participates in its native form and reports the selected files", async ({ page }) => {
  const input = page.locator("#form-fileinput");

  await expect(input).toHaveAttribute("name", "attachments");
  await expect(input).toHaveAttribute("form", "fileinput-form");
  await expect(input).toHaveJSProperty("required", true);
  expect(await input.evaluate((node) => (node as HTMLInputElement).checkValidity())).toBe(false);

  await input.setInputFiles([
    { name: "first.png", mimeType: "image/png", buffer: Buffer.from("first") },
    { name: "second.jpg", mimeType: "image/jpeg", buffer: Buffer.from("second") },
  ]);

  await expect(page.locator("#selected-files")).toHaveText("first.png, second.jpg");
  expect(
    await page.locator("#fileinput-form").evaluate((node) =>
      new FormData(node as HTMLFormElement)
        .getAll("attachments")
        .map((entry) => (entry as File).name),
    ),
  ).toEqual(["first.png", "second.jpg"]);
});

test("the native disabled attribute prevents interaction", async ({ page }) => {
  const enabled = page.locator("#enabled-fileinput");
  const disabled = page.locator("#disabled-fileinput");

  await expect(disabled).toBeDisabled();
  expect(await disabled.getAttribute("class")).toBe(await enabled.getAttribute("class"));

  await disabled.focus();
  await expect(disabled).not.toBeFocused();
});
