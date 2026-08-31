use dioxus::prelude::*;

use crate::components::select::{Select, SelectList, SelectTrigger, SelectValue};
use crate::examples::select::options::Options;

/// A select whose open state and value both belong to its caller.
///
/// Opening, dismissal and selection travel out through the change callbacks and
/// back in through the props, so nothing happens unless the caller lets it.
/// Arrow keys move through the options and skip the disabled one, typing jumps
/// to the option that matches, and Escape dismisses.
///
/// It is the only select on this page that is not held open, and it is last
/// with room under it, so that the popup it opens for real lands over nothing.
#[component]
pub fn Example() -> Element {
    let mut open = use_signal(|| false);
    let mut value = use_signal(|| Option::<String>::None);
    let mut changes = use_signal(|| 0_u32);
    let mut commits = use_signal(|| 0_u32);
    let mut focus_exits = use_signal(|| 0_u32);

    // What the select last chose, as something to render: the value itself is
    // absent until an option is chosen.
    let selected = value().unwrap_or_else(|| String::from("nothing"));

    rsx! {
        div { class: "flex flex-col items-start gap-4 pb-72",
            Select::<String> {
                id: "controlled",
                value: Some(ReadSignal::new(value)),
                on_change: move |next| value.set(next),
                on_commit: move |()| commits += 1,
                on_focus_exit: move |()| focus_exits += 1,
                open: Some(open()),
                on_open_change: move |next| {
                    open.set(next);
                    changes += 1;
                },
                SelectTrigger { id: "controlled-trigger",
                    SelectValue { placeholder: "Pick a fruit" }
                }
                SelectList { id: "controlled-list",
                    Options {}
                }
            }

            p { class: "text-sm opacity-70",
                "Changed "
                span { "data-testid": "changes", "{changes}" }
                " times, selected "
                span { "data-testid": "selected", "{selected}" }
            }
            span { class: "hidden", "data-testid": "commits", "{commits}" }
            span { class: "hidden", "data-testid": "focus-exits", "{focus_exits}" }
            button {
                id: "close-controlled",
                hidden: true,
                onclick: move |_| open.set(false),
                "Close from controlled state"
            }

            // The next tab stop after the select, which is what says the list
            // this component leaves in the document while the popup is closed
            // adds no tab stop of its own.
            a { id: "after", href: "#after", class: "link w-fit", "After the select" }
        }
    }
}
