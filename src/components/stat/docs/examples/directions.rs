use dioxus::prelude::*;

use crate::components::stat::{
    Stat, StatActions, StatDescription, StatFigure, StatTitle, StatValue, Stats, StatsDirection,
};

/// Every value of the direction Axis with the complete compound structure.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { "data-axis": "direction", class: "flex flex-col items-start gap-4",
            for direction in StatsDirection::ALL.iter().copied() {
                Stats { class: "bg-base-100 shadow-sm", direction,
                    Stat {
                        StatFigure { class: "text-secondary text-xl", "24h" }
                        StatTitle { "{direction:?} visits" }
                        StatValue { "8,900" }
                        StatDescription { "12% more this week" }
                    }
                    Stat {
                        StatTitle { "Conversions" }
                        StatValue { "510" }
                        StatDescription { "From the same reporting period" }
                        StatActions {
                            button { class: "btn btn-xs", "Open report" }
                        }
                    }
                }
            }
        }
    }
}
