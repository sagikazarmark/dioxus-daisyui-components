use dioxus::prelude::*;
use time::Date;

use crate::components::calendar::{
    Calendar, CalendarButtonSize, CalendarGrid, CalendarHeader, CalendarMonthTitle,
    CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton, CalendarView,
};
use crate::examples::calendar::dates::{TODAY, VIEW};

/// A calendar whose chosen date and month on show both belong to its caller.
///
/// Nothing here is lifted the way a dropdown's open state is (ADR-0006): the
/// primitive owns both either way, and every class this component emits for a
/// state is read off an attribute rather than off state it keeps. What a
/// controlled calendar changes is who decides: the month moves only because
/// the caller wrote the new one back, and a day is chosen only because they
/// took the callback's word for it.
#[component]
pub fn Example() -> Element {
    let mut selected = use_signal(|| Option::<Date>::None);
    let mut view = use_signal(|| VIEW);

    let chosen = match selected() {
        Some(date) => date.to_string(),
        None => "nothing".to_string(),
    };

    rsx! {
        div { class: "flex flex-col items-start gap-3",
            Calendar {
                id: "controlled",
                today: TODAY,
                selected_date: selected(),
                on_date_change: move |date| selected.set(date),
                view_date: view(),
                on_view_change: move |date| view.set(date),
                CalendarView {
                    CalendarHeader {
                        CalendarNavigation {
                            CalendarPreviousMonthButton {
                                id: "previous-month",
                                size: CalendarButtonSize::Sm,
                                "‹"
                            }
                            CalendarMonthTitle { id: "month-title" }
                            CalendarNextMonthButton {
                                id: "next-month",
                                size: CalendarButtonSize::Sm,
                                "›"
                            }
                        }
                    }
                    CalendarGrid {}
                }
            }

            p { class: "text-sm opacity-70",
                "Showing "
                span { "data-testid": "view", "{view()}" }
                ", chose "
                span { "data-testid": "selected", "{chosen}" }
            }
        }
    }
}
