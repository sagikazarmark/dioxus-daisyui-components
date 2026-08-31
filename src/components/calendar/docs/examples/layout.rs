use dioxus::prelude::*;

use crate::components::calendar::{
    Calendar, CalendarButtonSize, CalendarGrid, CalendarGridAppearance, CalendarHeader,
    CalendarMonthTitle, CalendarNavigation, CalendarNavigationAppearance, CalendarNextMonthButton,
    CalendarPreviousMonthButton, CalendarTitleAppearance, CalendarView, CalendarViewAppearance,
};
use crate::examples::calendar::dates::{TODAY, VIEW};

/// The four parts that lay a month out, each drawn and each switched off.
///
/// None of these is a daisyUI class: daisyUI has no calendar (ADR-0022), so
/// what stacks a header over a grid, spaces the two month buttons, sets the
/// title apart and lines the days up is Tailwind utilities this component
/// emits. Every one of them therefore comes with a value that emits nothing,
/// which is how a caller wins against a utility rather than tying with it
/// (ADR-0004).
///
/// The grid's axis covers three elements at once (the table, the weekday
/// headings and the cells) because the grid renders all three itself.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8",
            div { "data-axis": "view-appearance", class: "flex flex-wrap items-start gap-6",
                for appearance in CalendarViewAppearance::ALL.iter().copied() {
                    Month {
                        view: appearance,
                        navigation: CalendarNavigationAppearance::Default,
                        title: CalendarTitleAppearance::Default,
                        grid: CalendarGridAppearance::Default,
                    }
                }
            }

            div {
                "data-axis": "navigation-appearance",
                class: "flex flex-wrap items-start gap-6",
                for appearance in CalendarNavigationAppearance::ALL.iter().copied() {
                    Month {
                        view: CalendarViewAppearance::Default,
                        navigation: appearance,
                        title: CalendarTitleAppearance::Default,
                        grid: CalendarGridAppearance::Default,
                    }
                }
            }

            div { "data-axis": "title-appearance", class: "flex flex-wrap items-start gap-6",
                for appearance in CalendarTitleAppearance::ALL.iter().copied() {
                    Month {
                        view: CalendarViewAppearance::Default,
                        navigation: CalendarNavigationAppearance::Default,
                        title: appearance,
                        grid: CalendarGridAppearance::Default,
                    }
                }
            }

            div { "data-axis": "grid-appearance", class: "flex flex-wrap items-start gap-6",
                for appearance in CalendarGridAppearance::ALL.iter().copied() {
                    Month {
                        view: CalendarViewAppearance::Default,
                        navigation: CalendarNavigationAppearance::Default,
                        title: CalendarTitleAppearance::Default,
                        grid: appearance,
                    }
                }
            }
        }
    }
}

/// One calendar, with all four of the axes this example varies passed through.
#[component]
fn Month(
    view: CalendarViewAppearance,
    navigation: CalendarNavigationAppearance,
    title: CalendarTitleAppearance,
    grid: CalendarGridAppearance,
) -> Element {
    rsx! {
        Calendar { view_date: VIEW, today: TODAY,
            CalendarView { appearance: view,
                CalendarHeader {
                    CalendarNavigation { appearance: navigation,
                        CalendarPreviousMonthButton { size: CalendarButtonSize::Sm, "‹" }
                        CalendarMonthTitle { appearance: title }
                        CalendarNextMonthButton { size: CalendarButtonSize::Sm, "›" }
                    }
                }
                CalendarGrid { appearance: grid }
            }
        }
    }
}
