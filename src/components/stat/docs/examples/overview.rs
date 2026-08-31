use dioxus::prelude::*;

use crate::components::stat::{
    Stat, StatActions, StatDescription, StatFigure, StatTitle, StatValue, Stats,
};

/// Multiple statistics using every compound part.
#[component]
pub fn Example() -> Element {
    rsx! {
        Stats { id: "default-stats",
            Stat { id: "placement-stat",
                StatFigure { id: "placement-figure", class: "text-primary text-xl", "24h" }
                StatTitle { id: "placement-title", "Page views" }
                StatValue { id: "placement-value", "89,400" }
                StatDescription { id: "placement-description", "21% more than last month" }
                StatActions { id: "placement-actions",
                    button { class: "btn btn-xs btn-primary", "Details" }
                }
            }
            Stat {
                StatTitle { "Downloads" }
                StatValue { "31K" }
                StatDescription { "Jan 1 through Feb 1" }
            }
            Stat {
                StatTitle { "New users" }
                StatValue { "4,200" }
                StatDescription { "+8% this week" }
            }
        }
    }
}
