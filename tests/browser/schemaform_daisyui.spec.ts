import { expect, type Locator, type Page, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

import { axis, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "schemaform_daisyui" });
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

for (const theme of ["light", "dark"]) {
  test(`checkbox groups, format widgets and advisory findings pass axe in ${theme}`, async ({ page }) => {
    await openPreview(page, { component: "schemaform_daisyui", theme });
    await example(page, "advisory").locator('button[type="submit"]').click();
    // Audit each rendered form, independently of the Preview's tab chrome and
    // the other forms' identically named summary landmarks.
    for (const slug of ["multiple_choice", "formats", "advisory"]) {
      const results = await new AxeBuilder({ page })
        .include(`[data-example="${slug}"] form`)
        .analyze();
      expect(results.violations, slug).toEqual([]);
    }
  });
}

test.describe("controls", () => {
  test("format selects the native input type and write-only takes precedence", async ({ page }) => {
    const scope = example(page, "formats");
    for (const [name, type] of Object.entries({
      email: "email", "idn-email": "email", uri: "url", "uri-reference": "url",
      iri: "url", "iri-reference": "url", date: "date", "date-time": "datetime-local",
      time: "time", unknown: "text", plain: "text", number: "text", secret: "password",
    })) {
      const input = scope.locator(`input[name="/${name}"]`);
      await expect(input).toHaveClass(/\binput\b/);
      await expect(input).toHaveAttribute("type", type);
    }
  });

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

    await affordance(scope, "Clear Newsletter").click();
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
  test("a required array does not require individual choices or a nonempty selection", async ({ page }) => {
    const scope = example(page, "multiple_choice");
    const group = scope.getByRole("group", { name: /^Channels/ });
    await expect(group.getByRole("checkbox")).toHaveCount(2);
    for (const checkbox of await group.getByRole("checkbox").all()) {
      await expect(checkbox).not.toHaveAttribute("aria-required", "true");
      await expect(checkbox).not.toHaveAttribute("required");
      await expect(checkbox).not.toBeChecked();
    }
    await expect(group).toHaveAccessibleName("Channels (required)");
    await expect(group).not.toHaveAttribute("aria-required");
    await affordance(scope, "Remove Legacy topics").click();
    await affordance(scope, "Submit").click();
    await expect.poll(async () => JSON.parse(await scope.getByRole("status", { name: "Submitted topics" }).innerText()).channels)
      .toEqual([]);
  });

  test("multiple choice follows the node, toggles by keyboard, and describes every checkbox", async ({ page }) => {
    const scope = example(page, "multiple_choice");
    const group = scope.getByRole("group", { name: "Topics", exact: true });
    const rust = group.getByRole("checkbox", { name: "Rust", exact: true });
    const dioxus = group.getByRole("checkbox", { name: "Dioxus", exact: true });
    await expect(rust).toHaveClass(/\bcheckbox\b/);
    await expect(rust).toBeChecked();
    await expect(dioxus).not.toBeChecked();
    await dioxus.focus();
    await page.keyboard.press("Space");
    await expect(dioxus).toBeChecked();
    await rust.uncheck();
    await affordance(scope, "Reset topics").click();
    await expect(rust).toBeChecked();
    await expect(dioxus).not.toBeChecked();
    for (const checkbox of await group.getByRole("checkbox").all()) {
      const ids = (await checkbox.getAttribute("aria-describedby"))?.split(/\s+/) ?? [];
      expect(ids.length).toBeGreaterThan(0);
      for (const id of ids) await expect(scope.locator(`[id="${id}"]`)).toHaveCount(1);
    }
    const legacy = scope.getByRole("group", { name: "Legacy topics", exact: true });
    await expect(legacy.locator("[data-incompatible-value]")).toContainText("retired");
    const incompatibleId = await legacy.locator("[data-incompatible-value]").getAttribute("id");
    await expect(legacy.getByRole("checkbox").first()).toHaveAttribute("aria-describedby", new RegExp(incompatibleId!));
    await affordance(scope, "Remove Legacy topics").click();
    await dioxus.check();
    await affordance(scope, "Submit").click();
    await expect.poll(async () => JSON.parse(await scope.getByRole("status", { name: "Submitted topics" }).innerText()).topics.sort())
      .toEqual(["Dioxus", "Rust"]);
  });

  test("multiple choice can be removed, recreated by a toggle, and focused from a finding", async ({ page }) => {
    const scope = example(page, "multiple_choice");
    const group = scope.getByRole("group", { name: "Topics", exact: true });
    await affordance(scope, "Remove Topics").click();
    await expect(group.getByRole("checkbox", { checked: true })).toHaveCount(0);
    const rust = group.getByRole("checkbox", { name: "Rust", exact: true });
    await rust.check();
    await rust.uncheck();
    await affordance(scope, "Submit").click();
    await scope.locator("[data-finding-summary]").getByRole("button", { name: "Value does not satisfy minItems." }).click();
    await expect(group.getByRole("checkbox").first()).toBeFocused();
    for (const checkbox of await group.getByRole("checkbox").all()) {
      await expect(checkbox).toHaveAttribute("aria-invalid", "true");
      const errorsId = await checkbox.getAttribute("aria-errormessage");
      expect(errorsId).toBeTruthy();
      await expect(scope.locator(`[id="${errorsId}"]`)).toContainText("Value does not satisfy minItems.");
      expect((await checkbox.getAttribute("aria-describedby"))!.split(/\s+/)).toContain(errorsId);
      for (const id of (await checkbox.getAttribute("aria-describedby"))!.split(/\s+/)) {
        await expect(scope.locator(`[id="${id}"]`)).toHaveCount(1);
      }
    }
  });

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

test("advisory submission presents warnings and switching mode restores error framing", async ({ page }) => {
  const scope = example(page, "advisory");
  const submit = scope.locator('button[type="submit"]');
  await expect(submit).toHaveText("Submit");
  await expect(submit).toHaveClass(/\bbtn-warning\b/);
  await submit.focus();
  await page.keyboard.press("Enter");
  await expect(scope.getByRole("status", { name: "Submission result" })).toHaveText("Advisory submission: 2 findings");
  await expect(submit).toBeFocused();
  await expect(scope.locator(".alert-warning")).toBeVisible();
  await expect(scope.locator('[data-schemaform-daisyui="string"] [data-finding]')).toHaveClass(/\btext-warning\b/);
  await expect(scope.locator('[data-schemaform-daisyui="collection"] [data-finding]')).toHaveClass(/\btext-warning\b/);
  await scope.getByRole("checkbox", { name: "Advisory mode" }).click();
  await expect(submit).toHaveClass(/\bbtn-primary\b/);
  await expect(scope.locator(".alert-error")).toBeVisible();
  await expect(scope.locator('[data-schemaform-errors] [data-finding]')).toHaveCount(1);
  await expect(scope.locator('[data-schemaform-daisyui="collection"] [data-finding]')).toHaveClass(/\btext-error\b/);
});
