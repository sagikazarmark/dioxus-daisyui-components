use dioxus::prelude::*;
use dioxus_primitives::calendar::DateRange;
use time::macros::date;

use crate::components::date_picker::{
    DatePicker, DatePickerCalendar, DatePickerContent, DatePickerDaySegment, DatePickerInput,
    DatePickerInputValue, DatePickerMonthSegment, DatePickerPopover, DatePickerPopoverAppearance,
    DatePickerSeparator, DatePickerTrigger, DatePickerYearSegment,
};
use crate::examples::date_picker::overview::TODAY;

/// A picker that is disabled, one that will not be typed into, and one that
/// only accepts a fortnight.
///
/// The disabled one needs nothing emitted for the field: the primitive marks
/// every segment with the attribute this component's segment axis already
/// fades. The read-only one is the same picker with the calendar as the only way
/// in: the segments stay in the tab order and stay unwritable. The last has a
/// range it accepts and a week inside it that it does not, which is what the
/// days in its calendar are drawn from.
///
/// The last picker also switches off the utilities that make the popover the box
/// the calendar is positioned against, which is the axis a caller reaches for
/// when they are positioning it themselves.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-start gap-8 pb-80",
            DatePicker { id: "disabled", disabled: true, selected_date: Some(TODAY),
                DatePickerPopover {
                    DatePickerInput {
                        DatePickerInputValue {
                            DatePickerYearSegment {}
                            DatePickerSeparator {}
                            DatePickerMonthSegment {}
                            DatePickerSeparator {}
                            DatePickerDaySegment {}
                        }

                        DatePickerTrigger { aria_label: "Open the calendar", "📅" }
                    }
                }
            }

            DatePicker { id: "read-only", read_only: true, selected_date: Some(TODAY),
                DatePickerPopover {
                    DatePickerInput {
                        DatePickerInputValue {
                            DatePickerYearSegment {}
                            DatePickerSeparator {}
                            DatePickerMonthSegment {}
                            DatePickerSeparator {}
                            DatePickerDaySegment {}
                        }

                        DatePickerTrigger { aria_label: "Open the calendar", "📅" }
                    }

                    DatePickerContent {
                        DatePickerCalendar { today: TODAY }
                    }
                }
            }

            DatePicker {
                id: "limited",
                selected_date: Some(TODAY),
                min_date: date!(2026 - 06 - 08),
                max_date: date!(2026 - 06 - 21),
                disabled_ranges: vec![DateRange::new(date!(2026 - 06 - 17), date!(2026 - 06 - 19))],
                DatePickerPopover { appearance: DatePickerPopoverAppearance::None,
                    DatePickerInput {
                        DatePickerInputValue {
                            DatePickerYearSegment {}
                            DatePickerSeparator {}
                            DatePickerMonthSegment {}
                            DatePickerSeparator {}
                            DatePickerDaySegment {}
                        }

                        DatePickerTrigger { aria_label: "Open the calendar", "📅" }
                    }

                    DatePickerContent { id: "limited-calendar",
                        DatePickerCalendar { today: TODAY }
                    }
                }
            }
        }
    }
}
