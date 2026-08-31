use dioxus::prelude::*;

use crate::components::calendar::{
    Calendar, CalendarDayAppearance, CalendarDaySize, CalendarGrid, CalendarHeader,
    CalendarMonthTitle, CalendarNavigation, CalendarView,
};
use crate::examples::calendar::dates::{SELECTED, TODAY, VIEW};

/// The day's two axes: the daisyUI button size every day is drawn at, and
/// whether a day is painted from what it is at all.
///
/// The second is every state at once (chosen, today, the neighbouring months,
/// the days that cannot be chosen) because each of them is a variant of an
/// attribute the primitive sets rather than a class of daisyUI's own, and they
/// are switched off together (ADR-0022).
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8",
            div { "data-axis": "day-size", class: "flex flex-wrap items-start gap-4",
                for size in CalendarDaySize::ALL.iter().copied() {
                    Month { day_size: size, day_appearance: CalendarDayAppearance::Default }
                }
            }

            div { "data-axis": "day-appearance", class: "flex flex-wrap items-start gap-4",
                for appearance in CalendarDayAppearance::ALL.iter().copied() {
                    Month { day_size: CalendarDaySize::Sm, day_appearance: appearance }
                }
            }
        }
    }
}

/// One calendar, with the axes this example varies passed through to its grid.
#[component]
fn Month(day_size: CalendarDaySize, day_appearance: CalendarDayAppearance) -> Element {
    rsx! {
        Calendar {
            view_date: VIEW,
            today: TODAY,
            selected_date: Some(SELECTED),
            CalendarView {
                CalendarHeader {
                    CalendarNavigation {
                        CalendarMonthTitle {}
                    }
                }
                CalendarGrid { day_size, day_appearance }
            }
        }
    }
}
