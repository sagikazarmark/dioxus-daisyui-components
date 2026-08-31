use dioxus::prelude::*;
use time::Date;
use time::macros::date;

use crate::components::date_picker::{
    DatePicker, DatePickerCalendar, DatePickerContent, DatePickerDaySegment, DatePickerInput,
    DatePickerInputValue, DatePickerMonthSegment, DatePickerPopover, DatePickerSeparator,
    DatePickerTrigger, DatePickerYearSegment,
};

/// The date drawn as today, and the month the calendar opens on.
///
/// Pinned rather than read from the clock, because this page is also what the
/// screenshots are taken of: a picker that opened on the real month would be a
/// different image every day. A picker that says nothing about it opens on the
/// current month with today marked.
pub const TODAY: Date = date!(2026 - 06 - 15);

/// A date typed into a field, or chosen from the calendar under it.
///
/// The three segments are typed into a digit at a time and stepped with the
/// arrow keys; the left and right arrows move between them. The button opens the
/// calendar, and choosing a day fills the field in and closes it. All of that is
/// the primitive's; what this component adds is that the field is daisyUI's
/// `input` and the days are daisyUI's buttons.
#[component]
pub fn Example() -> Element {
    let mut chosen = use_signal(|| Option::<Date>::None);

    let value = match chosen() {
        Some(date) => date.to_string(),
        None => "nothing".to_string(),
    };

    rsx! {
        div { class: "flex flex-col items-start gap-3 pb-80",
            DatePicker {
                id: "picker",
                selected_date: chosen(),
                on_value_change: move |date| chosen.set(date),
                DatePickerPopover {
                    DatePickerInput {
                        DatePickerInputValue {
                            DatePickerYearSegment {}
                            DatePickerSeparator {}
                            DatePickerMonthSegment {}
                            DatePickerSeparator {}
                            DatePickerDaySegment {}
                        }

                        DatePickerTrigger { id: "picker-trigger", aria_label: "Open the calendar",
                            "📅"
                        }
                    }

                    DatePickerContent { id: "picker-calendar",
                        DatePickerCalendar { today: TODAY }
                    }
                }
            }

            p { class: "text-sm opacity-70",
                "Chose "
                span { "data-testid": "chosen", "{value}" }
            }
        }
    }
}
