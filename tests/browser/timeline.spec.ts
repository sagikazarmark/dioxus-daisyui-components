import { expect, type Locator, type Page, test } from "@playwright/test";

import { axis, computedStyle, expectVaries, openPreview } from "./preview";

test.beforeEach(async ({ page }) => {
  await openPreview(page, { component: "timeline" });
});

test.describe("structure and styling", () => {
  test("all six parts preserve the direct list and connector structure", async ({ page }) => {
    const timeline = page.locator("#overview-timeline");
    const first = page.locator("#overview-first");
    const middle = page.locator("#overview-middle");
    const last = page.locator("#overview-last");

    expect(await timeline.evaluate((node) => node.tagName)).toBe("UL");
    await expect(timeline.locator(":scope > li")).toHaveCount(3);
    await expect(timeline.locator(":scope > :not(li)")).toHaveCount(0);
    expect(await first.evaluate(childTags)).toEqual(["DIV", "DIV", "DIV", "HR"]);
    expect(await middle.evaluate(childTags)).toEqual(["HR", "DIV", "DIV", "DIV", "HR"]);
    expect(await last.evaluate(childTags)).toEqual(["HR", "DIV", "DIV", "DIV"]);

    await expectPartClasses(page, "#overview-timeline", "timeline", "timeline-horizontal");
    expect(await first.evaluate((node) => node.tagName)).toBe("LI");
    await expectPartClasses(page, "#overview-first .timeline-start", "timeline-start");
    await expectPartClasses(page, "#overview-first .timeline-middle", "timeline-middle");
    await expectPartClasses(
      page,
      "#overview-first .timeline-end",
      "timeline-end",
      "timeline-box",
    );
    expect(await page.locator("#overview-first-outgoing").evaluate((node) => node.tagName)).toBe(
      "HR",
    );
  });

  test("directions turn the timeline and its connector dimensions", async ({ page }) => {
    const timelines = axis(page, "direction");
    expectVaries(await computedStyle(timelines, "flex-direction"), "direction");

    const connectors = timelines.locator(":scope > li > hr:first-child");
    const boxes = await connectors.evaluateAll((nodes) =>
      nodes.map((node) => {
        const { width, height } = node.getBoundingClientRect();
        return { width, height };
      }),
    );

    expect(boxes).toHaveLength(2);
    expect(boxes[0].width).toBeGreaterThan(boxes[0].height);
    expect(boxes[1].height).toBeGreaterThan(boxes[1].width);
    await expectPartClasses(page, '[data-axis="direction"] > :first-child', "timeline-horizontal");
    await expectPartClasses(page, '[data-axis="direction"] > :last-child', "timeline-vertical");
  });

  test("compact moves start content across the line", async ({ page }) => {
    const offsets = await markerOffsets(axis(page, "compact"), ".timeline-start");

    expect(offsets).toHaveLength(2);
    expect(offsets[0]).toBeLessThan(0);
    expect(offsets[1]).toBeGreaterThan(0);
  });

  test("snap moves markers toward the start of their items", async ({ page }) => {
    const offsets = await axis(page, "snap").evaluateAll((timelines) =>
      timelines.map((timeline) => {
        const item = timeline.querySelector(":scope > li");
        const marker = item?.querySelector(":scope > .timeline-middle");
        if (!(item instanceof HTMLElement) || !(marker instanceof HTMLElement)) return NaN;
        return marker.getBoundingClientRect().left - item.getBoundingClientRect().left;
      }),
    );

    expect(offsets).toHaveLength(2);
    expect(offsets.every(Number.isFinite)).toBe(true);
    expect(offsets[0]).toBeGreaterThan(offsets[1]);
  });

  test("the shared Box appearance draws both content parts", async ({ page }) => {
    const timelines = axis(page, "content-appearance");
    const startBorders = (
      await computedStyle(timelines.locator(":scope > li > .timeline-start"), "border-top-width")
    ).map(parseFloat);
    const endBorders = (
      await computedStyle(timelines.locator(":scope > li > .timeline-end"), "border-top-width")
    ).map(parseFloat);

    expect(startBorders[1]).toBeGreaterThan(startBorders[0]);
    expect(endBorders[1]).toBeGreaterThan(endBorders[0]);
    await expectPartClasses(
      page,
      '[data-axis="content-appearance"] > :last-child .timeline-start',
      "timeline-start",
      "timeline-box",
    );
    await expectPartClasses(
      page,
      '[data-axis="content-appearance"] > :last-child .timeline-end',
      "timeline-end",
      "timeline-box",
    );
  });

  test("modifier defaults emit no compact, snap, or box classes", async ({ page }) => {
    await expect(classesOf(axis(page, "compact").first())).resolves.toEqual([
      "timeline",
      "timeline-horizontal",
    ]);
    await expect(classesOf(axis(page, "snap").first())).resolves.toEqual([
      "timeline",
      "timeline-horizontal",
    ]);
    await expect(
      classesOf(axis(page, "content-appearance").first().locator(".timeline-start")),
    ).resolves.toEqual(["timeline-start"]);
    await expect(
      classesOf(axis(page, "content-appearance").first().locator(".timeline-end")),
    ).resolves.toEqual(["timeline-end"]);
  });

  test("caller classes and attributes merge on all six parts", async ({ page }) => {
    await expectPartClasses(
      page,
      "#caller-timeline",
      "timeline",
      "timeline-vertical",
      "timeline-compact",
      "timeline-snap-icon",
      "min-h-64",
    );
    await expectPartClasses(page, "#caller-item", "min-h-28");
    await expectPartClasses(page, "#caller-start", "timeline-start", "timeline-box", "font-semibold");
    await expectPartClasses(page, "#caller-middle", "timeline-middle", "text-primary");
    await expectPartClasses(page, "#caller-end", "timeline-end", "timeline-box", "italic");
    await expectPartClasses(page, "#caller-connector-incoming", "bg-primary");
    await expectPartClasses(page, "#caller-connector-outgoing", "bg-secondary");

    for (const part of ["timeline", "item", "start", "middle", "end"])
      await expect(page.locator(`#caller-${part}`)).toHaveAttribute("data-owner", `caller-${part}`);
    for (const connector of ["incoming", "outgoing"])
      await expect(page.locator(`#caller-connector-${connector}`)).toHaveAttribute(
        "data-owner",
        `caller-connector-${connector}`,
      );

    const connectorColors = await computedStyle(
      page.locator("#caller-connector-incoming, #caller-connector-outgoing"),
      "background-color",
    );
    expect(connectorColors).toHaveLength(2);
    expect(connectorColors[0]).not.toBe(connectorColors[1]);
  });
});

function childTags(node: Element): string[] {
  return Array.from(node.children, (child) => child.tagName);
}

async function markerOffsets(timelines: Locator, contentSelector: string): Promise<number[]> {
  return timelines.evaluateAll(
    (nodes, selector) =>
      nodes.map((timeline) => {
        const content = timeline.querySelector(selector);
        const marker = timeline.querySelector(".timeline-middle");
        if (!(content instanceof HTMLElement) || !(marker instanceof HTMLElement)) return NaN;
        const contentBox = content.getBoundingClientRect();
        const markerBox = marker.getBoundingClientRect();
        return contentBox.top + contentBox.height / 2 - (markerBox.top + markerBox.height / 2);
      }),
    contentSelector,
  );
}

async function classesOf(locator: Locator): Promise<string[]> {
  return ((await locator.getAttribute("class")) ?? "").split(/\s+/).filter(Boolean);
}

async function expectPartClasses(
  page: Page,
  selector: string,
  ...expectedClasses: string[]
): Promise<void> {
  const classes = await classesOf(page.locator(selector));
  for (const expectedClass of expectedClasses) expect(classes).toContain(expectedClass);
}
