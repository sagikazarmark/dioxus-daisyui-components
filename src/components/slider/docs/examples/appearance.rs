use dioxus::prelude::*;

use crate::components::slider::{
    SliderAppearance, SliderRange, SliderRangeAppearance, SliderRoot, SliderThumb,
    SliderThumbAppearance, SliderTrack, SliderTrackAppearance,
};

/// All four appearance axes, which between them are every utility this component
/// emits where daisyUI's own declarations cannot reach (ADR-0016).
///
/// The root's lays the parts out and mutes a disabled slider; the track's draws
/// the groove; the range's draws the fill; the thumb's draws the handle. Each
/// `None` emits nothing at all, which is how a caller wins a tie against a
/// utility rather than trying to out-rank it (ADR-0004), and switching one off
/// is also the only honest way to see what daisyUI's class does and does not
/// supply on its own.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            div { "data-axis": "root", class: "flex flex-wrap items-center gap-4",
                for appearance in SliderAppearance::ALL.iter().copied() {
                    // Each one is wrapped in a box of its own that is positioned,
                    // which is what the axis hands over: with the root's
                    // utilities switched off, the parts are positioned against
                    // whatever ancestor is, and with no ancestor at all they
                    // would be laid out against the viewport.
                    div { class: "relative w-40",
                        SliderRoot {
                            appearance,
                            label: "Root: {appearance:?}",
                            default_value: 60.0,
                            class: "w-40",
                            SliderTrack {
                                SliderRange {}
                                SliderThumb {}
                            }
                        }
                    }
                }
            }

            div { "data-axis": "track", class: "flex flex-wrap items-center gap-4",
                for appearance in SliderTrackAppearance::ALL.iter().copied() {
                    SliderRoot {
                        label: "Track: {appearance:?}",
                        default_value: 60.0,
                        class: "w-40",
                        SliderTrack { appearance,
                            SliderRange {}
                            SliderThumb {}
                        }
                    }
                }
            }

            div { "data-axis": "range", class: "flex flex-wrap items-center gap-4",
                for appearance in SliderRangeAppearance::ALL.iter().copied() {
                    SliderRoot {
                        label: "Fill: {appearance:?}",
                        default_value: 60.0,
                        class: "w-40",
                        SliderTrack {
                            SliderRange { appearance }
                            SliderThumb {}
                        }
                    }
                }
            }

            div { "data-axis": "thumb", class: "flex flex-wrap items-center gap-4",
                for appearance in SliderThumbAppearance::ALL.iter().copied() {
                    SliderRoot {
                        label: "Handle: {appearance:?}",
                        default_value: 60.0,
                        class: "w-40",
                        SliderTrack {
                            SliderRange {}
                            SliderThumb { appearance }
                        }
                    }
                }
            }
        }
    }
}
