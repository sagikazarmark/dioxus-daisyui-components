use dioxus::prelude::*;
use dioxus_primitives::calendar::DateRange;
use time::macros::date;

use crate::components::calendar::{
    CalendarButtonSize, CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation,
    CalendarNextMonthButton, CalendarPreviousMonthButton, CalendarView, RangeCalendar,
};
use crate::examples::calendar::dates::{TODAY, VIEW};

/// The same calendar choosing a run of days rather than one.
///
/// The first click sets one end and the second sets the other; between them the
/// range follows the pointer. Every day of the range is painted as chosen, and
/// the ones between the two ends are painted differently again and squared off,
/// so the run reads as one band: three attributes the primitive sets, three
/// variants this component emits.
///
/// The first calendar is held on a range so that the three looks can be seen at
/// once; the second keeps its own.
#[component]
pub fn Example() -> Element {
    let mut range = use_signal(|| Option::<DateRange>::None);

    let chosen = match range() {
        Some(range) => format!("{} to {}", range.start(), range.end()),
        None => "nothing".to_string(),
    };

    rsx! {
        div { class: "flex flex-wrap items-start gap-8",
            RangeCalendar {
                id: "range-fixed",
                view_date: VIEW,
                today: TODAY,
                selected_range: Some(DateRange::new(date!(2026 - 06 - 08), date!(2026 - 06 - 14))),
                CalendarView {
                    CalendarHeader {
                        CalendarNavigation {
                            CalendarPreviousMonthButton { size: CalendarButtonSize::Sm, "‹" }
                            CalendarMonthTitle {}
                            CalendarNextMonthButton { size: CalendarButtonSize::Sm, "›" }
                        }
                    }
                    CalendarGrid {}
                }
            }

            div { class: "flex flex-col items-start gap-3",
                RangeCalendar {
                    id: "range-live",
                    view_date: VIEW,
                    today: TODAY,
                    selected_range: range(),
                    on_range_change: move |next| range.set(next),
                    CalendarView {
                        CalendarHeader {
                            CalendarNavigation {
                                CalendarPreviousMonthButton { size: CalendarButtonSize::Sm, "‹" }
                                CalendarMonthTitle {}
                                CalendarNextMonthButton { size: CalendarButtonSize::Sm, "›" }
                            }
                        }
                        CalendarGrid {}
                    }
                }

                p { class: "text-sm opacity-70",
                    "Chose "
                    span { "data-testid": "range", "{chosen}" }
                }
            }
        }
    }
}
