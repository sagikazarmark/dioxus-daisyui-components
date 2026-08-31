use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use dioxus::prelude::*;
use dioxus_field::{
    Binding, ChangeOrigin, FieldContext,
    testing::{FocusExitOrderProbe, FocusExitProbe},
};
use dioxus_html::{SerializedFocusData, SerializedMouseData};

use dioxus_daisyui::components::{
    checkbox::{Checkbox, CheckboxColor, CheckboxState},
    field::Field,
};

use crate::harness::*;

fn interaction_app(harness: InteractionHarness<CheckboxState>) -> Element {
    let value = use_signal(|| CheckboxState::Unchecked);
    let trio = harness.trio(ReadSignal::from(value));

    rsx! {
        Checkbox {
            value: Some((trio.value)()),
            on_change: trio.on_change,
            on_commit: trio.on_commit,
            aria_label: "Conformance checkbox",
        }
    }
}

fn interaction_dom(harness: InteractionHarness<CheckboxState>) -> InteractionDom {
    InteractionDom::mount(
        VirtualDom::new_with_props(interaction_app, harness),
        "click",
    )
}

fn bool_binding_interaction_app(harness: InteractionHarness<bool>) -> Element {
    let value = use_signal(|| false);
    let binding = harness.binding(ReadSignal::from(value));

    rsx! {
        Checkbox {
            bool_binding: binding,
            aria_label: "Bool binding checkbox",
        }
    }
}

fn bool_binding_interaction_dom(harness: InteractionHarness<bool>) -> InteractionDom {
    InteractionDom::mount(
        VirtualDom::new_with_props(bool_binding_interaction_app, harness),
        "click",
    )
}

#[derive(Clone, Default)]
struct ProgrammaticBoolHarness {
    binding: Rc<RefCell<Option<Binding<bool>>>>,
    writes: Rc<RefCell<Vec<(bool, ChangeOrigin)>>>,
}

fn programmatic_bool_app(harness: ProgrammaticBoolHarness) -> Element {
    let mut value = use_signal(|| false);
    let writes = Rc::clone(&harness.writes);
    let binding = Binding::new(
        ReadSignal::from(value),
        Callback::new(move |(next, origin)| {
            writes.borrow_mut().push((next, origin));
            value.set(next);
        }),
        Callback::new(|()| {}),
    );
    harness.binding.borrow_mut().replace(binding.clone());

    rsx! {
        Checkbox {
            bool_binding: binding,
            aria_label: "Programmatic bool checkbox",
        }
    }
}

#[derive(Clone)]
struct ExplicitBindingPrecedenceHarness {
    state_changes: dioxus_field::testing::ChangeOriginProbe<CheckboxState>,
    bool_changes: dioxus_field::testing::ChangeOriginProbe<bool>,
}

fn explicit_binding_precedence_app(harness: ExplicitBindingPrecedenceHarness) -> Element {
    let state = use_signal(|| CheckboxState::Checked);
    let boolean = use_signal(|| false);
    let binding = harness.state_changes.binding(ReadSignal::from(state));
    let bool_binding = harness.bool_changes.binding(ReadSignal::from(boolean));

    rsx! {
        Checkbox {
            binding,
            bool_binding,
            aria_label: "Explicit binding precedence checkbox",
        }
    }
}

fn bool_binding_over_state_context_app(harness: ExplicitBindingPrecedenceHarness) -> Element {
    let state = use_signal(|| CheckboxState::Checked);
    let boolean = use_signal(|| false);
    let context_binding = harness.state_changes.binding(ReadSignal::from(state));
    let bool_binding = harness.bool_changes.binding(ReadSignal::from(boolean));
    let context = FieldContext::new(context_binding);

    rsx! {
        Field { context,
            Checkbox {
                bool_binding,
                aria_label: "Bool binding over state context checkbox",
            }
        }
    }
}

fn click_checkbox(dom: &InteractionDom) {
    dom.dispatch("click", SerializedMouseData::default());
}

#[derive(Clone)]
struct ToggleFocusHarness {
    focus_exits: FocusExitProbe,
    commits: Rc<Cell<usize>>,
}

fn toggle_focus_app(harness: ToggleFocusHarness) -> Element {
    let value = use_signal(|| CheckboxState::Unchecked);
    let commits = Rc::clone(&harness.commits);
    let binding = Binding::new(
        ReadSignal::from(value),
        Callback::new(|_| {}),
        Callback::new(move |()| commits.set(commits.get() + 1)),
    )
    .with_focus_exit(harness.focus_exits.on_focus_exit());

    rsx! { Checkbox { binding, aria_label: "Focus contract checkbox" } }
}

#[derive(Clone)]
struct FocusExitHarness {
    probe: FocusExitOrderProbe,
    direct_calls: Rc<Cell<usize>>,
}

fn focus_exit_app(harness: FocusExitHarness) -> Element {
    let value = use_signal(|| CheckboxState::Unchecked);
    let binding = harness.probe.binding(ReadSignal::from(value));
    let callback_probe = harness.probe.clone();
    let direct_calls = Rc::clone(&harness.direct_calls);

    rsx! {
        Checkbox {
            binding,
            on_focus_exit: move |()| {
                callback_probe.assert_write_and_commit_before_focus_exit();
                direct_calls.set(direct_calls.get() + 1);
            },
            aria_label: "Ordered focus contract checkbox",
        }
    }
}

fn bool_focus_exit_app(harness: FocusExitHarness) -> Element {
    let value = use_signal(|| false);
    let binding = harness.probe.binding(ReadSignal::from(value));
    let callback_probe = harness.probe.clone();
    let direct_calls = Rc::clone(&harness.direct_calls);

    rsx! {
        Checkbox {
            bool_binding: binding,
            on_focus_exit: move |()| {
                callback_probe.assert_write_and_commit_before_focus_exit();
                direct_calls.set(direct_calls.get() + 1);
            },
            aria_label: "Ordered bool focus contract checkbox",
        }
    }
}

fn resolution_app(harness: ResolutionHarness<CheckboxState>) -> Element {
    let scaffold = use_resolution_scaffold(
        &harness,
        CheckboxState::Indeterminate,
        CheckboxState::Unchecked,
    );
    let context = scaffold.field_context();

    match harness.source {
        ResolutionSource::Explicit => rsx! {
            Field { context,
                Checkbox {
                    binding: scaffold.explicit_binding,
                    meta: scaffold.explicit_meta,
                    color: CheckboxColor::Primary,
                    id: "explicit-id",
                    name: "explicit-name",
                    required: false,
                    disabled: false,
                    aria_label: "Explicit checkbox",
                }
            }
        },
        ResolutionSource::Context => rsx! {
            Field { context,
                Checkbox {
                    disabled: false,
                    aria_label: "Context checkbox",
                }
            }
        },
        ResolutionSource::Metadata => rsx! {
            Field { context,
                Checkbox {
                    color: CheckboxColor::Default,
                    aria_label: "Metadata checkbox",
                }
            }
        },
        ResolutionSource::Internal => rsx! {
            Checkbox { aria_label: "Internal checkbox" }
        },
    }
}

fn resolution_dom(harness: ResolutionHarness<CheckboxState>) -> InteractionDom {
    InteractionDom::mount(VirtualDom::new_with_props(resolution_app, harness), "click")
}

fn bool_resolution_app(harness: ResolutionHarness<bool>) -> Element {
    let scaffold = use_resolution_scaffold(&harness, true, false);
    let context = scaffold.field_context();

    match harness.source {
        ResolutionSource::Explicit => rsx! {
            Field { context,
                Checkbox {
                    bool_binding: scaffold.explicit_binding,
                    meta: scaffold.explicit_meta,
                    color: CheckboxColor::Primary,
                    id: "explicit-id",
                    name: "explicit-name",
                    required: false,
                    disabled: false,
                    aria_label: "Explicit bool checkbox",
                }
            }
        },
        ResolutionSource::Context => rsx! {
            Field { context,
                Checkbox {
                    disabled: false,
                    aria_label: "Context bool checkbox",
                }
            }
        },
        ResolutionSource::Metadata => rsx! {
            Field { context,
                Checkbox {
                    color: CheckboxColor::Default,
                    aria_label: "Bool metadata checkbox",
                }
            }
        },
        ResolutionSource::Internal => rsx! {
            Checkbox { aria_label: "Internal bool precedence checkbox" }
        },
    }
}

fn bool_resolution_dom(harness: ResolutionHarness<bool>) -> InteractionDom {
    InteractionDom::mount(
        VirtualDom::new_with_props(bool_resolution_app, harness),
        "click",
    )
}

fn assert_explicit_checkbox_attributes(dom: &InteractionDom, checked: &str) {
    let attributes = &dom.attributes;
    assert_eq!(attributes.get("aria-checked"), Some(checked));
    assert_eq!(attributes.get("id"), Some("explicit-id"));
    assert_eq!(attributes.get("name"), Some("explicit-name"));
    assert!(attributes.class_contains("checkbox-primary"));
    assert!(!attributes.class_contains("checkbox-error"));
    assert_eq!(attributes.get("aria-invalid"), Some("false"));
    assert!(!attributes.has("disabled"));
    assert!(!attributes.has("required"));
    assert!(!attributes.has("data-disabled"));
    assert!(!attributes.has("data-required"));
}

fn assert_indeterminate_explicit_checkbox(dom: &InteractionDom) {
    assert_explicit_checkbox_attributes(dom, "mixed");
}

fn assert_checked_explicit_checkbox(dom: &InteractionDom) {
    assert_explicit_checkbox_attributes(dom, "true");
}

fn assert_checkbox_binding_resolution_precedence<T>(
    resolution_dom: fn(ResolutionHarness<T>) -> InteractionDom,
    assert_explicit: fn(&InteractionDom),
    explicit_written: T,
    context_written: T,
) where
    T: std::fmt::Debug + PartialEq + 'static,
{
    assert_binding_resolution_precedence(PrecedenceSpec {
        resolution_dom,
        explicit: PrecedencePhase {
            assert_attributes: assert_explicit,
            dispatch: |dom| click_checkbox(dom),
            written: explicit_written,
        },
        context: PrecedencePhase {
            assert_attributes: |dom| {
                let attributes = &dom.attributes;
                assert_eq!(attributes.get("aria-checked"), Some("false"));
                assert_eq!(attributes.get("id"), Some("context-id"));
                assert_eq!(attributes.get("name"), Some("context-name"));
                assert!(attributes.class_contains("checkbox-error"));
                assert_eq!(attributes.get("aria-invalid"), Some("true"));
                assert_eq!(attributes.get("data-required"), Some("true"));
                assert_eq!(attributes.get("data-touched"), Some("true"));
                assert_eq!(attributes.get("data-dirty"), Some("true"));
                assert!(!attributes.has("disabled"));
            },
            dispatch: |dom| click_checkbox(dom),
            written: context_written,
        },
        assert_metadata: |dom| {
            let attributes = &dom.attributes;
            assert_eq!(attributes.get("id"), Some("context-id"));
            assert_eq!(attributes.get("name"), Some("context-name"));
            assert_eq!(attributes.get("aria-invalid"), Some("true"));
            assert!(!attributes.class_contains("checkbox-error"));
            assert!(attributes.has("disabled"));
            assert!(!attributes.has("required"));
            assert_eq!(attributes.get("aria-required"), Some("true"));
            assert_eq!(attributes.get("data-disabled"), Some("true"));
            assert_eq!(attributes.get("data-required"), Some("true"));
        },
        internal: InternalPhase {
            assert_mounted: |dom| {
                assert_eq!(dom.attributes.get("aria-checked"), Some("false"));
                assert_eq!(dom.attributes.get("aria-invalid"), Some("false"));
                assert!(!dom.attributes.has("name"));
                assert!(!dom.attributes.has("required"));
            },
            dispatch: |dom| click_checkbox(dom),
            assert_updated: |dom| {
                assert_eq!(dom.attributes.get("aria-checked"), Some("true"));
            },
        },
    });
}

#[test]
fn commit_is_synchronously_observable_before_submit_handling_runs() {
    let harness = InteractionHarness::new();
    let dom = interaction_dom(harness.clone());

    click_checkbox(&dom);
    dom.dom.in_runtime(|| harness.submit());

    harness.commits.assert_commit_before_submit();
}

#[test]
fn writes_carry_their_change_origin() {
    let harness = InteractionHarness::new();
    let dom = interaction_dom(harness.clone());

    click_checkbox(&dom);

    harness
        .changes
        .assert_writes(&[(CheckboxState::Checked, ChangeOrigin::User)]);
}

#[test]
fn explicit_bool_binding_reads_writes_and_commits() {
    let harness = InteractionHarness::new();
    let dom = bool_binding_interaction_dom(harness.clone());

    assert_eq!(dom.attributes.get("aria-checked"), Some("false"));
    click_checkbox(&dom);
    dom.dom.in_runtime(|| harness.submit());

    harness.changes.assert_writes(&[(true, ChangeOrigin::User)]);
    harness.commits.assert_commit_before_submit();
}

#[test]
fn programmatic_bool_writes_retain_their_origin_and_update_rendering() {
    let harness = ProgrammaticBoolHarness::default();
    let mut dom = InteractionDom::mount(
        VirtualDom::new_with_props(programmatic_bool_app, harness.clone()),
        "click",
    );
    let binding = harness
        .binding
        .borrow()
        .clone()
        .expect("app should expose its bool binding");

    dom.dom
        .in_runtime(|| binding.write(true, ChangeOrigin::Programmatic));
    dom.render_reactive_updates();

    assert_eq!(dom.attributes.get("aria-checked"), Some("true"));
    assert_eq!(
        harness.writes.borrow().as_slice(),
        &[(true, ChangeOrigin::Programmatic)]
    );

    click_checkbox(&dom);
    assert_eq!(
        harness.writes.borrow().as_slice(),
        &[
            (true, ChangeOrigin::Programmatic),
            (false, ChangeOrigin::User),
        ]
    );
}

#[test]
fn checkbox_state_binding_wins_when_both_explicit_forms_are_supplied() {
    let harness = ExplicitBindingPrecedenceHarness {
        state_changes: dioxus_field::testing::ChangeOriginProbe::new(),
        bool_changes: dioxus_field::testing::ChangeOriginProbe::new(),
    };
    let dom = InteractionDom::mount(
        VirtualDom::new_with_props(explicit_binding_precedence_app, harness.clone()),
        "click",
    );

    assert_eq!(dom.attributes.get("aria-checked"), Some("true"));
    click_checkbox(&dom);

    harness
        .state_changes
        .assert_writes(&[(CheckboxState::Unchecked, ChangeOrigin::User)]);
    harness.bool_changes.assert_writes(&[]);
}

#[test]
fn explicit_bool_binding_wins_over_checkbox_state_field_context() {
    let harness = ExplicitBindingPrecedenceHarness {
        state_changes: dioxus_field::testing::ChangeOriginProbe::new(),
        bool_changes: dioxus_field::testing::ChangeOriginProbe::new(),
    };
    let dom = InteractionDom::mount(
        VirtualDom::new_with_props(bool_binding_over_state_context_app, harness.clone()),
        "click",
    );

    assert_eq!(dom.attributes.get("aria-checked"), Some("false"));
    click_checkbox(&dom);

    harness.state_changes.assert_writes(&[]);
    harness
        .bool_changes
        .assert_writes(&[(true, ChangeOrigin::User)]);
}

#[test]
fn toggle_commits_without_focus_exit() {
    let harness = ToggleFocusHarness {
        focus_exits: FocusExitProbe::new(),
        commits: Rc::new(Cell::new(0)),
    };
    let dom = InteractionDom::mount(
        VirtualDom::new_with_props(toggle_focus_app, harness.clone()),
        "click",
    );

    click_checkbox(&dom);

    assert_eq!(harness.commits.get(), 1);
    harness.focus_exits.assert_no_focus_exit();
}

#[test]
fn focus_exit_follows_write_and_commit_once() {
    let harness = FocusExitHarness {
        probe: FocusExitOrderProbe::new(),
        direct_calls: Rc::new(Cell::new(0)),
    };
    let dom = InteractionDom::mount(
        VirtualDom::new_with_props(focus_exit_app, harness.clone()),
        "click",
    );

    click_checkbox(&dom);
    dom.dispatch("focusout", SerializedFocusData::default());

    harness.probe.assert_write_and_commit_before_focus_exit();
    assert_eq!(harness.direct_calls.get(), 1);
}

#[test]
fn bool_focus_exit_follows_write_and_commit_once() {
    let harness = FocusExitHarness {
        probe: FocusExitOrderProbe::new(),
        direct_calls: Rc::new(Cell::new(0)),
    };
    let dom = InteractionDom::mount(
        VirtualDom::new_with_props(bool_focus_exit_app, harness.clone()),
        "click",
    );

    click_checkbox(&dom);
    dom.dispatch("focusout", SerializedFocusData::default());

    harness.probe.assert_write_and_commit_before_focus_exit();
    assert_eq!(harness.direct_calls.get(), 1);
}

#[test]
fn binding_resolution_precedence_holds_for_values_and_meta_flags() {
    assert_checkbox_binding_resolution_precedence(
        resolution_dom,
        assert_indeterminate_explicit_checkbox,
        CheckboxState::Unchecked,
        CheckboxState::Checked,
    );
}

#[test]
fn bool_binding_resolution_precedence_holds_for_values_and_meta_flags() {
    assert_checkbox_binding_resolution_precedence(
        bool_resolution_dom,
        assert_checked_explicit_checkbox,
        false,
        true,
    );
}

#[test]
fn focus_request_round_trips_to_the_widget_control() {
    assert_focus_request_round_trip(
        || {
            rsx! {
                Checkbox { aria_label: "Focus target" }
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
                Checkbox { aria_label: "Registered checkbox" }
            }
        },
        "click",
        "checkbox-description",
        "checkbox-error",
        true,
    );
}
