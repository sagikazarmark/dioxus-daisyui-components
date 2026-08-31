use dioxus::core::AttributeValue;
use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

/// daisyUI's minimum-digit Axis for a countdown value.
///
/// [`CountdownDigits::Natural`] writes no `--digits` custom property, so the
/// value takes its natural width. The other values reserve two or three digits
/// and display leading zeroes when needed.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum CountdownDigits {
    #[default]
    Natural,
    Two,
    Three,
}

impl CountdownDigits {
    /// Every value of this Axis, in the order the Preview renders them.
    pub const ALL: &'static [Self] = &[Self::Natural, Self::Two, Self::Three];

    const fn custom_property(self) -> Option<u8> {
        match self {
            Self::Natural => None,
            Self::Two => Some(2),
            Self::Three => Some(3),
        }
    }
}

/// The outer wrapper carrying daisyUI's `countdown` class.
///
/// Put one or more [`CountdownValue`] parts directly inside this element.
/// Text separators may sit between them. Classes passed by the caller
/// concatenate with `countdown`; every other caller attribute overrides the
/// component's.
#[component]
pub fn Countdown(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(span { class: "countdown" });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        span { ..merged, {children} }
    }
}

/// A numeric value directly inside a [`Countdown`].
///
/// `value` is clamped to `0..=999` once per render. That one normalized integer
/// supplies daisyUI's `--value`, the text node, and the default `aria-label`, so
/// reactive updates cannot leave the three representations out of step.
///
/// The value is a polite live region by default. A caller may override
/// `aria-label`, `aria-live`, and other global attributes. Caller inline styles
/// are preserved except for `--value` and `--digits`, which are owned by the
/// typed props and remain authoritative.
#[component]
pub fn CountdownValue(
    /// The integer daisyUI displays. Values below zero become zero; values over
    /// 999 become 999.
    value: ReadSignal<i32>,
    /// The minimum number of digits daisyUI displays.
    #[props(default)]
    digits: CountdownDigits,
    #[props(extends = GlobalAttributes)] mut attributes: Vec<Attribute>,
) -> Element {
    let normalized = value().clamp(0, 999);
    let caller_style = take_inline_style(&mut attributes);
    let style = countdown_style(&caller_style, normalized, digits);

    let base = attributes!(span {
        style,
        aria_live: "polite",
        aria_label: "{normalized}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        span { ..merged, "{normalized}" }
    }
}

/// Removes ordinary `style` attributes so textual caller declarations and the
/// component-owned custom properties can be emitted as one declaration block.
/// HTML attribute names are ASCII case-insensitive; consuming value-less style
/// attributes keeps them from erasing that block during attribute merging.
fn take_inline_style(attributes: &mut Vec<Attribute>) -> String {
    let mut styles = Vec::new();
    let mut index = 0;

    while index < attributes.len() {
        let is_inline_style = attributes[index].name.eq_ignore_ascii_case("style")
            && attributes[index].namespace.is_none();

        if is_inline_style {
            if let AttributeValue::Text(style) = attributes.remove(index).value {
                styles.push(style);
            }
        } else {
            index += 1;
        }
    }

    styles.join("; ")
}

fn countdown_style(caller_style: &str, value: i32, digits: CountdownDigits) -> String {
    let mut style = without_owned_properties(caller_style);

    if !style.is_empty() && !style.ends_with(';') {
        style.push(';');
    }
    if !style.is_empty() {
        style.push(' ');
    }

    style.push_str(&format!("--value: {value};"));
    if let Some(digits) = digits.custom_property() {
        style.push_str(&format!(" --digits: {digits};"));
    }

    style
}

/// Drops only top-level declarations owned by the Component. Semicolons in a
/// quoted value or CSS function remain part of the caller's declaration.
fn without_owned_properties(style: &str) -> String {
    let mut kept = Vec::new();
    let mut start = 0;
    let mut quote = None;
    let mut escaped = false;
    let mut parentheses = 0_u32;

    for (index, character) in style.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }

        if quote.is_some() && character == '\\' {
            escaped = true;
            continue;
        }

        if let Some(delimiter) = quote {
            if character == delimiter {
                quote = None;
            }
            continue;
        }

        match character {
            '\'' | '"' => quote = Some(character),
            '(' => parentheses += 1,
            ')' => parentheses = parentheses.saturating_sub(1),
            ';' if parentheses == 0 => {
                keep_declaration(&mut kept, &style[start..index]);
                start = index + character.len_utf8();
            }
            _ => {}
        }
    }

    keep_declaration(&mut kept, &style[start..]);
    kept.join("; ")
}

fn keep_declaration<'a>(kept: &mut Vec<&'a str>, declaration: &'a str) {
    let declaration = declaration.trim();
    if declaration.is_empty() {
        return;
    }

    let property = declaration
        .split_once(':')
        .map_or(declaration, |(property, _)| property)
        .trim();
    if !is_owned_property(property) {
        kept.push(declaration);
    }
}

fn is_owned_property(property: &str) -> bool {
    let mut normalized = String::with_capacity(property.len());
    let mut characters = property.chars().peekable();

    while let Some(character) = characters.next() {
        if character == '/' && characters.peek() == Some(&'*') {
            characters.next();
            while let Some(character) = characters.next() {
                if character == '*' && characters.peek() == Some(&'/') {
                    characters.next();
                    break;
                }
            }
        } else if !character.is_ascii_whitespace() {
            normalized.push(character);
        }
    }

    matches!(normalized.as_str(), "--value" | "--digits")
}
