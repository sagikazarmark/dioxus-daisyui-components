use dioxus::prelude::*;

use crate::components::card::{
    Card, CardActions, CardBody, CardBorder, CardLayout, CardSize, CardTitle,
};

/// A default card and three independent Axes set together.
#[component]
pub fn Example() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-start gap-4",
            Card { id: "default-card",
                CardBody {
                    CardTitle { "Default card" }
                    p { "A body keeps the title, copy and controls together." }
                    CardActions { button { class: "btn btn-primary", "Continue" } }
                }
            }
            Card {
                class: "w-64 bg-base-100 shadow-sm",
                size: CardSize::Lg,
                border: CardBorder::Dashed,
                layout: CardLayout::Side,
                CardBody {
                    CardTitle { "Combined Axes" }
                    p { "Large, dashed and laid out side by side." }
                }
            }
        }
    }
}
