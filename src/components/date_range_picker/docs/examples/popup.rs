use dioxus::prelude::*;
use dioxus_primitives::calendar::DateRange;
use time::macros::date;

use crate::components::date_range_picker::{
    DateRangePicker, DateRangePickerAlign, DateRangePickerCalendar,
    DateRangePickerCalendarAppearance, DateRangePickerContent, DateRangePickerContentAppearance,
    DateRangePickerInput, DateRangePickerPositioning, DateRangePickerPopover, DateRangePickerSide,
    DateRangePickerTrigger,
};
use crate::examples::date_range_picker::overview::TODAY;

/// Everything about the popup, one value of an axis per picker.
///
/// These rows are reached rather than rendered: a picker's open state lives in a
/// context the primitive keeps to itself, so nothing here can hold a calendar
/// open the way the select's page holds a popup open. Each picker is opened by
/// its own button, which is also how a reader meets them.
///
/// Every one of them starts on a range, so the calendar that opens shows the two
/// ends and the days between them painted apart.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8 pb-96",
            div {
                "data-axis-triggers": "side",
                class: "flex flex-wrap items-center justify-center gap-16",
                for side in DateRangePickerSide::ALL.iter().copied() {
                    Picker {
                        side,
                        align: DateRangePickerAlign::Start,
                        positioning: DateRangePickerPositioning::Default,
                        content: DateRangePickerContentAppearance::Default,
                        calendar: DateRangePickerCalendarAppearance::Default,
                    }
                }
            }

            div {
                "data-axis-triggers": "align",
                class: "flex flex-wrap items-center justify-center gap-16",
                for align in DateRangePickerAlign::ALL.iter().copied() {
                    Picker {
                        side: DateRangePickerSide::Bottom,
                        align,
                        positioning: DateRangePickerPositioning::Default,
                        content: DateRangePickerContentAppearance::Default,
                        calendar: DateRangePickerCalendarAppearance::Default,
                    }
                }
            }

            div {
                "data-axis-triggers": "positioning",
                class: "flex flex-wrap items-center justify-center gap-16",
                for positioning in DateRangePickerPositioning::ALL.iter().copied() {
                    Picker {
                        side: DateRangePickerSide::Bottom,
                        align: DateRangePickerAlign::Start,
                        positioning,
                        content: DateRangePickerContentAppearance::Default,
                        calendar: DateRangePickerCalendarAppearance::Default,
                    }
                }
            }

            div {
                "data-axis-triggers": "content-appearance",
                class: "flex flex-wrap items-center justify-center gap-16",
                for content in DateRangePickerContentAppearance::ALL.iter().copied() {
                    Picker {
                        side: DateRangePickerSide::Bottom,
                        align: DateRangePickerAlign::Start,
                        positioning: DateRangePickerPositioning::Default,
                        content,
                        calendar: DateRangePickerCalendarAppearance::Default,
                    }
                }
            }

            div {
                "data-axis-triggers": "calendar-appearance",
                class: "flex flex-wrap items-center justify-center gap-16",
                for calendar in DateRangePickerCalendarAppearance::ALL.iter().copied() {
                    Picker {
                        side: DateRangePickerSide::Bottom,
                        align: DateRangePickerAlign::Start,
                        positioning: DateRangePickerPositioning::Default,
                        content: DateRangePickerContentAppearance::Default,
                        calendar,
                    }
                }
            }
        }
    }
}

/// One picker, with all five of the axes these rows vary passed through.
#[component]
fn Picker(
    side: DateRangePickerSide,
    align: DateRangePickerAlign,
    positioning: DateRangePickerPositioning,
    content: DateRangePickerContentAppearance,
    calendar: DateRangePickerCalendarAppearance,
) -> Element {
    rsx! {
        DateRangePicker {
            side,
            align,
            selected_range: Some(DateRange::new(date!(2026 - 06 - 10), date!(2026 - 06 - 15))),
            DateRangePickerPopover {
                DateRangePickerInput {
                    span { "2026-06-10 to 2026-06-15" }

                    DateRangePickerTrigger { aria_label: "Open the calendar", "📅" }
                }

                DateRangePickerContent { positioning, appearance: content,
                    DateRangePickerCalendar { appearance: calendar, today: TODAY }
                }
            }
        }
    }
}
