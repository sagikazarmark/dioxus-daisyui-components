use dioxus::prelude::*;

use crate::components::calendar::{
    Calendar, CalendarAppearance, CalendarButtonSize, CalendarGrid, CalendarHeader,
    CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton,
    CalendarView,
};
use crate::examples::calendar::dates::{TODAY, VIEW};

/// The box the month sits in, drawn and switched off.
///
/// daisyUI has no calendar class to put here, so what draws the box is
/// utilities (a fill, daisyUI's own box radius, a border and some padding),
/// and a utility this component emits is one a caller can only tie with. So the
/// way to restyle the box is to switch this off and write your own (ADR-0004).
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "appearance", class: "flex flex-wrap items-start gap-6",
            for appearance in CalendarAppearance::ALL.iter().copied() {
                Calendar { appearance, view_date: VIEW, today: TODAY,
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
}
