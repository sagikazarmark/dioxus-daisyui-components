use dioxus::prelude::*;

use crate::components::calendar::{
    Calendar, CalendarButtonAppearance, CalendarButtonColor, CalendarButtonSize, CalendarHeader,
    CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton, CalendarView,
};
use crate::examples::calendar::dates::{TODAY, VIEW};

/// The month buttons' three axes, read off the button that goes back a month.
///
/// They are the daisyUI button's own axes, duplicated here because the registry
/// uses no cross-component dependencies, and they are on the buttons rather
/// than on the calendar, because that is the element daisyUI's classes are for.
/// The grid is left out of these rows: what they vary is the header.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8",
            div { "data-axis": "button-color", class: "flex flex-wrap items-center gap-3",
                for color in CalendarButtonColor::ALL.iter().copied() {
                    Header {
                        color,
                        size: CalendarButtonSize::Default,
                        appearance: CalendarButtonAppearance::None,
                    }
                }
            }

            div { "data-axis": "button-size", class: "flex flex-wrap items-center gap-3",
                for size in CalendarButtonSize::ALL.iter().copied() {
                    Header {
                        color: CalendarButtonColor::Default,
                        size,
                        appearance: CalendarButtonAppearance::Default,
                    }
                }
            }

            div { "data-axis": "button-appearance", class: "flex flex-wrap items-center gap-3",
                for appearance in CalendarButtonAppearance::ALL.iter().copied() {
                    Header {
                        color: CalendarButtonColor::Default,
                        size: CalendarButtonSize::Sm,
                        appearance,
                    }
                }
            }
        }
    }
}

/// One calendar's header, with the axes this example varies on its buttons.
#[component]
fn Header(
    color: CalendarButtonColor,
    size: CalendarButtonSize,
    appearance: CalendarButtonAppearance,
) -> Element {
    rsx! {
        Calendar { view_date: VIEW, today: TODAY,
            CalendarView {
                CalendarHeader {
                    CalendarNavigation {
                        CalendarPreviousMonthButton { color, size, appearance, "‹" }
                        CalendarNextMonthButton { color, size, appearance, "›" }
                    }
                }
            }
        }
    }
}
