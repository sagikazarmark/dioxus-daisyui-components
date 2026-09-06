use std::{any::Any, cell::RefCell, rc::Rc};

use dioxus::prelude::*;
use dioxus_field::{
    Binding, ChangeOrigin,
    testing::{FocusExitOrderProbe, FocusExitProbe},
};
use dioxus_html::{PlatformEventData, SerializedFocusData, SerializedFormData};

use dioxus_daisyui_components::components::{
    field::Field,
    otp::{Otp, OtpColor},
};

use crate::harness::*;

struct OtpDom {
    dom: VirtualDom,
    control: dioxus_core::ElementId,
    control_attributes: ControlAttributes,
    wrapper: dioxus_core::ElementId,
    wrapper_attributes: ControlAttributes,
}

impl OtpDom {
    fn mount(mut dom: VirtualDom) -> Self {
        set_test_event_converter();
        let mutations = dom.rebuild_to_vec();
        let control = mutations
            .edits
            .iter()
            .find_map(|edit| match edit {
                dioxus_core::Mutation::NewEventListener { name, id } if name == "input" => {
                    Some(*id)
                }
                _ => None,
            })
            .expect("OTP should render its input listener on the native input");
        let wrapper = mutations
            .edits
            .iter()
            .find_map(|edit| match edit {
                dioxus_core::Mutation::SetAttribute {
                    name: "class",
                    value: dioxus_core::AttributeValue::Text(value),
                    id,
                    ..
                } if value.split_ascii_whitespace().any(|class| class == "otp") => Some(*id),
                _ => None,
            })
            .expect("OTP should render its class on the label wrapper");
        let mut control_attributes = ControlAttributes::default();
        control_attributes.apply(&mutations.edits, control);
        let mut wrapper_attributes = ControlAttributes::default();
        wrapper_attributes.apply(&mutations.edits, wrapper);

        Self {
            dom,
            control,
            control_attributes,
            wrapper,
            wrapper_attributes,
        }
    }

    fn dispatch(&self, name: &str, value: &str) {
        let data = SerializedFormData::new(value.to_owned(), Vec::new());
        let data = PlatformEventData::new(Box::new(data));
        let event = Event::new(Rc::new(data) as Rc<dyn Any>, true);
        self.dom.runtime().handle_event(name, event, self.control);
    }

    fn dispatch_focus_out(&self) {
        let data = PlatformEventData::new(Box::new(SerializedFocusData::default()));
        let event = Event::new(Rc::new(data) as Rc<dyn Any>, true);
        self.dom
            .runtime()
            .handle_event("focusout", event, self.control);
    }
}

impl RendersReactiveUpdates for OtpDom {
    fn render_reactive_updates(&mut self) {
        let control = self.control;
        let control_attributes = &mut self.control_attributes;
        let wrapper = self.wrapper;
        let wrapper_attributes = &mut self.wrapper_attributes;
        settle(&mut self.dom, |mutations| {
            control_attributes.apply(&mutations.edits, control);
            wrapper_attributes.apply(&mutations.edits, wrapper);
        });
    }
}

fn interaction_app(harness: InteractionHarness<String>) -> Element {
    let value = use_signal(String::new);
    let trio = harness.trio(ReadSignal::from(value));

    rsx! {
        Otp {
            value: trio.value,
            on_change: trio.on_change,
            on_commit: trio.on_commit,
            aria_label: "Conformance OTP",
        }
    }
}

fn interaction_dom(harness: InteractionHarness<String>) -> OtpDom {
    OtpDom::mount(VirtualDom::new_with_props(interaction_app, harness))
}

#[derive(Clone)]
struct FocusExitHarness {
    binding_probe: FocusExitProbe,
    prop_probe: FocusExitProbe,
    reports: Rc<RefCell<Vec<&'static str>>>,
}

fn focus_exit_app(harness: FocusExitHarness) -> Element {
    let value = use_signal(String::new);
    let binding_probe = harness.binding_probe.on_focus_exit();
    let binding_reports = Rc::clone(&harness.reports);
    let binding: Binding<String> = value.into();
    let binding = binding.with_focus_exit(Callback::new(move |()| {
        binding_reports.borrow_mut().push("binding");
        binding_probe.call(());
    }));
    let prop_probe = harness.prop_probe.on_focus_exit();
    let prop_reports = Rc::clone(&harness.reports);

    rsx! {
        Otp {
            binding,
            on_focus_exit: move |()| {
                prop_reports.borrow_mut().push("prop");
                prop_probe.call(());
            },
            aria_label: "Focus-exit OTP",
        }
    }
}

fn focus_exit_order_app(probe: FocusExitOrderProbe) -> Element {
    let value = use_signal(String::new);

    rsx! {
        Otp {
            binding: probe.binding(ReadSignal::from(value)),
            aria_label: "Focus-exit order OTP",
        }
    }
}

fn resolution_app(harness: ResolutionHarness<String>) -> Element {
    let scaffold = use_resolution_scaffold(&harness, String::from("1234"), String::from("5678"));
    let context = scaffold.field_context();

    match harness.source {
        ResolutionSource::Explicit => rsx! {
            Field { context,
                Otp {
                    binding: scaffold.explicit_binding,
                    meta: scaffold.explicit_meta,
                    color: OtpColor::Primary,
                    id: "explicit-id",
                    name: "explicit-name",
                    required: false,
                    disabled: false,
                    aria_label: "Explicit OTP",
                }
            }
        },
        ResolutionSource::Context => rsx! {
            Field { context,
                Otp {
                    disabled: false,
                    required: false,
                    aria_label: "Context OTP",
                }
            }
        },
        ResolutionSource::Metadata => rsx! {
            Field { context,
                Otp {
                    color: OtpColor::Default,
                    aria_label: "Metadata OTP",
                }
            }
        },
        ResolutionSource::Internal => rsx! {
            Otp { aria_label: "Internal OTP" }
        },
    }
}

fn resolution_dom(harness: ResolutionHarness<String>) -> OtpDom {
    OtpDom::mount(VirtualDom::new_with_props(resolution_app, harness))
}

#[test]
fn commit_is_synchronously_observable_before_submit_handling_runs() {
    let input_harness = InteractionHarness::new();
    let input_dom = interaction_dom(input_harness.clone());

    input_dom.dispatch("input", "123");
    input_dom.dispatch("input", "abcd");
    input_dom.dispatch("input", "1234");
    input_dom.dispatch("change", "1234");
    input_dom.dom.in_runtime(|| input_harness.submit());

    input_harness.commits.assert_commit_before_submit();

    let change_harness = InteractionHarness::new();
    let change_dom = interaction_dom(change_harness.clone());

    change_dom.dispatch("input", "56");
    change_dom.dispatch("change", "56");
    change_dom.dom.in_runtime(|| change_harness.submit());

    change_harness.commits.assert_commit_before_submit();
}

#[test]
fn writes_carry_their_change_origin() {
    let harness = InteractionHarness::new();
    let dom = interaction_dom(harness.clone());

    dom.dispatch("input", "123");

    harness
        .changes
        .assert_writes(&[("123".to_owned(), ChangeOrigin::User)]);
}

#[test]
fn unchanged_departure_reports_rich_binding_then_direct_prop_once_without_commit() {
    let binding_probe = FocusExitProbe::new();
    let prop_probe = FocusExitProbe::new();
    let reports = Rc::new(RefCell::new(Vec::new()));
    let dom = OtpDom::mount(VirtualDom::new_with_props(
        focus_exit_app,
        FocusExitHarness {
            binding_probe: binding_probe.clone(),
            prop_probe: prop_probe.clone(),
            reports: Rc::clone(&reports),
        },
    ));

    dom.dispatch_focus_out();

    binding_probe.assert_focus_exit_once();
    prop_probe.assert_focus_exit_once();
    assert_eq!(*reports.borrow(), ["binding", "prop"]);
}

#[test]
fn native_change_commit_does_not_imply_focus_exit() {
    let probe = FocusExitOrderProbe::new();
    let dom = OtpDom::mount(VirtualDom::new_with_props(
        focus_exit_order_app,
        probe.clone(),
    ));

    dom.dispatch("change", "12");

    probe.assert_commit_without_focus_exit();
}

#[test]
fn complete_code_commit_stays_deduped_before_focus_exit() {
    let probe = FocusExitOrderProbe::new();
    let dom = OtpDom::mount(VirtualDom::new_with_props(
        focus_exit_order_app,
        probe.clone(),
    ));

    dom.dispatch("input", "1234");
    dom.dispatch("change", "1234");
    dom.dispatch_focus_out();

    probe.assert_write_and_commit_before_focus_exit();
}

#[test]
fn binding_resolution_precedence_holds_for_values_and_meta_flags() {
    assert_binding_resolution_precedence(PrecedenceSpec {
        resolution_dom,
        explicit: PrecedencePhase {
            assert_attributes: |dom| {
                let attributes = &dom.control_attributes;
                assert_eq!(attributes.get("value"), Some("1234"));
                assert_eq!(attributes.get("id"), Some("explicit-id"));
                assert_eq!(attributes.get("name"), Some("explicit-name"));
                assert!(!attributes.class_contains("otp-primary"));
                assert!(!attributes.class_contains("otp-error"));
                assert!(dom.wrapper_attributes.class_contains("otp-primary"));
                assert!(!dom.wrapper_attributes.class_contains("otp-error"));
                assert_eq!(attributes.get("aria-invalid"), Some("false"));
                assert!(!attributes.has("disabled"));
                assert!(!attributes.has("required"));
                assert!(!attributes.has("data-disabled"));
                assert!(!attributes.has("data-required"));
            },
            dispatch: |dom| dom.dispatch("input", "4321"),
            written: "4321".to_owned(),
        },
        context: PrecedencePhase {
            assert_attributes: |dom| {
                let attributes = &dom.control_attributes;
                assert_eq!(attributes.get("value"), Some("5678"));
                assert_eq!(attributes.get("id"), Some("context-id"));
                assert_eq!(attributes.get("name"), Some("context-name"));
                assert!(!attributes.class_contains("otp-error"));
                assert!(dom.wrapper_attributes.class_contains("otp-error"));
                assert_eq!(attributes.get("aria-invalid"), Some("true"));
                assert!(!attributes.has("disabled"));
                assert!(!attributes.has("required"));
                assert!(!attributes.has("data-disabled"));
                assert!(!attributes.has("data-required"));
                assert_eq!(attributes.get("data-touched"), Some("true"));
                assert_eq!(attributes.get("data-dirty"), Some("true"));
            },
            dispatch: |dom| dom.dispatch("input", "8765"),
            written: "8765".to_owned(),
        },
        assert_metadata: |dom| {
            let attributes = &dom.control_attributes;
            assert_eq!(attributes.get("id"), Some("context-id"));
            assert_eq!(attributes.get("name"), Some("context-name"));
            assert_eq!(attributes.get("aria-invalid"), Some("true"));
            assert!(!attributes.class_contains("otp-error"));
            assert!(!dom.wrapper_attributes.class_contains("otp-error"));
            assert!(attributes.has("disabled"));
            assert!(attributes.has("required"));
            assert_eq!(attributes.get("data-disabled"), Some("true"));
            assert_eq!(attributes.get("data-required"), Some("true"));
        },
        internal: InternalPhase {
            assert_mounted: |dom| {
                assert_eq!(dom.control_attributes.get("value"), Some(""));
                assert_eq!(dom.control_attributes.get("aria-invalid"), Some("false"));
            },
            dispatch: |dom| dom.dispatch("input", "123"),
            assert_updated: |dom| {
                assert_eq!(dom.control_attributes.get("value"), Some("123"));
            },
        },
    });
}

#[test]
fn focus_request_round_trips_to_the_widget_control() {
    assert_focus_request_round_trip(
        || {
            rsx! {
                Otp { aria_label: "Focus target" }
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
                Otp { aria_label: "Registered OTP" }
            }
        },
        "input",
        "otp-description",
        "otp-error",
        true,
    );
}
