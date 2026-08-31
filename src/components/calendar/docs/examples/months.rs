use dioxus::prelude::*;

use crate::components::calendar::{
    Calendar, CalendarButtonSize, CalendarGrid, CalendarHeader, CalendarMonthTitle,
    CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton, CalendarView,
};
use crate::examples::calendar::dates::{SELECTED, TODAY, VIEW};

/// Two months side by side, which is what the view part is for.
///
/// Each view takes its offset from the order it is rendered in, so the second
/// shows the month after the first. One value and one set of arrow keys cover
/// both, and either month button moves the pair: they change the month the
/// calendar is on, and the second view is always one past it.
///
/// The buttons are split across the two headers, which is where a reader
/// expects them: back on the left of the first month, forward on the right of
/// the second. Each has to sit inside a view, because what a month button knows
/// about the ends of the range it may navigate it reads from the view it is in.
#[component]
pub fn Example() -> Element {
    rsx! {
        Calendar {
            id: "months",
            view_date: VIEW,
            today: TODAY,
            selected_date: Some(SELECTED),
            div { class: "flex flex-wrap gap-6",
                CalendarView {
                    CalendarHeader {
                        CalendarNavigation {
                            CalendarPreviousMonthButton { size: CalendarButtonSize::Sm, "‹" }
                            CalendarMonthTitle {}
                            span {}
                        }
                    }
                    CalendarGrid {}
                }

                CalendarView {
                    CalendarHeader {
                        CalendarNavigation {
                            span {}
                            CalendarMonthTitle {}
                            CalendarNextMonthButton { size: CalendarButtonSize::Sm, "›" }
                        }
                    }
                    CalendarGrid {}
                }
            }
        }
    }
}
