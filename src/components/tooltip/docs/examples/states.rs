use dioxus::prelude::*;

use crate::components::button::{Button, ButtonColor};
use crate::components::tooltip::{Tooltip, TooltipContent, TooltipTrigger};

/// A tooltip this page holds open, and one that has been switched off.
///
/// The controlled one is why the open state is lifted at all: daisyUI reveals a
/// bubble on hover and on keyboard focus without being asked, and a tooltip
/// that is open because its caller says so reaches neither of those. The state
/// travels out through the change callback (pointing at the trigger reports it)
/// and back in through the open prop.
///
/// The disabled one is the primitive's: it neither opens on hover nor renders a
/// bubble, and daisyUI has no disabled tooltip to draw.
#[component]
pub fn Example() -> Element {
    let mut open = use_signal(|| false);
    let mut changes = use_signal(|| 0_u32);

    rsx! {
        div { class: "flex flex-wrap items-center gap-6 pt-12",
            Tooltip {
                id: "controlled",
                open: Some(open()),
                on_open_change: move |next| {
                    open.set(next);
                    changes += 1;
                },
                TooltipTrigger { class: "text-sm underline decoration-dotted", "Controlled" }
                TooltipContent { "Held open by the page" }
            }

            Button {
                color: ButtonColor::Primary,
                onclick: move |_| open.toggle(),
                "Toggle it"
            }

            Tooltip { id: "disabled", disabled: true,
                TooltipTrigger { class: "text-sm underline decoration-dotted opacity-50", "Disabled" }
                TooltipContent { "Never shown" }
            }

            p { class: "text-sm opacity-70",
                "Changed "
                span { "data-testid": "changes", "{changes}" }
                " times"
            }
        }
    }
}
