use dioxus::prelude::*;

use crate::components::calendar::{
    Calendar, CalendarButtonSize, CalendarGrid, CalendarHeader, CalendarMonthTitle,
    CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton, CalendarView,
};
use crate::examples::calendar::dates::{TODAY, VIEW};

/// A calendar that answers to nobody: it holds the date it is on and the month
/// it is showing.
///
/// A click chooses a day and a second click on the same day clears it. The
/// arrow keys walk the grid a day at a time, Shift with them jumps a month, and
/// the two buttons change the month without touching the choice. All of that is
/// the primitive's; what this component adds is that a day is a daisyUI button
/// and the day that is chosen is painted (ADR-0022).
#[component]
pub fn Example() -> Element {
    let mut chosen = use_signal(|| Option::<time::Date>::None);

    rsx! {
        div { class: "flex flex-col items-start gap-3",
            Calendar {
                view_date: VIEW,
                today: TODAY,
                on_date_change: move |date| chosen.set(date),
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
                match chosen() {
                    Some(date) => format!("Chose {date}"),
                    None => "Nothing chosen".to_string(),
                }
            }
        }
    }
}
