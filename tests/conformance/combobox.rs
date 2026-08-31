use std::{cell::Cell, rc::Rc};

use dioxus::prelude::*;
use dioxus_field::{
    Binding, ChangeOrigin,
    testing::{FocusExitOrderProbe, FocusExitProbe},
};
use dioxus_html::{SerializedFocusData, SerializedKeyboardData, SerializedMouseData};

use dioxus_daisyui_components::components::{
    combobox::{Combobox, ComboboxColor, ComboboxInput, ComboboxList, ComboboxOption},
    field::Field,
};

use crate::harness::*;

fn interaction_app(harness: InteractionHarness<Option<String>>) -> Element {
    let value = use_signal(|| Option::<String>::None);
    let trio = harness.trio(ReadSignal::from(value));

    rsx! {
        Combobox::<String> {
            value: Some(trio.value),
            on_change: trio.on_change,
            on_commit: trio.on_commit,
            ComboboxInput { aria_label: "Conformance combobox" }
            ComboboxList {
                ComboboxOption::<String> {
                    value: "selected value".to_owned(),
                    index: 0usize,
                    "Selected value"
                }
            }
        }
    }
}

fn interaction_dom(harness: InteractionHarness<Option<String>>) -> InteractionDom {
    let mut dom = InteractionDom::mount(
        VirtualDom::new_with_props(interaction_app, harness),
        "keydown",
    );
    dom.render_reactive_updates();
    dom
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
        Combobox::<String> { binding,
            ComboboxInput { aria_label: "Focus Exit combobox" }
            ComboboxList {
                ComboboxOption::<String> {
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
        Combobox::<String> {
            binding,
            on_focus_exit: move |()| {
                order_probe.assert_write_and_commit_before_focus_exit();
                on_prop_focus_exit.call(());
            },
            ComboboxInput { aria_label: "Focus Exit order combobox" }
            ComboboxList {
                ComboboxOption::<String> {
                    value: "selected value".to_owned(),
                    index: 0usize,
                    "Selected value"
                }
            }
        }
    }
}

fn focus_exit_order_dom(
    harness: FocusExitOrderHarness,
) -> (InteractionDom, dioxus_core::ElementId) {
    set_test_event_converter();
    let mut dom = VirtualDom::new_with_props(focus_exit_order_app, harness);
    let mutations = dom.rebuild_to_vec();
    let listener = |name| {
        mutations
            .edits
            .iter()
            .find_map(|edit| match edit {
                dioxus_core::Mutation::NewEventListener { name: current, id }
                    if *current == name =>
                {
                    Some(*id)
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("combobox should render its {name} listener"))
    };
    let control = listener("keydown");
    let root = listener("focusout");
    let mut attributes = ControlAttributes::default();
    attributes.apply(&mutations.edits, control);

    let mut interaction = InteractionDom {
        dom,
        control,
        attributes,
    };
    interaction.render_reactive_updates();

    (interaction, root)
}

fn choose_first_combobox_option(dom: &mut InteractionDom) {
    dom.dispatch("click", SerializedMouseData::default());
    dom.render_reactive_updates();
    dom.dispatch(
        "keydown",
        SerializedKeyboardData::new(
            Key::ArrowDown,
            Code::ArrowDown,
            Location::Standard,
            false,
            Modifiers::empty(),
            false,
        ),
    );
    dom.render_reactive_updates();
    dom.dispatch(
        "keydown",
        SerializedKeyboardData::new(
            Key::Enter,
            Code::Enter,
            Location::Standard,
            false,
            Modifiers::empty(),
            false,
        ),
    );
}

fn options() -> Element {
    rsx! {
        ComboboxList {
            ComboboxOption::<String> {
                value: "next value".to_owned(),
                index: 0usize,
                "Next value"
            }
            ComboboxOption::<String> {
                value: "explicit value".to_owned(),
                index: 1usize,
                "Explicit value"
            }
            ComboboxOption::<String> {
                value: "context value".to_owned(),
                index: 2usize,
                "Context value"
            }
        }
    }
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
                Combobox::<String> {
                    binding: scaffold.explicit_binding,
                    meta: scaffold.explicit_meta,
                    required: false,
                    disabled: false,
                    name: "explicit-name",
                    ComboboxInput {
                        id: "explicit-id",
                        color: ComboboxColor::Primary,
                        aria_label: "Explicit combobox",
                    }
                    {options()}
                }
            }
        },
        ResolutionSource::Context => rsx! {
            Field { context,
                Combobox::<String> {
                    required: false,
                    disabled: false,
                    ComboboxInput { aria_label: "Context combobox" }
                    {options()}
                }
            }
        },
        ResolutionSource::Metadata => rsx! {
            Field { context,
                Combobox::<String> {
                    ComboboxInput {
                        color: ComboboxColor::Default,
                        aria_label: "Metadata combobox",
                    }
                    {options()}
                }
            }
        },
        ResolutionSource::Internal => rsx! {
            Combobox::<String> {
                ComboboxInput { aria_label: "Internal combobox" }
                {options()}
            }
        },
    }
}

fn resolution_dom(harness: ResolutionHarness<Option<String>>) -> InteractionDom {
    let mut dom = InteractionDom::mount(
        VirtualDom::new_with_props(resolution_app, harness),
        "keydown",
    );
    dom.render_reactive_updates();
    dom
}

fn rerender_app(harness: Rc<RerenderHarness>) -> Element {
    let (is_alternate, meta) =
        use_rerender_scaffold(&harness, "initial-combobox", "alternate-combobox");

    rsx! {
        Combobox::<String> {
            meta,
            name: if is_alternate { "alternate-name" } else { "initial-name" },
            required: Some(is_alternate),
            ComboboxInput { aria_label: "Rerender combobox" }
            {options()}
        }
    }
}

#[test]
fn commit_is_synchronously_observable_before_submit_handling_runs() {
    let harness = InteractionHarness::new();
    let mut dom = interaction_dom(harness.clone());

    choose_first_combobox_option(&mut dom);
    dom.dom.in_runtime(|| harness.submit());

    harness.commits.assert_commit_before_submit();
}

#[test]
fn writes_carry_their_change_origin() {
    let harness = InteractionHarness::new();
    let mut dom = interaction_dom(harness.clone());

    choose_first_combobox_option(&mut dom);

    harness
        .changes
        .assert_writes(&[(Some(String::from("selected value")), ChangeOrigin::User)]);
}

#[test]
fn selection_commit_does_not_report_focus_exit() {
    let binding_probe = FocusExitProbe::new();
    let commits = Rc::new(Cell::new(0));
    let mut dom = InteractionDom::mount(
        VirtualDom::new_with_props(
            focus_exit_app,
            FocusExitHarness {
                binding_probe: binding_probe.clone(),
                commits: Rc::clone(&commits),
            },
        ),
        "keydown",
    );
    dom.render_reactive_updates();

    choose_first_combobox_option(&mut dom);

    assert_eq!(commits.get(), 1);
    binding_probe.assert_no_focus_exit();
}

#[test]
fn focus_exit_follows_selection_write_and_commit_then_calls_the_prop() {
    let order_probe = FocusExitOrderProbe::new();
    let prop_probe = FocusExitProbe::new();
    let (mut dom, root) = focus_exit_order_dom(FocusExitOrderHarness {
        order_probe: order_probe.clone(),
        prop_probe: prop_probe.clone(),
    });

    choose_first_combobox_option(&mut dom);
    dom.control = root;
    dom.dispatch("focusout", SerializedFocusData::default());
    dom.dom.render_immediate_to_vec();

    order_probe.assert_write_and_commit_before_focus_exit();
    prop_probe.assert_focus_exit_once();
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
            dispatch: |dom| choose_first_combobox_option(dom),
            written: Some(String::from("next value")),
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
            dispatch: |dom| choose_first_combobox_option(dom),
            written: Some(String::from("next value")),
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
                assert_eq!(dom.attributes.get("value"), Some(""));
                assert_eq!(dom.attributes.get("aria-invalid"), Some("false"));
            },
            dispatch: |dom| choose_first_combobox_option(dom),
            assert_updated: |dom| {
                assert_eq!(dom.attributes.get("value"), Some("next value"));
            },
        },
    });
}

#[test]
fn focus_request_round_trips_to_the_widget_control() {
    assert_focus_request_round_trip(
        || {
            rsx! {
                Combobox::<String> {
                    ComboboxInput { aria_label: "Focus target" }
                    {options()}
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
                Combobox::<String> {
                    ComboboxInput { aria_label: "Registered combobox" }
                    {options()}
                }
            }
        },
        "keydown",
        "combobox-description",
        "combobox-error",
        true,
    );
}

#[test]
fn private_field_context_tracks_root_prop_changes() {
    let harness = Rc::new(RerenderHarness::default());
    let mut dom = InteractionDom::mount(
        VirtualDom::new_with_props(rerender_app, Rc::clone(&harness)),
        "keydown",
    );
    dom.render_reactive_updates();

    assert_eq!(dom.attributes.get("id"), Some("initial-combobox"));
    assert_eq!(dom.attributes.get("name"), Some("initial-name"));
    assert!(!dom.attributes.has("required"));
    assert!(!dom.attributes.class_contains("input-error"));

    harness.show_alternate();
    dom.render_reactive_updates();

    assert_eq!(dom.attributes.get("id"), Some("alternate-combobox"));
    assert_eq!(dom.attributes.get("name"), Some("alternate-name"));
    assert!(dom.attributes.has("required"));
    assert!(dom.attributes.class_contains("input-error"));
}
