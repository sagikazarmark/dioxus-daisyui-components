use dioxus::prelude::*;

use crate::components::stat::{
    Stat, StatActions, StatDescription, StatFigure, StatTitle, StatValue, Stats, StatsDirection,
};

/// A caller's classes and attributes joined with every compound part's own.
#[component]
pub fn Example() -> Element {
    rsx! {
        Stats {
            id: "caller-stats",
            class: "rounded-none bg-base-100 shadow-sm",
            title: "Caller stats",
            direction: StatsDirection::Vertical,
            Stat { id: "caller-stat", class: "px-2", title: "Caller stat",
                StatFigure {
                    id: "caller-figure",
                    class: "text-secondary",
                    title: "Caller figure",
                    "24h"
                }
                StatTitle {
                    id: "caller-title",
                    class: "italic",
                    title: "Caller title",
                    "Active accounts"
                }
                StatValue {
                    id: "caller-value",
                    class: "text-primary",
                    title: "Caller value",
                    "1,240"
                }
                StatDescription {
                    id: "caller-description",
                    class: "uppercase",
                    title: "Caller description",
                    "Updated today"
                }
                StatActions {
                    id: "caller-actions",
                    class: "justify-self-end",
                    title: "Caller actions",
                    button { class: "btn btn-xs", "Manage" }
                }
            }
        }
    }
}
