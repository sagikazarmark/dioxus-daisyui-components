use std::{cell::Cell, rc::Rc};

use dioxus::prelude::*;
use dioxus_field::{
    Binding, ChangeOrigin,
    testing::{FocusExitOrderProbe, FocusExitProbe},
};
use dioxus_html::{SerializedFocusData, SerializedMouseData};

use dioxus_daisyui_components::components::{
    field::Field,
    switch::{Switch, SwitchColor},
};

use crate::harness::*;

fn interaction_app(harness: InteractionHarness<bool>) -> Element {
    let value = use_signal(|| false);
    let trio = harness.trio(ReadSignal::from(value));

    rsx! {
        Switch {
            value: Some((trio.value)()),
            on_change: trio.on_change,
            on_commit: trio.on_commit,
            aria_label: "Conformance switch",
        }
    }
}

fn interaction_dom(harness: InteractionHarness<bool>) -> InteractionDom {
    InteractionDom::mount(
        VirtualDom::new_with_props(interaction_app, harness),
        "click",
    )
}

fn click_switch(dom: &InteractionDom) {
    dom.dispatch("click", SerializedMouseData::default());
}

#[derive(Clone)]
struct ToggleFocusHarness {
    focus_exits: FocusExitProbe,
    commits: Rc<Cell<usize>>,
}

fn toggle_focus_app(harness: ToggleFocusHarness) -> Element {
    let value = use_signal(|| false);
    let commits = Rc::clone(&harness.commits);
    let binding = Binding::new(
        ReadSignal::from(value),
        Callback::new(|_| {}),
        Callback::new(move |()| commits.set(commits.get() + 1)),
    )
    .with_focus_exit(harness.focus_exits.on_focus_exit());

    rsx! { Switch { binding, aria_label: "Focus contract switch" } }
}

#[derive(Clone)]
struct FocusExitHarness {
    probe: FocusExitOrderProbe,
    direct_calls: Rc<Cell<usize>>,
}

fn focus_exit_app(harness: FocusExitHarness) -> Element {
    let value = use_signal(|| false);
    let binding = harness.probe.binding(ReadSignal::from(value));
    let callback_probe = harness.probe.clone();
    let direct_calls = Rc::clone(&harness.direct_calls);

    rsx! {
        Switch {
            binding,
            on_focus_exit: move |()| {
                callback_probe.assert_write_and_commit_before_focus_exit();
                direct_calls.set(direct_calls.get() + 1);
            },
            aria_label: "Ordered focus contract switch",
        }
    }
}

fn resolution_app(harness: ResolutionHarness<bool>) -> Element {
    let scaffold = use_resolution_scaffold(&harness, true, false);
    let context = scaffold.field_context();

    match harness.source {
        ResolutionSource::Explicit => rsx! {
            Field { context,
                Switch {
                    binding: scaffold.explicit_binding,
                    meta: scaffold.explicit_meta,
                    color: SwitchColor::Primary,
                    id: "explicit-id",
                    name: "explicit-name",
                    required: false,
                    disabled: false,
                    aria_label: "Explicit switch",
                }
            }
        },
        ResolutionSource::Context => rsx! {
            Field { context,
                Switch {
                    disabled: false,
                    aria_label: "Context switch",
                }
            }
        },
        ResolutionSource::Metadata => rsx! {
            Field { context,
                Switch {
                    color: SwitchColor::Default,
                    aria_label: "Metadata switch",
                }
            }
        },
        ResolutionSource::Internal => rsx! {
            Switch { aria_label: "Internal switch" }
        },
    }
}

fn resolution_dom(harness: ResolutionHarness<bool>) -> InteractionDom {
    InteractionDom::mount(VirtualDom::new_with_props(resolution_app, harness), "click")
}

#[test]
fn commit_is_synchronously_observable_before_submit_handling_runs() {
    let harness = InteractionHarness::new();
    let dom = interaction_dom(harness.clone());

    click_switch(&dom);
    dom.dom.in_runtime(|| harness.submit());

    harness.commits.assert_commit_before_submit();
}

#[test]
fn writes_carry_their_change_origin() {
    let harness = InteractionHarness::new();
    let dom = interaction_dom(harness.clone());

    click_switch(&dom);

    harness.changes.assert_writes(&[(true, ChangeOrigin::User)]);
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

    click_switch(&dom);

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

    click_switch(&dom);
    dom.dispatch("focusout", SerializedFocusData::default());

    harness.probe.assert_write_and_commit_before_focus_exit();
    assert_eq!(harness.direct_calls.get(), 1);
}

#[test]
fn binding_resolution_precedence_holds_for_values_and_meta_flags() {
    assert_binding_resolution_precedence(PrecedenceSpec {
        resolution_dom,
        explicit: PrecedencePhase {
            assert_attributes: |dom| {
                let attributes = &dom.attributes;
                // The primitive's bool-valued ARIA mutation has presence semantics in
                // VirtualDom, while its string-valued data state preserves both values.
                assert_eq!(attributes.get("data-state"), Some("checked"));
                assert_eq!(attributes.get("id"), Some("explicit-id"));
                assert_eq!(attributes.get("name"), Some("explicit-name"));
                assert!(attributes.class_contains("toggle-primary"));
                assert!(!attributes.class_contains("toggle-error"));
                assert_eq!(attributes.get("aria-invalid"), Some("false"));
                assert!(!attributes.has("disabled"));
                assert!(!attributes.has("required"));
                assert_eq!(attributes.get("data-disabled"), Some("false"));
                assert!(!attributes.has("data-required"));
            },
            dispatch: |dom| click_switch(dom),
            written: false,
        },
        context: PrecedencePhase {
            assert_attributes: |dom| {
                let attributes = &dom.attributes;
                assert_eq!(attributes.get("data-state"), Some("unchecked"));
                assert_eq!(attributes.get("id"), Some("context-id"));
                assert_eq!(attributes.get("name"), Some("context-name"));
                assert!(attributes.class_contains("toggle-error"));
                assert_eq!(attributes.get("aria-invalid"), Some("true"));
                assert_eq!(attributes.get("data-required"), Some("true"));
                assert_eq!(attributes.get("data-touched"), Some("true"));
                assert_eq!(attributes.get("data-dirty"), Some("true"));
                assert!(!attributes.has("disabled"));
                assert_eq!(attributes.get("data-disabled"), Some("false"));
            },
            dispatch: |dom| click_switch(dom),
            written: true,
        },
        assert_metadata: |dom| {
            let attributes = &dom.attributes;
            assert_eq!(attributes.get("id"), Some("context-id"));
            assert_eq!(attributes.get("name"), Some("context-name"));
            assert_eq!(attributes.get("aria-invalid"), Some("true"));
            assert!(!attributes.class_contains("toggle-error"));
            assert!(attributes.has("disabled"));
            assert!(!attributes.has("required"));
            assert_eq!(attributes.get("aria-required"), Some("true"));
            assert_eq!(attributes.get("data-disabled"), Some("true"));
            assert_eq!(attributes.get("data-required"), Some("true"));
        },
        internal: InternalPhase {
            assert_mounted: |dom| {
                assert_eq!(dom.attributes.get("data-state"), Some("unchecked"));
                assert_eq!(dom.attributes.get("aria-invalid"), Some("false"));
                assert!(!dom.attributes.has("name"));
                assert!(!dom.attributes.has("required"));
            },
            dispatch: |dom| click_switch(dom),
            assert_updated: |dom| {
                assert_eq!(dom.attributes.get("data-state"), Some("checked"));
            },
        },
    });
}

#[test]
fn focus_request_round_trips_to_the_widget_control() {
    assert_focus_request_round_trip(
        || {
            rsx! {
                Switch { aria_label: "Focus target" }
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
                Switch { aria_label: "Registered switch" }
            }
        },
        "click",
        "switch-description",
        "switch-error",
        true,
    );
}
