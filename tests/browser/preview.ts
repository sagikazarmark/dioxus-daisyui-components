import { expect, type Locator, type Page } from "@playwright/test";

import {
  computedStyle,
  addresses as documentationAddresses,
  example,
  type Motion,
  openPreview as openDocumentation,
  themes,
} from "@sagikazarmark/dioxus-registry-preview-playwright";

export { computedStyle, example, themes };

/** One address in the preview: a component page, under a theme. */
export type Address = {
  component?: string;
  path?: string;
  theme?: string;
};

/**
 * Opens one address in the preview and waits for it to be there.
 *
 * The preview is a wasm app, so the document is empty until it boots; every
 * spec goes through here rather than through `page.goto` so that no assertion
 * can run against a page that has not rendered yet.
 *
 * A component page is a path and the theme is a query parameter, which is how
 * the preview addresses them: `/components/button?theme=dark`. An address with
 * no component in it is the site's own front page.
 */
export async function openPreview(
  page: Page,
  address: Address = {},
  motion: Motion = "still",
): Promise<void> {
  await openDocumentation(
    page,
    {
      page:
        address.component === undefined
          ? undefined
          : {
              id: address.component,
              path: address.path ?? `/components/${address.component}`,
            },
      theme: address.theme,
    },
    motion,
  );
}

/**
 * Every address a screenshot is taken at: each component page under each
 * baseline theme.
 *
 * Component pages and their paths come from the Preview's rendered catalog,
 * including browser-enabled pages omitted from visible navigation. A Component
 * the Registry gains is therefore covered without this suite being touched.
 *
 * The themes are the ones the switcher marks as baselines rather than every
 * theme it offers: daisyUI ships thirty-five and the preview switches between
 * all of them, which would be a quarter of a gigabyte of images to review a
 * colour change in. The preview marks which three are baselines; this reads its
 * answer rather than keeping one.
 *
 * Theme IDs are read from `data-value` rather than text because labels may
 * change and the theme menu is closed; the attribute is readable either way.
 */
export async function addresses(page: Page): Promise<Required<Address>[]> {
  return (await documentationAddresses(page)).map((address) => ({
    component: address.page.id,
    path: address.page.path,
    theme: address.theme,
  }));
}

/**
 * Everything the preview renders for one axis, in the order its variant list
 * is in.
 *
 * A row marked with an axis renders one component per value of that axis and
 * nothing else, so its children are the variant list as the browser has it,
 * except that a component may render more than one element, and a primitive
 * that stands in for a form control renders a hidden input beside it. Those
 * are excluded here rather than in the preview, because they are the
 * primitive's to render and hiding them from the accessibility tree is exactly
 * what marks them as not the styled element.
 */
export function axis(page: Page, name: string): Locator {
  return page.locator(`[data-axis="${name}"] > *:not([aria-hidden="true"])`);
}

/**
 * The triggers the preview renders for an axis whose values cannot stand side
 * by side, one per value and in the same variant-list order.
 *
 * The dialog's position axis is what this exists for: every value of it is a
 * full-viewport modal, so a row of them would be six elements stacked on one
 * another rather than a rendered set to read. A spec opens them one at a time
 * and measures what each produced.
 *
 * Marked apart from `data-axis` rather than sharing it, because the two carry
 * different promises: a row of the one *is* the rendered set, and a row of this
 * one is only the way to reach it.
 */
export function axisTriggers(page: Page, name: string): Locator {
  return page.locator(`[data-axis-triggers="${name}"] > *:not([aria-hidden="true"])`);
}

/**
 * Asserts that every value of an axis renders differently from every other one,
 * in the property that axis drives.
 *
 * What this asserts of the rendered set is relational (every value differs
 * from every other) rather than a value per variant, because daisyUI owns what
 * a colour or a size is. Naming the numbers here would copy a theme this
 * registry deliberately defines none of, and would fail on a daisyUI release
 * that repaints one. What is being proved is that the class reached the
 * stylesheet and applies, and a variant that emitted nothing would show up as a
 * value shared with the one that emits no class at all.
 */
export async function expectAxisVaries(page: Page, name: string, property: string): Promise<void> {
  expectVaries(await computedStyle(axis(page, name), property), name);
}

/**
 * The same, for an axis whose variant list runs from the smallest value to the
 * largest: the rendered values are strictly increasing, which also says every
 * one of them differs from every other.
 */
export async function expectAxisGrows(page: Page, name: string, property: string): Promise<void> {
  expectGrows((await computedStyle(axis(page, name), property)).map(parseFloat), name);
}

/**
 * The assertion the two above are made of, over values a spec read for itself.
 *
 * An axis whose values are not a row of siblings, or which daisyUI expresses
 * across more than one property, is read by the spec that knows how, and then
 * says the same thing about what it read as every other axis does.
 */
export function expectVaries(values: unknown[], name: string): void {
  expect(values.length, `the ${name} axis rendered nothing`).toBeGreaterThan(1);
  expect(new Set(values).size, `two ${name} values render alike: ${values}`).toBe(values.length);
}

/** The same, for values a spec read off an axis that runs smallest to largest. */
export function expectGrows(values: number[], name: string): void {
  expectVaries(values, name);
  expect(values, `${name} values do not grow in variant-list order: ${values}`).toEqual(
    [...values].sort((a, b) => a - b),
  );
}
