use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use dioxus::prelude::*;
use dioxus_field::{
    Binding, ChangeOrigin, FieldContext, FieldMetaValues,
    testing::{FocusExitOrderProbe, FocusExitProbe},
    use_field_meta_state,
};
use dioxus_html::SerializedFocusData;
use dioxus_primitives::dioxus_attributes::attributes;

use dioxus_daisyui_components::components::{
    field::Field,
    input::{Input, InputColor, InputSize},
};

use crate::harness::*;

fn interaction_app(harness: InteractionHarness<String>) -> Element {
    let value = use_signal(String::new);
    let trio = harness.trio(ReadSignal::from(value));

    rsx! {
        Input {
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
    adorned: bool,
}

fn focus_exit_app(harness: FocusExitHarness) -> Element {
    let value = use_signal(String::new);
    let binding = harness.probe.binding(ReadSignal::from(value));
    let callback_probe = harness.probe.clone();
    let direct_calls = Rc::clone(&harness.direct_calls);

    rsx! {
        Input {
            binding,
            suffix: harness.adorned.then(|| rsx! { "EUR" }),
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
    adorned: bool,
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
        Input {
            binding,
            suffix: harness.adorned.then(|| rsx! { "EUR" }),
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
                Input {
                    binding: scaffold.explicit_binding,
                    meta: scaffold.explicit_meta,
                    color: InputColor::Primary,
                    id: "explicit-id",
                    name: "explicit-name",
                    required: false,
                    disabled: false,
                    aria_label: "Explicit input",
                }
            }
        },
        ResolutionSource::Context => rsx! {
            Field { context,
                Input {
                    disabled: false,
                    required: false,
                    aria_label: "Context input",
                }
            }
        },
        ResolutionSource::Metadata => rsx! {
            Field { context,
                Input {
                    color: InputColor::Default,
                    aria_label: "Metadata input",
                }
            }
        },
        ResolutionSource::Internal => rsx! {
            Input { aria_label: "Internal input" }
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
        adorned: false,
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
        adorned: false,
    };
    let dom = focus_exit_dom(harness.clone());

    dom.dispatch("focusin", SerializedFocusData::default());
    dispatch_input_event(&dom, "input", "changed");
    dispatch_input_event(&dom, "change", "changed");
    dom.dispatch("focusout", SerializedFocusData::default());

    harness.probe.assert_write_and_commit_before_focus_exit();
    assert_eq!(harness.direct_calls.get(), 1);
}

// The adorned arm keeps the complete logical focus scope on the native input:
// the same dispatch sequence reports the same observable write/commit/Focus
// Exit ordering, once. Which element reports it is not pinned (ADR-0031).
#[test]
fn adorned_changed_focus_session_reports_binding_then_direct_focus_exit_once() {
    let harness = FocusExitHarness {
        probe: FocusExitOrderProbe::new(),
        direct_calls: Rc::new(Cell::new(0)),
        adorned: true,
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
        adorned: false,
    };
    let dom = unchanged_focus_exit_dom(harness.clone());

    dom.dispatch("focusin", SerializedFocusData::default());
    dom.dispatch("focusout", SerializedFocusData::default());

    assert_eq!(harness.binding_commits.get(), 0);
    assert_eq!(harness.prop_commits.get(), 0);
    harness.binding_probe.assert_focus_exit_once();
}

#[test]
fn adorned_unchanged_focus_session_reports_focus_exit_without_commit() {
    let harness = UnchangedFocusExitHarness {
        binding_probe: FocusExitProbe::new(),
        binding_commits: Rc::new(Cell::new(0)),
        prop_commits: Rc::new(Cell::new(0)),
        adorned: true,
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
                assert!(attributes.class_contains("input-primary"));
                assert!(!attributes.class_contains("input-error"));
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
                assert!(attributes.class_contains("input-error"));
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
            assert!(!attributes.class_contains("input-error"));
            assert!(attributes.has("disabled"));
            assert!(attributes.has("required"));
            assert_eq!(attributes.get("data-disabled"), Some("true"));
            assert_eq!(attributes.get("data-required"), Some("true"));
        },
        internal: InternalPhase {
            assert_mounted: |dom| {
                assert!(
                    dom.attributes
                        .get("id")
                        .is_some_and(|id| id.starts_with("dxf-field-"))
                );
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

/// Every dynamic `class` written during mount, by element, so a test can read
/// the wrapper's classes beside the control's own.
fn mounted_class_attributes(
    app: fn() -> Element,
) -> (HashMap<dioxus_core::ElementId, String>, InteractionDom) {
    InteractionDom::mount_with_edits(VirtualDom::new(app), "input", |edits| {
        edits
            .iter()
            .filter_map(|edit| match edit {
                dioxus_core::Mutation::SetAttribute {
                    name: "class",
                    value: dioxus_core::AttributeValue::Text(value),
                    id,
                    ..
                } => Some((*id, value.clone())),
                _ => None,
            })
            .collect()
    })
}

fn class_of<'a>(
    classes: &'a HashMap<dioxus_core::ElementId, String>,
    element: dioxus_core::ElementId,
) -> &'a str {
    classes.get(&element).map(String::as_str).unwrap_or("")
}

/// The wrapper's classes: the one non-control element whose class list carries
/// daisyUI's `input`.
fn wrapper_class_of(
    classes: &HashMap<dioxus_core::ElementId, String>,
    control: dioxus_core::ElementId,
) -> &str {
    classes
        .iter()
        .find(|(id, value)| {
            **id != control && value.split_ascii_whitespace().any(|class| class == "input")
        })
        .map(|(_, value)| value.as_str())
        .expect("the adorned mount should write the wrapper's classes")
}

#[test]
fn adorned_axis_classes_relocate_to_the_wrapper_and_caller_attributes_stay_native() {
    fn app() -> Element {
        rsx! {
            Input {
                color: InputColor::Primary,
                size: InputSize::Lg,
                class: "tabular",
                id: "routed-input",
                suffix: rsx! { "EUR" },
                wrapper_attributes: attributes!(span { class: "w-full" }),
                aria_label: "Routed input",
            }
        }
    }

    let (classes, dom) = mounted_class_attributes(app);
    let control = class_of(&classes, dom.control);
    let wrapper = wrapper_class_of(&classes, dom.control);

    for class in ["input", "input-primary", "input-lg", "w-full"] {
        assert!(
            wrapper.split_ascii_whitespace().any(|value| value == class),
            "wrapper should carry {class}, got {wrapper:?}"
        );
        assert!(
            !control.split_ascii_whitespace().any(|value| value == class),
            "the native input should not carry {class}, got {control:?}"
        );
    }
    assert!(dom.attributes.class_contains("tabular"));
    assert!(
        !wrapper
            .split_ascii_whitespace()
            .any(|value| value == "tabular")
    );
    assert_eq!(dom.attributes.get("id"), Some("routed-input"));
}

#[test]
fn adorned_field_invalidity_emits_input_error_on_the_wrapper() {
    fn app() -> Element {
        let meta = use_field_meta_state(FieldMetaValues {
            invalid: Some(true),
            ..FieldMetaValues::default()
        });

        rsx! {
            Field { context: FieldContext::empty().with_meta(meta),
                Input {
                    suffix: rsx! { "EUR" },
                    aria_label: "Invalid input",
                }
            }
        }
    }

    let (classes, dom) = mounted_class_attributes(app);
    let wrapper = wrapper_class_of(&classes, dom.control);

    assert!(
        wrapper
            .split_ascii_whitespace()
            .any(|value| value == "input-error"),
        "producer invalidity should emit input-error on the wrapper, got {wrapper:?}"
    );
    assert!(!dom.attributes.class_contains("input-error"));
    assert_eq!(dom.attributes.get("aria-invalid"), Some("true"));
}

// Adorned is `prefix.is_some() || suffix.is_some()`: an empty `Some` selects
// the wrapper arm so a conditional adornment keeps a stable slot instead of
// remounting the native input (ADR-0031).
#[test]
fn an_empty_some_selects_the_wrapper_arm_and_the_bare_arm_stays_bare() {
    fn adorned(harness: ()) -> Element {
        let () = harness;
        rsx! {
            Input {
                suffix: rsx! {},
                aria_label: "Empty slot input",
            }
        }
    }
    fn bare(harness: ()) -> Element {
        let () = harness;
        rsx! {
            Input { aria_label: "Bare input" }
        }
    }

    assert!(
        mounts_wrapper_mousedown(adorned),
        "an empty Some should render the wrapper and its focus-forward listener"
    );
    assert!(
        !mounts_wrapper_mousedown(bare),
        "the bare arm should render no wrapper or mousedown listener"
    );
}

/// Whether mounting the app registers a `mousedown` listener, which only the
/// adorned wrapper arm does.
fn mounts_wrapper_mousedown(app: fn(()) -> Element) -> bool {
    InteractionDom::mount_with_edits(VirtualDom::new_with_props(app, ()), "input", |edits| {
        edits.iter().any(|edit| {
            matches!(
                edit,
                dioxus_core::Mutation::NewEventListener { name, .. } if name == "mousedown"
            )
        })
    })
    .0
}

#[derive(Clone)]
struct SlotContentHarness {
    probe: FocusExitOrderProbe,
    content: Rc<RefCell<Option<Signal<&'static str>>>>,
}

fn slot_content_app(harness: SlotContentHarness) -> Element {
    let value = use_signal(String::new);
    let binding = harness.probe.binding(ReadSignal::from(value));
    let content = use_signal(|| "EUR");
    harness.content.borrow_mut().replace(content);

    rsx! {
        Input {
            binding,
            suffix: rsx! { "{content}" },
        }
    }
}

// Adornment content may change within `Some` mid-session without remounting
// the native input: the focus session survives the change and still reports
// its write, commit, and Focus Exit in order, once.
#[test]
fn adornment_content_changes_within_some_without_remounting_the_control() {
    let harness = SlotContentHarness {
        probe: FocusExitOrderProbe::new(),
        content: Rc::new(RefCell::new(None)),
    };
    let mut dom = InteractionDom::mount(
        VirtualDom::new_with_props(slot_content_app, harness.clone()),
        "input",
    );

    dom.dispatch("focusin", SerializedFocusData::default());
    harness
        .content
        .borrow_mut()
        .as_mut()
        .expect("app should expose its slot content signal")
        .set("USD");
    let mutations = dom.dom.render_immediate_to_vec();
    assert!(
        !mutations
            .edits
            .iter()
            .any(|edit| { matches!(edit, dioxus_core::Mutation::NewEventListener { .. }) }),
        "slot content changes must not remount the control: {:?}",
        mutations.edits
    );

    dispatch_input_event(&dom, "input", "changed");
    dispatch_input_event(&dom, "change", "changed");
    dom.dispatch("focusout", SerializedFocusData::default());

    harness.probe.assert_write_and_commit_before_focus_exit();
}

#[test]
fn focus_request_round_trips_to_the_widget_control() {
    assert_focus_request_round_trip(
        || {
            rsx! {
                Input { aria_label: "Focus target" }
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
                Input { aria_label: "Registered input" }
            }
        },
        "input",
        "input-description",
        "input-error",
        true,
    );
}
