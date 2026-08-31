import { expect, type Locator, type Page, test } from "@playwright/test";

import { axis, computedStyle, expectGrows, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "hover_card" });
});

test.describe("styling", () => {
  // The side axis is Tailwind utilities rather than daisyUI classes (ADR-0015),
  // so what it does is read where it lands: each value puts the panel in a
  // different place against its own trigger. Boxes rather than computed styles,
  // because `top: auto` and `bottom: 100%` describe a position without saying
  // where it ended up.
  test("every side puts the panel somewhere of its own", async ({ page }) => {
    const placed = await placements(page, "side");

    expectVaries(
      placed.map(({ panel, trigger }) => `${panel.x - trigger.x},${panel.y - trigger.y}`),
      "side",
    );

    const [top, right, bottom, left] = placed;
    expect(top.panel.y + top.panel.height).toBeLessThanOrEqual(top.trigger.y + 1);
    expect(right.panel.x + 1).toBeGreaterThanOrEqual(right.trigger.x + right.trigger.width);
    expect(bottom.panel.y + 1).toBeGreaterThanOrEqual(bottom.trigger.y + bottom.trigger.height);
    expect(left.panel.x + left.panel.width).toBeLessThanOrEqual(left.trigger.x + 1);
  });

  test("every alignment sits the panel differently along that side", async ({ page }) => {
    const placed = await placements(page, "align");

    expectVaries(
      placed.map(({ panel, trigger }) => panel.x - trigger.x),
      "align",
    );

    // The default side is below the trigger, so the alignment runs across it:
    // start shares the trigger's leading edge, end shares its trailing one.
    const [start, , end] = placed;
    expect(Math.abs(start.panel.x - start.trigger.x)).toBeLessThan(2);
    expect(
      Math.abs(end.panel.x + end.panel.width - (end.trigger.x + end.trigger.width)),
    ).toBeLessThan(2);
  });

  // daisyUI's card sizes the body's padding and the title's font rather than
  // anything on the card itself, so the axis is read a level in.
  test("every size pads the body differently", async ({ page }) => {
    expectGrows(
      (await computedStyle(bodies(page, "size"), "padding-top")).map(parseFloat),
      "size",
    );
  });

  // Style and width together, because tailwind's own reset gives every element
  // a `solid` border style at zero width, so a card with no border class and
  // one with `card-border` agree on the style and differ only in the width.
  test("every border draws an edge of its own", async ({ page }) => {
    const styles = await computedStyle(panels(page, "border"), "border-top-style");
    const widths = await computedStyle(panels(page, "border"), "border-top-width");

    expectVaries(
      styles.map((style, index) => `${style} ${widths[index]}`),
      "border",
    );
  });

  test("the panel's appearance axis decides whether it is painted", async ({ page }) => {
    expectVaries(await computedStyle(panels(page, "appearance"), "background-color"), "appearance");
  });

  test("the positioning axis decides whether the panel leaves the flow", async ({ page }) => {
    expectVaries(await computedStyle(panels(page, "positioning"), "position"), "positioning");
  });

  test("the root's appearance axis decides whether it is a positioning context", async ({
    page,
  }) => {
    expectVaries(await computedStyle(axis(page, "root"), "position"), "root");
  });

  test("a caller's classes join the panel's own", async ({ page }) => {
    const panel = page.locator("#caller-attributes");
    const classes = ((await panel.getAttribute("class")) ?? "").split(/\s+/);

    expect(classes).toContain("card");
    expect(classes).toContain("bg-base-100");
    expect(classes).toContain("w-56");

    // And the caller's applies rather than merely surviving the merge: this
    // component emits no width at all, so the only one on the element is theirs.
    const width = (await panel.boundingBox())?.width ?? 0;
    expect(width).toBeCloseTo(224, 0);
  });

  test("a caller repaints the panel by switching this component's utilities off", async ({
    page,
  }) => {
    // Two utilities only tie, and a tie is settled by generated-stylesheet order
    // rather than by the class attribute (ADR-0004).
    await expect(page.locator("#caller-paint")).toHaveCSS("border-radius", "0px");
  });
});

test.describe("behaviour", () => {
  const trigger = (page: Page) => page.locator("#overview-trigger");
  const panel = (page: Page) => page.locator("#overview-panel");

  test("the card opens on hover and closes when the pointer leaves", async ({ page }) => {
    await expect(panel(page)).toHaveCount(0);

    await trigger(page).hover();
    await expect(panel(page)).toBeVisible();

    await page.mouse.move(0, 0);
    await expect(panel(page)).toHaveCount(0);
  });

  // The panel is for reading rather than for reaching into, and this is the
  // proof of it: moving the pointer off the trigger closes the card, even when
  // where it moves to is the panel itself. See the component's documentation:
  // it is the primitive's behaviour, and it is why nothing interactive belongs
  // in a hover card.
  test("the pointer cannot travel from the trigger into the panel", async ({ page }) => {
    await trigger(page).hover();
    await expect(panel(page)).toBeVisible();

    const box = (await panel(page).boundingBox())!;
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2, { steps: 10 });

    await expect(panel(page)).toHaveCount(0);
  });

  test("the card opens on keyboard focus and closes on blur", async ({ page }) => {
    await trigger(page).focus();
    await expect(panel(page)).toBeVisible();

    await trigger(page).blur();
    await expect(panel(page)).toHaveCount(0);
  });

  test("the trigger is described by the panel while it is open", async ({ page }) => {
    await expect(trigger(page)).not.toHaveAttribute("aria-describedby", /./);

    await trigger(page).focus();

    // The relationship the panel is announced by, which is the primitive's and
    // which the id this component passes through has to leave intact.
    const describedby = await trigger(page).getAttribute("aria-describedby");
    expect(describedby, "the trigger is described by nothing").toBeTruthy();
    await expect(page.locator(`#${describedby}`)).toBeVisible();
  });

  test("the panel reports the side and the alignment its utilities placed it on", async ({
    page,
  }) => {
    await trigger(page).focus();

    // The axes do two things at once (emit the utilities and tell the
    // primitive) so what it reports and where the panel is cannot disagree.
    await expect(panel(page)).toHaveAttribute("data-side", "bottom");
    await expect(panel(page)).toHaveAttribute("data-align", "center");
  });
});

/** The panels one axis row rendered, in the order its variant list is in. */
function panels(page: Page, name: string): Locator {
  return page.locator(`[data-axis="${name}"] > * > [role="tooltip"]`);
}

/** The bodies inside those panels, which is where a card's size lands. */
function bodies(page: Page, name: string): Locator {
  return page.locator(`[data-axis="${name}"] > * > [role="tooltip"] > .card-body`);
}

/** Where each panel of an axis row ended up, against the trigger it belongs to. */
async function placements(
  page: Page,
  name: string,
): Promise<{ panel: DOMRect; trigger: DOMRect }[]> {
  return page.locator(`[data-axis="${name}"] > *`).evaluateAll((roots) =>
    roots.map((root) => ({
      panel: root.querySelector('[role="tooltip"]')!.getBoundingClientRect().toJSON(),
      trigger: root.querySelector('[role="button"]')!.getBoundingClientRect().toJSON(),
    })),
  );
}
