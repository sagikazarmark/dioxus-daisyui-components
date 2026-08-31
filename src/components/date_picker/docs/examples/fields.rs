use dioxus::prelude::*;

use crate::components::date_picker::{
    DatePicker, DatePickerColor, DatePickerDaySegment, DatePickerInput, DatePickerInputValue,
    DatePickerMonthSegment, DatePickerPopover, DatePickerPopoverAppearance,
    DatePickerSegmentAppearance, DatePickerSeparator, DatePickerSize, DatePickerTrigger,
    DatePickerTriggerAppearance, DatePickerYearSegment,
};
use crate::examples::date_picker::overview::TODAY;

/// The field's colour and size, and the two axes on what is inside it.
///
/// The colour and the size are the daisyUI input's, on the element daisyUI wrote
/// them for. The segments and the trigger are this component's own: daisyUI has
/// no date picker, so what tells a segment from the text around it is utilities
/// and what the button looks like is daisyUI's own button classes.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8",
            div { "data-axis": "color", class: "flex flex-wrap items-center gap-4",
                for color in DatePickerColor::ALL.iter().copied() {
                    Field {
                        color,
                        size: DatePickerSize::Default,
                        segments: DatePickerSegmentAppearance::Default,
                        trigger: DatePickerTriggerAppearance::Default,
                        popover: DatePickerPopoverAppearance::Default,
                    }
                }
            }

            div { "data-axis": "size", class: "flex flex-wrap items-center gap-4",
                for size in DatePickerSize::ALL.iter().copied() {
                    Field {
                        color: DatePickerColor::Default,
                        size,
                        segments: DatePickerSegmentAppearance::Default,
                        trigger: DatePickerTriggerAppearance::Default,
                        popover: DatePickerPopoverAppearance::Default,
                    }
                }
            }

            div { "data-axis": "segment-appearance", class: "flex flex-wrap items-center gap-4",
                for segments in DatePickerSegmentAppearance::ALL.iter().copied() {
                    Field {
                        color: DatePickerColor::Default,
                        size: DatePickerSize::Default,
                        segments,
                        trigger: DatePickerTriggerAppearance::Default,
                        popover: DatePickerPopoverAppearance::Default,
                    }
                }
            }

            div { "data-axis": "trigger-appearance", class: "flex flex-wrap items-center gap-4",
                for trigger in DatePickerTriggerAppearance::ALL.iter().copied() {
                    Field {
                        color: DatePickerColor::Default,
                        size: DatePickerSize::Default,
                        segments: DatePickerSegmentAppearance::Default,
                        trigger,
                        popover: DatePickerPopoverAppearance::Default,
                    }
                }
            }

            // The popover's own axis, which is not paint at all: it is the two
            // utilities that make the element the calendar is positioned
            // against. Switched off, a caller is positioning the popup
            // themselves.
            div { "data-axis": "popover-appearance", class: "flex flex-wrap items-center gap-4",
                for popover in DatePickerPopoverAppearance::ALL.iter().copied() {
                    Field {
                        color: DatePickerColor::Default,
                        size: DatePickerSize::Default,
                        segments: DatePickerSegmentAppearance::Default,
                        trigger: DatePickerTriggerAppearance::Default,
                        popover,
                    }
                }
            }
        }
    }
}

/// One picker's field, with the axes this example varies on it.
///
/// The calendar is left out: what these rows vary is the field, and a picker
/// with no popup in it is still a picker with a field.
#[component]
fn Field(
    color: DatePickerColor,
    size: DatePickerSize,
    segments: DatePickerSegmentAppearance,
    trigger: DatePickerTriggerAppearance,
    popover: DatePickerPopoverAppearance,
) -> Element {
    rsx! {
        DatePicker { selected_date: Some(TODAY),
            DatePickerPopover { appearance: popover,
                DatePickerInput { color, size,
                    DatePickerInputValue {
                        DatePickerYearSegment { appearance: segments }
                        DatePickerSeparator { appearance: segments }
                        DatePickerMonthSegment { appearance: segments }
                        DatePickerSeparator { appearance: segments }
                        DatePickerDaySegment { appearance: segments }
                    }

                    DatePickerTrigger { appearance: trigger, aria_label: "Open the calendar", "📅" }
                }
            }
        }
    }
}
