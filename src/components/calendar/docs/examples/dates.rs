use dioxus::prelude::*;
use time::Date;
use time::macros::date;

use crate::components::calendar::{
    Calendar, CalendarButtonSize, CalendarGrid, CalendarHeader, CalendarMonthTitle,
    CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton, CalendarView,
};

/// The month every calendar on this page opens on.
///
/// Every example here pins the month, the date drawn as today and the date it
/// starts on, rather than reading the clock, because this page is also what the
/// screenshots are taken of, and a calendar that opened on the real month would
/// be a different image every day. A calendar that says nothing about any of
/// them opens on the current month with today marked, which is the primitive's
/// default and this component's.
pub const VIEW: Date = date!(2026 - 06 - 01);

/// The date drawn as today, which is the one the ring is on.
pub const TODAY: Date = date!(2026 - 06 - 15);

/// The date the calendars that start on one start on.
pub const SELECTED: Date = date!(2026 - 06 - 10);

/// Those dates in a calendar, which is the composition every other example on
/// this page varies one part of.
#[component]
pub fn Example() -> Element {
    rsx! {
        Calendar {
            view_date: VIEW,
            today: TODAY,
            selected_date: Some(SELECTED),
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
    }
}
