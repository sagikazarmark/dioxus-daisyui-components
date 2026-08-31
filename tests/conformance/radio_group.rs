use std::{any::Any, cell::Cell, rc::Rc};

use dioxus::prelude::*;
use dioxus_field::{
    Binding, ChangeOrigin, FieldContext,
    testing::{FocusExitOrderProbe, FocusExitProbe},
};
use dioxus_html::{PlatformEventData, SerializedFocusData, SerializedMouseData};

use dioxus_daisyui::components::{
    field::Field,
    radio_group::{RadioGroup, RadioItem, RadioItemColor},
};

use crate::harness::*;

struct RadioGroupDom {
    dom: VirtualDom,
    group: dioxus_core::ElementId,
    items: Vec<dioxus_core::ElementId>,
    group_attributes: ControlAttributes,
    item_attributes: Vec<ControlAttributes>,
}

impl RadioGroupDom {
    fn mount(mut dom: VirtualDom) -> Self {
        set_test_event_converter();
        let mutations = dom.rebuild_to_vec();
        let group = mutations
            .edits
            .iter()
            .find_map(|edit| match edit {
                dioxus_core::Mutation::NewEventListener { name, id } if name == "focusout" => {
                    Some(*id)
                }
                _ => None,
            })
            .expect("radio group should render its focusout listener");
        let items: Vec<_> = mutations
            .edits
            .iter()
            .filter_map(|edit| match edit {
                dioxus_core::Mutation::NewEventListener { name, id } if name == "click" => {
                    Some(*id)
                }
                _ => None,
            })
            .collect();
        assert_eq!(items.len(), 2, "test app should render two radio items");

        let mut group_attributes = ControlAttributes::default();
        group_attributes.apply(&mutations.edits, group);
        let item_attributes = items
            .iter()
            .map(|item| {
                let mut attributes = ControlAttributes::default();
                attributes.apply(&mutations.edits, *item);
                attributes
            })
            .collect();

        Self {
            dom,
            group,
            items,
            group_attributes,
            item_attributes,
        }
    }

    fn click(&self, index: usize) {
        let data = PlatformEventData::new(Box::new(SerializedMouseData::default()));
        let event = Event::new(Rc::new(data) as Rc<dyn Any>, true);
        self.dom
            .runtime()
            .handle_event("click", event, self.items[index]);
    }

    fn focus_in(&self, index: usize) {
        self.dispatch_focus("focusin", index);
    }

    fn focus_out(&self, index: usize) {
        self.dispatch_focus("focusout", index);
    }

    fn dispatch_focus(&self, name: &str, index: usize) {
        let data = PlatformEventData::new(Box::new(SerializedFocusData::default()));
        let event = Event::new(Rc::new(data) as Rc<dyn Any>, true);
        self.dom
            .runtime()
            .handle_event(name, event, self.items[index]);
    }
}

impl RendersReactiveUpdates for RadioGroupDom {
    fn render_reactive_updates(&mut self) {
        for _ in 0..2 {
            let mutations = self.dom.render_immediate_to_vec();
            self.group_attributes.apply(&mutations.edits, self.group);
            for (item, attributes) in self.items.iter().zip(&mut self.item_attributes) {
                attributes.apply(&mutations.edits, *item);
            }
        }
    }
}

fn interaction_app(harness: InteractionHarness<String>) -> Element {
    let value = use_signal(|| String::from("one"));
    let trio = harness.trio(ReadSignal::from(value));

    rsx! {
        RadioGroup {
            value: Some((trio.value)()),
            on_change: trio.on_change,
            on_commit: trio.on_commit,
            aria_label: "Conformance radio group",
            RadioItem { value: "two".to_owned(), index: 0usize, aria_label: "Two" }
            RadioItem { value: "one".to_owned(), index: 1usize, aria_label: "One" }
        }
    }
}

#[derive(Clone)]
struct ChoiceFocusHarness {
    focus_exits: FocusExitProbe,
    commits: Rc<Cell<usize>>,
}

fn choice_focus_app(harness: ChoiceFocusHarness) -> Element {
    let value = use_signal(|| String::from("one"));
    let commits = Rc::clone(&harness.commits);
    let binding = Binding::new(
        ReadSignal::from(value),
        Callback::new(|_| {}),
        Callback::new(move |()| commits.set(commits.get() + 1)),
    )
    .with_focus_exit(harness.focus_exits.on_focus_exit());

    rsx! {
        RadioGroup { binding, aria_label: "Focus contract radio group",
            RadioItem { value: "two".to_owned(), index: 0usize, aria_label: "Two" }
            RadioItem { value: "one".to_owned(), index: 1usize, aria_label: "One" }
        }
    }
}

#[derive(Clone)]
struct FocusExitHarness {
    probe: FocusExitOrderProbe,
    direct_calls: Rc<Cell<usize>>,
}

fn focus_exit_app(harness: FocusExitHarness) -> Element {
    let value = use_signal(|| String::from("one"));
    let binding = harness.probe.binding(ReadSignal::from(value));
    let callback_probe = harness.probe.clone();
    let direct_calls = Rc::clone(&harness.direct_calls);

    rsx! {
        RadioGroup {
            binding,
            on_focus_exit: move |()| {
                callback_probe.assert_write_and_commit_before_focus_exit();
                direct_calls.set(direct_calls.get() + 1);
            },
            aria_label: "Ordered focus contract radio group",
            RadioItem { value: "two".to_owned(), index: 0usize, aria_label: "Two" }
            RadioItem { value: "one".to_owned(), index: 1usize, aria_label: "One" }
        }
    }
}

fn resolution_app(harness: ResolutionHarness<String>) -> Element {
    let scaffold =
        use_resolution_scaffold(&harness, String::from("explicit"), String::from("context"));
    let context = scaffold.field_context();

    match harness.source {
        ResolutionSource::Explicit => rsx! {
            Field { context,
                RadioGroup {
                    binding: scaffold.explicit_binding,
                    meta: scaffold.explicit_meta,
                    id: "explicit-id",
                    name: "explicit-name",
                    required: false,
                    disabled: false,
                    aria_label: "Explicit radio group",
                    RadioItem {
                        id: "explicit-item-id",
                        value: "explicit".to_owned(),
                        index: 0usize,
                        color: RadioItemColor::Primary,
                        aria_label: "Explicit option",
                    }
                    RadioItem {
                        value: "context".to_owned(),
                        index: 1usize,
                        color: RadioItemColor::Primary,
                        aria_label: "Context option",
                    }
                }
            }
        },
        ResolutionSource::Context => rsx! {
            Field { context,
                RadioGroup {
                    required: false,
                    disabled: false,
                    aria_label: "Context radio group",
                    RadioItem {
                        value: "explicit".to_owned(),
                        index: 0usize,
                        aria_label: "Explicit option",
                    }
                    RadioItem {
                        value: "context".to_owned(),
                        index: 1usize,
                        aria_label: "Context option",
                    }
                }
            }
        },
        ResolutionSource::Metadata => rsx! {
            Field { context,
                RadioGroup { aria_label: "Metadata radio group",
                    RadioItem {
                        value: "explicit".to_owned(),
                        index: 0usize,
                        color: RadioItemColor::Default,
                        aria_label: "Explicit option",
                    }
                    RadioItem {
                        value: "context".to_owned(),
                        index: 1usize,
                        color: RadioItemColor::Default,
                        aria_label: "Context option",
                    }
                }
            }
        },
        ResolutionSource::Internal => rsx! {
            RadioGroup { aria_label: "Internal radio group",
                RadioItem {
                    value: "explicit".to_owned(),
                    index: 0usize,
                    aria_label: "Explicit option",
                }
                RadioItem {
                    value: "context".to_owned(),
                    index: 1usize,
                    aria_label: "Context option",
                }
            }
        },
    }
}

fn resolution_dom(harness: ResolutionHarness<String>) -> RadioGroupDom {
    let mut dom = RadioGroupDom::mount(VirtualDom::new_with_props(resolution_app, harness));
    dom.render_reactive_updates();
    dom
}

fn rerender_app(harness: Rc<RerenderHarness>) -> Element {
    let (is_alternate, meta) =
        use_rerender_scaffold(&harness, "initial-radio-group", "alternate-radio-group");

    rsx! {
        RadioGroup {
            meta,
            name: if is_alternate { "alternate-name" } else { "initial-name" },
            required: Some(is_alternate),
            aria_label: "Rerender radio group",
            RadioItem { value: "first".to_owned(), index: 0usize, aria_label: "First" }
            RadioItem { value: "second".to_owned(), index: 1usize, aria_label: "Second" }
        }
    }
}

#[derive(Clone)]
struct FocusRegistrationApp {
    context: FieldContext,
}

fn focus_registration_app(props: FocusRegistrationApp) -> Element {
    rsx! {
        Field { context: props.context,
            RadioGroup {
                default_value: "chosen".to_owned(),
                aria_label: "Focus target",
                RadioItem { value: "first".to_owned(), index: 0usize, aria_label: "First" }
                RadioItem { value: "chosen".to_owned(), index: 1usize, aria_label: "Chosen" }
            }
        }
    }
}

#[test]
fn commit_is_synchronously_observable_before_submit_handling_runs() {
    let harness = InteractionHarness::new();
    let dom = RadioGroupDom::mount(VirtualDom::new_with_props(interaction_app, harness.clone()));

    dom.click(0);
    dom.dom.in_runtime(|| harness.submit());

    harness.commits.assert_commit_before_submit();
}

#[test]
fn writes_carry_their_change_origin() {
    let harness = InteractionHarness::new();
    let dom = RadioGroupDom::mount(VirtualDom::new_with_props(interaction_app, harness.clone()));

    dom.click(0);

    harness
        .changes
        .assert_writes(&[("two".to_owned(), ChangeOrigin::User)]);
}

#[test]
fn choice_commits_without_focus_exit() {
    let harness = ChoiceFocusHarness {
        focus_exits: FocusExitProbe::new(),
        commits: Rc::new(Cell::new(0)),
    };
    let dom = RadioGroupDom::mount(VirtualDom::new_with_props(
        choice_focus_app,
        harness.clone(),
    ));

    dom.click(0);

    assert_eq!(harness.commits.get(), 1);
    harness.focus_exits.assert_no_focus_exit();
}

#[test]
fn focus_exit_follows_write_and_commit_once() {
    let harness = FocusExitHarness {
        probe: FocusExitOrderProbe::new(),
        direct_calls: Rc::new(Cell::new(0)),
    };
    let mut dom = RadioGroupDom::mount(VirtualDom::new_with_props(focus_exit_app, harness.clone()));

    dom.click(0);
    dom.focus_in(0);
    dom.focus_out(0);
    dom.render_reactive_updates();

    harness.probe.assert_write_and_commit_before_focus_exit();
    assert_eq!(harness.direct_calls.get(), 1);
}

#[test]
fn binding_resolution_precedence_holds_for_values_and_meta_flags() {
    assert_binding_resolution_precedence(PrecedenceSpec {
        resolution_dom,
        explicit: PrecedencePhase {
            assert_attributes: |dom| {
                let group = &dom.group_attributes;
                let item = &dom.item_attributes[0];
                assert_eq!(group.get("id"), Some("explicit-id"));
                assert!(!group.has("name"));
                assert_eq!(group.get("aria-invalid"), Some("false"));
                assert!(!group.has("aria-required"));
                assert!(!group.has("data-disabled"));
                assert!(!group.has("disabled"));
                assert!(!group.has("required"));
                assert!(!group.has("data-required"));
                assert_eq!(item.get("id"), Some("explicit-item-id"));
                assert!(item.has("aria-checked"));
                assert_eq!(item.get("tabindex"), Some("0"));
                assert!(item.class_contains("radio-primary"));
                assert!(!item.class_contains("radio-error"));
                assert!(!item.has("aria-invalid"));
            },
            dispatch: |dom| dom.click(1),
            written: "context".to_owned(),
        },
        context: PrecedencePhase {
            assert_attributes: |dom| {
                let group = &dom.group_attributes;
                assert_eq!(group.get("id"), Some("context-id"));
                assert!(!group.has("name"));
                assert_eq!(group.get("aria-invalid"), Some("true"));
                assert!(!group.has("aria-required"));
                assert!(!group.has("data-disabled"));
                assert_eq!(group.get("data-touched"), Some("true"));
                assert_eq!(group.get("data-dirty"), Some("true"));
                assert!(!group.has("disabled"));
                assert!(!group.has("required"));
                assert!(dom.item_attributes[0].class_contains("radio-error"));
                assert!(dom.item_attributes[1].has("aria-checked"));
                assert_eq!(dom.item_attributes[1].get("tabindex"), Some("0"));
            },
            dispatch: |dom| dom.click(0),
            written: "explicit".to_owned(),
        },
        assert_metadata: |dom| {
            let group = &dom.group_attributes;
            assert_eq!(group.get("id"), Some("context-id"));
            assert!(!group.has("name"));
            assert!(group.has("aria-invalid"));
            assert!(group.has("aria-required"));
            assert_eq!(group.get("aria-disabled"), Some("true"));
            assert!(group.has("data-disabled"));
            assert!(!group.has("disabled"));
            assert!(!group.has("required"));
            assert_eq!(group.get("data-required"), Some("true"));
            assert!(!dom.item_attributes[0].class_contains("radio-error"));
            assert!(dom.item_attributes[0].has("disabled"));
        },
        internal: InternalPhase {
            assert_mounted: |dom| {
                assert_eq!(dom.group_attributes.get("aria-invalid"), Some("false"));
                assert!(!dom.item_attributes[0].has("aria-checked"));
            },
            dispatch: |dom| dom.click(0),
            assert_updated: |dom| {
                assert!(dom.item_attributes[0].has("aria-checked"));
            },
        },
    });
}

#[test]
fn focus_request_is_registered_with_field_context() {
    let context = FieldContext::empty();
    let request = context.focus_request();
    let mut dom =
        VirtualDom::new_with_props(focus_registration_app, FocusRegistrationApp { context });
    dom.rebuild_to_vec();

    assert!(dom.in_runtime(|| request.request()));
}

#[test]
fn error_and_description_ids_appear_on_mount_and_vanish_on_drop() {
    let harness = Rc::new(IdRegistrationHarness::default());
    let mut dom = RadioGroupDom::mount(VirtualDom::new_with_props(
        id_registration_app,
        IdRegistrationApp {
            harness: Rc::clone(&harness),
            control: || {
                rsx! {
                    RadioGroup { aria_label: "Registered radio group",
                        RadioItem { value: "first".to_owned(), index: 0usize, aria_label: "First" }
                        RadioItem { value: "second".to_owned(), index: 1usize, aria_label: "Second" }
                    }
                }
            },
            description_id: "radio-group-description",
            error_id: "radio-group-error",
        },
    ));
    dom.render_reactive_updates();

    assert_eq!(
        dom.group_attributes.get("aria-describedby"),
        Some("radio-group-description radio-group-error")
    );
    assert_eq!(
        dom.group_attributes.get("aria-errormessage"),
        Some("radio-group-error")
    );

    hide_registered_parts(&harness, &mut dom);
    assert!(!dom.group_attributes.has("aria-describedby"));
    assert!(!dom.group_attributes.has("aria-errormessage"));
}

#[test]
fn private_field_context_tracks_root_prop_changes() {
    let harness = Rc::new(RerenderHarness::default());
    let mut dom = RadioGroupDom::mount(VirtualDom::new_with_props(
        rerender_app,
        Rc::clone(&harness),
    ));

    assert_eq!(dom.group_attributes.get("id"), Some("initial-radio-group"));
    assert!(!dom.group_attributes.has("name"));
    assert!(!dom.group_attributes.has("required"));
    assert!(!dom.item_attributes[0].class_contains("radio-error"));

    harness.show_alternate();
    dom.render_reactive_updates();

    assert_eq!(
        dom.group_attributes.get("id"),
        Some("alternate-radio-group")
    );
    assert!(!dom.group_attributes.has("name"));
    assert!(dom.group_attributes.has("aria-required"));
    assert!(!dom.group_attributes.has("required"));
    assert!(dom.item_attributes[0].class_contains("radio-error"));
}
