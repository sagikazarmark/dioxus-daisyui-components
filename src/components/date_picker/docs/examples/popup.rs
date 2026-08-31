use dioxus::prelude::*;

use crate::components::date_picker::{
    DatePicker, DatePickerAlign, DatePickerCalendar, DatePickerCalendarAppearance,
    DatePickerContent, DatePickerContentAppearance, DatePickerDaySegment, DatePickerInput,
    DatePickerInputValue, DatePickerMonthSegment, DatePickerPositioning, DatePickerPopover,
    DatePickerSeparator, DatePickerSide, DatePickerTrigger, DatePickerYearSegment,
};
use crate::examples::date_picker::overview::TODAY;

/// Everything about the popup, one value of an axis per picker.
///
/// These rows are reached rather than rendered: a date picker's open state lives
/// in a context the primitive keeps to itself, so nothing here can hold a
/// calendar open the way the select's page holds a popup open. Each picker is
/// opened by its own button, which is also how a reader meets them.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8 pb-96",
            div {
                "data-axis-triggers": "side",
                class: "flex flex-wrap items-center justify-center gap-16",
                for side in DatePickerSide::ALL.iter().copied() {
                    Picker {
                        side,
                        align: DatePickerAlign::Start,
                        positioning: DatePickerPositioning::Default,
                        content: DatePickerContentAppearance::Default,
                        calendar: DatePickerCalendarAppearance::Default,
                    }
                }
            }

            div {
                "data-axis-triggers": "align",
                class: "flex flex-wrap items-center justify-center gap-16",
                for align in DatePickerAlign::ALL.iter().copied() {
                    Picker {
                        side: DatePickerSide::Bottom,
                        align,
                        positioning: DatePickerPositioning::Default,
                        content: DatePickerContentAppearance::Default,
                        calendar: DatePickerCalendarAppearance::Default,
                    }
                }
            }

            div {
                "data-axis-triggers": "positioning",
                class: "flex flex-wrap items-center justify-center gap-16",
                for positioning in DatePickerPositioning::ALL.iter().copied() {
                    Picker {
                        side: DatePickerSide::Bottom,
                        align: DatePickerAlign::Start,
                        positioning,
                        content: DatePickerContentAppearance::Default,
                        calendar: DatePickerCalendarAppearance::Default,
                    }
                }
            }

            div {
                "data-axis-triggers": "content-appearance",
                class: "flex flex-wrap items-center justify-center gap-16",
                for content in DatePickerContentAppearance::ALL.iter().copied() {
                    Picker {
                        side: DatePickerSide::Bottom,
                        align: DatePickerAlign::Start,
                        positioning: DatePickerPositioning::Default,
                        content,
                        calendar: DatePickerCalendarAppearance::Default,
                    }
                }
            }

            div {
                "data-axis-triggers": "calendar-appearance",
                class: "flex flex-wrap items-center justify-center gap-16",
                for calendar in DatePickerCalendarAppearance::ALL.iter().copied() {
                    Picker {
                        side: DatePickerSide::Bottom,
                        align: DatePickerAlign::Start,
                        positioning: DatePickerPositioning::Default,
                        content: DatePickerContentAppearance::Default,
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
    side: DatePickerSide,
    align: DatePickerAlign,
    positioning: DatePickerPositioning,
    content: DatePickerContentAppearance,
    calendar: DatePickerCalendarAppearance,
) -> Element {
    rsx! {
        DatePicker { side, align, selected_date: Some(TODAY),
            DatePickerPopover {
                DatePickerInput {
                    DatePickerInputValue {
                        DatePickerYearSegment {}
                        DatePickerSeparator {}
                        DatePickerMonthSegment {}
                        DatePickerSeparator {}
                        DatePickerDaySegment {}
                    }

                    DatePickerTrigger { aria_label: "Open the calendar", "📅" }
                }

                DatePickerContent { positioning, appearance: content,
                    DatePickerCalendar { appearance: calendar, today: TODAY }
                }
            }
        }
    }
}
