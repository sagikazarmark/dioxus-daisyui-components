use std::{cell::Cell, rc::Rc};

use dioxus::prelude::*;
use dioxus_field::{
    Binding, ChangeOrigin,
    testing::{FocusExitOrderProbe, FocusExitProbe},
};
use dioxus_html::SerializedFocusData;

use dioxus_daisyui_components::components::{
    field::Field,
    textarea::{Textarea, TextareaColor},
};

use crate::harness::*;

fn interaction_app(harness: InteractionHarness<String>) -> Element {
    let value = use_signal(String::new);
    let trio = harness.trio(ReadSignal::from(value));

    rsx! {
        Textarea {
            value: trio.value,
            on_change: trio.on_change,
            on_commit: trio.on_commit,
        }
    }
}

fn interaction_dom(harness: InteractionHarness<String>) -> InteractionDom {
    InteractionDom::mount(
        VirtualDom::new_with_props(interaction_app, harness),
        "input",
    )
}

#[derive(Clone)]
struct FocusExitHarness {
    probe: FocusExitOrderProbe,
    direct_calls: Rc<Cell<usize>>,
}

fn focus_exit_app(harness: FocusExitHarness) -> Element {
    let value = use_signal(String::new);
    let binding = harness.probe.binding(ReadSignal::from(value));
    let callback_probe = harness.probe.clone();
    let direct_calls = Rc::clone(&harness.direct_calls);

    rsx! {
        Textarea {
            binding,
            on_focus_exit: move |()| {
                callback_probe.assert_write_and_commit_before_focus_exit();
                direct_calls.set(direct_calls.get() + 1);
            },
        }
    }
}

fn focus_exit_dom(harness: FocusExitHarness) -> InteractionDom {
    InteractionDom::mount(VirtualDom::new_with_props(focus_exit_app, harness), "input")
}

#[derive(Clone)]
struct UnchangedFocusExitHarness {
    binding_probe: FocusExitProbe,
    binding_commits: Rc<Cell<usize>>,
    prop_commits: Rc<Cell<usize>>,
}

fn unchanged_focus_exit_app(harness: UnchangedFocusExitHarness) -> Element {
    let value = use_signal(String::new);
    let binding_commits = Rc::clone(&harness.binding_commits);
    let prop_commits = Rc::clone(&harness.prop_commits);
    let binding = Binding::new(
        ReadSignal::from(value),
        Callback::new(|_| {}),
        Callback::new(move |()| binding_commits.set(binding_commits.get() + 1)),
    )
    .with_focus_exit(harness.binding_probe.on_focus_exit());

    rsx! {
        Textarea {
            binding,
            on_commit: move |()| prop_commits.set(prop_commits.get() + 1),
        }
    }
}

fn unchanged_focus_exit_dom(harness: UnchangedFocusExitHarness) -> InteractionDom {
    InteractionDom::mount(
        VirtualDom::new_with_props(unchanged_focus_exit_app, harness),
        "input",
    )
}

fn resolution_app(harness: ResolutionHarness<String>) -> Element {
    let scaffold = use_resolution_scaffold(
        &harness,
        String::from("explicit value"),
        String::from("context value"),
    );
    let context = scaffold.field_context();

    match harness.source {
        ResolutionSource::Explicit => rsx! {
            Field { context,
                Textarea {
                    binding: scaffold.explicit_binding,
                    meta: scaffold.explicit_meta,
                    color: TextareaColor::Primary,
                    id: "explicit-id",
                    name: "explicit-name",
                    required: false,
                    disabled: false,
                    aria_label: "Explicit textarea",
                }
            }
        },
        ResolutionSource::Context => rsx! {
            Field { context,
                Textarea {
                    disabled: false,
                    required: false,
                    aria_label: "Context textarea",
                }
            }
        },
        ResolutionSource::Metadata => rsx! {
            Field { context,
                Textarea {
                    color: TextareaColor::Default,
                    aria_label: "Metadata textarea",
                }
            }
        },
        ResolutionSource::Internal => rsx! {
            Textarea { aria_label: "Internal textarea" }
        },
    }
}

fn resolution_dom(harness: ResolutionHarness<String>) -> InteractionDom {
    InteractionDom::mount(VirtualDom::new_with_props(resolution_app, harness), "input")
}

#[test]
fn commit_is_synchronously_observable_before_submit_handling_runs() {
    let harness = InteractionHarness::new();
    let dom = interaction_dom(harness.clone());

    dispatch_input_event(&dom, "change", "committed");
    dom.dom.in_runtime(|| harness.submit());

    harness.commits.assert_commit_before_submit();
}

#[test]
fn writes_carry_their_change_origin() {
    let harness = InteractionHarness::new();
    let dom = interaction_dom(harness.clone());

    dispatch_input_event(&dom, "input", "typed by the user");

    harness
        .changes
        .assert_writes(&[("typed by the user".to_owned(), ChangeOrigin::User)]);
}

#[test]
fn native_change_commits_without_focus_exit() {
    let harness = FocusExitHarness {
        probe: FocusExitOrderProbe::new(),
        direct_calls: Rc::new(Cell::new(0)),
    };
    let dom = focus_exit_dom(harness.clone());

    dispatch_input_event(&dom, "change", "committed");

    harness.probe.assert_commit_without_focus_exit();
    assert_eq!(harness.direct_calls.get(), 0);
}

#[test]
fn changed_focus_session_reports_binding_then_direct_focus_exit_once() {
    let harness = FocusExitHarness {
        probe: FocusExitOrderProbe::new(),
        direct_calls: Rc::new(Cell::new(0)),
    };
    let dom = focus_exit_dom(harness.clone());

    dom.dispatch("focusin", SerializedFocusData::default());
    dispatch_input_event(&dom, "input", "changed");
    dispatch_input_event(&dom, "change", "changed");
    dom.dispatch("focusout", SerializedFocusData::default());

    harness.probe.assert_write_and_commit_before_focus_exit();
    assert_eq!(harness.direct_calls.get(), 1);
}

#[test]
fn unchanged_focus_session_reports_focus_exit_without_commit() {
    let harness = UnchangedFocusExitHarness {
        binding_probe: FocusExitProbe::new(),
        binding_commits: Rc::new(Cell::new(0)),
        prop_commits: Rc::new(Cell::new(0)),
    };
    let dom = unchanged_focus_exit_dom(harness.clone());

    dom.dispatch("focusin", SerializedFocusData::default());
    dom.dispatch("focusout", SerializedFocusData::default());

    assert_eq!(harness.binding_commits.get(), 0);
    assert_eq!(harness.prop_commits.get(), 0);
    harness.binding_probe.assert_focus_exit_once();
}

#[test]
fn binding_resolution_precedence_holds_for_values_and_meta_flags() {
    assert_binding_resolution_precedence(PrecedenceSpec {
        resolution_dom,
        explicit: PrecedencePhase {
            assert_attributes: |dom| {
                let attributes = &dom.attributes;
                assert_eq!(attributes.get("value"), Some("explicit value"));
                assert_eq!(attributes.get("id"), Some("explicit-id"));
                assert_eq!(attributes.get("name"), Some("explicit-name"));
                assert!(attributes.class_contains("textarea-primary"));
                assert!(!attributes.class_contains("textarea-error"));
                assert_eq!(attributes.get("aria-invalid"), Some("false"));
                assert!(!attributes.has("disabled"));
                assert!(!attributes.has("required"));
                assert!(!attributes.has("data-disabled"));
                assert!(!attributes.has("data-required"));
            },
            dispatch: |dom| dispatch_input_event(dom, "input", "explicit change"),
            written: "explicit change".to_owned(),
        },
        context: PrecedencePhase {
            assert_attributes: |dom| {
                let attributes = &dom.attributes;
                assert_eq!(attributes.get("value"), Some("context value"));
                assert_eq!(attributes.get("id"), Some("context-id"));
                assert_eq!(attributes.get("name"), Some("context-name"));
                assert!(attributes.class_contains("textarea-error"));
                assert_eq!(attributes.get("aria-invalid"), Some("true"));
                assert!(!attributes.has("disabled"));
                assert!(!attributes.has("required"));
                assert!(!attributes.has("data-disabled"));
                assert!(!attributes.has("data-required"));
                assert_eq!(attributes.get("data-touched"), Some("true"));
                assert_eq!(attributes.get("data-dirty"), Some("true"));
            },
            dispatch: |dom| dispatch_input_event(dom, "input", "context change"),
            written: "context change".to_owned(),
        },
        assert_metadata: |dom| {
            let attributes = &dom.attributes;
            assert_eq!(attributes.get("id"), Some("context-id"));
            assert_eq!(attributes.get("name"), Some("context-name"));
            assert_eq!(attributes.get("aria-invalid"), Some("true"));
            assert!(!attributes.class_contains("textarea-error"));
            assert!(attributes.has("disabled"));
            assert!(attributes.has("required"));
            assert_eq!(attributes.get("data-disabled"), Some("true"));
            assert_eq!(attributes.get("data-required"), Some("true"));
        },
        internal: InternalPhase {
            assert_mounted: |dom| {
                assert_eq!(dom.attributes.get("value"), Some(""));
                assert_eq!(dom.attributes.get("aria-invalid"), Some("false"));
            },
            dispatch: |dom| dispatch_input_event(dom, "input", "internal change"),
            assert_updated: |dom| {
                assert_eq!(dom.attributes.get("value"), Some("internal change"));
            },
        },
    });
}

#[test]
fn focus_request_round_trips_to_the_widget_control() {
    assert_focus_request_round_trip(
        || {
            rsx! {
                Textarea { aria_label: "Focus target" }
            }
        },
        mount_control,
    );
}

#[test]
fn error_and_description_ids_appear_on_mount_and_vanish_on_drop() {
    assert_error_and_description_id_registration(
        || {
            rsx! {
                Textarea { aria_label: "Registered textarea" }
            }
        },
        "input",
        "textarea-description",
        "textarea-error",
        true,
    );
}
