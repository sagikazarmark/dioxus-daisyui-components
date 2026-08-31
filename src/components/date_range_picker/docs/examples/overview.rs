use dioxus::prelude::*;
use dioxus_primitives::calendar::DateRange;
use time::Date;
use time::macros::date;

use crate::components::date_range_picker::{
    DateRangePicker, DateRangePickerCalendar, DateRangePickerContent, DateRangePickerInput,
    DateRangePickerPopover, DateRangePickerTrigger,
};

/// The date drawn as today, and the month the calendar opens on.
///
/// Pinned rather than read from the clock, because this page is also what the
/// screenshots are taken of.
pub const TODAY: Date = date!(2026 - 06 - 15);

/// A run of days chosen from a calendar, shown in a field the caller writes.
///
/// The first click sets one end of the range and the second sets the other; the
/// days between them are painted as one band, and the popup closes once both
/// ends are known.
///
/// **What the field shows is the caller's**, because the primitive's segmented
/// range input cannot be used yet; see the component's documentation. The
/// field here is daisyUI's `input` around a line of text and the button that
/// opens the calendar.
#[component]
pub fn Example() -> Element {
    let mut chosen = use_signal(|| Option::<DateRange>::None);

    let value = match chosen() {
        Some(range) => format!("{} to {}", range.start(), range.end()),
        None => "nothing".to_string(),
    };

    rsx! {
        div { class: "flex flex-col items-start gap-3 pb-80",
            DateRangePicker {
                id: "picker",
                selected_range: chosen(),
                on_range_change: move |range| chosen.set(range),
                DateRangePickerPopover {
                    DateRangePickerInput {
                        span { "{value}" }

                        DateRangePickerTrigger {
                            id: "picker-trigger",
                            aria_label: "Open the calendar",
                            "📅"
                        }
                    }

                    DateRangePickerContent { id: "picker-calendar",
                        DateRangePickerCalendar { today: TODAY }
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
