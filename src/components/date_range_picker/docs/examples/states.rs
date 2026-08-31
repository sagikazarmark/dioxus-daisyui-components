use dioxus::prelude::*;
use dioxus_primitives::calendar::DateRange;
use time::macros::date;

use crate::components::date_range_picker::{
    DateRangePicker, DateRangePickerCalendar, DateRangePickerContent, DateRangePickerInput,
    DateRangePickerPopover, DateRangePickerPopoverAppearance, DateRangePickerTrigger,
};
use crate::examples::date_range_picker::overview::TODAY;

/// A picker that is disabled, one that will not be typed into, and one that only
/// accepts a fortnight.
///
/// The disabled one needs nothing emitted for the field: the primitive marks
/// every segment with the attribute this component's segment axis already fades.
/// The read-only one is the same picker with the calendar as the only way in;
/// the segments stay in the tab order and stay unwritable. The last has a
/// fortnight it accepts and three days inside it that it does not, which is what
/// the days in its calendar are drawn from.
///
/// The last picker also switches off the utilities that make the popover the box
/// the calendar is positioned against, which is the axis a caller reaches for
/// when they are positioning it themselves.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-start gap-8 pb-80",
            DateRangePicker {
                id: "disabled",
                disabled: true,
                selected_range: Some(DateRange::new(date!(2026 - 06 - 10), date!(2026 - 06 - 15))),
                DateRangePickerPopover {
                    DateRangePickerInput {
                        span { "2026-06-10 to 2026-06-15" }

                        DateRangePickerTrigger { aria_label: "Open the calendar", "📅" }
                    }
                }
            }

            DateRangePicker {
                id: "read-only",
                read_only: true,
                selected_range: Some(DateRange::new(date!(2026 - 06 - 10), date!(2026 - 06 - 15))),
                DateRangePickerPopover {
                    DateRangePickerInput {
                        span { "2026-06-10 to 2026-06-15" }

                        DateRangePickerTrigger { aria_label: "Open the calendar", "📅" }
                    }

                    DateRangePickerContent {
                        DateRangePickerCalendar { today: TODAY }
                    }
                }
            }

            DateRangePicker {
                id: "limited",
                selected_range: Some(DateRange::new(date!(2026 - 06 - 10), date!(2026 - 06 - 15))),
                min_date: date!(2026 - 06 - 08),
                max_date: date!(2026 - 06 - 21),
                disabled_ranges: vec![DateRange::new(date!(2026 - 06 - 17), date!(2026 - 06 - 19))],
                DateRangePickerPopover { appearance: DateRangePickerPopoverAppearance::None,
                    DateRangePickerInput {
                        span { "2026-06-10 to 2026-06-15" }

                        DateRangePickerTrigger { aria_label: "Open the calendar", "📅" }
                    }

                    DateRangePickerContent { id: "limited-calendar",
                        DateRangePickerCalendar { today: TODAY }
                    }
                }
            }
        }
    }
}
