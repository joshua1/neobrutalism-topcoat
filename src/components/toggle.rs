// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, Class, PromotedStr, StaticClass, View, class, component, view},
};

/// How a [`toggle`] relates to other toggles.
///
/// [`Default`] is `ToggleKind::Independent`, used when no kind is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ToggleKind {
    /// Presses and unpresses on its own, backed by a checkbox.
    #[default]
    Independent,
    /// One of a set sharing a `name`, of which only one can be pressed, backed
    /// by a radio button.
    Exclusive,
}

impl ToggleKind {
    /// The native input type that gives this kind its behavior.
    #[must_use]
    pub fn input_type(self) -> PromotedStr {
        match self {
            Self::Independent => PromotedStr(&"checkbox"),
            Self::Exclusive => PromotedStr(&"radio"),
        }
    }
}

/// The visual style of a [`toggle`].
///
/// [`Default`] is `ToggleVariant::Default`, used when no variant is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ToggleVariant {
    /// A bordered toggle on the secondary background.
    #[default]
    Default,
    /// Kept for parity with Octane; it currently looks the same as `Default`.
    Outline,
}

impl ToggleVariant {
    /// Classes for the toggle's unpressed fill.
    fn classes(self) -> StaticClass {
        match self {
            Self::Default => class!("bg-secondary-background text-foreground"),
            Self::Outline => class!("bg-secondary-background text-foreground"),
        }
    }
}

/// The size of a [`toggle`].
///
/// [`Default`] is `ToggleSize::Default`, used when no size is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ToggleSize {
    /// The standard toggle size.
    #[default]
    Default,
    /// A compact toggle with smaller text and icons.
    Sm,
    /// A prominent toggle.
    Lg,
}

impl ToggleSize {
    /// Classes for the toggle dimensions.
    fn classes(self) -> StaticClass {
        match self {
            Self::Default => class!(
                "h-10 min-w-10 px-3 has-data-[icon=inline-end]:pr-2 \
                 has-data-[icon=inline-start]:pl-2",
            ),
            Self::Sm => class!(
                "h-9 min-w-9 px-2.5 text-[0.8rem] has-data-[icon=inline-end]:pr-1.5 \
                 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3.5",
            ),
            Self::Lg => class!(
                "h-11 min-w-11 px-4 has-data-[icon=inline-end]:pr-2.5 \
                 has-data-[icon=inline-start]:pl-2.5",
            ),
        }
    }
}

/// Classes shared by every variant and size. The toggle is a `<label>`, so the
/// pressed, focused and disabled states come from the hidden input inside it.
const BASE: StaticClass = class!(
    "group/toggle inline-flex items-center justify-center gap-1 rounded-base border-2 \
     border-border text-sm font-heading whitespace-nowrap transition-all outline-none \
     ring-offset-white cursor-pointer select-none \
     has-focus-visible:outline-hidden has-focus-visible:ring-2 has-focus-visible:ring-black \
     has-focus-visible:ring-offset-2 has-disabled:pointer-events-none has-disabled:opacity-50 \
     has-checked:bg-main has-checked:text-main-foreground \
     [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
);

/// Builds the full class list for a toggle of the given `variant` and `size`.
///
/// The pressed styles key off a checked input inside the element, so use it on
/// a `<label>` wrapping a checkbox or radio button.
#[must_use]
#[allow(dead_code)]
pub fn toggle_variants(
    variant: ToggleVariant,
    size: ToggleSize,
) -> Class<(StaticClass, StaticClass, StaticClass)> {
    class!(BASE, variant.classes(), size.classes())
}

/// A two-state button that stays pressed when selected.
///
/// It is a `<label>` around a visually hidden checkbox (or radio button, for
/// `kind: ToggleKind::Exclusive`), so it keeps its state without JavaScript,
/// can be focused and toggled with the keyboard, and submits with a form.
/// Classes in `attrs` apply to the `<label>`; every other attribute
/// (`checked=""` for pressed, `disabled=""`, `name`, `value`, `aria-label`,
/// `@change`) goes on the `<input>`.
///
/// ```ignore
/// view! {
///     toggle(
///         attrs: attributes! { aria-label="Toggle bold" checked="" },
///         icon(data: iconify_icon!("lucide:bold"))
///     )
/// }
/// ```
#[component]
pub async fn toggle(
    /// Whether the toggle presses on its own or as one of a set.
    #[default]
    kind: ToggleKind,
    /// The visual style.
    #[default]
    variant: ToggleVariant,
    /// The dimensions.
    #[default]
    size: ToggleSize,
    /// Extra attributes for the `<input>`; classes go on the `<label>`.
    #[default]
    mut attrs: Attributes,
    /// The toggle's content, usually an icon and an optional label.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    // `sr-only` keeps the input focusable and announced as a checkbox, named
    // by the label around it; `display: none` would remove it from both.
    Ok(view! {
        <label
            data-slot="toggle"
            class=(class!(BASE, variant.classes(), size.classes(), attrs.remove("class")))
        >
            <input type=(kind.input_type()) class="sr-only" (attrs)>
            (child)
        </label>
    })
}
