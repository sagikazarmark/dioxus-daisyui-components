use dioxus::prelude::*;

use crate::components::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionItemMarker, AccordionTrigger,
};

/// An accordion that lets every item be open at once, with one item disabled
/// and a tally of what the set reported.
///
/// `allow_multiple_open` and `collapsible` are the primitive's, and they are
/// why the open state is mirrored here rather than lifted: the set decides what
/// opening one item does to the others, so an item that owned its own state
/// would be able to disagree with the set about it. Each item reports what the
/// set decided through `on_change`, which is what puts daisyUI's open class on
/// the item, and what this example records.
///
/// The tally is kept per item rather than as a running count, because
/// `on_change` reports a **state** rather than a change: every item reports
/// whenever the set is re-evaluated, including on the first render and when
/// some other item was the one that moved. Writing each item's state into its
/// own slot is idempotent, which is what that calls for.
#[component]
pub fn Example() -> Element {
    let mut open = use_signal(|| [false; 3]);

    let entries = [
        ("Build", "Runs on every push.", false),
        ("Deploy", "Runs on a tag.", false),
        ("Rollback", "Not available on this plan.", true),
    ];

    rsx! {
        div { class: "flex flex-col gap-3",
            Accordion { id: "multiple", allow_multiple_open: true,
                for (index , (title , body , disabled)) in entries.into_iter().enumerate() {
                    AccordionItem {
                        index,
                        disabled,
                        marker: AccordionItemMarker::Plus,
                        on_change: move |next| open.write()[index] = next,
                        AccordionTrigger { "{title}" }
                        AccordionContent { "{body}" }
                    }
                }
            }

            p { class: "text-sm opacity-70",
                span { "data-testid": "open", "{open().iter().filter(|item| **item).count()}" }
                " open"
            }
        }
    }
}
