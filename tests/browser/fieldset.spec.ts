import { expect, type Locator, test } from "@playwright/test";

import { openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "fieldset" });
});

test("renders native parts with exact classes and the legend first", async ({ page }) => {
  const fieldset = page.locator("#default-fieldset");
  const legend = page.locator("#default-legend");

  await expect(fieldset).toHaveJSProperty("tagName", "FIELDSET");
  await expect(legend).toHaveJSProperty("tagName", "LEGEND");
  expect(
    await fieldset.evaluate((node) => ({
      disabled: node.hasAttribute("disabled"),
      form: node.hasAttribute("form"),
      name: node.hasAttribute("name"),
    })),
  ).toEqual({ disabled: false, form: false, name: false });
  expect(await classes(fieldset)).toEqual(["fieldset"]);
  expect(await classes(legend)).toEqual(["fieldset-legend"]);

  await expect(fieldset).toHaveCSS("display", "grid");
  await expect(legend).toHaveCSS("display", "flex");

  const fieldsets = page.locator('[data-page="fieldset"] fieldset');
  const firstChildren = fieldsets.locator(":scope > :first-child");
  expect(await firstChildren.count()).toBe(await fieldsets.count());
  expect(await firstChildren.evaluateAll((nodes) => nodes.every((node) => node.tagName === "LEGEND")))
    .toBe(true);
});

test("the first legend names the group", async ({ page }) => {
  const group = page.getByRole("group", { name: "Page details", exact: true });

  await expect(group).toHaveJSProperty("tagName", "FIELDSET");
});

test("preserves native form ownership, naming, disabled descendants, and the legend exception", async ({
  page,
}) => {
  const form = page.locator("#fieldset-form");
  const fieldset = page.locator("#disabled-fieldset");
  const legendControl = page.locator("#legend-exception");
  const disabledControl = page.locator("#disabled-email");

  await expect(fieldset).toHaveAttribute("form", "fieldset-form");
  await expect(fieldset).toHaveAttribute("name", "account-settings");
  expect(
    await fieldset.evaluate((node) => ({
      ownAttribute: node.hasAttribute("disabled"),
      ownProperty: (node as HTMLFieldSetElement).disabled,
      nativeState: node.matches(":disabled"),
    })),
  ).toEqual({ ownAttribute: true, ownProperty: true, nativeState: true });
  expect(await fieldset.evaluate((node) => (node as HTMLFieldSetElement).form?.id)).toBe(
    "fieldset-form",
  );
  expect(await fieldset.evaluate((node) => (node as HTMLFieldSetElement).name)).toBe(
    "account-settings",
  );

  const ownedNames = await form.evaluate((node) =>
    Array.from((node as HTMLFormElement).elements, (element) =>
      (element as HTMLFieldSetElement | HTMLInputElement).name,
    ),
  );
  expect(ownedNames).toContain("account-settings");
  expect(ownedNames).toContain("account-email");

  await expect(legendControl).toBeEnabled();
  await expect(disabledControl).toBeDisabled();
  expect(
    await disabledControl.evaluate((node) => ({
      ownAttribute: node.hasAttribute("disabled"),
      ownProperty: (node as HTMLInputElement).disabled,
      nativeState: node.matches(":disabled"),
    })),
  ).toEqual({ ownAttribute: false, ownProperty: false, nativeState: true });

  expect(
    await form.evaluate((node) =>
      Object.fromEntries(new FormData(node as HTMLFormElement).entries()),
    ),
  ).toEqual({});
});

test("merges caller classes and attributes onto both parts", async ({ page }) => {
  const fieldset = page.locator("#caller-fieldset");
  const legend = page.locator("#caller-legend");

  expect(await classes(fieldset)).toEqual(
    ["fieldset", "border-error", "rounded-none", "border-2", "p-3"].sort(),
  );
  expect(await classes(legend)).toEqual(["fieldset-legend", "text-error", "uppercase"].sort());
  await expect(fieldset).toHaveAttribute("title", "Caller fieldset");
  await expect(legend).toHaveAttribute("title", "Caller legend");

  await expect(fieldset).toHaveCSS("border-radius", "0px");
  await expect(legend).toHaveCSS("text-transform", "uppercase");
});

async function classes(element: Locator): Promise<string[]> {
  return ((await element.getAttribute("class")) ?? "").split(/\s+/).filter(Boolean).sort();
}
