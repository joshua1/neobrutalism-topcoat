// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

/// The spacing of a [`card`].
///
/// [`Default`] is `CardSize::Default`, used when no size is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum CardSize {
    /// Roomy padding and gaps (`--spacing(6)`).
    #[default]
    Default,
    /// Tighter padding and gaps (`--spacing(4)`).
    Sm,
}

impl CardSize {
    /// The value of the card's `data-size` attribute, which its classes read.
    const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
        }
    }
}

/// Classes for the card panel. Every section pads itself with
/// `--card-spacing`, which `data-size="sm"` shrinks.
const CARD: StaticClass = class!(
    "group/card rounded-base flex flex-col shadow-shadow border-2 gap-(--card-spacing) \
     py-(--card-spacing) border-border bg-background text-foreground font-base \
     [--card-spacing:--spacing(6)] data-[size=sm]:[--card-spacing:--spacing(4)]",
);

/// A neobrutalism panel that groups related content.
///
/// Pass sections as children: a [`card_header`], a [`card_content`] and a
/// [`card_footer`], as needed. `size` defaults to `Default`. `attrs` are
/// forwarded to the `<div>`, with extra classes appended to its classes.
///
/// The sections are spaced with the `--card-spacing` custom property. To use
/// a different spacing, set it with an inline style, which wins over the
/// card's own classes:
/// `attrs: attributes! { style="--card-spacing: calc(var(--spacing) * 8)" }`.
///
/// ```ignore
/// view! {
///     card(
///         attrs: attributes! { class="w-full max-w-sm" },
///         card_header(
///             card_title("Login to your account")
///             card_description("Enter your email below to login to your account")
///             card_action(<a href="/signup">"Sign Up"</a>)
///         )
///         card_content(<p>"..."</p>)
///         card_footer(button(attrs: attributes! { class="w-full" }, "Login"))
///     )
/// }
/// ```
#[component]
pub async fn card(
    #[default] size: CardSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="card"
            data-size=(size.as_str())
            class=(class!(CARD, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The opening section of a [`card`], stacking a [`card_title`] and a
/// [`card_description`]. A [`card_action`] child moves into a second column.
#[component]
pub async fn card_header(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="card-header"
            class=(class!(
                "@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start \
                 gap-1.5 px-(--card-spacing) has-data-[slot=card-action]:grid-cols-[1fr_auto] \
                 [.border-b]:pb-(--card-spacing)",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The heading of a [`card`].
#[component]
pub async fn card_title(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="card-title"
            class=(class!("font-heading leading-none", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The supporting text under a [`card_title`].
#[component]
pub async fn card_description(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="card-description"
            class=(class!("text-sm font-base", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A control in the top-right corner of a [`card_header`], such as a link or
/// a badge. It spans the title and description rows.
#[component]
pub async fn card_action(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="card-action"
            class=(class!(
                "col-start-2 row-span-2 row-start-1 self-start justify-self-end",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The main body of a [`card`].
#[component]
pub async fn card_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="card-content"
            class=(class!("px-(--card-spacing)", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The closing section of a [`card`], a horizontal row for actions.
#[component]
pub async fn card_footer(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="card-footer"
            class=(class!(
                "flex items-center px-(--card-spacing) [.border-t]:pt-(--card-spacing)",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}
