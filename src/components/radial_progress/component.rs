use dioxus::core::AttributeValue;
use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::progress;

const MIN: f64 = 0.0;
const MAX: f64 = 100.0;

/// A circular progress indicator carrying daisyUI's `radial-progress` class.
///
/// The progress Primitive renders the `div[role=progressbar]`, owns its ARIA
/// attributes, and receives a fixed range of 0 through 100. This wrapper
/// normalizes the typed value into that range and publishes the same number as
/// daisyUI's unitless `--value`, so the ring, its head, and the accessibility
/// value cannot disagree.
///
/// Values below the range clamp to 0 and values above it clamp to 100. `NaN`
/// normalizes to 0; infinities clamp to the corresponding endpoint.
///
/// Caller classes concatenate with `radial-progress`. Caller inline styles are
/// preserved, including `--size` and `--thickness`, but the Registry appends an
/// important `--value` declaration because the typed `value` prop is
/// authoritative. Caller children render inside the ring as its visible label.
#[component]
pub fn RadialProgress(
    /// Progress in the component's fixed 0 through 100 range.
    value: ReadSignal<f64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let normalized = normalize(value());
    let mut attributes = attributes;
    let caller_style = take_style_and_filter_progress_attributes(&mut attributes);
    let style = compose_style(caller_style, normalized);

    let base = attributes!(div {
        class: "radial-progress",
        style: "{style}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        progress::Progress {
            value: Some(normalized),
            max: MAX,
            attributes: merged,
            {children}
        }
    }
}

fn normalize(value: f64) -> f64 {
    if value.is_nan() {
        MIN
    } else {
        value.clamp(MIN, MAX)
    }
}

/// Removes the caller's complete inline style so it can be composed instead of
/// replacing the Primitive's generated style when the attributes are spread.
/// Primitive-owned progress attributes are removed for the same reason: its
/// attribute spread comes last, but those semantics must continue to come from
/// the typed props passed above.
fn take_style_and_filter_progress_attributes(attributes: &mut Vec<Attribute>) -> String {
    let mut style = String::new();

    attributes.retain(|attribute| {
        if attribute.namespace.is_some() {
            return true;
        }

        if attribute.name.eq_ignore_ascii_case("style") {
            if let AttributeValue::Text(value) = &attribute.value {
                style.clone_from(value);
            }
            return false;
        }

        ![
            "aria-valuemax",
            "aria-valuemin",
            "aria-valuenow",
            "data-max",
            "data-state",
            "data-value",
            "role",
        ]
        .iter()
        .any(|owned| attribute.name.eq_ignore_ascii_case(owned))
    });

    style
}

fn compose_style(caller_style: String, value: f64) -> String {
    // The Primitive calculates this percentage, but its later attribute spread
    // replaces its complete style attribute with ours. Preserve that output
    // before adding the caller's declarations and the radial value.
    let mut style = format!("--progress-value: {value}%;");

    if !caller_style.trim().is_empty() {
        style.push(' ');
        style.push_str(&caller_style);
        if !caller_style.trim_end().ends_with(';') {
            style.push(';');
        }
    }

    style.push_str(&format!(" --value: {value} !important;"));
    style
}
