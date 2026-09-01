use std::{any::Any, cell::RefCell, collections::HashMap, future::Future, pin::Pin, rc::Rc};

use dioxus::prelude::*;
use dioxus_field::{
    Binding, BindingPropTrio, ChangeOrigin, FieldContext, FieldMeta, FieldMetaValues,
    testing::{ChangeOriginProbe, CommitOrderProbe, FocusRoundTripProbe},
    use_field_meta_state,
};
use dioxus_html::{
    HtmlEventConverter, MountedData, MountedResult, PlatformEventData, RenderedElementBacking,
    SerializedFormData, SerializedHtmlEventConverter, SerializedKeyboardData,
};

use dioxus_daisyui_components::components::field::{Field, FieldDescription, FieldError};

#[derive(Clone)]
pub(crate) struct InteractionHarness<T> {
    pub(crate) changes: ChangeOriginProbe<T>,
    pub(crate) commits: CommitOrderProbe,
    on_submit: Rc<RefCell<Option<Callback<()>>>>,
}

impl<T: 'static> InteractionHarness<T> {
    pub(crate) fn new() -> Self {
        Self {
            changes: ChangeOriginProbe::new(),
            commits: CommitOrderProbe::new(),
            on_submit: Rc::new(RefCell::new(None)),
        }
    }

    pub(crate) fn trio(&self, value: ReadSignal<T>) -> BindingPropTrio<T> {
        self.binding(value).into_trio()
    }

    pub(crate) fn binding(&self, value: ReadSignal<T>) -> Binding<T> {
        self.on_submit
            .borrow_mut()
            .replace(self.commits.on_submit());
        self.changes
            .binding_with_commit(value, self.commits.on_commit())
    }

    pub(crate) fn submit(&self) {
        self.on_submit
            .borrow()
            .expect("app should expose its submit handler")
            .call(());
    }
}

#[derive(Clone, Copy)]
pub(crate) enum ResolutionSource {
    Explicit,
    Context,
    Metadata,
    Internal,
}

#[derive(Clone)]
pub(crate) struct ResolutionHarness<T> {
    pub(crate) source: ResolutionSource,
    pub(crate) explicit_changes: ChangeOriginProbe<T>,
    pub(crate) context_changes: ChangeOriginProbe<T>,
}

pub(crate) struct ResolutionMetaIds {
    pub(crate) explicit_id: &'static str,
    pub(crate) explicit_name: &'static str,
    pub(crate) context_id: &'static str,
    pub(crate) context_name: &'static str,
}

impl Default for ResolutionMetaIds {
    fn default() -> Self {
        Self {
            explicit_id: "explicit-meta-id",
            explicit_name: "explicit-meta-name",
            context_id: "context-id",
            context_name: "context-name",
        }
    }
}

pub(crate) struct ResolutionScaffold<T: 'static> {
    pub(crate) explicit_binding: Binding<T>,
    pub(crate) context_binding: Binding<T>,
    pub(crate) explicit_meta: FieldMeta,
    pub(crate) context_meta: FieldMeta,
}

impl<T: 'static> ResolutionScaffold<T> {
    pub(crate) fn field_context(&self) -> FieldContext {
        FieldContext::new(self.context_binding.clone()).with_meta(self.context_meta)
    }
}

pub(crate) fn use_resolution_scaffold<T: 'static>(
    harness: &ResolutionHarness<T>,
    explicit_value: T,
    context_value: T,
) -> ResolutionScaffold<T> {
    use_resolution_scaffold_with_ids(
        harness,
        explicit_value,
        context_value,
        ResolutionMetaIds::default(),
    )
}

pub(crate) fn use_resolution_scaffold_with_ids<T: 'static>(
    harness: &ResolutionHarness<T>,
    explicit_value: T,
    context_value: T,
    ids: ResolutionMetaIds,
) -> ResolutionScaffold<T> {
    let explicit_value = use_signal(|| explicit_value);
    let context_value = use_signal(|| context_value);
    let explicit_binding = harness
        .explicit_changes
        .binding(ReadSignal::from(explicit_value));
    let context_binding = harness
        .context_changes
        .binding(ReadSignal::from(context_value));
    let explicit_meta = use_field_meta_state(FieldMetaValues {
        id: Some(Rc::from(ids.explicit_id)),
        name: Some(Rc::from(ids.explicit_name)),
        required: true,
        disabled: true,
        invalid: Some(false),
        ..FieldMetaValues::default()
    });
    let context_meta = use_field_meta_state(FieldMetaValues {
        id: Some(Rc::from(ids.context_id)),
        name: Some(Rc::from(ids.context_name)),
        required: true,
        disabled: true,
        invalid: Some(true),
        touched: true,
        dirty: true,
        ..FieldMetaValues::default()
    });

    ResolutionScaffold {
        explicit_binding,
        context_binding,
        explicit_meta,
        context_meta,
    }
}

pub(crate) trait RendersReactiveUpdates {
    fn render_reactive_updates(&mut self);
}

pub(crate) struct PrecedencePhase<T, D> {
    pub(crate) assert_attributes: fn(&D),
    pub(crate) dispatch: fn(&mut D),
    pub(crate) written: T,
}

pub(crate) struct InternalPhase<D> {
    pub(crate) assert_mounted: fn(&D),
    pub(crate) dispatch: fn(&mut D),
    pub(crate) assert_updated: fn(&D),
}

pub(crate) struct PrecedenceSpec<T: 'static, D> {
    pub(crate) resolution_dom: fn(ResolutionHarness<T>) -> D,
    pub(crate) explicit: PrecedencePhase<T, D>,
    pub(crate) context: PrecedencePhase<T, D>,
    pub(crate) assert_metadata: fn(&D),
    pub(crate) internal: InternalPhase<D>,
}

pub(crate) fn assert_binding_resolution_precedence<T, D>(spec: PrecedenceSpec<T, D>)
where
    T: std::fmt::Debug + PartialEq + 'static,
    D: RendersReactiveUpdates,
{
    let explicit_changes = ChangeOriginProbe::new();
    let context_changes = ChangeOriginProbe::new();
    let mut explicit_dom = (spec.resolution_dom)(ResolutionHarness {
        source: ResolutionSource::Explicit,
        explicit_changes: explicit_changes.clone(),
        context_changes: context_changes.clone(),
    });
    (spec.explicit.assert_attributes)(&explicit_dom);
    (spec.explicit.dispatch)(&mut explicit_dom);
    explicit_changes.assert_writes(&[(spec.explicit.written, ChangeOrigin::User)]);
    context_changes.assert_writes(&[]);

    let explicit_changes = ChangeOriginProbe::new();
    let context_changes = ChangeOriginProbe::new();
    let mut context_dom = (spec.resolution_dom)(ResolutionHarness {
        source: ResolutionSource::Context,
        explicit_changes: explicit_changes.clone(),
        context_changes: context_changes.clone(),
    });
    (spec.context.assert_attributes)(&context_dom);
    (spec.context.dispatch)(&mut context_dom);
    explicit_changes.assert_writes(&[]);
    context_changes.assert_writes(&[(spec.context.written, ChangeOrigin::User)]);

    let metadata_dom = (spec.resolution_dom)(ResolutionHarness {
        source: ResolutionSource::Metadata,
        explicit_changes: ChangeOriginProbe::new(),
        context_changes: ChangeOriginProbe::new(),
    });
    (spec.assert_metadata)(&metadata_dom);

    let explicit_changes = ChangeOriginProbe::new();
    let context_changes = ChangeOriginProbe::new();
    let mut internal_dom = (spec.resolution_dom)(ResolutionHarness {
        source: ResolutionSource::Internal,
        explicit_changes: explicit_changes.clone(),
        context_changes: context_changes.clone(),
    });
    (spec.internal.assert_mounted)(&internal_dom);

    (spec.internal.dispatch)(&mut internal_dom);
    internal_dom.render_reactive_updates();
    (spec.internal.assert_updated)(&internal_dom);
    explicit_changes.assert_writes(&[]);
    context_changes.assert_writes(&[]);
}

#[derive(Clone)]
struct FocusApp {
    context: FieldContext,
    probe: FocusRoundTripProbe,
    on_focus: Rc<RefCell<Option<Callback<()>>>>,
    control: fn() -> Element,
}

fn focus_app(props: FocusApp) -> Element {
    props.on_focus.borrow_mut().replace(props.probe.on_focus());
    let FocusApp {
        context, control, ..
    } = props;

    rsx! {
        Field { context, {control()} }
    }
}

pub(crate) fn assert_focus_request_round_trip(
    control: fn() -> Element,
    mount: fn(VirtualDom) -> (VirtualDom, dioxus_core::ElementId),
) {
    let probe = FocusRoundTripProbe::new();
    let context = FieldContext::empty();
    let request = context.focus_request();
    let on_focus = Rc::new(RefCell::new(None));
    let (mut dom, control_id) = mount(VirtualDom::new_with_props(
        focus_app,
        FocusApp {
            context,
            probe: probe.clone(),
            on_focus: Rc::clone(&on_focus),
            control,
        },
    ));
    dispatch_mounted(
        &dom,
        control_id,
        FocusBacking {
            on_focus: on_focus
                .borrow()
                .expect("app should expose the focus probe callback"),
        },
    );

    assert!(dom.in_runtime(|| request.request()));
    dom.render_immediate_to_vec();

    probe.assert_focus_round_trip();
}

pub(crate) struct InteractionDom {
    pub(crate) dom: VirtualDom,
    pub(crate) control: dioxus_core::ElementId,
    pub(crate) attributes: ControlAttributes,
}

#[derive(Default)]
pub(crate) struct ControlAttributes(HashMap<&'static str, String>);

impl ControlAttributes {
    pub(crate) fn apply(
        &mut self,
        edits: &[dioxus_core::Mutation],
        control: dioxus_core::ElementId,
    ) {
        for edit in edits {
            let dioxus_core::Mutation::SetAttribute {
                name, value, id, ..
            } = edit
            else {
                continue;
            };
            if *id != control {
                continue;
            }

            match value {
                dioxus_core::AttributeValue::Text(value) => {
                    self.0.insert(name, value.clone());
                }
                dioxus_core::AttributeValue::Float(value) => {
                    self.0.insert(name, value.to_string());
                }
                dioxus_core::AttributeValue::Int(value) => {
                    self.0.insert(name, value.to_string());
                }
                dioxus_core::AttributeValue::Bool(true) => {
                    self.0.insert(name, String::new());
                }
                dioxus_core::AttributeValue::Bool(false) | dioxus_core::AttributeValue::None => {
                    self.0.remove(name);
                }
                dioxus_core::AttributeValue::Listener(_) | dioxus_core::AttributeValue::Any(_) => {}
            }
        }
    }

    pub(crate) fn get(&self, name: &str) -> Option<&str> {
        self.0.get(name).map(String::as_str)
    }

    pub(crate) fn has(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&'static str, &str)> {
        self.0.iter().map(|(name, value)| (*name, value.as_str()))
    }

    pub(crate) fn class_contains(&self, class: &str) -> bool {
        self.get("class")
            .is_some_and(|classes| classes.split_ascii_whitespace().any(|value| value == class))
    }
}

impl InteractionDom {
    pub(crate) fn mount(dom: VirtualDom, interaction_listener: &str) -> Self {
        Self::mount_with_edits(dom, interaction_listener, |_| ()).1
    }

    /// Mounts as [`InteractionDom::mount`] does, handing the mount's raw edits
    /// to `read` first, for tests that assert on elements other than the
    /// control itself.
    pub(crate) fn mount_with_edits<T>(
        mut dom: VirtualDom,
        interaction_listener: &str,
        read: impl FnOnce(&[dioxus_core::Mutation]) -> T,
    ) -> (T, Self) {
        set_test_event_converter();
        let mutations = dom.rebuild_to_vec();
        let observed = read(&mutations.edits);
        let control = mutations
            .edits
            .iter()
            .find_map(|edit| match edit {
                dioxus_core::Mutation::NewEventListener { name, id }
                    if name == interaction_listener =>
                {
                    Some(*id)
                }
                _ => None,
            })
            .expect("component should render its interaction listener");
        let mut attributes = ControlAttributes::default();
        attributes.apply(&mutations.edits, control);

        (
            observed,
            Self {
                dom,
                control,
                attributes,
            },
        )
    }

    pub(crate) fn dispatch(&self, name: &str, data: impl Any) {
        let data = PlatformEventData::new(Box::new(data));
        let event = Event::new(Rc::new(data) as Rc<dyn Any>, true);
        self.dom.runtime().handle_event(name, event, self.control);
    }
}

impl RendersReactiveUpdates for InteractionDom {
    fn render_reactive_updates(&mut self) {
        for _ in 0..2 {
            let mutations = self.dom.render_immediate_to_vec();
            self.attributes.apply(&mutations.edits, self.control);
        }
    }
}

struct TestEventConverter(SerializedHtmlEventConverter);

macro_rules! delegate_event_conversions {
    ($($method:ident => $data:ident),+ $(,)?) => {
        $(
            fn $method(&self, event: &PlatformEventData) -> dioxus_html::$data {
                self.0.$method(event)
            }
        )+
    };
}

impl HtmlEventConverter for TestEventConverter {
    delegate_event_conversions! {
        convert_animation_data => AnimationData,
        convert_cancel_data => CancelData,
        convert_clipboard_data => ClipboardData,
        convert_composition_data => CompositionData,
        convert_drag_data => DragData,
        convert_focus_data => FocusData,
        convert_form_data => FormData,
        convert_image_data => ImageData,
        convert_keyboard_data => KeyboardData,
        convert_media_data => MediaData,
        convert_mouse_data => MouseData,
        convert_pointer_data => PointerData,
        convert_resize_data => ResizeData,
        convert_scroll_data => ScrollData,
        convert_selection_data => SelectionData,
        convert_toggle_data => ToggleData,
        convert_touch_data => TouchData,
        convert_transition_data => TransitionData,
        convert_visible_data => VisibleData,
        convert_wheel_data => WheelData,
    }

    fn convert_mounted_data(&self, event: &PlatformEventData) -> MountedData {
        let backing = event
            .downcast::<FocusBacking>()
            .expect("mounted events should carry the test element backing")
            .clone();
        MountedData::new(backing)
    }
}

pub(crate) fn set_test_event_converter() {
    dioxus::html::set_event_converter(Box::new(TestEventConverter(SerializedHtmlEventConverter)));
}

#[derive(Clone)]
struct FocusBacking {
    on_focus: Callback<()>,
}

impl RenderedElementBacking for FocusBacking {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_focus(&self, focus: bool) -> Pin<Box<dyn Future<Output = MountedResult<()>>>> {
        let on_focus = self.on_focus;
        Box::pin(async move {
            if focus {
                on_focus.call(());
            }
            Ok(())
        })
    }
}

pub(crate) fn mount_control(mut dom: VirtualDom) -> (VirtualDom, dioxus_core::ElementId) {
    set_test_event_converter();
    let mutations = dom.rebuild_to_vec();
    let control = mutations
        .edits
        .iter()
        .find_map(|edit| match edit {
            dioxus_core::Mutation::NewEventListener { name, id } if name == "mounted" => Some(*id),
            _ => None,
        })
        .expect("component should render a mounted listener on its control");

    (dom, control)
}

fn dispatch_mounted(dom: &VirtualDom, control: dioxus_core::ElementId, data: FocusBacking) {
    let data = PlatformEventData::new(Box::new(data));
    let event = Event::new(Rc::new(data) as Rc<dyn Any>, false);
    dom.runtime().handle_event("mounted", event, control);
}

pub(crate) fn dispatch_input_event(dom: &InteractionDom, name: &str, value: &str) {
    dom.dispatch(name, SerializedFormData::new(value.to_owned(), Vec::new()));
}

#[derive(Default)]
pub(crate) struct IdRegistrationHarness {
    show_parts: RefCell<Option<Signal<bool>>>,
}

#[derive(Clone)]
pub(crate) struct IdRegistrationApp {
    pub(crate) harness: Rc<IdRegistrationHarness>,
    pub(crate) control: fn() -> Element,
    pub(crate) description_id: &'static str,
    pub(crate) error_id: &'static str,
}

pub(crate) fn id_registration_app(props: IdRegistrationApp) -> Element {
    let meta = use_field_meta_state(FieldMetaValues {
        invalid: Some(true),
        ..FieldMetaValues::default()
    });
    let show_parts = use_signal(|| true);
    props.harness.show_parts.borrow_mut().replace(show_parts);
    let IdRegistrationApp {
        control,
        description_id,
        error_id,
        ..
    } = props;

    rsx! {
        Field { context: FieldContext::empty().with_meta(meta),
            {control()}
            if show_parts() {
                FieldDescription { id: description_id, "Description" }
                FieldError { id: error_id }
            }
        }
    }
}

pub(crate) fn assert_error_and_description_id_registration(
    control: fn() -> Element,
    interaction_listener: &str,
    description_id: &'static str,
    error_id: &'static str,
    exposes_validity: bool,
) {
    let harness = Rc::new(IdRegistrationHarness::default());
    let mut dom = InteractionDom::mount(
        VirtualDom::new_with_props(
            id_registration_app,
            IdRegistrationApp {
                harness: Rc::clone(&harness),
                control,
                description_id,
                error_id,
            },
        ),
        interaction_listener,
    );
    dom.render_reactive_updates();

    assert_eq!(
        dom.attributes.get("aria-describedby"),
        Some(format!("{description_id} {error_id}").as_str())
    );
    if exposes_validity {
        assert_eq!(dom.attributes.get("aria-errormessage"), Some(error_id));
    } else {
        assert!(!dom.attributes.has("aria-errormessage"));
    }

    hide_registered_parts(&harness, &mut dom);
    assert!(!dom.attributes.has("aria-describedby"));
    assert!(!dom.attributes.has("aria-errormessage"));
}

#[derive(Default)]
pub(crate) struct RerenderHarness {
    alternate: RefCell<Option<Signal<bool>>>,
}

impl RerenderHarness {
    pub(crate) fn show_alternate(&self) {
        self.alternate
            .borrow_mut()
            .as_mut()
            .expect("app should expose its alternate-state signal")
            .set(true);
    }
}

pub(crate) fn use_rerender_scaffold(
    harness: &RerenderHarness,
    initial_id: &'static str,
    alternate_id: &'static str,
) -> (bool, FieldMeta) {
    let alternate = use_signal(|| false);
    harness.alternate.borrow_mut().replace(alternate);
    let initial_meta = use_field_meta_state(FieldMetaValues {
        id: Some(Rc::from(initial_id)),
        invalid: Some(false),
        ..FieldMetaValues::default()
    });
    let alternate_meta = use_field_meta_state(FieldMetaValues {
        id: Some(Rc::from(alternate_id)),
        invalid: Some(true),
        ..FieldMetaValues::default()
    });
    let is_alternate = alternate();
    let meta = if is_alternate {
        alternate_meta
    } else {
        initial_meta
    };

    (is_alternate, meta)
}

pub(crate) fn hide_registered_parts(
    harness: &IdRegistrationHarness,
    dom: &mut impl RendersReactiveUpdates,
) {
    harness
        .show_parts
        .borrow_mut()
        .as_mut()
        .expect("app should expose part visibility")
        .set(false);
    dom.render_reactive_updates();
}

pub(crate) struct SliderInteractionDom {
    pub(crate) interaction: InteractionDom,
    root: dioxus_core::ElementId,
    pub(crate) root_attributes: ControlAttributes,
}

impl SliderInteractionDom {
    pub(crate) fn mount(mut dom: VirtualDom) -> Self {
        set_test_event_converter();
        let mutations = dom.rebuild_to_vec();
        let thumb = element_with_listeners(&mutations.edits, "keydown", "keyup");
        let root = listener_element(&mutations.edits, "pointerup");
        let mut thumb_attributes = ControlAttributes::default();
        thumb_attributes.apply(&mutations.edits, thumb);
        let mut root_attributes = ControlAttributes::default();
        root_attributes.apply(&mutations.edits, root);

        Self {
            interaction: InteractionDom {
                dom,
                control: thumb,
                attributes: thumb_attributes,
            },
            root,
            root_attributes,
        }
    }

    pub(crate) fn dispatch_arrow(&self, name: &str) {
        self.interaction.dispatch(
            name,
            SerializedKeyboardData::new(
                Key::ArrowRight,
                Code::ArrowRight,
                Location::Standard,
                false,
                Modifiers::empty(),
                false,
            ),
        );
    }
}

impl RendersReactiveUpdates for SliderInteractionDom {
    fn render_reactive_updates(&mut self) {
        for _ in 0..2 {
            let mutations = self.interaction.dom.render_immediate_to_vec();
            self.interaction
                .attributes
                .apply(&mutations.edits, self.interaction.control);
            self.root_attributes.apply(&mutations.edits, self.root);
        }
    }
}

fn listener_element(edits: &[dioxus_core::Mutation], listener: &str) -> dioxus_core::ElementId {
    edits
        .iter()
        .find_map(|edit| match edit {
            dioxus_core::Mutation::NewEventListener { name, id } if name == listener => Some(*id),
            _ => None,
        })
        .unwrap_or_else(|| panic!("slider should render its {listener} listener"))
}

fn element_with_listeners(
    edits: &[dioxus_core::Mutation],
    first: &str,
    second: &str,
) -> dioxus_core::ElementId {
    edits
        .iter()
        .find_map(|edit| match edit {
            dioxus_core::Mutation::NewEventListener { name, id }
                if name == first
                    && edits.iter().any(|other| {
                        matches!(
                            other,
                            dioxus_core::Mutation::NewEventListener { name, id: other_id }
                                if name == second && other_id == id
                        )
                    }) =>
            {
                Some(*id)
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("slider should render its {first} and {second} listeners"))
}

pub(crate) fn mount_slider_control(mut dom: VirtualDom) -> (VirtualDom, dioxus_core::ElementId) {
    set_test_event_converter();
    let mutations = dom.rebuild_to_vec();
    let control = element_with_listeners(&mutations.edits, "keydown", "keyup");
    let has_mounted = mutations.edits.iter().any(|edit| {
        matches!(
            edit,
            dioxus_core::Mutation::NewEventListener { name, id }
                if name == "mounted" && *id == control
        )
    });
    assert!(
        has_mounted,
        "slider thumb should render its mounted listener"
    );

    (dom, control)
}
