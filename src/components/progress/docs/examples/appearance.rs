use dioxus::prelude::*;

use crate::components::progress::{
    ProgressColor, ProgressIndicator, ProgressIndicatorAppearance, ProgressRoot,
};

/// Both values of the appearance axis, and what the second one is for.
///
/// daisyUI's own fill is a pseudo-element of a native `progress`, so on the
/// element the primitive renders there is no class that fills anything and the
/// utilities that draw it are the registry's (ADR-0012). Being the registry's,
/// they only tie with a caller's, so the axis switches them off rather than
/// asking a caller to out-rank them (ADR-0004). `None` leaves the track bare.
///
/// The last bar is the axis being used rather than shown: the utilities are off
/// and the fill is the caller's own, striped rather than solid, sized from the
/// same custom property the primitive publishes for ours.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex w-full max-w-sm flex-col gap-6",
            div { "data-axis": "appearance", class: "flex flex-col gap-3",
                for appearance in ProgressIndicatorAppearance::ALL.iter().copied() {
                    ProgressRoot {
                        color: ProgressColor::Primary,
                        value: Some(60.0),
                        aria_label: "{appearance:?}",
                        ProgressIndicator { appearance }
                    }
                }
            }

            ProgressRoot {
                color: ProgressColor::Primary,
                value: Some(60.0),
                aria_label: "A fill of the caller's own",
                ProgressIndicator {
                    appearance: ProgressIndicatorAppearance::None,
                    id: "caller-fill",
                    class: "block h-full w-[var(--progress-value,0%)] bg-[image:repeating-linear-gradient(45deg,currentColor_0_6px,transparent_6px_12px)]",
                }
            }
        }
    }
}
