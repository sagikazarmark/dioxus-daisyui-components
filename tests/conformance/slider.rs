use std::{cell::RefCell, rc::Rc};

use dioxus::prelude::*;
use dioxus_field::{
    Binding, ChangeOrigin,
    testing::{FocusExitOrderProbe, FocusExitProbe},
};
use dioxus_html::SerializedFocusData;

use dioxus_daisyui::components::{
    field::Field,
    slider::{Slider, SliderColor},
};

use crate::harness::*;

fn interaction_app(harness: InteractionHarness<f64>) -> Element {
    let value = use_signal(|| 40.0);
    let trio = harness.trio(ReadSignal::from(value));

    rsx! {
        Slider {
            value: Some((trio.value)()),
            on_change: trio.on_change,
            on_commit: trio.on_commit,
            label: "Conformance slider",
        }
    }
}

fn interaction_dom(harness: InteractionHarness<f64>) -> SliderInteractionDom {
    SliderInteractionDom::mount(VirtualDom::new_with_props(interaction_app, harness))
}

#[derive(Clone)]
struct FocusExitHarness {
    binding_probe: FocusExitProbe,
    prop_probe: FocusExitProbe,
    reports: Rc<RefCell<Vec<&'static str>>>,
}

fn focus_exit_app(harness: FocusExitHarness) -> Element {
    let value = use_signal(|| 40.0);
    let binding_probe = harness.binding_probe.on_focus_exit();
    let binding_reports = Rc::clone(&harness.reports);
    let binding: Binding<f64> = value.into();
    let binding = binding.with_focus_exit(Callback::new(move |()| {
        binding_reports.borrow_mut().push("binding");
        binding_probe.call(());
    }));
    let prop_probe = harness.prop_probe.on_focus_exit();
    let prop_reports = Rc::clone(&harness.reports);

    rsx! {
        Slider {
            binding,
            on_focus_exit: move |()| {
                prop_reports.borrow_mut().push("prop");
                prop_probe.call(());
            },
            label: "Focus-exit slider",
        }
    }
}

fn focus_exit_order_app(probe: FocusExitOrderProbe) -> Element {
    let value = use_signal(|| 40.0);

    rsx! {
        Slider {
            binding: probe.binding(ReadSignal::from(value)),
            label: "Focus-exit order slider",
        }
    }
}

fn resolution_app(harness: ResolutionHarness<f64>) -> Element {
    let scaffold = use_resolution_scaffold(&harness, 25.0, 50.0);
    let context = scaffold.field_context();

    match harness.source {
        ResolutionSource::Explicit => rsx! {
            Field { context,
                Slider {
                    binding: scaffold.explicit_binding,
                    meta: scaffold.explicit_meta,
                    color: SliderColor::Primary,
                    id: "explicit-id",
                    name: "explicit-name",
                    required: false,
                    disabled: false,
                    label: "Explicit slider",
                }
            }
        },
        ResolutionSource::Context => rsx! {
            Field { context,
                Slider {
                    required: false,
                    disabled: false,
                    label: "Context slider",
                }
            }
        },
        ResolutionSource::Metadata => rsx! {
            Field { context,
                Slider {
                    color: SliderColor::Default,
                    label: "Metadata slider",
                }
            }
        },
        ResolutionSource::Internal => rsx! {
            Slider { label: "Internal slider" }
        },
    }
}

fn resolution_dom(harness: ResolutionHarness<f64>) -> SliderInteractionDom {
    SliderInteractionDom::mount(VirtualDom::new_with_props(resolution_app, harness))
}

#[test]
fn commit_is_synchronously_observable_before_submit_handling_runs() {
    let harness = InteractionHarness::new();
    let dom = interaction_dom(harness.clone());

    dom.dispatch_arrow("keydown");
    dom.dispatch_arrow("keyup");
    dom.interaction.dom.in_runtime(|| harness.submit());

    harness.commits.assert_commit_before_submit();
}

#[test]
fn writes_carry_their_change_origin() {
    let harness = InteractionHarness::new();
    let dom = interaction_dom(harness.clone());

    dom.dispatch_arrow("keydown");

    harness.changes.assert_writes(&[(41.0, ChangeOrigin::User)]);
}

#[test]
fn key_commit_does_not_imply_focus_exit_and_later_exit_reports_both_surfaces_once() {
    let binding_probe = FocusExitProbe::new();
    let prop_probe = FocusExitProbe::new();
    let reports = Rc::new(RefCell::new(Vec::new()));
    let mut dom = SliderInteractionDom::mount(VirtualDom::new_with_props(
        focus_exit_app,
        FocusExitHarness {
            binding_probe: binding_probe.clone(),
            prop_probe: prop_probe.clone(),
            reports: Rc::clone(&reports),
        },
    ));

    dom.dispatch_arrow("keydown");
    dom.dispatch_arrow("keyup");

    binding_probe.assert_no_focus_exit();
    prop_probe.assert_no_focus_exit();

    dom.interaction
        .dispatch("focusout", SerializedFocusData::default());
    dom.render_reactive_updates();

    binding_probe.assert_focus_exit_once();
    prop_probe.assert_focus_exit_once();
    assert_eq!(*reports.borrow(), ["binding", "prop"]);
}

#[test]
fn write_and_commit_are_observed_before_focus_exit() {
    let probe = FocusExitOrderProbe::new();
    let mut dom = SliderInteractionDom::mount(VirtualDom::new_with_props(
        focus_exit_order_app,
        probe.clone(),
    ));

    dom.dispatch_arrow("keydown");
    dom.dispatch_arrow("keyup");
    dom.interaction
        .dispatch("focusout", SerializedFocusData::default());
    dom.render_reactive_updates();

    probe.assert_write_and_commit_before_focus_exit();
}

#[test]
fn binding_resolution_precedence_holds_for_values_and_meta_flags() {
    assert_binding_resolution_precedence(PrecedenceSpec {
        resolution_dom,
        explicit: PrecedencePhase {
            assert_attributes: |dom| {
                assert_eq!(dom.interaction.attributes.get("aria-valuenow"), Some("25"));
                assert_eq!(dom.root_attributes.get("id"), Some("explicit-id"));
                assert!(!dom.root_attributes.has("name"));
                assert!(dom.root_attributes.class_contains("range-primary"));
                assert!(!dom.root_attributes.class_contains("range-error"));
                assert!(!dom.root_attributes.has("aria-invalid"));
                assert!(!dom.root_attributes.has("disabled"));
                assert!(!dom.root_attributes.has("required"));
                assert!(!dom.root_attributes.has("data-required"));
            },
            dispatch: |dom| dom.dispatch_arrow("keydown"),
            written: 26.0,
        },
        context: PrecedencePhase {
            assert_attributes: |dom| {
                assert_eq!(dom.interaction.attributes.get("aria-valuenow"), Some("50"));
                assert_eq!(dom.root_attributes.get("id"), Some("context-id"));
                assert!(!dom.root_attributes.has("name"));
                assert!(dom.root_attributes.class_contains("range-error"));
                assert!(!dom.root_attributes.has("aria-invalid"));
                assert!(!dom.root_attributes.has("disabled"));
                assert!(!dom.root_attributes.has("required"));
                assert!(!dom.root_attributes.has("data-required"));
                assert_eq!(dom.root_attributes.get("data-touched"), Some("true"));
                assert_eq!(dom.root_attributes.get("data-dirty"), Some("true"));
            },
            dispatch: |dom| dom.dispatch_arrow("keydown"),
            written: 51.0,
        },
        assert_metadata: |dom| {
            assert_eq!(dom.root_attributes.get("id"), Some("context-id"));
            assert!(!dom.root_attributes.has("name"));
            assert!(!dom.root_attributes.has("aria-invalid"));
            assert!(!dom.root_attributes.class_contains("range-error"));
            assert_eq!(dom.root_attributes.get("aria-disabled"), Some("true"));
            assert_eq!(dom.root_attributes.get("aria-required"), Some("true"));
            assert!(!dom.root_attributes.has("disabled"));
            assert!(!dom.root_attributes.has("required"));
            assert_eq!(dom.root_attributes.get("data-disabled"), Some("true"));
            assert_eq!(dom.root_attributes.get("data-required"), Some("true"));
        },
        internal: InternalPhase {
            assert_mounted: |dom| {
                assert_eq!(dom.interaction.attributes.get("aria-valuenow"), Some("0"));
                assert!(!dom.root_attributes.has("aria-invalid"));
            },
            dispatch: |dom| dom.dispatch_arrow("keydown"),
            assert_updated: |dom| {
                assert_eq!(dom.interaction.attributes.get("aria-valuenow"), Some("1"));
            },
        },
    });
}

#[test]
fn focus_request_round_trips_to_the_widget_control() {
    assert_focus_request_round_trip(
        || {
            rsx! {
                Slider { label: "Focus target" }
            }
        },
        mount_slider_control,
    );
}

#[test]
fn error_and_description_ids_appear_on_mount_and_vanish_on_drop() {
    let harness = Rc::new(IdRegistrationHarness::default());
    let mut dom = SliderInteractionDom::mount(VirtualDom::new_with_props(
        id_registration_app,
        IdRegistrationApp {
            harness: Rc::clone(&harness),
            control: || {
                rsx! {
                    Slider { label: "Registered slider" }
                }
            },
            description_id: "slider-description",
            error_id: "slider-error",
        },
    ));
    dom.render_reactive_updates();

    assert_eq!(
        dom.root_attributes.get("aria-describedby"),
        Some("slider-description slider-error")
    );
    assert!(!dom.root_attributes.has("aria-errormessage"));

    hide_registered_parts(&harness, &mut dom);
    assert!(!dom.root_attributes.has("aria-describedby"));
    assert!(!dom.root_attributes.has("aria-errormessage"));
}
