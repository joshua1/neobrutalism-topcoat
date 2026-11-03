// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

use super::{
    button::{ButtonVariant, button},
    input::input,
    textarea::textarea,
};

/// Classes for the group's shared border, its focus ring (drawn while the
/// control inside has focus) and the layouts each addon position needs.
const GROUP: StaticClass = class!(
    "group/input-group relative flex h-10 w-full min-w-0 items-center rounded-base border-2 \
     border-border bg-secondary-background transition-colors outline-none \
     in-data-[slot=combobox-content]:focus-within:border-inherit \
     in-data-[slot=combobox-content]:focus-within:ring-0 has-disabled:opacity-50 \
     has-[[data-slot=input-group-control]:focus-visible]:ring-2 \
     has-[[data-slot=input-group-control]:focus-visible]:ring-black \
     has-[[data-slot=input-group-control]:focus-visible]:ring-offset-2 \
     has-[[data-slot][aria-invalid=true]]:border-red-500 \
     has-[>[data-align=block-end]]:h-auto has-[>[data-align=block-end]]:flex-col \
     has-[>[data-align=block-start]]:h-auto has-[>[data-align=block-start]]:flex-col \
     has-[>textarea]:h-auto has-[>[data-align=block-end]]:[&>input]:pt-3 \
     has-[>[data-align=block-start]]:[&>input]:pb-3 \
     has-[>[data-align=inline-end]]:[&>input]:pr-1.5 \
     has-[>[data-align=inline-start]]:[&>input]:pl-1.5",
);

/// A neobrutalism input with addons (icons, text, buttons) inside one border.
///
/// Pass an [`input_group_input`] or [`input_group_textarea`] and any number
/// of [`input_group_addon`]s as children, in any order: addons place
/// themselves by their `align`. `attrs` are forwarded to the
/// `<div role="group">`, with extra classes appended to its classes.
///
/// ```ignore
/// view! {
///     input_group(attrs: attributes! { class="max-w-sm" },
///         input_group_input(attrs: attributes! { placeholder="Search..." })
///         input_group_addon(icon(data: iconify_icon!("lucide:search")))
///     )
/// }
/// ```
#[component]
pub async fn input_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="input-group"
            role="group"
            class=(class!(GROUP, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Where an [`input_group_addon`] sits in its group.
///
/// [`Default`] is `InputGroupAddonAlign::InlineStart`, used when no align is
/// given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum InputGroupAddonAlign {
    /// Before the control, on the same line.
    #[default]
    InlineStart,
    /// After the control, on the same line.
    InlineEnd,
    /// Above the control, full width.
    BlockStart,
    /// Below the control, full width.
    BlockEnd,
}

impl InputGroupAddonAlign {
    /// The `data-align` value, which the group's classes read.
    fn as_str(self) -> &'static str {
        match self {
            Self::InlineStart => "inline-start",
            Self::InlineEnd => "inline-end",
            Self::BlockStart => "block-start",
            Self::BlockEnd => "block-end",
        }
    }

    /// Classes for the addon's position and padding.
    fn classes(self) -> StaticClass {
        match self {
            Self::InlineStart => class!(
                "order-first pl-2 has-[>button]:ml-[-0.3rem] has-[>kbd]:ml-[-0.15rem] \
                 has-[>button[data-solid]]:m-0 has-[>button[data-solid]]:self-stretch \
                 has-[>button[data-solid]]:p-0",
            ),
            Self::InlineEnd => class!(
                "order-last pr-2 has-[>button]:mr-[-0.3rem] has-[>kbd]:mr-[-0.15rem] \
                 has-[>button[data-solid]]:m-0 has-[>button[data-solid]]:self-stretch \
                 has-[>button[data-solid]]:p-0",
            ),
            Self::BlockStart => class!(
                "order-first w-full justify-start px-2.5 pt-2 \
                 group-has-[>input]/input-group:pt-2 [.border-b-2]:pb-2",
            ),
            Self::BlockEnd => class!(
                "order-last w-full justify-start px-2.5 pb-2 \
                 group-has-[>input]/input-group:pb-2 [.border-t-2]:pt-2",
            ),
        }
    }
}

/// Classes shared by every addon position.
const ADDON: StaticClass = class!(
    "flex h-auto cursor-text items-center justify-center gap-2 py-1.5 text-sm font-base \
     text-foreground select-none group-data-[disabled=true]/input-group:opacity-50 \
     [&>svg:not([class*='size-'])]:size-4",
);

/// Focuses the group's input when the addon is clicked outside a button,
/// the way clicking a label would.
const FOCUS_INPUT: &str = "if (!event.target.closest('button')) \
     this.parentElement.querySelector('input')?.focus()";

/// An icon, text or button area inside an [`input_group`].
///
/// `align` defaults to `InlineStart`. Clicking the addon (but not a button
/// in it) focuses the group's `<input>`. `attrs` are forwarded to the
/// `<div>`, with extra classes appended to its classes.
#[component]
pub async fn input_group_addon(
    #[default] align: InputGroupAddonAlign,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="group"
            data-slot="input-group-addon"
            data-align=(align.as_str())
            class=(class!(ADDON, align.classes(), attrs.remove("class")))
            onclick=(FOCUS_INPUT)
            (attrs)
        >
            (child)
        </div>
    })
}

/// The size of an [`input_group_button`].
///
/// [`Default`] is `InputGroupButtonSize::Xs`, used when no size is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum InputGroupButtonSize {
    /// A short button that fits inside the group's height.
    #[default]
    Xs,
    /// A taller button.
    Sm,
    /// An extra-small square icon button.
    IconXs,
    /// A small square icon button.
    IconSm,
}

impl InputGroupButtonSize {
    /// The `data-size` value.
    fn as_str(self) -> &'static str {
        match self {
            Self::Xs => "xs",
            Self::Sm => "sm",
            Self::IconXs => "icon-xs",
            Self::IconSm => "icon-sm",
        }
    }

    /// Classes for the button's dimensions.
    ///
    /// Octane's `cn` (tailwind-merge) drops the button's own `h-10 px-4 py-2`
    /// when these are added. `class!` only joins, so the classes that must
    /// win carry Tailwind's `!` (important) modifier instead.
    fn classes(self) -> StaticClass {
        match self {
            Self::Xs => class!(
                "h-6! gap-1! rounded-base px-1.5! [&>svg:not([class*='size-'])]:size-3.5",
            ),
            Self::Sm => class!("h-8! px-2.5!"),
            Self::IconXs => class!("size-6! p-0! has-[>svg]:p-0"),
            Self::IconSm => class!("size-8! p-0! has-[>svg]:p-0"),
        }
    }
}

/// Classes for every input-group button: flat, without the press-in shadow.
/// `flex!` replaces the button's own `inline-flex`.
const GROUP_BUTTON: StaticClass = class!(
    "flex! items-center gap-2 text-sm shadow-none! hover:translate-x-0! hover:translate-y-0!",
);

/// Classes for a ghost button: no border, no fill. Marked important to
/// override the neutral variant's border and fill, as `cn` would.
const GHOST: StaticClass = class!("border-0! bg-transparent!");

/// Classes for a solid button, which attaches to the group's edge. Its
/// `px-3!` must beat the important size padding, as it does in Octane.
const SOLID: StaticClass = class!(
    "in-data-[align=inline-end]:h-full! in-data-[align=inline-end]:rounded-none \
     in-data-[align=inline-end]:rounded-r-base in-data-[align=inline-end]:border-0 \
     in-data-[align=inline-end]:border-l-2 in-data-[align=inline-end]:border-border \
     in-data-[align=inline-end]:px-3! in-data-[align=inline-start]:h-full! \
     in-data-[align=inline-start]:rounded-none in-data-[align=inline-start]:rounded-l-base \
     in-data-[align=inline-start]:border-0 in-data-[align=inline-start]:border-r-2 \
     in-data-[align=inline-start]:border-border in-data-[align=inline-start]:px-3!",
);

/// A [`button`] sized to sit inside an [`input_group_addon`].
///
/// Without a `variant` the button is a ghost: no border or fill, flat in the
/// group. With one it is solid and attaches to the group's edge.
/// `size` defaults to `Xs`. It is `type="button"` unless `attrs` gives a
/// `type`. `attrs` are forwarded to the `<button>`, with extra classes
/// appended; name icon-only buttons with `aria-label`.
///
/// ```ignore
/// view! {
///     input_group_addon(align: InputGroupAddonAlign::InlineEnd,
///         input_group_button(variant: Some(ButtonVariant::Default), size: InputGroupButtonSize::Sm,
///             "Search"
///         )
///     )
/// }
/// ```
#[component]
pub async fn input_group_button(
    #[default] variant: Option<ButtonVariant>,
    #[default] size: InputGroupButtonSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let solid = variant.is_some();
    let default_type = !attrs.contains_key("type");
    Ok(view! {
        button(
            variant: variant.unwrap_or(ButtonVariant::Neutral),
            attrs: attributes! {
                if default_type {
                    type="button"
                }
                data-size=(size.as_str())
                data-solid=(solid)
                class=(class!(
                    GROUP_BUTTON,
                    size.classes(),
                    GHOST if !solid,
                    SOLID if solid,
                    attrs.remove("class"),
                ))
                (attrs)
            },
            (child)
        )
    })
}

/// Plain text inside an [`input_group_addon`], such as a unit or hint.
#[component]
pub async fn input_group_text(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            class=(class!(
                "flex items-center gap-2 text-sm font-base text-foreground \
                 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </span>
    })
}

/// Classes that strip an input's own border, fill and ring so the group
/// draws them. Marked important where they override the input's classes, as
/// `cn` would.
const CONTROL: StaticClass = class!(
    "flex-1 rounded-none! border-0! bg-transparent! shadow-none ring-0 \
     focus-visible:ring-0! focus-visible:ring-offset-0! disabled:bg-transparent \
     aria-invalid:ring-0",
);

/// An [`input`] that fills an [`input_group`].
///
/// `attrs` are forwarded to the `<input>`, with extra classes appended.
#[component]
pub async fn input_group_input(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        input(attrs: attributes! {
            data-slot="input-group-control"
            class=(class!(CONTROL, attrs.remove("class")))
            (attrs)
        })
    })
}

/// A [`textarea`] that fills an [`input_group`]. Use a `BlockStart` or
/// `BlockEnd` addon to put a toolbar above or below it.
///
/// Pass the initial text as children. `attrs` are forwarded to the
/// `<textarea>`, with extra classes appended.
#[component]
pub async fn input_group_textarea(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        textarea(
            attrs: attributes! {
                data-slot="input-group-control"
                class=(class!(CONTROL, "resize-none py-2", attrs.remove("class")))
                (attrs)
            },
            (child)
        )
    })
}
