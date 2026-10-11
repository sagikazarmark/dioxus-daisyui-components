//! Attributes that `GlobalAttributes` and an element both define, named on a Component that
//! extends both (ADR-0034).
//!
//! Without the Component's inherent builder methods, naming one of these is E0034, so compiling
//! this file is half the test. The form controls are also mounted, to show the attribute reaches
//! the native element, and through the Composition sugar, the inner control rather than the Field.

use dioxus::prelude::*;
use dioxus_field::FieldContext;

use dioxus_daisyui_components::components::{
    accordion::AccordionTrigger,
    alert_dialog::{AlertDialogAction, AlertDialogCancel},
    avatar::AvatarImage,
    button::Button,
    collapsible::CollapsibleTrigger,
    drawer::DrawerTrigger,
    dropdown_menu::DropdownMenuTrigger,
    input::{Input, InputField},
    menubar::MenubarTrigger,
    native_select::{NativeSelect, NativeSelectField, NativeSelectOption},
    navbar::NavbarTrigger,
    popover::PopoverTrigger,
    slider::SliderThumb,
    tabs::TabTrigger,
    tag_group::TagRemoveButton,
    textarea::{Textarea, TextareaField},
    toast::ToastCloseButton,
    toggle::Toggle,
    toggle_group::ToggleItem,
    toolbar::ToolbarButton,
};

use crate::harness::InteractionDom;

/// Every colliding name on every Component that extends both sets, with the value types a caller
/// writes on a native element. Never rendered: most of these parts need their Root's context.
#[allow(dead_code)]
fn every_colliding_attribute_compiles() -> Element {
    let options = || vec![NativeSelectOption::new(String::from("one"), "One")];
    rsx! {
        Input {
            alt: "",
            autofocus: true,
            height: 1,
            spellcheck: false,
            tabindex: 0,
            width: 1,
        }
        InputField {
            context: FieldContext::empty(),
            label: "Command",
            alt: "",
            autofocus: true,
            height: 1,
            spellcheck: "false",
            tabindex: 0,
            width: 1,
        }
        Textarea { autofocus: true, spellcheck: false }
        TextareaField { context: FieldContext::empty(), label: "Notes", autofocus: true, spellcheck: "false" }
        NativeSelect { options: options(), autofocus: true }
        NativeSelectField { context: FieldContext::empty(), label: "Choice", options: options(), autofocus: true }
        AvatarImage { src: "", width: 1, height: 1 }
        Button { autofocus: true }
        AccordionTrigger { autofocus: true }
        AlertDialogAction { autofocus: true }
        AlertDialogCancel { autofocus: true }
        CollapsibleTrigger { autofocus: true }
        DrawerTrigger { autofocus: true }
        DropdownMenuTrigger { autofocus: true }
        MenubarTrigger { autofocus: true }
        NavbarTrigger { autofocus: true }
        PopoverTrigger { autofocus: true }
        SliderThumb { autofocus: true }
        TabTrigger { value: "tab", index: 0usize, autofocus: true }
        TagRemoveButton { autofocus: true }
        ToastCloseButton { autofocus: true }
        Toggle { autofocus: true }
        ToggleItem { index: 0usize, autofocus: true }
        ToolbarButton { index: 0usize, autofocus: true }
    }
}

fn mounted(app: fn() -> Element) -> InteractionDom {
    InteractionDom::mount(VirtualDom::new(app), "input")
}

fn assert_reaches_the_control(dom: &InteractionDom, names: &[(&str, &str)]) {
    for (name, value) in names {
        assert_eq!(
            dom.attributes.get(name),
            Some(*value),
            "`{name}` should reach the native control"
        );
    }
}

#[test]
fn input_attributes_both_sets_define_reach_the_native_input() {
    fn app() -> Element {
        rsx! {
            Input {
                alt: "Alternative",
                autofocus: true,
                height: 10,
                spellcheck: "false",
                tabindex: 3,
                width: 20,
            }
        }
    }

    assert_reaches_the_control(
        &mounted(app),
        &[
            ("alt", "Alternative"),
            ("autofocus", ""),
            ("height", "10"),
            ("spellcheck", "false"),
            ("tabindex", "3"),
            ("width", "20"),
        ],
    );
}

#[test]
fn input_field_attributes_both_sets_define_reach_the_inner_input() {
    fn app() -> Element {
        rsx! {
            InputField { context: FieldContext::empty(), label: "Command", autofocus: true, spellcheck: "false" }
        }
    }

    assert_reaches_the_control(&mounted(app), &[("autofocus", ""), ("spellcheck", "false")]);
}

#[test]
fn textarea_attributes_both_sets_define_reach_the_native_textarea() {
    fn app() -> Element {
        rsx! {
            Textarea { autofocus: true, spellcheck: "false" }
        }
    }
    fn field() -> Element {
        rsx! {
            TextareaField { context: FieldContext::empty(), label: "Notes", autofocus: true, spellcheck: "false" }
        }
    }

    for app in [app as fn() -> Element, field] {
        assert_reaches_the_control(&mounted(app), &[("autofocus", ""), ("spellcheck", "false")]);
    }
}

#[test]
fn native_select_autofocus_reaches_the_native_select() {
    fn app() -> Element {
        let options = vec![NativeSelectOption::new(String::from("one"), "One")];
        rsx! {
            NativeSelect { options, autofocus: true }
        }
    }
    fn field() -> Element {
        let options = vec![NativeSelectOption::new(String::from("one"), "One")];
        rsx! {
            NativeSelectField { context: FieldContext::empty(), label: "Choice", options, autofocus: true }
        }
    }

    for app in [app as fn() -> Element, field] {
        assert_reaches_the_control(&mounted(app), &[("autofocus", "")]);
    }
}
