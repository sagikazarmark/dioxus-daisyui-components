use dioxus::core::{AttributeValue, ListenerCallback};
use dioxus::prelude::*;
use dioxus_field::{
    Binding, ChangeOrigin, FieldControlOptions, FieldMeta, FieldSurface, merge_attributes,
    use_binding, use_field_meta, use_focus_registration,
};
use dioxus_primitives::combobox;
use dioxus_primitives::combobox::default_combobox_filter;
use dioxus_primitives::dioxus_attributes::attributes;
use std::{cell::Cell, rc::Rc};

/// daisyUI's colour axis for a combobox, which colours the field's border and
/// the outline it takes on focus.
///
/// The class strings are the input's rather than a combobox's (daisyUI has no
/// combobox at all) and they carry no caveat here: the element they land on is
/// a real `input`, which is what `.input` was written for.
///
/// [`ComboboxColor::Default`] emits no class at all, which is daisyUI's own
/// uncoloured field rather than a synonym for [`ComboboxColor::Neutral`].
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ComboboxColor {
    #[default]
    Default,
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}

impl ComboboxColor {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[
        Self::Default,
        Self::Neutral,
        Self::Primary,
        Self::Secondary,
        Self::Accent,
        Self::Info,
        Self::Success,
        Self::Warning,
        Self::Error,
    ];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Neutral => "input-neutral",
            Self::Primary => "input-primary",
            Self::Secondary => "input-secondary",
            Self::Accent => "input-accent",
            Self::Info => "input-info",
            Self::Success => "input-success",
            Self::Warning => "input-warning",
            Self::Error => "input-error",
        }
    }
}

/// daisyUI's size axis for a combobox, which sizes the field rather than the
/// popup under it.
///
/// [`ComboboxSize::Default`] emits no class, which renders at the same size as
/// daisyUI's explicit `input-md`.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ComboboxSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

impl ComboboxSize {
    /// Every value of this axis, from the smallest to the largest, which is the
    /// order the preview renders them in.
    pub const ALL: &'static [Self] = &[Self::Xs, Self::Sm, Self::Default, Self::Lg, Self::Xl];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Xs => "input-xs",
            Self::Sm => "input-sm",
            Self::Default => "",
            Self::Lg => "input-lg",
            Self::Xl => "input-xl",
        }
    }
}

/// daisyUI's side axis for the popup, which is the side of the field it
/// opens on.
///
/// The popup is a borrowed dropdown (ADR-0005), so this is the dropdown's axis
/// and it inherits the dropdown's rule with it: every value emits a class,
/// including the default one (ADR-0008).
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ComboboxSide {
    /// Above the field.
    Top,
    /// Under the field, which is where daisyUI puts an unplaced dropdown.
    #[default]
    Bottom,
    /// To the left of the field, in either writing direction; daisyUI's
    /// horizontal placements are physical rather than logical.
    Left,
    /// To the right of the field, mirroring [`ComboboxSide::Left`].
    Right,
}

impl ComboboxSide {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Top, Self::Bottom, Self::Left, Self::Right];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Top => "dropdown-top",
            Self::Bottom => "dropdown-bottom",
            Self::Left => "dropdown-left",
            Self::Right => "dropdown-right",
        }
    }
}

/// daisyUI's align axis for the popup, which is where it sits along the
/// side [`ComboboxSide`] opened it on.
///
/// Every value emits a class here too, for the reason [`ComboboxSide`]
/// records.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ComboboxAlign {
    /// The inline start edge, which follows the writing direction. This is
    /// where daisyUI puts an unaligned dropdown.
    #[default]
    Start,
    /// Centred on the field.
    Center,
    /// The inline end edge, mirroring [`ComboboxAlign::Start`].
    End,
}

impl ComboboxAlign {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Start, Self::Center, Self::End];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Start => "dropdown-start",
            Self::Center => "dropdown-center",
            Self::End => "dropdown-end",
        }
    }
}

/// daisyUI's size axis for the menu inside the popup, which sizes the options
/// rather than the box they are in.
///
/// It is a separate axis from [`ComboboxSize`] because daisyUI's are separate:
/// `input-lg` sizes a field and `menu-lg` sizes a list, and nothing in daisyUI
/// ties one to the other. It is an axis at all, rather than something a caller
/// passes through `class`, because the split (ADR-0005) leaves it unreachable
/// otherwise: `menu-lg` only works on the element carrying `menu`, and that is
/// the list rendered inside the box.
///
/// [`ComboboxListSize::Default`] emits no class, which renders at the same size
/// as daisyUI's explicit `menu-md`.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ComboboxListSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

impl ComboboxListSize {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Xs, Self::Sm, Self::Default, Self::Lg, Self::Xl];

    /// The daisyUI class name for this value, as a complete string literal so
    /// Tailwind's scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Xs => "menu-xs",
            Self::Sm => "menu-sm",
            Self::Default => "",
            Self::Lg => "menu-lg",
            Self::Xl => "menu-xl",
        }
    }
}

/// Whether [`ComboboxList`] emits the utilities that draw the box the options
/// sit in.
///
/// daisyUI's `dropdown-content` only positions the element: the fill, the
/// corners and the shadow in its own examples are Tailwind utilities on the
/// same element. They are emitted here instead, which inverts the usual
/// convention: [`ComboboxListAppearance::Default`] emits classes and
/// [`ComboboxListAppearance::None`] emits nothing. A utility this component
/// emits only ties with a caller's, and a tie is settled by generated-stylesheet
/// order rather than by the class attribute, so switching ours off is the way to
/// win it (ADR-0004).
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ComboboxListAppearance {
    #[default]
    Default,
    None,
}

impl ComboboxListAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The Tailwind utilities for this value, as complete string literals so
    /// Tailwind's scanner can see them.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "bg-base-100 rounded-box shadow-sm",
            Self::None => "",
        }
    }
}

/// Whether [`ComboboxOption`] emits the class that paints the option the
/// keyboard is on.
///
/// This is the one thing a combobox cannot borrow from the select, and ADR-0021
/// records why: the select's highlight is daisyUI's own `:focus-visible` rule,
/// which works because the primitive moves real DOM focus onto the option. A
/// combobox never does (focus stays in the field so that typing keeps working)
/// so the option that is highlighted is named by `aria-activedescendant` and
/// reported as `data-highlighted`, which daisyUI matches nowhere.
///
/// [`ComboboxOptionAppearance::Default`] therefore emits a **Bridged utility**:
/// daisyUI's own `menu-focus`, written as a variant of the attribute the
/// primitive already sets, so that nothing is recomputed in Rust and the
/// primitive stays the only owner of the state.
/// [`ComboboxOptionAppearance::None`] emits nothing, for the reason
/// [`ComboboxListAppearance`] records (ADR-0004).
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum ComboboxOptionAppearance {
    #[default]
    Default,
    None,
}

impl ComboboxOptionAppearance {
    /// Every value of this axis, in the order the preview renders them.
    pub const ALL: &'static [Self] = &[Self::Default, Self::None];

    /// The class for this value, as a complete string literal so Tailwind's
    /// scanner can see it.
    pub const fn class(self) -> &'static str {
        match self {
            Self::Default => "data-[highlighted=true]:menu-focus",
            Self::None => "",
        }
    }
}

/// The selected value, as [`Combobox`] holds it for the options to read.
///
/// This is the half of the lift (ADR-0006) that is this component's own, and it
/// is the select's for the same reason: daisyUI marks the chosen row of a menu
/// with `menu-active`, `.menu` matches no ARIA attribute at all, and the
/// primitive's context is private, so an option cannot read the value from the
/// primitive and [`Combobox`] provides it alongside.
struct Selection<T: Clone + PartialEq + 'static> {
    value: Memo<Option<T>>,
}

// Written out rather than derived: a derived `Clone` would demand `T: Clone` of
// the context itself, and a derived `Copy` would demand `T: Copy`, neither of
// which the value inside needs, because a `Memo` is `Copy` whatever it holds.
impl<T: Clone + PartialEq + 'static> Clone for Selection<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Clone + PartialEq + 'static> Copy for Selection<T> {}

/// Field metadata and explicit state resolved by [`Combobox`] for the input
/// part that renders the actual control.
#[derive(Clone, PartialEq)]
struct ComboboxField {
    meta: FieldMeta,
    required: Option<bool>,
    disabled: Option<bool>,
    name: Option<String>,
}

/// The outer element of a combobox, carrying daisyUI's `dropdown` classes.
///
/// Both the open state and the value are **lifted** here (ADR-0006), for the
/// reasons the select lifts them: daisyUI hides `.dropdown-content` outright
/// unless `dropdown-open` is on this element, and marks a menu's chosen row with
/// `menu-active`, and it matches no attribute the primitive sets for either. So
/// both classes have to be emitted from Rust, and this component has to know
/// both pieces of state to emit them. It seeds a signal from each `default_*`
/// prop, always hands the primitive a controlled value, and intercepts the
/// change callbacks, which leaves a controlled caller and an uncontrolled one
/// both working and this component the only writer. The selected value is the
/// field-shaped value; the query remains independent and primitive-owned.
///
/// **The query is not lifted.** Nothing daisyUI draws depends on what has been
/// typed, so it is passed straight through and the primitive stays its only
/// owner, which is what keeps the filtering, and the field's own display value,
/// exactly where they were.
///
/// Classes passed by the caller concatenate with this element's own; every other
/// attribute the caller passes overrides them.
#[component]
pub fn Combobox<T: Clone + PartialEq + 'static>(
    /// daisyUI's side axis for the popup.
    #[props(default)]
    side: ComboboxSide,
    /// daisyUI's align axis for the popup.
    #[props(default)]
    align: ComboboxAlign,
    /// An explicit Field binding for the selected value, which wins over Field
    /// Context.
    binding: Option<Binding<Option<T>>>,
    /// Explicit Field metadata, which wins over Field Context. Its attributes
    /// are rendered by [`ComboboxInput`].
    meta: Option<FieldMeta>,
    /// The controlled value of the combobox. `Some` makes it controlled, and
    /// the signal's own `None` means nothing is chosen.
    #[props(default)]
    value: Option<ReadSignal<Option<T>>>,
    /// The value the combobox starts on when it is not controlled.
    #[props(default)]
    default_value: Option<T>,
    /// Called with the chosen value after user selection.
    on_change: Option<EventHandler<Option<T>>>,
    /// Called after every selection ends its interaction unit.
    on_commit: Option<EventHandler<()>>,
    /// Called when focus leaves the input, popup listbox, and options as one
    /// logical focus scope.
    on_focus_exit: Option<EventHandler<()>>,
    /// The controlled open state of the popup.
    #[props(default)]
    open: ReadSignal<Option<bool>>,
    /// The state the popup starts in when it is not controlled.
    #[props(default)]
    default_open: bool,
    /// Called when the open state changes.
    #[props(default)]
    on_open_change: Callback<bool>,
    /// The controlled text the options are filtered by.
    #[props(default)]
    query: ReadSignal<Option<String>>,
    /// The text the options are filtered by when the query is not controlled.
    #[props(default)]
    default_query: ReadSignal<String>,
    /// Called when the typed text changes.
    #[props(default)]
    on_query_change: Callback<String>,
    /// Which options a query keeps. The default repeats the primitive's own,
    /// a case-insensitive substring match, since a prop declared here has to
    /// carry one.
    #[props(default = Callback::new(|(query, text): (String, String)| default_combobox_filter(&query, &text)))]
    filter: Callback<(String, String), bool>,
    /// Whether the combobox is required. Omission falls back to Field metadata.
    #[props(default)]
    required: Option<bool>,
    /// Whether the combobox is disabled, which leaves the field inert and the
    /// popup unopenable. Omission falls back to Field metadata.
    #[props(default)]
    disabled: Option<bool>,
    /// The control name used in forms. Omission falls back to Field metadata.
    #[props(default)]
    name: Option<String>,
    /// Whether arrow-key navigation wraps around at the ends of the list. The
    /// default repeats the primitive's own, since a prop declared here has to
    /// carry one.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    roving_loop: ReadSignal<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut uncontrolled_open = use_signal(|| default_open);
    let binding = use_binding(binding, default_value.clone());
    let meta = use_field_meta(meta);
    let resolved_disabled = disabled.unwrap_or_else(|| meta.disabled());
    let field_value = ComboboxField {
        meta,
        required,
        disabled,
        name,
    };
    let mut field = use_context_provider(|| Signal::new(field_value.clone()));
    let field_changed = {
        let current = field.peek();
        *current != field_value
    };
    if field_changed {
        field.set(field_value);
    }

    // Read here rather than inside the markup, and eagerly rather than only
    // when the other is absent, so that this component subscribes to whichever
    // of them is driving and re-renders, which is what puts the modifier class
    // below on the element and takes it off again.
    let is_open = open().unwrap_or(uncontrolled_open());

    // The value goes on to the options through a context rather than through a
    // class on this element, because the class it decides (`menu-active`) is
    // on the option itself.
    let binding_value = binding.read;
    let selected = use_memo(move || match value {
        Some(value) => value(),
        None => binding_value(),
    });
    use_context_provider(|| Selection { value: selected });
    let controlled: ReadSignal<Option<T>> = selected.into();

    let side = side.class();
    let align = align.class();
    // Tier 2, and half of why the state is lifted: daisyUI's own modifier
    // class, emitted from Rust as a complete literal so that Tailwind's scanner
    // sees it too.
    let state = if is_open { "dropdown-open" } else { "" };

    let base = attributes!(div {
        class: "dropdown {side} {align} {state}",
    });
    let mut merged = merge_attributes(vec![base, attributes]);
    let caller_focus_in = take_event_listener(&mut merged, "onfocusin");
    let caller_focus_out = take_event_listener(&mut merged, "onfocusout");
    let focus_generation = use_hook(|| Rc::new(Cell::new(0_u64)));
    let focus_in_generation = Rc::clone(&focus_generation);
    let focus_out_generation = focus_generation;
    let focus_exit_binding = binding.clone();
    let focus_scope = attributes!(div {
        onfocusin: move |event: FocusEvent| {
            if let Some(listener) = &caller_focus_in {
                listener.call(event.into_any());
            }
            focus_in_generation.set(focus_in_generation.get().wrapping_add(1));
        },
        onfocusout: move |event: FocusEvent| {
            if let Some(listener) = &caller_focus_out {
                listener.call(event.into_any());
            }
            let generation = focus_out_generation.get().wrapping_add(1);
            focus_out_generation.set(generation);
            let focus_out_generation = Rc::clone(&focus_out_generation);
            let binding = focus_exit_binding.clone();
            spawn(async move {
                let mut deferred = document::eval("setTimeout(() => dioxus.send(true), 0);");
                let _: Result<bool, _> = deferred.recv().await;
                if focus_out_generation.get() == generation {
                    binding.focus_exit();
                    if let Some(handler) = &on_focus_exit {
                        handler.call(());
                    }
                }
            });
        },
    });
    merged = merge_attributes(vec![merged, focus_scope]);
    let change_binding = binding.clone();
    let commit_binding = binding;

    rsx! {
        combobox::Combobox::<T> {
            value: Some(controlled),
            on_value_change: move |value: Option<T>| {
                change_binding.write(value.clone(), ChangeOrigin::User);
                if let Some(handler) = &on_change {
                    handler.call(value);
                }
                commit_binding.commit();
                if let Some(handler) = &on_commit {
                    handler.call(());
                }
            },
            open: Some(is_open),
            on_open_change: move |open| {
                uncontrolled_open.set(open);
                on_open_change.call(open);
            },
            query,
            default_query,
            on_query_change,
            filter,
            disabled: resolved_disabled,
            roving_loop,
            attributes: merged,
            {children}
        }
    }
}

fn take_event_listener(attributes: &mut Vec<Attribute>, name: &str) -> Option<ListenerCallback> {
    let index = attributes.iter().position(|attribute| {
        attribute.name == name && matches!(attribute.value, AttributeValue::Listener(_))
    })?;
    match attributes.remove(index).value {
        AttributeValue::Listener(listener) => Some(listener),
        _ => unreachable!(),
    }
}

/// The field a combobox is typed into, carrying daisyUI's `input` class.
///
/// This is the one part the select cannot lend: a select's field is a `button`
/// wearing a field's class, and this one is the `input` the class was written
/// for, so `.input`'s own rules for what is inside it, its placeholder and its
/// disabled state all apply as daisyUI wrote them.
///
/// What the field shows is the primitive's: the query while the popup is open,
/// and the chosen option's text once it is closed.
///
/// Classes passed by the caller concatenate with the field's own; every other
/// attribute the caller passes overrides them.
#[component]
pub fn ComboboxInput(
    /// An explicit colour, or `None` to derive error colour from Field metadata.
    #[props(default)]
    color: Option<ComboboxColor>,
    /// daisyUI's size axis.
    #[props(default)]
    size: ComboboxSize,
    /// What the field shows while nothing has been typed and nothing is chosen.
    #[props(default)]
    placeholder: ReadSignal<String>,
    /// The id of this element. Declared rather than left to the attribute list,
    /// because the primitive generates one and the list points its
    /// `aria-controls` back at it.
    #[props(default)]
    id: ReadSignal<Option<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let field = use_context::<Signal<ComboboxField>>();
    let field = field.read().clone();
    let meta = field.meta;
    let color = color.map_or_else(
        || {
            if meta.invalid() { "input-error" } else { "" }
        },
        ComboboxColor::class,
    );
    let size = size.class();
    let explicit_id = id();
    let mut control: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let focus_control = use_callback(move |()| {
        if let Some(control) = control() {
            spawn(async move {
                let _ = control.set_focus(true).await;
            });
        }
    });
    use_focus_registration(focus_control);

    let base = attributes!(input {
        class: "input {color} {size}",
    });
    let meta_attributes = meta.attributes_for(
        &FieldControlOptions::new()
            .disabled(field.disabled)
            .required(field.required)
            .id(explicit_id.map(Rc::from))
            .name(field.name.map(Rc::from))
            .surface(FieldSurface::NATIVE),
    );
    let mounted = attributes!(input {
        onmounted: move |event: MountedEvent| control.set(Some(event.data())),
    });
    let merged = merge_attributes(vec![meta_attributes, base, attributes, mounted]);

    rsx! {
        combobox::ComboboxInput { placeholder, id, attributes: merged }
    }
}

/// The box a combobox's options sit in, carrying daisyUI's `dropdown-content`
/// class and holding the `menu` list.
///
/// This is the dropdown's split, borrowed whole (ADR-0005) for the fourth time.
/// daisyUI puts both classes on one `ul`, which is impossible here for the same
/// three reasons: the primitive's list element is a hardcoded `div` with no `as`
/// prop, its context is private, and every visual rule `.menu` has for a row is
/// written against a literal `li`. So the positioning and the box go on the
/// primitive's element, and `menu` goes on a list rendered inside it.
///
/// The list is marked presentational, as [`ComboboxOption`]'s own wrapper is:
/// the primitive gives this element the `listbox` role and the options the
/// `option` role, and a plain list between them would break the ownership a
/// screen reader announces option counts and positions from.
///
/// **The list is emitted whether the popup is open or not.** The primitive drops
/// its own element while the popup is closed and renders the children where they
/// stand, so that every option can register the text the field displays when it
/// is the chosen one. The list is hidden instead, by a utility that fires
/// exactly when it is not inside the box: `[:not(.dropdown-content)>&]:hidden`.
///
/// Classes passed by the caller concatenate with the box's own, which is the
/// element worth reaching: a width or a fill belongs on the box rather than on
/// the list inside it.
#[component]
pub fn ComboboxList(
    /// Whether to emit the utilities that draw the box.
    #[props(default)]
    appearance: ComboboxListAppearance,
    /// daisyUI's size axis for the menu, which sizes the options.
    #[props(default)]
    size: ComboboxListSize,
    /// The id of this element. Declared rather than left to the attribute list,
    /// because the primitive generates one and then points the field's
    /// `aria-controls` at it; an id that arrived as an attribute would be
    /// written over the one the field names.
    #[props(default)]
    id: ReadSignal<Option<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let appearance = appearance.class();
    let size = size.class();

    let base = attributes!(div {
        class: "dropdown-content {appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        combobox::ComboboxList { id, attributes: merged,
            // The list is stretched to the box rather than left to size itself,
            // for the reason the select's is: `.menu` is `width: fit-content`,
            // which on daisyUI's single element is the box's width as well.
            ul {
                role: "none",
                class: "menu w-full {size} [:not(.dropdown-content)>&]:hidden",
                {children}
            }
        }
    }
}

/// One option of a combobox, wrapped in the list item daisyUI's `menu` styles it
/// through.
///
/// The wrapper is marked presentational for the reason [`ComboboxList`]'s list
/// is, and costs nothing behaviourally: options register with the primitive's
/// focus collection by the `index` they are given rather than by where they sit
/// in the DOM.
///
/// **The wrapper hides itself when it is empty**, which is what a filtered-out
/// option leaves behind: the primitive renders nothing for an option the query
/// does not keep, and daisyUI draws an empty list item as a divider rule. The
/// utility is Defeatable like every other one this registry emits, but switching
/// it off means drawing a rule per filtered-out option.
///
/// Everything the caller passes travels to the option rather than to the
/// wrapper, which is the element daisyUI styles and the primitive gives the
/// `option` role to.
#[component]
pub fn ComboboxOption<T: Clone + PartialEq + 'static>(
    /// Whether to emit the class that paints the option the keyboard is on.
    #[props(default)]
    appearance: ComboboxOptionAppearance,
    /// What this option is worth, which is what the combobox's value becomes
    /// when it is chosen.
    value: ReadSignal<T>,
    /// Where this option falls in the keyboard navigation order.
    index: ReadSignal<usize>,
    /// What the field shows and the filter matches on when this option is the
    /// chosen one. The primitive falls back to the option's own value, which
    /// only works where that value is a string.
    #[props(default)]
    text_value: ReadSignal<Option<String>>,
    /// Whether this option is disabled.
    #[props(default)]
    disabled: ReadSignal<bool>,
    /// The id of this element, declared for the reason [`ComboboxList`]'s is:
    /// the primitive generates one and reports it as the focused option, which
    /// is what the field's `aria-activedescendant` names.
    #[props(default)]
    id: ReadSignal<Option<String>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let selection = use_context::<Selection<T>>();

    // Tier 2 on the chosen state, on the option rather than on its wrapper:
    // daisyUI's active row is `li > .menu-active`, and `.menu` matches no ARIA
    // attribute at all, so `aria-selected`, which the primitive does set, is
    // invisible to it.
    let selected = if *selection.value.read() == Some(value()) {
        "menu-active"
    } else {
        ""
    };

    // The highlight, which is neither tier: a Bridged utility over the
    // attribute the primitive reports it as (ADR-0021).
    let appearance = appearance.class();

    // Tier 2 on the disabled state, and this one on the wrapper: daisyUI mutes
    // a disabled row through `.menu-disabled` on the list item, and the
    // primitive reports the state as `aria-disabled` and `data-disabled`, which
    // daisyUI matches nowhere.
    let state = if disabled() { "menu-disabled" } else { "" };

    let base = attributes!(div {
        class: "{selected} {appearance}",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        li { role: "none", class: "empty:hidden {state}",
            combobox::ComboboxOption::<T> {
                value,
                index,
                text_value,
                disabled,
                id,
                attributes: merged,
                {children}
            }
        }
    }
}

/// What the popup shows when the query matches no option, wrapped in the list
/// item daisyUI's `menu-title` styles.
///
/// The title class rather than a row's: `.menu`'s row rules skip a
/// `.menu-title`, which is what keeps this line from being padded like an
/// option, highlighted on hover and given a pointer cursor, none of which
/// belongs on a line nobody can choose.
///
/// The wrapper hides itself when it is empty for the reason
/// [`ComboboxOption`]'s does: the primitive renders nothing here while any
/// option is still visible.
#[component]
pub fn ComboboxEmpty(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        li { role: "none", class: "empty:hidden menu-title",
            combobox::ComboboxEmpty { attributes, {children} }
        }
    }
}
