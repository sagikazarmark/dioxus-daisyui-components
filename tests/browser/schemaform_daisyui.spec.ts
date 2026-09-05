import { expect, type Locator, type Page, test } from "@playwright/test";

import { axis, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "schemaform_daisyui" });
});

// Every example on the page is a whole form, and every form has a control bound
// at `/name`, so a locator is always scoped to the example it means.
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
  // utilities on the elements the package renders itself, read here off the
  // collection fieldset, which carries the most of them.
  test("every appearance renders the collection differently", async ({ page }) => {
    const rendered = await axis(page, "appearance")
      .locator('fieldset[data-schemaform-daisyui="collection"]')
      .evaluateAll((fieldsets) =>
        fieldsets.map((fieldset) => {
          const style = getComputedStyle(fieldset);
          return [style.paddingTop, style.borderTopWidth, style.rowGap].join(" ");
        }),
      );

    expectVaries(rendered, "appearance");
  });

  test("appearance None keeps the component classes and nothing else", async ({ page }) => {
    const plain = axis(page, "appearance").nth(1);
    await expect(plain).toHaveAttribute("data-value", "None");

    const fieldset = plain.locator('fieldset[data-schemaform-daisyui="collection"]');
    expect(((await fieldset.getAttribute("class")) ?? "").trim()).toBe("fieldset");

    const card = plain.locator('[data-schemaform-daisyui="collection-item"]').first();
    expect(((await card.getAttribute("class")) ?? "").trim().split(/\s+/).sort()).toEqual(
      ["card", "card-border", "card-sm"],
    );
  });
});

test.describe("controls", () => {
  test("the native checkbox writes the boolean", async ({ page }) => {
    const active = example(page, "controls").locator('input[name="/active"]');
    await expect(active).toBeChecked();
    await active.click();
    // A rejected write would resynchronise the checkbox; a stable unchecked
    // state means the core accepted it.
    await expect(active).not.toBeChecked();
  });

  test("the nullable checkbox shows null as indeterminate and reaches it through set-null", async ({ page }) => {
    const scope = example(page, "controls");
    const newsletter = scope.locator('[role="checkbox"][name="/newsletter"]');
    await expect(newsletter).toHaveAttribute("aria-checked", "mixed");

    await newsletter.click();
    await expect(newsletter).toHaveAttribute("aria-checked", "false");

    await affordance(scope, "Set Newsletter to null").click();
    await expect(newsletter).toHaveAttribute("aria-checked", "mixed");
  });

  test("the radio widget selects an option per item", async ({ page }) => {
    const scope = example(page, "controls");
    const yearly = scope.getByRole("radio", { name: "yearly", exact: true });
    const monthly = scope.getByRole("radio", { name: "monthly", exact: true });
    await expect(yearly).toHaveAttribute("aria-checked", "true");

    await monthly.click();
    await expect(monthly).toHaveAttribute("aria-checked", "true");
    await expect(yearly).toHaveAttribute("aria-checked", "false");
  });

  test("the compound select shows the chosen option on its trigger", async ({ page }) => {
    const scope = example(page, "controls");
    const region = scope.locator('button[name="/region"]');
    await expect(region).toContainText("eu");

    await region.click();
    await page.getByRole("option", { name: "us", exact: true }).click();
    await expect(region).toContainText("us");
    await expect(region).toHaveAttribute("aria-expanded", "false");
  });

  test("write-only widgets rest on their placeholder after every write", async ({ page }) => {
    const scope = example(page, "controls");

    const mfa = scope.locator('select[name="/mfa"]');
    await expect(mfa).toHaveValue("");
    await mfa.selectOption("false");
    await expect(mfa).toHaveValue("");

    const recovery = scope.locator('select[name="/recovery"]');
    await expect(recovery).toHaveValue("");
    await recovery.selectOption({ label: "sms" });
    await expect(recovery).toHaveValue("");

    const secret = scope.locator('input[name="/secret"]');
    await expect(secret).toHaveAttribute("type", "password");
    await expect(secret).toHaveValue("");
  });

  test("an unparseable edit stays as typed behind an error the input references", async ({ page }) => {
    const scope = example(page, "controls");
    const quantity = scope.locator('input[name="/quantity"]');

    await quantity.fill("abc");
    await expect(quantity).toHaveAttribute("aria-invalid", "true");
    await expect(quantity).toHaveValue("abc");
    const errors = scope.locator(`[id="${await quantity.getAttribute("aria-errormessage")}"]`);
    await expect(errors).toContainText("Enter a valid integer.");

    await quantity.fill("2");
    await expect(quantity).toHaveAttribute("aria-invalid", "false");
    await expect(errors).toBeEmpty();
  });
});

test.describe("arrays", () => {
  test("items are cards named by noun and position, with positional actions", async ({ page }) => {
    const scope = example(page, "arrays");
    await expect(scope.getByRole("group", { name: "Tags", exact: true })).toHaveCount(1);
    await expect(scope.getByRole("group", { name: "Tags item 1", exact: true })).toHaveCount(1);
    await expect(scope.getByRole("group", { name: "Tags item 2", exact: true })).toHaveCount(1);

    // The adapter's gating shows in which buttons exist.
    await expect(affordance(scope, "Move Tags item at position 1 up")).toHaveCount(0);
    await expect(affordance(scope, "Move Tags item at position 1 down")).toHaveCount(1);
    await expect(affordance(scope, "Move Tags item at position 2 down")).toHaveCount(0);
  });

  test("insert seeds an item, focuses it and announces it", async ({ page }) => {
    const scope = example(page, "arrays");
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
    const scope = example(page, "arrays");
    await affordance(scope, "Remove Tags item at position 1").click();
    await eventually(() => scope.locator('input[name="/tags/1"]').count(), (count) => count === 0);
    await affordance(scope, "Remove Tags item at position 1").click();

    const empty = scope.locator('[data-schemaform-daisyui="collection-empty"]');
    await expect(empty).toHaveText("Nothing here yet.");
    await expect(scope.locator('input[name="/tags/0"]')).toHaveCount(0);
    await expect(affordance(scope, "Add Tags item")).toHaveCount(1);
  });

  test("removal is withdrawn at minItems", async ({ page }) => {
    const scope = example(page, "arrays");
    await affordance(scope, "Remove Team item at position 2").click();
    await eventually(() => scope.locator('input[name="/team/1/name"]').count(), (count) => count === 0);
    await expect(affordance(scope, /^Remove Team item/)).toHaveCount(0);
  });
});
