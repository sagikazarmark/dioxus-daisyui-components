import { expect, type Locator, test } from "@playwright/test";

import { axis, computedStyle, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "separator" });
});

test.describe("styling", () => {
  // The colour is read off the rule rather than off the element: daisyUI draws
  // both halves of the line in `::before` and `::after`, and a colour class
  // paints those. The element itself is transparent under every value, so
  // reading its own background would say the same thing nine times.
  test("every colour paints the rule a colour of its own", async ({ page }) => {
    expectVaries(await computedStyle(axis(page, "color"), "background-color", "::before"), "color");
  });

  // Placement is not a property that varies but a half of the line that goes
  // missing (`divider-start` drops the rule before the content and
  // `divider-end` the one after it) so what has to differ per value is the
  // pair, not either half.
  test("every placement drops a different half of the rule", async ({ page }) => {
    const separators = axis(page, "placement");
    const before = await computedStyle(separators, "display", "::before");
    const after = await computedStyle(separators, "display", "::after");

    expectVaries(
      before.map((display, index) => `${display}/${after[index]}`),
      "placement",
    );
  });

  test("a caller's classes join the component's own", async ({ page }) => {
    const separator = page.locator("#caller-attributes");
    const classes = ((await separator.getAttribute("class")) ?? "").split(/\s+/);

    // The component's own, including the orientation it emits, and then the
    // caller's, on one element.
    expect(classes).toContain("divider");
    expect(classes).toContain("divider-vertical");
    expect(classes).toContain("divider-primary");
    expect(classes).toContain("m-0");

    // And the caller's applies, rather than merely surviving the merge, over
    // a margin daisyUI's own class sets, which the caller wins on cascade
    // layers rather than on specificity (ADR-0004).
    await expect(separator).toHaveCSS("margin-top", "0px");
  });

  // The cost of emitting an orientation class for both values (ADR-0008): a
  // caller who follows daisyUI's own advice for a layout that changes at a
  // breakpoint writes `md:divider-horizontal` and meets the `divider-vertical`
  // this component already emitted. They tie on specificity and on layer, so
  // the win comes from tailwind generating responsive variants after the
  // utilities they vary: a source-order argument, and therefore one worth
  // asserting in a browser rather than in prose.
  test("a caller's responsive orientation beats the emitted one", async ({ page }) => {
    const separator = page.locator("#caller-responsive");

    // The viewport these run at is wider than the breakpoint, so the caller's
    // class is the one in effect.
    expect(page.viewportSize()?.width ?? 0).toBeGreaterThanOrEqual(768);
    await expectRuns(separator, "down");
  });
});

test.describe("behaviour", () => {
  test("a separator is announced with the orientation of the line it draws", async ({ page }) => {
    const across = page.locator("#rule-across");
    const down = page.locator("#rule-down");

    // ARIA names a separator after its line and daisyUI names its divider
    // after the layout the line sits in, so the class the component emits is
    // the inverse of what it announces. Both halves are asserted together
    // here: that is the whole of the trade, and a component that had passed
    // daisyUI's name straight through would announce the wrong one.
    await expect(across).toHaveRole("separator");
    await expect(across).toHaveAttribute("aria-orientation", "horizontal");
    expect((await across.getAttribute("class")) ?? "").toContain("divider-vertical");
    await expectRuns(across, "across");

    await expect(down).toHaveRole("separator");
    await expect(down).toHaveAttribute("aria-orientation", "vertical");
    expect((await down.getAttribute("class")) ?? "").toContain("divider-horizontal");
    await expectRuns(down, "down");
  });

  test("a decorative separator draws a rule and announces nothing", async ({ page }) => {
    const decorative = page.locator("#rule-decorative");

    // Still a line: the classes and the rule it draws are the same ones.
    await expectRuns(decorative, "across");

    // And absent from the accessibility tree, so a screen reader is not told
    // about a rule that divides nothing. The orientation goes with it: what is
    // left is a `data-orientation` for styling, which announces nothing.
    await expect(decorative).toHaveRole("none");
    await expect(decorative).not.toHaveAttribute("aria-orientation");
  });
});

/** Asserts which way a separator's rule runs, by the box the element takes. */
async function expectRuns(separator: Locator, direction: "across" | "down"): Promise<void> {
  const box = await separator.boundingBox();
  expect(box, "the separator rendered no box at all").not.toBeNull();

  const { width, height } = box!;
  if (direction === "across") {
    expect(width, `expected a rule across, got ${width}x${height}`).toBeGreaterThan(height);
  } else {
    expect(height, `expected a rule down, got ${width}x${height}`).toBeGreaterThan(width);
  }
}
