import { expect, type Locator, test } from "@playwright/test";

import { expectAxisVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "field" });
});

test("skins every headless part and every appearance value", async ({ page }) => {
  const field = page.locator("#profile-name-field");
  const label = page.locator("#profile-name-label");
  const description = field.getByText("This is shown on your public profile.", { exact: true });
  const error = field.locator('[aria-live="polite"]');

  await expect(field).toHaveJSProperty("tagName", "DIV");
  await expect(label).toHaveJSProperty("tagName", "LABEL");
  await expect(description).toHaveJSProperty("tagName", "DIV");
  await expect(error).toHaveJSProperty("tagName", "DIV");
  expect(await classes(field)).toEqual(["gap-2", "grid", "min-w-0", "max-w-sm"].sort());
  expect(await classes(label)).toEqual(["label"]);
  expect(await classes(description)).toEqual(["label", "min-w-0", "whitespace-normal"].sort());
  expect(await classes(error)).toEqual(["text-error"]);

  await expect(field).toHaveCSS("display", "grid");
  await expect(label).toHaveCSS("display", "flex");
  await expect(description).toHaveCSS("display", "flex");
  await expectAxisVaries(page, "field-appearance", "display");
  await expectAxisVaries(page, "description-appearance", "white-space");
  await expectAxisVaries(page, "error-appearance", "color");
});

test("preserves caller classes and attributes on every Compound part", async ({ page }) => {
  const field = page.locator("#caller-field");
  const label = page.locator("#caller-label");
  const description = page.locator("#caller-description");
  const error = page.locator("#caller-error");

  expect(await classes(field)).toEqual(
    ["grid", "gap-2", "min-w-0", "max-w-sm", "border", "border-error", "p-3"].sort(),
  );
  expect(await classes(label)).toEqual(["label", "font-bold", "uppercase"].sort());
  expect(await classes(description)).toEqual(
    ["label", "min-w-full", "whitespace-pre-wrap", "italic"].sort(),
  );
  expect(await classes(error)).toEqual(["text-error", "underline"].sort());
  await expect(field).toHaveAttribute("title", "Caller field");
  await expect(label).toHaveAttribute("title", "Caller label");
  await expect(description).toHaveAttribute("title", "Caller description");
  await expect(error).toHaveAttribute("title", "Caller error");

  await expect(label).toHaveCSS("font-weight", "700");
  await expect(description).toHaveCSS("font-style", "italic");
  await expect(description).toHaveCSS("white-space", "pre-wrap");
  await expect(error).toHaveCSS("text-decoration-line", "underline");
});

test("wires label, description, invalidity, and errors from Field metadata", async ({ page }) => {
  const input = page.locator("#profile-name");
  const label = page.locator("#profile-name-label");
  const field = page.locator("#profile-name-field");
  const description = field.getByText("This is shown on your public profile.", { exact: true });
  const error = field.locator('[aria-live="polite"]');
  const descriptionId = await description.getAttribute("id");
  const errorId = await error.getAttribute("id");

  expect(descriptionId).toBeTruthy();
  expect(errorId).toBeTruthy();
  expect(descriptionId).not.toBe(errorId);

  await expect(label).toHaveAttribute("for", "profile-name");
  await expect(input).toHaveAccessibleName("Profile name");
  await expect(input).toHaveAttribute("aria-labelledby", "profile-name-label");
  await expect(input).toHaveAttribute("aria-describedby", `${descriptionId} ${errorId}`);
  await expect(input).toHaveAttribute("aria-errormessage", errorId!);
  await expect(input).toHaveAttribute("aria-invalid", "true");
  await expect(input).toHaveClass(/\binput-error\b/);
  await expect(description).toHaveAttribute("data-invalid", "true");
  await expect(error).toHaveAttribute("data-invalid", "true");
  await expect(error).toHaveAttribute("aria-live", "polite");
  await expect(error).toHaveText("Choose a profile name.");
  await expect(error.locator(":scope > *")).toHaveCount(1);

  await page.locator("#toggle-profile-name-validity").click();

  await expect(input).toHaveAttribute("aria-invalid", "false");
  await expect(input).not.toHaveAttribute("aria-errormessage", /.+/);
  await expect(input).toHaveAttribute("aria-describedby", descriptionId!);
  await expect(input).not.toHaveClass(/\binput-error\b/);
  await expect(error).toBeAttached();
  await expect(error).toBeEmpty();
  await expect(error.locator(":scope > *")).toHaveCount(0);

  await page.locator("#toggle-profile-name-validity").click();

  await expect(input).toHaveAttribute("aria-invalid", "true");
  await expect(input).toHaveAttribute("aria-errormessage", errorId!);
  await expect(input).toHaveAttribute("aria-describedby", `${descriptionId} ${errorId}`);
  await expect(error).toHaveText("Choose a profile name.");
  await expect(error.locator(":scope > *")).toHaveCount(1);
});

test("composes the same metadata wiring around a Checkbox", async ({ page }) => {
  const field = page.locator("#product-updates-field");
  const checkbox = page.getByRole("checkbox", { name: "Product updates", exact: true });
  const label = page.locator("#product-updates-label");
  const description = field.getByText("Receive a short email when a release ships.", {
    exact: true,
  });
  const error = field.locator('[aria-live="polite"]');
  const descriptionId = await description.getAttribute("id");
  const errorId = await error.getAttribute("id");

  expect(descriptionId).toBeTruthy();
  expect(errorId).toBeTruthy();
  expect(descriptionId).not.toBe(errorId);

  await expect(checkbox).toHaveAttribute("id", "product-updates");
  await expect(label).toHaveAttribute("for", "product-updates");
  await expect(checkbox).toHaveAttribute("aria-labelledby", "product-updates-label");
  await expect(checkbox).toHaveAttribute("aria-describedby", `${descriptionId} ${errorId}`);
  await expect(checkbox).toHaveAttribute("aria-errormessage", errorId!);
  await expect(checkbox).toHaveAttribute("aria-invalid", "true");
  await expect(checkbox).toHaveClass(/\bcheckbox-error\b/);
  await expect(checkbox).not.toBeChecked();

  await label.click();

  await expect(checkbox).toBeChecked();
  await expect(page.getByTestId("product-updates-value")).toHaveText("Current state: Checked");
});

for (const viewport of [
  { name: "desktop", width: 1100, height: 800 },
  { name: "mobile", width: 390, height: 844 },
]) {
  test(`wraps descriptions without widening controls at ${viewport.name} width`, async ({ page }) => {
    await page.setViewportSize({ width: viewport.width, height: viewport.height });
    await openPreview(page, { component: "field" });

    const layout = await page.locator("#responsive-fields").evaluate((containerNode) => {
      const fields = Array.from(containerNode.children, (node) => node.getBoundingClientRect());
      const container = containerNode.getBoundingClientRect();
      return {
        clientWidth: containerNode.clientWidth,
        scrollWidth: containerNode.scrollWidth,
        container: { left: container.left, right: container.right, width: container.width },
        fields: fields.map(({ left, right, width }) => ({ left, right, width })),
      };
    });

    expect(layout.scrollWidth).toBeLessThanOrEqual(layout.clientWidth + 1);
    for (const field of layout.fields) {
      expect(field.left).toBeGreaterThanOrEqual(layout.container.left - 1);
      expect(field.right).toBeLessThanOrEqual(layout.container.right + 1);
    }
    if (viewport.width >= 640) {
      expect(layout.fields[0].right).toBeLessThanOrEqual(layout.fields[1].left);
    } else {
      for (const field of layout.fields) {
        expect(Math.abs(field.width - layout.container.width)).toBeLessThanOrEqual(1);
      }
    }

    for (const kind of ["input", "textarea"]) {
      const field = page.locator(`#responsive-${kind}-field`);
      const control = page.locator(`#responsive-${kind}`);
      const description = field.locator('[id^="dxf-description-"]');
      const descriptionId = await description.getAttribute("id");
      const sizes = await field.evaluate((fieldNode, selectedKind) => {
        const controlNode = document.querySelector(`#responsive-${selectedKind}`);
        const descriptionNode = fieldNode.querySelector('[id^="dxf-description-"]');
        if (!(controlNode instanceof HTMLElement) || !(descriptionNode instanceof HTMLElement)) {
          throw new Error(`responsive ${selectedKind} Field is incomplete`);
        }

        const descriptionStyle = getComputedStyle(descriptionNode);
        return {
          field: fieldNode.getBoundingClientRect().width,
          control: controlNode.getBoundingClientRect().width,
          description: descriptionNode.getBoundingClientRect().width,
          descriptionScroll: descriptionNode.scrollWidth,
          descriptionHeight: descriptionNode.getBoundingClientRect().height,
          lineHeight: Number.parseFloat(descriptionStyle.lineHeight),
        };
      }, kind);

      await expect(control).toHaveClass(/\bw-full\b/);
      expect(descriptionId).toBeTruthy();
      await expect(control).toHaveAttribute("aria-describedby", descriptionId!);
      await expect(description).toHaveCSS("white-space", "normal");
      expect(Math.abs(sizes.control - sizes.field)).toBeLessThanOrEqual(1);
      expect(sizes.description).toBeLessThanOrEqual(sizes.field + 1);
      expect(sizes.descriptionScroll).toBeLessThanOrEqual(Math.ceil(sizes.field) + 1);
      expect(sizes.descriptionHeight).toBeGreaterThan(sizes.lineHeight * 1.5);
    }
  });
}

async function classes(element: Locator): Promise<string[]> {
  return ((await element.getAttribute("class")) ?? "").split(/\s+/).filter(Boolean).sort();
}
