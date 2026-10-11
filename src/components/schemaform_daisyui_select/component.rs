//! The `daisyui:select` widget: a choice presented as the registry's compound select.

use std::sync::Arc;

use dioxus::prelude::*;
use schemaform::WidgetSymbol;
use schemaform_dioxus::{
    ChoiceIdentity, ControlRegistry, ControlRenderContext, ControlRenderer, use_choice_edit,
};

use crate::components::schemaform_daisyui::{
    Addon, Appearance, DaisyuiControlRenderer, Options, WidgetLayout, editable, editable_field,
    kind_name, placeholder, use_choice_binding,
};
use crate::components::select::{Select, SelectList, SelectOption, SelectTrigger, SelectValue};

/// The widget symbol a UI schema control names to render a choice as the registry's compound
/// select rather than the native one.
pub const SELECT_WIDGET: &str = "daisyui:select";

/// The `schemaform_daisyui` add-on that registers [`SELECT_WIDGET`].
///
/// Add it to `SchemaformDaisyui`'s `addons`, or apply [`Addon::controls`] to a registry composed
/// by hand. Without it, a UI schema naming the symbol fails to bind with the adapter's missing
/// widget finding.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SelectAddon;

impl Addon for SelectAddon {
    fn controls(&self, registry: ControlRegistry, options: Options) -> ControlRegistry {
        let symbol =
            WidgetSymbol::parse(SELECT_WIDGET).expect("the select widget symbol is non-empty");
        registry.widget(
            symbol,
            Arc::new(SelectRenderer(DaisyuiControlRenderer::from(options))),
        )
    }
}

/// The renderer registered for [`SELECT_WIDGET`]: a choice is the compound select, and every
/// other kind renders as `schemaform_daisyui` renders it, since an exact widget request reaches
/// its renderer whatever the control's kind.
struct SelectRenderer(DaisyuiControlRenderer);

impl ControlRenderer for SelectRenderer {
    fn render(&self, context: ControlRenderContext) -> Element {
        self.0.render_with_choice(context, |context, appearance| {
            rsx! { SelectControl { context, appearance } }
        })
    }
}

/// One daisyUI choice control as the registry's compound select: the renderer's hook-safe child
/// component, and what a host's own control renderer renders to present a choice this way.
///
/// Every option, the null option included, is a `SelectOption` over the option's opaque identity
/// whose text is the localized label; the trigger shows that text for the selected option and the
/// placeholder otherwise. The trigger carries the control's element id and name. The registry
/// reports focus exit one task after the trigger loses focus, so touched state lands one task
/// later than with the native select.
#[component]
pub fn SelectControl(
    /// The control the adapter hands the renderer.
    context: ControlRenderContext,
    /// Whether the control emits its layout utilities.
    appearance: Appearance,
) -> Element {
    let edit = use_choice_edit(&context);
    let binding = use_choice_binding(edit.clone());
    if let Err(rendered) = editable(&context, appearance) {
        return rendered;
    }
    let control = context.control();
    let kind = kind_name(control.kind);
    let placeholder = placeholder(&context);
    let options = edit.options.clone();

    editable_field(
        &context,
        appearance,
        binding,
        WidgetLayout::Stacked,
        rsx! {
            Select::<ChoiceIdentity> {
                SelectTrigger {
                    "data-schemaform-control": kind,
                    "data-write-only-replacement": control.write_only.then_some(""),
                    SelectValue { placeholder }
                }
                SelectList {
                    for (index , option) in options.into_iter().enumerate() {
                        SelectOption::<ChoiceIdentity> {
                            key: "{option.identity.as_str()}",
                            value: option.identity.clone(),
                            index,
                            text_value: Some(option.label.clone()),
                            disabled: option.disabled,
                            "{option.label}"
                        }
                    }
                }
            }
        },
    )
}
