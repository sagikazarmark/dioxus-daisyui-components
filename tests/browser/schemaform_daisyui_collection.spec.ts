import { expect, type Locator, type Page, test } from "@playwright/test";

import { axis, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "schemaform_daisyui_collection" });
});

// Each form is independent, so a locator is always scoped to its Example.
function example(page: Page, slug: string): Locator {
  return page.locator(`[data-example="${slug}"]`);
}

// An affordance rendered as a button, found by its accessible name: the ids
// depend on bind order, the names do not.
function affordance(scope: Locator, name: string | RegExp): Locator {
  return scope.getByRole("button", { name, exact: typeof name === "string" });
}

// The core applies every edit as it happens and the page settles a task later,
// so nothing below is asserted synchronously.
async function eventually<T>(probe: () => Promise<T>, predicate: (value: T) => boolean): Promise<T> {
  await expect.poll(async () => predicate(await probe())).toBe(true);
  return probe();
}

test.describe("styling", () => {
  // The axis row renders one whole form per value; what differs is the layout
  // utilities on the elements the add-on renders itself, read here off the
  // collection fieldset, which carries the most of them.
  test("every appearance renders the collection differently", async ({ page }) => {
    // Each value's form has two arrays; the first stands for both.
    const rendered = [];
    for (const value of await axis(page, "appearance").all()) {
      rendered.push(
        await value
          .locator('fieldset[data-schemaform-daisyui="collection"]')
          .first()
          .evaluate((fieldset) => {
            const style = getComputedStyle(fieldset);
            return [style.paddingTop, style.borderTopWidth, style.rowGap].join(" ");
          }),
      );
    }

    expectVaries(rendered, "appearance");
  });

  test("appearance None keeps the component classes and nothing else", async ({ page }) => {
    const plain = axis(page, "appearance").nth(1);
    await expect(plain).toHaveAttribute("data-value", "None");

    const fieldset = plain.locator('fieldset[data-schemaform-daisyui="collection"]').first();
    expect(((await fieldset.getAttribute("class")) ?? "").trim()).toBe("fieldset");

    const card = plain.locator('[data-schemaform-daisyui="collection-item"]').first();
    expect(((await card.getAttribute("class")) ?? "").trim().split(/\s+/).sort()).toEqual(
      ["card", "card-border", "card-sm"],
    );
  });
});

test.describe("arrays", () => {
  test("items are cards named by noun and position, with positional actions", async ({ page }) => {
    const scope = example(page, "overview");
    await expect(scope.getByRole("group", { name: "Tags", exact: true })).toHaveCount(1);
    await expect(scope.getByRole("group", { name: "Tags item 1", exact: true })).toHaveCount(1);
    await expect(scope.getByRole("group", { name: "Tags item 2", exact: true })).toHaveCount(1);

    // The adapter's gating shows in which buttons exist.
    await expect(affordance(scope, "Move Tags item at position 1 up")).toHaveCount(0);
    await expect(affordance(scope, "Move Tags item at position 1 down")).toHaveCount(1);
    await expect(affordance(scope, "Move Tags item at position 2 down")).toHaveCount(0);
  });

  test("insert seeds an item, focuses it and announces it", async ({ page }) => {
    const scope = example(page, "overview");
    await affordance(scope, "Insert Tags item before position 1").click();

    const inserted = scope.locator('input[name="/tags/0"]');
    await expect(inserted).toHaveValue("fresh");
    await expect(inserted).toBeFocused();
    await expect(scope.locator('input[name="/tags/1"]')).toHaveValue("rust");
    await expect(
      scope.getByRole("group", { name: "Tags", exact: true }).locator("[data-array-status]"),
    ).toHaveText("Tags item inserted at position 1.");
  });

  test("emptying the collection shows its empty state and keeps append", async ({ page }) => {
    const scope = example(page, "overview");
    await affordance(scope, "Remove Tags item at position 1").click();
    await eventually(() => scope.locator('input[name="/tags/1"]').count(), (count) => count === 0);
    await affordance(scope, "Remove Tags item at position 1").click();

    const empty = scope.locator('[data-schemaform-daisyui="collection-empty"]');
    await expect(empty).toHaveText("Nothing here yet.");
    await expect(scope.locator('input[name="/tags/0"]')).toHaveCount(0);
    await expect(affordance(scope, "Add Tags item")).toHaveCount(1);
  });

  test("removal is withdrawn at minItems", async ({ page }) => {
    const scope = example(page, "overview");
    await affordance(scope, "Remove Team item at position 2").click();
    await eventually(() => scope.locator('input[name="/team/1/name"]').count(), (count) => count === 0);
    await expect(affordance(scope, /^Remove Team item/)).toHaveCount(0);
  });

  test("applied by hand, the collection frames the built-in controls", async ({ page }) => {
    const scope = example(page, "composition");
    await expect(scope.locator('fieldset[data-schemaform-daisyui="collection"]')).toHaveCount(2);
    await expect(scope.locator('[data-schemaform-daisyui="shell"]')).toHaveCount(1);
  });
});

test.describe("button classes", () => {
  test("item and append buttons carry their role and, unset, the package's look", async ({ page }) => {
    const scope = example(page, "overview");
    const item = scope.locator('[data-schemaform-button="item"]').first();
    await expect(item).toHaveClass(/\bjoin-item\b/);
    await expect(item).toHaveClass(/\bbtn-sm\b/);
    await expect(scope.locator('[data-schemaform-button="append"]').first()).toHaveClass(/\bbtn-outline\b/);
  });

  test("a role given a class renders btn and the class in place of the package's look", async ({ page }) => {
    const scope = example(page, "buttons");
    const item = scope.locator('[data-schemaform-button="item"]').first();
    for (const kept of ["btn", "join-item", "btn-square", "btn-soft", "btn-sm"]) {
      await expect(item).toHaveClass(new RegExp(`\\b${kept}\\b`));
    }

    const append = scope.locator('[data-schemaform-button="append"]').first();
    await expect(append).toHaveClass(/\bbtn-accent\b/);
    await expect(append).not.toHaveClass(/\bbtn-outline\b/);
  });
});
