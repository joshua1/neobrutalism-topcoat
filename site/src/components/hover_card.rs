//! A card previewing a link's destination, shown while the link is hovered or
//! focused, done in CSS.
//!
//! [`hover_card`] is a `group/hover-card` wrapper; [`hover_card_content`] shows
//! on `group-hover` and `group-focus-within` after a delay, positioned
//! absolutely against the wrapper. The close delay lets the pointer cross the
//! gap into the card. No runtime is needed.
//!
//! Not matched from Base UI: the card does not move to stay inside the
//! viewport or escape `overflow: hidden` ancestors, and Escape does not hide
//! it. Touch users may never see it, so keep its content available elsewhere.

// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

/// Wraps a [`hover_card_trigger`] and its [`hover_card_content`].
///
/// `delay` and `close_delay` are in milliseconds and default to 600 and 300,
/// like Base UI's `PreviewCard`. (Octane sets them on the trigger; here they go
/// on the wrapper, which sets the CSS variables the card's transition reads, so
/// don't pass a `style` in `attrs`.)
///
/// ```ignore
/// view! {
///     hover_card(
///         delay: 10,
///         close_delay: 100,
///         hover_card_trigger(attrs: attributes! { href="/people/ada" }, "@ada")
///         hover_card_content(
///             <div class="font-heading">"Ada Lovelace"</div>
///             <div>"Owner, joined in 2024."</div>
///         )
///     )
/// }
/// ```
#[component]
pub async fn hover_card(
    /// Milliseconds the pointer must rest on the trigger before the card opens.
    #[default(600)]
    delay: u32,
    /// Milliseconds the card stays open after the pointer leaves.
    #[default(300)]
    close_delay: u32,
    /// Extra attributes for the wrapper `<span>`.
    #[default]
    mut attrs: Attributes,
    /// The trigger and the card.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    let style = format!("--hover-card-delay: {delay}ms; --hover-card-close-delay: {close_delay}ms");

    Ok(view! {
        <span
            data-slot="hover-card"
            style=(style)
            class=(class!("group/hover-card relative inline-flex w-fit", attrs.remove("class")))
            (attrs)
        >
            (child)
        </span>
    })
}

/// The link that shows the [`hover_card`] it is inside, rendered as an `<a>`.
///
/// Pass its `href` in `attrs`; without one it cannot take keyboard focus. To
/// style it as a button, add `class=(button_variants(..))`.
#[component]
pub async fn hover_card_trigger(
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <a data-slot="hover-card-trigger" (attrs)>(child)</a>
    })
}

/// The side of the trigger a [`hover_card_content`] opens on.
///
/// [`Default`] is `HoverCardSide::Bottom`, used when no side is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum HoverCardSide {
    /// Above the trigger.
    Top,
    /// To the right of the trigger.
    Right,
    /// Below the trigger.
    #[default]
    Bottom,
    /// To the left of the trigger.
    Left,
}

impl HoverCardSide {
    /// The value of the content's `data-side` attribute.
    fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }

    /// Classes that place the card 4px off the side and slide it in toward it.
    fn classes(self) -> StaticClass {
        match self {
            Self::Top => class!(
                "bottom-full mb-1 origin-bottom translate-y-2 \
                 group-hover/hover-card:translate-y-0 group-focus-within/hover-card:translate-y-0",
            ),
            Self::Bottom => class!(
                "top-full mt-1 origin-top -translate-y-2 \
                 group-hover/hover-card:translate-y-0 group-focus-within/hover-card:translate-y-0",
            ),
            Self::Left => class!(
                "right-full mr-1 origin-right translate-x-2 \
                 group-hover/hover-card:translate-x-0 group-focus-within/hover-card:translate-x-0",
            ),
            Self::Right => class!(
                "left-full ml-1 origin-left -translate-x-2 \
                 group-hover/hover-card:translate-x-0 group-focus-within/hover-card:translate-x-0",
            ),
        }
    }
}

/// How a [`hover_card_content`] lines up with its trigger along its side.
///
/// [`Default`] is `HoverCardAlign::Center`, used when no alignment is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum HoverCardAlign {
    /// Lines up the starting edges (left, or top on the left and right sides).
    Start,
    /// Centers the card on the trigger.
    #[default]
    Center,
    /// Lines up the ending edges (right, or bottom on the left and right sides).
    End,
}

/// Classes that line the card up with the trigger along `side`.
fn alignment(side: HoverCardSide, align: HoverCardAlign) -> StaticClass {
    use HoverCardAlign::{Center, End, Start};
    use HoverCardSide::{Bottom, Left, Right, Top};

    match (side, align) {
        (Top | Bottom, Start) => class!("left-0"),
        (Top | Bottom, Center) => class!("left-1/2 -translate-x-1/2"),
        (Top | Bottom, End) => class!("right-0"),
        (Left | Right, Start) => class!("top-0"),
        (Left | Right, Center) => class!("top-1/2 -translate-y-1/2"),
        (Left | Right, End) => class!("bottom-0"),
    }
}

/// Classes for the card and its hidden state. The transition waits for the
/// close delay when hiding and for the open delay when showing.
///
/// Base UI portals the card to `<body>`; this one stays next to its trigger,
/// so `leading-normal` gives it the body's line height instead of the
/// surrounding text's.
const CONTENT: StaticClass = class!(
    "z-50 w-64 rounded-base border-2 border-border bg-background p-4 font-base text-foreground \
     leading-normal outline-none invisible absolute opacity-0 scale-95 \
     transition-[opacity,scale,translate,visibility] duration-150 ease-out \
     [transition-delay:var(--hover-card-close-delay)] \
     group-hover/hover-card:[transition-delay:var(--hover-card-delay)] \
     group-focus-within/hover-card:[transition-delay:var(--hover-card-delay)] \
     group-hover/hover-card:visible group-hover/hover-card:opacity-100 \
     group-hover/hover-card:scale-100 group-focus-within/hover-card:visible \
     group-focus-within/hover-card:opacity-100 group-focus-within/hover-card:scale-100",
);

/// The card a [`hover_card`] shows.
///
/// `side` defaults to `Bottom` and `align` to `Center`.
#[component]
pub async fn hover_card_content(
    /// The side of the trigger to open on.
    #[default]
    side: HoverCardSide,
    /// How to line up with the trigger.
    #[default]
    align: HoverCardAlign,
    /// Extra attributes for the card.
    #[default]
    mut attrs: Attributes,
    /// The card's content.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="hover-card-content"
            data-side=(side.as_str())
            class=(class!(CONTENT, side.classes(), alignment(side, align), attrs.remove("class")))
            (attrs)
        >
            (child)
        </span>
    })
}
