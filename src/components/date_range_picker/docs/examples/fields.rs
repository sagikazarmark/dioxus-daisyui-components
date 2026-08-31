use dioxus::prelude::*;
use dioxus_primitives::calendar::DateRange;
use time::macros::date;

use crate::components::date_range_picker::{
    DateRangePicker, DateRangePickerColor, DateRangePickerInput, DateRangePickerPopover,
    DateRangePickerPopoverAppearance, DateRangePickerSize, DateRangePickerTrigger,
    DateRangePickerTriggerAppearance,
};

/// The field's colour and size, the button inside it, and the box the popup is
/// positioned against.
///
/// The colour and the size are the daisyUI input's, on the element daisyUI wrote
/// them for. What the field *holds* is the caller's: the range, written however
/// they write a date, because the primitive's segmented range input cannot be
/// used yet; see the component's documentation.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8",
            div {
                "data-axis": "color",
                class: "flex flex-wrap items-center gap-4",
                for color in DateRangePickerColor::ALL.iter().copied() {
                    Field {
                        color,
                        size: DateRangePickerSize::Default,
                        trigger: DateRangePickerTriggerAppearance::Default,
                        popover: DateRangePickerPopoverAppearance::Default,
                    }
                }
            }

            div {
                "data-axis": "size",
                class: "flex flex-wrap items-center gap-4",
                for size in DateRangePickerSize::ALL.iter().copied() {
                    Field {
                        color: DateRangePickerColor::Default,
                        size,
                        trigger: DateRangePickerTriggerAppearance::Default,
                        popover: DateRangePickerPopoverAppearance::Default,
                    }
                }
            }

            div {
                "data-axis": "trigger-appearance",
                class: "flex flex-wrap items-center gap-4",
                for trigger in DateRangePickerTriggerAppearance::ALL.iter().copied() {
                    Field {
                        color: DateRangePickerColor::Default,
                        size: DateRangePickerSize::Default,
                        trigger,
                        popover: DateRangePickerPopoverAppearance::Default,
                    }
                }
            }

            // The popover's own axis, which is not paint at all: it is the two
            // utilities that make the element the calendar is positioned
            // against. Switched off, a caller is positioning the popup
            // themselves.
            div {
                "data-axis": "popover-appearance",
                class: "flex flex-wrap items-center gap-4",
                for popover in DateRangePickerPopoverAppearance::ALL.iter().copied() {
                    Field {
                        color: DateRangePickerColor::Default,
                        size: DateRangePickerSize::Default,
                        trigger: DateRangePickerTriggerAppearance::Default,
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
    color: DateRangePickerColor,
    size: DateRangePickerSize,
    trigger: DateRangePickerTriggerAppearance,
    popover: DateRangePickerPopoverAppearance,
) -> Element {
    rsx! {
        DateRangePicker {
            selected_range: Some(DateRange::new(date!(2026 - 06 - 10), date!(2026 - 06 - 15))),
            DateRangePickerPopover { appearance: popover,
                DateRangePickerInput { color, size,
                    span { "2026-06-10 to 2026-06-15" }

                    DateRangePickerTrigger { appearance: trigger, aria_label: "Open the calendar", "📅" }
                }
            }
        }
    }
}
