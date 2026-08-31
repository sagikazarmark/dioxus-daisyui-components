use dioxus::prelude::*;

use crate::components::combobox::{Combobox, ComboboxInput, ComboboxList};
use crate::examples::combobox::options::Options;

/// A combobox whose open state, value and query all belong to its caller.
///
/// The first two are the ones this component lifts either way (ADR-0006); a
/// caller taking them is the same state travelling one level further out. The
/// query is not lifted: nothing daisyUI draws depends on what has been typed, so
/// it is the primitive's, and a caller who wants it takes it from there.
///
/// It is the only combobox on this page that is not held open, and it is last
/// with room under it, so that the popup it opens for real lands over nothing.
#[component]
pub fn Example() -> Element {
    let mut open = use_signal(|| false);
    let mut value = use_signal(|| Option::<String>::None);
    let mut query = use_signal(String::new);
    let mut changes = use_signal(|| 0_u32);
    let mut commits = use_signal(|| 0_u32);
    let mut focus_exits = use_signal(|| 0_u32);

    // What the combobox last chose, as something to render: the value itself is
    // absent until an option is chosen.
    let selected = value().unwrap_or_else(|| String::from("nothing"));

    rsx! {
        div { class: "flex flex-col items-start gap-4 pb-72",
            Combobox::<String> {
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
                query: Some(query()),
                on_query_change: move |next| query.set(next),
                ComboboxInput { id: "controlled-input", placeholder: "Pick a fruit" }
                ComboboxList { id: "controlled-list",
                    Options {}
                }
            }

            p { class: "text-sm opacity-70",
                "Opened and closed "
                span { "data-testid": "changes", "{changes}" }
                " times, typed "
                span { "data-testid": "query", "{query}" }
                ", chose "
                span { "data-testid": "selected", "{selected}" }
            }
            span { class: "hidden", "data-testid": "commits", "{commits}" }
            span { class: "hidden", "data-testid": "focus-exits", "{focus_exits}" }

            // The next tab stop after the combobox, which is what says the list
            // this component leaves in the document while the popup is closed
            // adds no tab stop of its own.
            a { id: "after", href: "#after", class: "link w-fit", "After the combobox" }
        }
    }
}
