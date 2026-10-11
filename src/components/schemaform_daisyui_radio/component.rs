//! The `daisyui:radio` widget: a choice presented as the registry's radio group.

use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_field::{Binding, ChangeOrigin};
use schemaform::WidgetSymbol;
use schemaform_dioxus::{
    ChoiceEdit, ControlRegistry, ControlRenderContext, ControlRenderer, use_choice_edit,
};

use crate::components::radio_group::{RadioGroup, RadioItem};
use crate::components::schemaform_daisyui::{
    Addon, Appearance, DaisyuiControlRenderer, Options, WidgetLayout, choice_handles, editable,
    editable_field, kind_name, use_focus_exit, use_no_commit,
};

/// The widget symbol a UI schema control names to render a choice as a radio group.
pub const RADIO_WIDGET: &str = "daisyui:radio";

/// The `schemaform_daisyui` add-on that registers [`RADIO_WIDGET`].
///
/// Add it to `SchemaformDaisyui`'s `addons`, or apply [`Addon::controls`] to a registry composed
/// by hand. Without it, a UI schema naming the symbol fails to bind with the adapter's missing
/// widget finding.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RadioAddon;

impl Addon for RadioAddon {
    fn controls(&self, registry: ControlRegistry, options: Options) -> ControlRegistry {
        let symbol =
            WidgetSymbol::parse(RADIO_WIDGET).expect("the radio widget symbol is non-empty");
        registry.widget(
            symbol,
            Arc::new(RadioRenderer(DaisyuiControlRenderer::from(options))),
        )
    }
}

/// The renderer registered for [`RADIO_WIDGET`]: a choice is a radio group, and every other kind
/// renders as `schemaform_daisyui` renders it, since an exact widget request reaches its renderer
/// whatever the control's kind.
struct RadioRenderer(DaisyuiControlRenderer);

impl ControlRenderer for RadioRenderer {
    fn render(&self, context: ControlRenderContext) -> Element {
        self.0.render_with_choice(context, |context, appearance| {
            rsx! { RadioGroupControl { context, appearance } }
        })
    }
}

/// One daisyUI choice control as a radio group: the renderer's hook-safe child component, and
/// what a host's own control renderer renders to present a choice this way.
///
/// Every option, the null option included, is a `RadioItem` whose value is the option's opaque
/// identity string, labelled by the text beside it. The group root carries the control's
/// element id and ARIA state; the hidden form participants the registry renders carry its name.
/// Nothing is checked while nothing is selected, which is also a write-only control's resting
/// state after every write. The registry reports focus exit one task after the group loses
/// focus, so touched state lands one task later than with the native select.
#[component]
pub fn RadioGroupControl(
    /// The control the adapter hands the renderer.
    context: ControlRenderContext,
    /// Whether the control emits its layout utilities.
    appearance: Appearance,
) -> Element {
    let edit = use_choice_edit(&context);
    let binding = use_radio_binding(edit.clone());
    if let Err(rendered) = editable(&context, appearance) {
        return rendered;
    }
    let control = context.control();
    let kind = kind_name(control.kind);
    let element_id = context.presentation().element_id.clone();
    // Each item and its label carry ids derived from the control's element id and the option's
    // identity, so they are stable per option and unique within the form.
    let items = edit
        .options
        .iter()
        .map(|option| {
            let item_id = format!("{element_id}-{}", option.identity.as_str());
            let label_id = format!("{item_id}-label");
            (option.clone(), item_id, label_id)
        })
        .collect::<Vec<_>>();

    let row_class = appearance.utilities("flex items-center gap-2");
    let item_label_class = appearance.utilities("text-sm");
    editable_field(
        &context,
        appearance,
        binding,
        WidgetLayout::Stacked,
        rsx! {
            RadioGroup {
                "data-schemaform-control": kind,
                "data-write-only-replacement": control.write_only.then_some(""),
                for (index , (option , item_id , item_label_id)) in items.into_iter().enumerate() {
                    div { key: "{item_id}", class: row_class,
                        RadioItem {
                            id: item_id.clone(),
                            value: option.identity.as_str().to_owned(),
                            index,
                            disabled: option.disabled,
                            aria_labelledby: item_label_id.clone(),
                        }
                        span { id: item_label_id, class: item_label_class, "{option.label}" }
                    }
                }
            }
        },
    )
}

/// Adapts a [`ChoiceEdit`] to the string binding a registry `RadioGroup` speaks.
///
/// This is a hook: call it unconditionally in the renderer's child component, after
/// [`schemaform_dioxus::use_choice_edit`]. The value is the selected option's identity as
/// [`schemaform_dioxus::ChoiceIdentity::as_str`], or the empty string while nothing is selected,
/// so an item whose `value` is its option's identity string is the checked one. Write maps the
/// string back to the option carrying it and selects that option; a string no option carries
/// selects nothing, which the edit treats as a no-op. Commit and Focus Exit behave as for
/// `schemaform_daisyui::use_choice_binding`.
pub fn use_radio_binding(edit: ChoiceEdit) -> Binding<String> {
    let selected = edit.selected;
    let value = use_memo(move || {
        selected
            .read()
            .as_ref()
            .map(|identity| identity.as_str().to_owned())
            .unwrap_or_default()
    });
    let read = use_hook(|| ReadSignal::new(value));
    let identity = choice_handles(&edit);
    let select = edit.select;
    let blur = edit.blur;
    // `use_callback` keeps the handle and replaces the closure on every render, so the lookup
    // always sees the option list of the latest render.
    let options = edit.options;
    let write = use_callback(move |(value, _origin): (String, ChangeOrigin)| {
        let identity = options
            .iter()
            .find(|option| option.identity.as_str() == value)
            .map(|option| option.identity.clone());
        select.call(identity);
    });
    let commit = use_no_commit();
    let focus_exit = use_focus_exit(blur);
    Binding::new_with_identity(read, write, commit, identity)
        .with_focus_exit_using_identity(focus_exit)
}
