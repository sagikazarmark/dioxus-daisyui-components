use std::{cell::Cell, rc::Rc};

use dioxus::prelude::*;
use dioxus_field::{
    Binding, ChangeOrigin,
    testing::{FocusExitOrderProbe, FocusExitProbe},
};
use dioxus_html::SerializedFocusData;

use dioxus_daisyui_components::components::{
    field::Field,
    native_select::{NativeSelect, NativeSelectColor, NativeSelectOption},
};

use crate::harness::*;

fn flavors() -> Vec<NativeSelectOption<String>> {
    vec![
        NativeSelectOption::new(String::from("orange"), "Orange"),
        NativeSelectOption::new(String::from("lemon"), "Lemon"),
        NativeSelectOption::new(String::from("cherry"), "Cherry"),
    ]
}

fn interaction_app(harness: InteractionHarness<Option<String>>) -> Element {
    let value: Signal<Option<String>> = use_signal(|| None);
    let trio = harness.trio(ReadSignal::from(value));

    rsx! {
        NativeSelect {
            options: flavors(),
            aria_label: "Interaction select",
            value: trio.value,
            on_change: trio.on_change,
            on_commit: trio.on_commit,
        }
    }
}

fn interaction_dom(harness: InteractionHarness<Option<String>>) -> InteractionDom {
    InteractionDom::mount(
        VirtualDom::new_with_props(interaction_app, harness),
        "input",
    )
}

fn form_value_app(harness: InteractionHarness<Option<String>>) -> Element {
    let value: Signal<Option<String>> = use_signal(|| None);
    let trio = harness.trio(ReadSignal::from(value));

    rsx! {
        NativeSelect {
            options: vec![
                NativeSelectOption::new(String::from("pending"), "Pending").form_value("pending"),
                NativeSelectOption::new(String::from("active"), "Active").form_value("active"),
            ],
            aria_label: "Form value select",
            value: trio.value,
            on_change: trio.on_change,
            on_commit: trio.on_commit,
        }
    }
}

fn form_value_dom(harness: InteractionHarness<Option<String>>) -> InteractionDom {
    InteractionDom::mount(VirtualDom::new_with_props(form_value_app, harness), "input")
}

fn colliding_form_value_app() -> Element {
    rsx! {
        NativeSelect {
            // The explicit form value collides with the first option's
            // positional index.
            options: vec![
                NativeSelectOption::new(String::from("orange"), "Orange"),
                NativeSelectOption::new(String::from("lemon"), "Lemon").form_value("0"),
            ],
            aria_label: "Colliding select",
        }
    }
}

fn empty_form_value_app() -> Element {
    rsx! {
        NativeSelect {
            options: vec![
                NativeSelectOption::new(String::from("orange"), "Orange").form_value(""),
            ],
            aria_label: "Empty form value select",
        }
    }
}

#[derive(Clone)]
struct FocusExitHarness {
    probe: FocusExitOrderProbe,
    direct_calls: Rc<Cell<usize>>,
}

fn focus_exit_app(harness: FocusExitHarness) -> Element {
    let value: Signal<Option<String>> = use_signal(|| None);
    let binding = harness.probe.binding(ReadSignal::from(value));
    let callback_probe = harness.probe.clone();
    let direct_calls = Rc::clone(&harness.direct_calls);

    rsx! {
        NativeSelect {
            options: flavors(),
            aria_label: "Focus exit select",
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
    let value: Signal<Option<String>> = use_signal(|| None);
    let binding_commits = Rc::clone(&harness.binding_commits);
    let prop_commits = Rc::clone(&harness.prop_commits);
    let binding = Binding::new(
        ReadSignal::from(value),
        Callback::new(|_| {}),
        Callback::new(move |()| binding_commits.set(binding_commits.get() + 1)),
    )
    .with_focus_exit(harness.binding_probe.on_focus_exit());

    rsx! {
        NativeSelect {
            options: flavors(),
            aria_label: "Unchanged focus exit select",
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

fn resolution_options() -> Vec<NativeSelectOption<String>> {
    vec![
        NativeSelectOption::new(String::from("explicit value"), "Explicit value"),
        NativeSelectOption::new(String::from("context value"), "Context value"),
        NativeSelectOption::new(String::from("explicit change"), "Explicit change"),
        NativeSelectOption::new(String::from("context change"), "Context change"),
        NativeSelectOption::new(String::from("internal change"), "Internal change"),
    ]
}

fn resolution_app(harness: ResolutionHarness<Option<String>>) -> Element {
    let scaffold = use_resolution_scaffold(
        &harness,
        Some(String::from("explicit value")),
        Some(String::from("context value")),
    );
    let context = scaffold.field_context();

    match harness.source {
        ResolutionSource::Explicit => rsx! {
            Field { context,
                NativeSelect {
                    options: resolution_options(),
                    binding: scaffold.explicit_binding,
                    meta: scaffold.explicit_meta,
                    color: NativeSelectColor::Primary,
                    id: "explicit-id",
                    name: "explicit-name",
                    required: false,
                    disabled: false,
                    aria_label: "Explicit select",
                }
            }
        },
        ResolutionSource::Context => rsx! {
            Field { context,
                NativeSelect {
                    options: resolution_options(),
                    disabled: false,
                    required: false,
                    aria_label: "Context select",
                }
            }
        },
        ResolutionSource::Metadata => rsx! {
            Field { context,
                NativeSelect {
                    options: resolution_options(),
                    color: NativeSelectColor::Default,
                    aria_label: "Metadata select",
                }
            }
        },
        ResolutionSource::Internal => rsx! {
            NativeSelect {
                options: resolution_options(),
                aria_label: "Internal select",
            }
        },
    }
}

fn resolution_dom(harness: ResolutionHarness<Option<String>>) -> InteractionDom {
    InteractionDom::mount(VirtualDom::new_with_props(resolution_app, harness), "input")
}

#[test]
fn commit_is_synchronously_observable_before_submit_handling_runs() {
    let harness = InteractionHarness::new();
    let dom = interaction_dom(harness.clone());

    dispatch_input_event(&dom, "change", "1");
    dom.dom.in_runtime(|| harness.submit());

    harness.commits.assert_commit_before_submit();
}

#[test]
fn writes_carry_their_change_origin() {
    let harness = InteractionHarness::new();
    let dom = interaction_dom(harness.clone());

    dispatch_input_event(&dom, "input", "1");

    harness
        .changes
        .assert_writes(&[(Some("lemon".to_owned()), ChangeOrigin::User)]);
}

#[test]
fn placeholder_and_unknown_option_values_write_nothing() {
    let harness = InteractionHarness::new();
    let dom = interaction_dom(harness.clone());

    dispatch_input_event(&dom, "input", "");
    dispatch_input_event(&dom, "input", "not an option");

    harness.changes.assert_writes(&[]);
}

#[test]
fn explicit_form_values_replace_the_positional_mapping() {
    let harness = InteractionHarness::new();
    let dom = form_value_dom(harness.clone());

    dispatch_input_event(&dom, "input", "0");
    dispatch_input_event(&dom, "input", "active");

    harness
        .changes
        .assert_writes(&[(Some("active".to_owned()), ChangeOrigin::User)]);
}

#[test]
fn colliding_and_empty_emitted_values_are_rejected() {
    use std::panic;
    use std::sync::{Arc, Mutex};

    // Dioxus catches a component's panic and renders nothing in its place, so
    // the assertions are observed through the panic hook rather than through
    // `should_panic`.
    let messages: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&messages);
    let previous = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        recorded.lock().unwrap().push(info.to_string());
    }));
    let _ = VirtualDom::new(colliding_form_value_app).rebuild_to_vec();
    let _ = VirtualDom::new(empty_form_value_app).rebuild_to_vec();
    panic::set_hook(previous);

    let messages = messages.lock().unwrap();
    assert!(
        messages
            .iter()
            .any(|message| message.contains("unique value strings")),
        "a form value colliding with a positional index must be rejected"
    );
    assert!(
        messages
            .iter()
            .any(|message| message.contains("non-empty value strings")),
        "an empty form value must be rejected"
    );
}

#[test]
fn native_change_commits_without_focus_exit() {
    let harness = FocusExitHarness {
        probe: FocusExitOrderProbe::new(),
        direct_calls: Rc::new(Cell::new(0)),
    };
    let dom = focus_exit_dom(harness.clone());

    dispatch_input_event(&dom, "change", "1");

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
    dispatch_input_event(&dom, "input", "1");
    dispatch_input_event(&dom, "change", "1");
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
                assert_eq!(attributes.get("value"), Some("0"));
                assert_eq!(attributes.get("id"), Some("explicit-id"));
                assert_eq!(attributes.get("name"), Some("explicit-name"));
                assert!(attributes.class_contains("select-primary"));
                assert!(!attributes.class_contains("select-error"));
                assert_eq!(attributes.get("aria-invalid"), Some("false"));
                assert!(!attributes.has("disabled"));
                assert!(!attributes.has("required"));
                assert!(!attributes.has("data-disabled"));
                assert!(!attributes.has("data-required"));
            },
            dispatch: |dom| dispatch_input_event(dom, "input", "2"),
            written: Some("explicit change".to_owned()),
        },
        context: PrecedencePhase {
            assert_attributes: |dom| {
                let attributes = &dom.attributes;
                assert_eq!(attributes.get("value"), Some("1"));
                assert_eq!(attributes.get("id"), Some("context-id"));
                assert_eq!(attributes.get("name"), Some("context-name"));
                assert!(attributes.class_contains("select-error"));
                assert_eq!(attributes.get("aria-invalid"), Some("true"));
                assert!(!attributes.has("disabled"));
                assert!(!attributes.has("required"));
                assert!(!attributes.has("data-disabled"));
                assert!(!attributes.has("data-required"));
                assert_eq!(attributes.get("data-touched"), Some("true"));
                assert_eq!(attributes.get("data-dirty"), Some("true"));
            },
            dispatch: |dom| dispatch_input_event(dom, "input", "3"),
            written: Some("context change".to_owned()),
        },
        assert_metadata: |dom| {
            let attributes = &dom.attributes;
            assert_eq!(attributes.get("id"), Some("context-id"));
            assert_eq!(attributes.get("name"), Some("context-name"));
            assert_eq!(attributes.get("aria-invalid"), Some("true"));
            assert!(!attributes.class_contains("select-error"));
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
            dispatch: |dom| dispatch_input_event(dom, "input", "4"),
            assert_updated: |dom| {
                assert_eq!(dom.attributes.get("value"), Some("4"));
            },
        },
    });
}

#[test]
fn focus_request_round_trips_to_the_widget_control() {
    assert_focus_request_round_trip(
        || {
            rsx! {
                NativeSelect {
                    options: vec![NativeSelectOption::new(String::from("one"), "One")],
                    aria_label: "Focus target",
                }
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
                NativeSelect {
                    options: vec![NativeSelectOption::new(String::from("one"), "One")],
                    aria_label: "Registered select",
                }
            }
        },
        "input",
        "native-select-description",
        "native-select-error",
        true,
    );
}
