use std::{cell::Cell, rc::Rc};

use dioxus::prelude::*;
use dioxus_field::{
    Binding, ChangeOrigin,
    testing::{ChangeOriginProbe, FocusExitOrderProbe, FocusExitProbe},
};
use dioxus_html::{SerializedFocusData, SerializedPointerData};

use dioxus_daisyui::components::{
    field::Field,
    select::{Select, SelectColor, SelectList, SelectOption, SelectTrigger, SelectValue},
};

use crate::harness::*;

fn interaction_app(harness: InteractionHarness<Option<String>>) -> Element {
    let value = use_signal(|| Option::<String>::None);
    let trio = harness.trio(ReadSignal::from(value));

    rsx! {
        Select::<String> {
            value: Some(trio.value),
            on_change: trio.on_change,
            on_commit: trio.on_commit,
            open: Some(true),
            SelectTrigger { aria_label: "Conformance select",
                SelectValue {}
            }
            SelectList {
                SelectOption::<String> {
                    value: "selected value".to_owned(),
                    index: 0usize,
                    "Selected value"
                }
            }
        }
    }
}

fn interaction_dom(harness: InteractionHarness<Option<String>>) -> InteractionDom {
    let dom = InteractionDom::mount(
        VirtualDom::new_with_props(interaction_app, harness),
        "click",
    );
    retarget_to_option(dom)
}

#[derive(Clone)]
struct FocusExitHarness {
    binding_probe: FocusExitProbe,
    commits: Rc<Cell<usize>>,
}

fn focus_exit_app(harness: FocusExitHarness) -> Element {
    let value = use_signal(|| Option::<String>::None);
    let commits = Rc::clone(&harness.commits);
    let binding = Binding::new(
        ReadSignal::from(value),
        Callback::new(|_| {}),
        Callback::new(move |()| commits.set(commits.get() + 1)),
    )
    .with_focus_exit(harness.binding_probe.on_focus_exit());

    rsx! {
        Select::<String> { binding, open: Some(true),
            SelectTrigger { aria_label: "Focus Exit select",
                SelectValue {}
            }
            SelectList {
                SelectOption::<String> {
                    value: "selected value".to_owned(),
                    index: 0usize,
                    "Selected value"
                }
            }
        }
    }
}

#[derive(Clone)]
struct FocusExitOrderHarness {
    order_probe: FocusExitOrderProbe,
    prop_probe: FocusExitProbe,
}

fn focus_exit_order_app(harness: FocusExitOrderHarness) -> Element {
    let value = use_signal(|| Option::<String>::None);
    let binding = harness.order_probe.binding(ReadSignal::from(value));
    let order_probe = harness.order_probe;
    let on_prop_focus_exit = harness.prop_probe.on_focus_exit();

    rsx! {
        Select::<String> {
            binding,
            open: Some(true),
            on_focus_exit: move |()| {
                order_probe.assert_write_and_commit_before_focus_exit();
                on_prop_focus_exit.call(());
            },
            SelectTrigger { aria_label: "Focus Exit order select",
                SelectValue {}
            }
            SelectList {
                SelectOption::<String> {
                    value: "selected value".to_owned(),
                    index: 0usize,
                    "Selected value"
                }
            }
        }
    }
}

fn retarget_to_option(mut dom: InteractionDom) -> InteractionDom {
    for _ in 0..3 {
        let mutations = dom.dom.render_immediate_to_vec();
        let option = mutations.edits.iter().find_map(|edit| match edit {
            dioxus_core::Mutation::NewEventListener { name, id } if name == "pointerdown" => {
                Some(*id)
            }
            _ => None,
        });
        if let Some(option) = option {
            let mut attributes = ControlAttributes::default();
            attributes.apply(&mutations.edits, option);
            dom.control = option;
            dom.attributes = attributes;
            return dom;
        }
    }

    panic!("select should render an interactive option after opening");
}

fn choose_option(dom: &InteractionDom) {
    dom.dispatch("pointerdown", primary_pointer());
    dom.dispatch("pointerup", primary_pointer());
}

fn primary_pointer() -> SerializedPointerData {
    serde_json::from_value(serde_json::json!({
        "alt_key": false,
        "button": 0,
        "buttons": 1,
        "client_x": 1.0,
        "client_y": 1.0,
        "ctrl_key": false,
        "meta_key": false,
        "offset_x": 1.0,
        "offset_y": 1.0,
        "page_x": 1.0,
        "page_y": 1.0,
        "screen_x": 1.0,
        "screen_y": 1.0,
        "shift_key": false,
        "pointer_id": 1,
        "width": 1.0,
        "height": 1.0,
        "pressure": 0.5,
        "tangential_pressure": 0.0,
        "tilt_x": 0,
        "tilt_y": 0,
        "twist": 0,
        "pointer_type": "mouse",
        "is_primary": true
    }))
    .expect("pointer fixture should deserialize")
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
                Select::<String> {
                    binding: scaffold.explicit_binding,
                    meta: scaffold.explicit_meta,
                    required: false,
                    disabled: false,
                    name: "explicit-name",
                    open: Some(true),
                    SelectTrigger {
                        id: "explicit-id",
                        color: SelectColor::Primary,
                        aria_label: "Explicit select",
                        SelectValue {}
                    }
                    SelectList {
                        SelectOption::<String> {
                            value: "explicit value".to_owned(),
                            index: 0usize,
                            "Explicit value"
                        }
                    }
                }
            }
        },
        ResolutionSource::Context => rsx! {
            Field { context,
                Select::<String> {
                    required: false,
                    disabled: false,
                    open: Some(true),
                    SelectTrigger { aria_label: "Context select",
                        SelectValue {}
                    }
                    SelectList {
                        SelectOption::<String> {
                            value: "context value".to_owned(),
                            index: 0usize,
                            "Context value"
                        }
                    }
                }
            }
        },
        ResolutionSource::Metadata => rsx! {
            Field { context,
                Select::<String> {
                    open: Some(true),
                    SelectTrigger {
                        color: SelectColor::Default,
                        aria_label: "Metadata select",
                        SelectValue {}
                    }
                    SelectList {
                        SelectOption::<String> {
                            value: "context value".to_owned(),
                            index: 0usize,
                            "Context value"
                        }
                    }
                }
            }
        },
        ResolutionSource::Internal => rsx! {
            Select::<String> { open: Some(true),
                SelectTrigger { aria_label: "Internal select",
                    SelectValue {}
                }
                SelectList {
                    SelectOption::<String> {
                        value: "internal value".to_owned(),
                        index: 0usize,
                        "Internal value"
                    }
                }
            }
        },
    }
}

fn resolution_dom(
    source: ResolutionSource,
    explicit_changes: ChangeOriginProbe<Option<String>>,
    context_changes: ChangeOriginProbe<Option<String>>,
    interaction_listener: &str,
) -> InteractionDom {
    let dom = InteractionDom::mount(
        VirtualDom::new_with_props(
            resolution_app,
            ResolutionHarness {
                source,
                explicit_changes,
                context_changes,
            },
        ),
        "click",
    );
    if interaction_listener == "pointerdown" {
        retarget_to_option(dom)
    } else {
        dom
    }
}

fn rerender_app(harness: Rc<RerenderHarness>) -> Element {
    let (is_alternate, meta) =
        use_rerender_scaffold(&harness, "initial-select", "alternate-select");

    rsx! {
        Select::<String> {
            meta,
            name: if is_alternate { "alternate-name" } else { "initial-name" },
            required: Some(is_alternate),
            SelectTrigger { aria_label: "Rerender select",
                SelectValue {}
            }
            SelectList {
                SelectOption::<String> {
                    value: "value".to_owned(),
                    index: 0usize,
                    "Value"
                }
            }
        }
    }
}

#[test]
fn commit_is_synchronously_observable_before_submit_handling_runs() {
    let harness = InteractionHarness::new();
    let dom = interaction_dom(harness.clone());

    choose_option(&dom);
    dom.dom.in_runtime(|| harness.submit());

    harness.commits.assert_commit_before_submit();
}

#[test]
fn writes_carry_their_change_origin() {
    let harness = InteractionHarness::new();
    let dom = interaction_dom(harness.clone());

    choose_option(&dom);

    harness
        .changes
        .assert_writes(&[(Some(String::from("selected value")), ChangeOrigin::User)]);
}

#[test]
fn selection_commit_does_not_report_focus_exit() {
    let binding_probe = FocusExitProbe::new();
    let commits = Rc::new(Cell::new(0));
    let dom = retarget_to_option(InteractionDom::mount(
        VirtualDom::new_with_props(
            focus_exit_app,
            FocusExitHarness {
                binding_probe: binding_probe.clone(),
                commits: Rc::clone(&commits),
            },
        ),
        "click",
    ));

    choose_option(&dom);

    assert_eq!(commits.get(), 1);
    binding_probe.assert_no_focus_exit();
}

#[test]
fn focus_exit_follows_selection_write_and_commit_then_calls_the_prop() {
    let order_probe = FocusExitOrderProbe::new();
    let prop_probe = FocusExitProbe::new();
    let dom = InteractionDom::mount(
        VirtualDom::new_with_props(
            focus_exit_order_app,
            FocusExitOrderHarness {
                order_probe: order_probe.clone(),
                prop_probe: prop_probe.clone(),
            },
        ),
        "focusout",
    );
    let root = dom.control;
    let mut dom = retarget_to_option(dom);

    choose_option(&dom);
    dom.control = root;
    dom.dispatch("focusout", SerializedFocusData::default());
    dom.dom.render_immediate_to_vec();

    order_probe.assert_write_and_commit_before_focus_exit();
    prop_probe.assert_focus_exit_once();
}

#[test]
fn binding_resolution_precedence_holds_for_values_and_meta_flags() {
    let explicit_changes = ChangeOriginProbe::new();
    let context_changes = ChangeOriginProbe::new();
    let explicit_dom = resolution_dom(
        ResolutionSource::Explicit,
        explicit_changes.clone(),
        context_changes.clone(),
        "pointerdown",
    );
    assert!(explicit_dom.attributes.has("aria-selected"));
    assert!(explicit_dom.attributes.class_contains("menu-active"));
    choose_option(&explicit_dom);
    explicit_changes.assert_writes(&[(Some(String::from("explicit value")), ChangeOrigin::User)]);
    context_changes.assert_writes(&[]);
    let explicit_trigger = resolution_dom(
        ResolutionSource::Explicit,
        ChangeOriginProbe::new(),
        ChangeOriginProbe::new(),
        "click",
    );
    let explicit_attributes = &explicit_trigger.attributes;
    assert_eq!(explicit_attributes.get("id"), Some("explicit-id"));
    assert_eq!(explicit_attributes.get("name"), Some("explicit-name"));
    assert!(explicit_attributes.class_contains("select-primary"));
    assert!(!explicit_attributes.class_contains("select-error"));
    assert!(!explicit_attributes.has("aria-invalid"));
    assert!(!explicit_attributes.has("disabled"));
    assert!(!explicit_attributes.has("required"));
    assert!(!explicit_attributes.has("data-disabled"));
    assert!(!explicit_attributes.has("data-required"));

    let explicit_changes = ChangeOriginProbe::new();
    let context_changes = ChangeOriginProbe::new();
    let context_dom = resolution_dom(
        ResolutionSource::Context,
        explicit_changes.clone(),
        context_changes.clone(),
        "pointerdown",
    );
    assert!(context_dom.attributes.has("aria-selected"));
    assert!(context_dom.attributes.class_contains("menu-active"));
    choose_option(&context_dom);
    explicit_changes.assert_writes(&[]);
    context_changes.assert_writes(&[(Some(String::from("context value")), ChangeOrigin::User)]);
    let context_trigger = resolution_dom(
        ResolutionSource::Context,
        ChangeOriginProbe::new(),
        ChangeOriginProbe::new(),
        "click",
    );
    let context_attributes = &context_trigger.attributes;
    assert_eq!(context_attributes.get("id"), Some("context-id"));
    assert_eq!(context_attributes.get("name"), Some("context-name"));
    assert!(context_attributes.class_contains("select-error"));
    assert!(!context_attributes.has("aria-invalid"));
    assert!(!context_attributes.has("disabled"));
    assert!(!context_attributes.has("required"));
    assert!(!context_attributes.has("data-disabled"));
    assert!(!context_attributes.has("data-required"));
    assert_eq!(context_attributes.get("data-touched"), Some("true"));
    assert_eq!(context_attributes.get("data-dirty"), Some("true"));

    let metadata_trigger = resolution_dom(
        ResolutionSource::Metadata,
        ChangeOriginProbe::new(),
        ChangeOriginProbe::new(),
        "click",
    );
    let metadata_attributes = &metadata_trigger.attributes;
    assert_eq!(metadata_attributes.get("id"), Some("context-id"));
    assert_eq!(metadata_attributes.get("name"), Some("context-name"));
    assert!(!metadata_attributes.has("aria-invalid"));
    assert!(!metadata_attributes.class_contains("select-error"));
    assert!(metadata_attributes.has("disabled"));
    assert!(!metadata_attributes.has("required"));
    assert_eq!(metadata_attributes.get("data-disabled"), Some("true"));
    assert_eq!(metadata_attributes.get("data-required"), Some("true"));

    let explicit_changes = ChangeOriginProbe::new();
    let context_changes = ChangeOriginProbe::new();
    let mut internal_dom = resolution_dom(
        ResolutionSource::Internal,
        explicit_changes.clone(),
        context_changes.clone(),
        "pointerdown",
    );
    assert!(!internal_dom.attributes.has("aria-selected"));
    choose_option(&internal_dom);
    internal_dom.render_reactive_updates();
    assert!(internal_dom.attributes.has("aria-selected"));
    explicit_changes.assert_writes(&[]);
    context_changes.assert_writes(&[]);
}

#[test]
fn focus_request_round_trips_to_the_widget_control() {
    assert_focus_request_round_trip(
        || {
            rsx! {
                Select::<String> {
                    SelectTrigger { aria_label: "Focus target",
                        SelectValue {}
                    }
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
                Select::<String> {
                    SelectTrigger { aria_label: "Registered select",
                        SelectValue {}
                    }
                }
            }
        },
        "click",
        "select-description",
        "select-error",
        false,
    );
}

#[test]
fn private_field_context_tracks_root_prop_changes() {
    let harness = Rc::new(RerenderHarness::default());
    let mut dom = InteractionDom::mount(
        VirtualDom::new_with_props(rerender_app, Rc::clone(&harness)),
        "click",
    );

    assert_eq!(dom.attributes.get("id"), Some("initial-select"));
    assert_eq!(dom.attributes.get("name"), Some("initial-name"));
    assert!(!dom.attributes.has("required"));
    assert!(!dom.attributes.class_contains("select-error"));

    harness.show_alternate();
    dom.render_reactive_updates();

    assert_eq!(dom.attributes.get("id"), Some("alternate-select"));
    assert_eq!(dom.attributes.get("name"), Some("alternate-name"));
    assert!(!dom.attributes.has("required"));
    assert_eq!(dom.attributes.get("data-required"), Some("true"));
    assert!(dom.attributes.class_contains("select-error"));
}
