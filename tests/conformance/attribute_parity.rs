use std::{collections::BTreeMap, rc::Rc};

use dioxus::prelude::*;
use dioxus_field::{FieldContext, FieldMetaValues, use_field_meta_state};

use dioxus_daisyui_components::components::{
    checkbox::Checkbox,
    combobox::{Combobox, ComboboxInput, ComboboxList, ComboboxOption},
    field::{Field, FieldDescription, FieldError},
    input::Input,
    native_select::{NativeSelect, NativeSelectOption},
    otp::Otp,
    radio_group::{RadioGroup, RadioItem},
    select::{Select, SelectTrigger, SelectValue},
    slider::Slider,
    switch::Switch,
    textarea::Textarea,
};

use crate::harness::{ControlAttributes, InteractionDom, RendersReactiveUpdates};

const CONTROL_ID: &str = "attribute-parity-control";
const EXPLICIT_ID: &str = "attribute-parity-explicit-control";
const META_NAME: &str = "attribute-parity-meta";
const EXPLICIT_NAME: &str = "attribute-parity-explicit";
const DESCRIPTION_ID: &str = "attribute-parity-description";
const ERROR_ID: &str = "attribute-parity-error";
const SECOND_ERROR_ID: &str = "attribute-parity-second-error";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    Metadata,
    ExplicitFalse,
    ExplicitTrue,
}

impl Phase {
    fn name(self) -> Option<String> {
        (self != Self::Metadata).then(|| EXPLICIT_NAME.to_owned())
    }

    fn explicit_flag(self) -> Option<bool> {
        match self {
            Self::Metadata => None,
            Self::ExplicitFalse => Some(false),
            Self::ExplicitTrue => Some(true),
        }
    }

    fn explicit_id(self) -> Option<String> {
        (self != Self::Metadata).then(|| EXPLICIT_ID.to_owned())
    }

    fn explicit_id_attributes(self) -> Vec<Attribute> {
        self.explicit_id()
            .map_or_else(Vec::new, |id| vec![Attribute::new("id", id, None, false)])
    }

    fn metadata_flag(self) -> bool {
        self != Self::ExplicitTrue
    }

    fn resolved_flag(self) -> bool {
        self != Self::ExplicitFalse
    }
}

#[derive(Clone)]
struct AttributeParityApp {
    phase: Phase,
    control: fn(Phase) -> Element,
}

fn attribute_parity_app(props: AttributeParityApp) -> Element {
    let metadata_flag = props.phase.metadata_flag();
    let meta = use_field_meta_state(FieldMetaValues {
        id: Some(Rc::from(CONTROL_ID)),
        name: Some(Rc::from(META_NAME)),
        required: metadata_flag,
        disabled: metadata_flag,
        invalid: Some(true),
        errors: vec![Rc::from("Invalid value")],
        touched: true,
        dirty: true,
    });
    let context = FieldContext::empty().with_meta(meta);
    let AttributeParityApp { phase, control } = props;

    rsx! {
        Field { context,
            {control(phase)}
            FieldDescription { id: DESCRIPTION_ID, "Description" }
            FieldError { id: ERROR_ID }
            FieldError { id: SECOND_ERROR_ID }
        }
    }
}

fn checkbox(phase: Phase) -> Element {
    rsx! {
        Checkbox {
            attributes: phase.explicit_id_attributes(),
            name: phase.name(),
            required: phase.explicit_flag(),
            disabled: phase.explicit_flag(),
            aria_label: "Attribute parity checkbox",
        }
    }
}

fn switch(phase: Phase) -> Element {
    rsx! {
        Switch {
            attributes: phase.explicit_id_attributes(),
            name: phase.name(),
            required: phase.explicit_flag(),
            disabled: phase.explicit_flag(),
            aria_label: "Attribute parity switch",
        }
    }
}

fn radio_group(phase: Phase) -> Element {
    rsx! {
        RadioGroup {
            attributes: phase.explicit_id_attributes(),
            name: phase.name(),
            required: phase.explicit_flag(),
            disabled: phase.explicit_flag(),
            aria_label: "Attribute parity radio group",
            RadioItem { value: "option".to_owned(), index: 0usize, aria_label: "Option" }
        }
    }
}

fn slider(phase: Phase) -> Element {
    rsx! {
        Slider {
            attributes: phase.explicit_id_attributes(),
            name: phase.name(),
            required: phase.explicit_flag(),
            disabled: phase.explicit_flag(),
            label: "Attribute parity slider",
        }
    }
}

fn input(phase: Phase) -> Element {
    match phase {
        Phase::Metadata => rsx! { Input { aria_label: "Attribute parity input" } },
        Phase::ExplicitFalse => rsx! {
            Input {
                id: EXPLICIT_ID,
                name: EXPLICIT_NAME,
                required: false,
                disabled: false,
                aria_label: "Attribute parity input",
            }
        },
        Phase::ExplicitTrue => rsx! {
            Input {
                id: EXPLICIT_ID,
                name: EXPLICIT_NAME,
                required: true,
                disabled: true,
                aria_label: "Attribute parity input",
            }
        },
    }
}

fn textarea(phase: Phase) -> Element {
    match phase {
        Phase::Metadata => rsx! { Textarea { aria_label: "Attribute parity textarea" } },
        Phase::ExplicitFalse => rsx! {
            Textarea {
                id: EXPLICIT_ID,
                name: EXPLICIT_NAME,
                required: false,
                disabled: false,
                aria_label: "Attribute parity textarea",
            }
        },
        Phase::ExplicitTrue => rsx! {
            Textarea {
                id: EXPLICIT_ID,
                name: EXPLICIT_NAME,
                required: true,
                disabled: true,
                aria_label: "Attribute parity textarea",
            }
        },
    }
}

fn native_select(phase: Phase) -> Element {
    let options = vec![NativeSelectOption::new(String::from("one"), "One")];
    match phase {
        Phase::Metadata => rsx! {
            NativeSelect { options, aria_label: "Attribute parity native select" }
        },
        Phase::ExplicitFalse => rsx! {
            NativeSelect {
                options,
                id: EXPLICIT_ID,
                name: EXPLICIT_NAME,
                required: false,
                disabled: false,
                aria_label: "Attribute parity native select",
            }
        },
        Phase::ExplicitTrue => rsx! {
            NativeSelect {
                options,
                id: EXPLICIT_ID,
                name: EXPLICIT_NAME,
                required: true,
                disabled: true,
                aria_label: "Attribute parity native select",
            }
        },
    }
}

fn otp(phase: Phase) -> Element {
    rsx! {
        Otp {
            attributes: phase.explicit_id_attributes(),
            name: phase.name(),
            required: phase.explicit_flag(),
            disabled: phase.explicit_flag(),
            aria_label: "Attribute parity OTP",
        }
    }
}

fn select(phase: Phase) -> Element {
    rsx! {
        Select::<String> {
            name: phase.name(),
            required: phase.explicit_flag(),
            disabled: phase.explicit_flag(),
            SelectTrigger {
                attributes: phase.explicit_id_attributes(),
                aria_label: "Attribute parity select",
                SelectValue {}
            }
        }
    }
}

fn combobox(phase: Phase) -> Element {
    rsx! {
        Combobox::<String> {
            name: phase.name(),
            required: phase.explicit_flag(),
            disabled: phase.explicit_flag(),
            ComboboxInput {
                id: phase.explicit_id(),
                aria_label: "Attribute parity combobox",
            }
            ComboboxList {
                ComboboxOption::<String> {
                    value: "option".to_owned(),
                    index: 0usize,
                    "Option"
                }
            }
        }
    }
}

struct ControlSpec {
    name: &'static str,
    control: fn(Phase) -> Element,
    listener: &'static str,
    surface: Surface,
    structural_attributes: &'static [&'static str],
    explicit_false_attributes: &'static [&'static str],
}

#[derive(Clone, Copy)]
enum Surface {
    Native,
    ButtonWidget,
    AriaWidget,
    Slider,
    Select,
}

impl Surface {
    fn name(self) -> bool {
        matches!(self, Self::Native | Self::ButtonWidget | Self::Select)
    }

    fn validity(self) -> bool {
        !matches!(self, Self::Slider | Self::Select)
    }

    fn required(self) -> Option<&'static str> {
        match self {
            Self::Native => Some("required"),
            Self::ButtonWidget | Self::AriaWidget | Self::Slider => Some("aria-required"),
            Self::Select => None,
        }
    }

    fn disabled(self) -> &'static str {
        match self {
            Self::Native | Self::ButtonWidget | Self::Select => "disabled",
            Self::AriaWidget | Self::Slider => "aria-disabled",
        }
    }
}

// These are widget- and role-specific attributes, not Field metadata. Listing
// them per control keeps normalization explicit while ADR-0007 leaves their
// ordinary Component behavior to the Preview.
const CONTROLS: &[ControlSpec] = &[
    ControlSpec {
        name: "checkbox",
        control: checkbox,
        listener: "click",
        surface: Surface::ButtonWidget,
        structural_attributes: &[
            "aria-checked",
            "aria-label",
            "class",
            "data-state",
            "role",
            "type",
            "value",
        ],
        explicit_false_attributes: &[],
    },
    ControlSpec {
        name: "switch",
        control: switch,
        listener: "click",
        surface: Surface::ButtonWidget,
        structural_attributes: &[
            "aria-checked",
            "aria-label",
            "class",
            "data-state",
            "role",
            "type",
            "value",
        ],
        // The Primitive serializes its false disabled state instead of omitting it.
        explicit_false_attributes: &["data-disabled"],
    },
    ControlSpec {
        name: "radio_group",
        control: radio_group,
        listener: "focusout",
        surface: Surface::AriaWidget,
        structural_attributes: &[
            "aria-label",
            "aria-orientation",
            "class",
            "data-field-group",
            "data-orientation",
            "role",
            "tabindex",
        ],
        explicit_false_attributes: &[],
    },
    ControlSpec {
        name: "slider",
        control: slider,
        listener: "pointerup",
        surface: Surface::Slider,
        structural_attributes: &[
            "aria-orientation",
            "class",
            "data-orientation",
            "data-slider-pointer-capture",
            "dir",
            "role",
            "style",
        ],
        explicit_false_attributes: &[],
    },
    ControlSpec {
        name: "input",
        control: input,
        listener: "input",
        surface: Surface::Native,
        structural_attributes: &["aria-label", "class", "value"],
        explicit_false_attributes: &[],
    },
    ControlSpec {
        name: "textarea",
        control: textarea,
        listener: "input",
        surface: Surface::Native,
        structural_attributes: &["aria-label", "class", "value"],
        explicit_false_attributes: &[],
    },
    ControlSpec {
        name: "native_select",
        control: native_select,
        listener: "input",
        surface: Surface::Native,
        structural_attributes: &["aria-label", "class", "value"],
        explicit_false_attributes: &[],
    },
    ControlSpec {
        name: "otp",
        control: otp,
        listener: "input",
        surface: Surface::Native,
        structural_attributes: &[
            "aria-label",
            "autocomplete",
            "inputmode",
            "maxlength",
            "pattern",
            "type",
            "value",
        ],
        explicit_false_attributes: &[],
    },
    ControlSpec {
        name: "select",
        control: select,
        listener: "click",
        surface: Surface::Select,
        structural_attributes: &[
            "aria-controls",
            "aria-expanded",
            "aria-haspopup",
            "aria-label",
            "class",
            "data-state",
            "role",
            "type",
        ],
        explicit_false_attributes: &[],
    },
    ControlSpec {
        name: "combobox",
        control: combobox,
        listener: "keydown",
        surface: Surface::Native,
        structural_attributes: &[
            "aria-activedescendant",
            "aria-autocomplete",
            "aria-controls",
            "aria-expanded",
            "aria-haspopup",
            "aria-label",
            "autocomplete",
            "class",
            "data-state",
            "placeholder",
            "role",
            "type",
            "value",
        ],
        explicit_false_attributes: &[],
    },
];

fn mount_attributes(spec: &ControlSpec, phase: Phase) -> ControlAttributes {
    let mut dom = InteractionDom::mount(
        VirtualDom::new_with_props(
            attribute_parity_app,
            AttributeParityApp {
                phase,
                control: spec.control,
            },
        ),
        spec.listener,
    );
    dom.render_reactive_updates();

    dom.attributes
}

fn expected_attributes(spec: &ControlSpec, phase: Phase) -> BTreeMap<&'static str, String> {
    let mut expected = BTreeMap::from([
        (
            "aria-describedby",
            format!("{DESCRIPTION_ID} {ERROR_ID} {SECOND_ERROR_ID}"),
        ),
        ("data-dirty", "true".to_owned()),
        ("data-invalid", "true".to_owned()),
        ("data-touched", "true".to_owned()),
        (
            "id",
            match phase {
                Phase::Metadata => CONTROL_ID,
                Phase::ExplicitFalse | Phase::ExplicitTrue => EXPLICIT_ID,
            }
            .to_owned(),
        ),
    ]);
    if spec.surface.name() {
        expected.insert(
            "name",
            match phase {
                Phase::Metadata => META_NAME,
                Phase::ExplicitFalse | Phase::ExplicitTrue => EXPLICIT_NAME,
            }
            .to_owned(),
        );
    }
    if spec.surface.validity() {
        expected.extend([
            ("aria-errormessage", ERROR_ID.to_owned()),
            ("aria-invalid", "true".to_owned()),
        ]);
    }
    if phase.resolved_flag() {
        expected.extend([
            ("data-disabled", "true".to_owned()),
            ("data-required", "true".to_owned()),
            (
                spec.surface.disabled(),
                attribute_state(spec.surface.disabled()),
            ),
        ]);
        if let Some(required) = spec.surface.required() {
            expected.insert(required, attribute_state(required));
        }
    }
    expected
}

fn attribute_state(name: &str) -> String {
    if name.starts_with("aria-") {
        "true".to_owned()
    } else {
        String::new()
    }
}

fn field_attributes(
    attributes: &ControlAttributes,
    spec: &ControlSpec,
    phase: Phase,
) -> BTreeMap<&'static str, String> {
    attributes
        .iter()
        .filter(|(name, _)| !spec.structural_attributes.contains(name))
        .filter(|(name, _)| {
            phase != Phase::ExplicitFalse || !spec.explicit_false_attributes.contains(name)
        })
        .map(|(name, value)| (name, value.to_owned()))
        .collect()
}

#[test]
fn field_aware_controls_retain_the_same_metadata_attribute_surface() {
    for phase in [Phase::Metadata, Phase::ExplicitFalse, Phase::ExplicitTrue] {
        for spec in CONTROLS {
            let attributes = mount_attributes(spec, phase);

            assert_eq!(
                field_attributes(&attributes, spec, phase),
                expected_attributes(spec, phase),
                "{} must retain the Field attribute surface in the {phase:?} phase",
                spec.name,
            );
        }
    }
}
