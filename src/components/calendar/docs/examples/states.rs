use dioxus::prelude::*;
use dioxus_primitives::calendar::DateRange;
use time::macros::date;

use crate::components::calendar::{
    Calendar, CalendarButtonSize, CalendarGrid, CalendarHeader, CalendarMonthTitle,
    CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton, CalendarView,
};
use crate::examples::calendar::dates::{SELECTED, TODAY, VIEW};

/// A calendar with days it will not let anybody near, and one that is disabled
/// outright.
///
/// The first has a week marked unavailable and a range it cannot navigate out
/// of: the month buttons go inert at the ends of it, which is the primitive
/// putting the native `disabled` attribute on a `btn` and daisyUI's own rule
/// picking it up. The second is disabled as a whole, which does the same to
/// every day in it.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-start gap-8",
            Calendar {
                id: "unavailable",
                view_date: VIEW,
                today: TODAY,
                selected_date: Some(SELECTED),
                min_date: date!(2026 - 05 - 01),
                max_date: date!(2026 - 07 - 31),
                disabled_ranges: vec![DateRange::new(date!(2026 - 06 - 17), date!(2026 - 06 - 21))],
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

            Calendar {
                id: "disabled",
                view_date: VIEW,
                today: TODAY,
                selected_date: Some(SELECTED),
                disabled: true,
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
}
