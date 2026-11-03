// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, Class, StaticClass, View, class, component, view},
};

/// The visual style of a [`button`].
///
/// [`Default`] is `ButtonVariant::Default`, used when no variant is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ButtonVariant {
    /// A main-filled button with a hard shadow that presses in on hover.
    #[default]
    Default,
    /// A main-filled button without the shadow.
    NoShadow,
    /// A neutral (secondary background) button with a hard shadow.
    Neutral,
    /// A main-filled button that gains its shadow on hover, lifting out.
    Reverse,
}

impl ButtonVariant {
    /// Classes for the button variant and its hover state.
    fn classes(self) -> StaticClass {
        match self {
            Self::Default => class!(
                "text-main-foreground bg-main border-2 border-border shadow-shadow \
                 hover:translate-x-boxShadowX hover:translate-y-boxShadowY hover:shadow-none",
            ),
            Self::NoShadow => class!("text-main-foreground bg-main border-2 border-border"),
            Self::Neutral => class!(
                "bg-secondary-background text-foreground border-2 border-border shadow-shadow \
                 hover:translate-x-boxShadowX hover:translate-y-boxShadowY hover:shadow-none",
            ),
            Self::Reverse => class!(
                "text-main-foreground bg-main border-2 border-border \
                 hover:translate-x-reverseBoxShadowX hover:translate-y-reverseBoxShadowY \
                 hover:shadow-shadow",
            ),
        }
    }
}

/// The size of a [`button`].
///
/// [`Default`] is `ButtonSize::Default`, used when no size is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ButtonSize {
    /// The standard button size.
    #[default]
    Default,
    /// An extra-small button with smaller icons.
    Xs,
    /// A compact button.
    Sm,
    /// A prominent button.
    Lg,
    /// A square button sized for a single icon.
    Icon,
    /// An extra-small square icon button.
    IconXs,
    /// A small square icon button.
    IconSm,
    /// A large square icon button.
    IconLg,
}

impl ButtonSize {
    /// Classes for the button dimensions.
    ///
    /// The extra-small sizes override the base `gap-2` and `[&_svg]:size-4`.
    /// `class!` keeps both, so their values carry the `!` suffix to win.
    fn classes(self) -> StaticClass {
        match self {
            Self::Default => class!("h-10 px-4 py-2"),
            Self::Xs => class!("h-8 gap-1.5! px-2.5 text-xs [&_svg]:size-3.5!"),
            Self::Sm => class!("h-9 px-3"),
            Self::Lg => class!("h-11 px-8"),
            Self::Icon => class!("size-10"),
            Self::IconXs => class!("size-8 [&_svg]:size-3.5!"),
            Self::IconSm => class!("size-9"),
            Self::IconLg => class!("size-11"),
        }
    }
}

/// Classes shared by every variant and size.
const BASE: StaticClass = class!(
    "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-base \
     text-sm font-base ring-offset-white transition-all \
     [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 \
     focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-black \
     focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
);

/// Builds the full class list for a button of the given `variant` and `size`.
///
/// Use it to give button styling to an element that is not a `<button>`, such
/// as a link styled as a button:
///
/// ```ignore
/// view! {
///     <a href="/login" class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))>
///         "Sign in"
///     </a>
/// }
/// ```
#[must_use]
#[allow(dead_code)]
pub fn button_variants(
    variant: ButtonVariant,
    size: ButtonSize,
) -> Class<(StaticClass, StaticClass, StaticClass)> {
    class!(BASE, variant.classes(), size.classes())
}

/// A neobrutalism button.
///
/// `variant` and `size` default to `Default`. Pass the content as children.
/// `attrs` are forwarded to the `<button>`, with extra classes appended to its
/// classes. Use [`button_variants`] to apply the same styling to another
/// element.
///
/// ```ignore
/// view! {
///     button(
///         variant: ButtonVariant::Reverse,
///         attrs: attributes! { type="submit" },
///         "Save"
///     )
/// }
/// ```
#[component]
pub async fn button(
    #[default] variant: ButtonVariant,
    #[default] size: ButtonSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    // A wrapper (e.g. a toast action) may pass its own `data-slot`; render
    // ours only when it didn't, so the attribute never appears twice.
    let slot = (!attrs.contains_key("data-slot")).then_some("button");

    Ok(view! {
        <button
            data-slot=(slot)
            class=(class!(
                BASE,
                variant.classes(),
                size.classes(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </button>
    })
}
