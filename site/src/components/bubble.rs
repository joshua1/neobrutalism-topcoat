// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

/// The fill of a [`bubble`]'s content.
///
/// [`Default`] is `BubbleVariant::Default`, used when no variant is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum BubbleVariant {
    /// The main accent color, for your own messages.
    #[default]
    Default,
    /// The secondary (white) surface.
    Secondary,
    /// The page background, for other people's messages.
    Muted,
    /// Black with white text, for failed actions.
    Destructive,
}

impl BubbleVariant {
    /// Classes that color the bubble's content and its hover state.
    fn classes(self) -> StaticClass {
        match self {
            Self::Default => class!(
                "*:data-[slot=bubble-content]:bg-main *:data-[slot=bubble-content]:text-main-foreground \
                 [&>[data-slot=bubble-content]:is(button,a):hover]:bg-main/80",
            ),
            Self::Secondary => class!(
                "*:data-[slot=bubble-content]:bg-secondary-background \
                 *:data-[slot=bubble-content]:text-foreground \
                 [&>[data-slot=bubble-content]:is(button,a):hover]:bg-background",
            ),
            Self::Muted => class!(
                "*:data-[slot=bubble-content]:bg-background *:data-[slot=bubble-content]:text-foreground \
                 [&>[data-slot=bubble-content]:is(button,a):hover]:bg-secondary-background",
            ),
            Self::Destructive => class!(
                "*:data-[slot=bubble-content]:bg-black *:data-[slot=bubble-content]:text-white \
                 [&>[data-slot=bubble-content]:is(button,a):hover]:bg-black/80",
            ),
        }
    }

    /// The `data-variant` value.
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Secondary => "secondary",
            Self::Muted => "muted",
            Self::Destructive => "destructive",
        }
    }
}

/// Which side a [`bubble`] or its [`bubble_reactions`] line up with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum BubbleAlign {
    /// The start (left in left-to-right text).
    Start,
    /// The end (right in left-to-right text).
    End,
}

impl BubbleAlign {
    /// The `data-align` value.
    fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }

    /// Classes that pin [`bubble_reactions`] to a corner.
    fn reaction_classes(self) -> StaticClass {
        match self {
            Self::Start => class!("left-3"),
            Self::End => class!("right-3"),
        }
    }
}

/// Which edge of a [`bubble`] its [`bubble_reactions`] overlap.
///
/// [`Default`] is `BubbleSide::Bottom`, used when no side is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum BubbleSide {
    /// The top edge.
    Top,
    /// The bottom edge.
    #[default]
    Bottom,
}

impl BubbleSide {
    /// The `data-side` value.
    fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }

    /// Classes that move [`bubble_reactions`] over the edge.
    fn classes(self) -> StaticClass {
        match self {
            Self::Top => class!("top-0 -translate-y-3/4"),
            Self::Bottom => class!("bottom-0 translate-y-3/4"),
        }
    }
}

/// A stack of consecutive [`bubble`]s.
#[component]
#[allow(dead_code)]
pub async fn bubble_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="bubble-group"
            class=(class!("flex min-w-0 flex-col gap-2", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A chat bubble. Wraps a [`bubble_content`] and optional [`bubble_reactions`].
///
/// `variant` defaults to `Default` and `align` to `BubbleAlign::Start`. An
/// end-aligned bubble, or any bubble inside an end-aligned `message`, moves to
/// the end of the row.
///
/// ```ignore
/// view! {
///     bubble(variant: BubbleVariant::Muted, bubble_content("Hey there!"))
///     bubble(align: BubbleAlign::End, bubble_content("Hi!"))
/// }
/// ```
#[component]
pub async fn bubble(
    #[default] variant: BubbleVariant,
    #[default(BubbleAlign::Start)] align: BubbleAlign,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="bubble"
            data-variant=(variant.as_str())
            data-align=(align.as_str())
            class=(class!(
                "group/bubble relative flex w-fit max-w-[80%] min-w-0 flex-col gap-1 \
                 group-data-[align=end]/message:self-end data-[align=end]:self-end",
                variant.classes(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Classes for a [`bubble_content`].
///
/// Use them to render the content as a button or link, keeping
/// `data-slot="bubble-content"` so the bubble colors it:
///
/// ```ignore
/// view! {
///     bubble(
///         <button type="button" data-slot="bubble-content" class=(BUBBLE_CONTENT)>
///             "Retry"
///         </button>
///     )
/// }
/// ```
#[allow(dead_code)]
pub const BUBBLE_CONTENT: StaticClass = class!(
    "w-fit max-w-full min-w-0 overflow-hidden rounded-base border-2 border-border px-3 py-2.5 \
     text-sm font-base leading-5 wrap-break-word group-data-[align=end]/bubble:self-end \
     [button]:text-left [button,a]:transition-colors [button,a]:outline-none \
     [button,a]:focus-visible:ring-2 [button,a]:focus-visible:ring-black \
     [button,a]:focus-visible:ring-offset-2",
);

/// The bordered text box of a [`bubble`], colored by the bubble's variant.
///
/// To render it as a button or link, use [`BUBBLE_CONTENT`] on that element.
#[component]
#[allow(dead_code)]
pub async fn bubble_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="bubble-content"
            class=(class!(BUBBLE_CONTENT, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A pill of reactions overlapping an edge of a [`bubble`].
///
/// `side` defaults to `Bottom` and `align` to `BubbleAlign::End`.
///
/// ```ignore
/// view! {
///     bubble_reactions(side: BubbleSide::Top, align: BubbleAlign::Start, <span>"3"</span>)
/// }
/// ```
#[component]
#[allow(dead_code)]
pub async fn bubble_reactions(
    #[default] side: BubbleSide,
    #[default(BubbleAlign::End)] align: BubbleAlign,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="bubble-reactions"
            data-align=(align.as_str())
            data-side=(side.as_str())
            class=(class!(
                "absolute z-10 flex w-fit shrink-0 items-center justify-center gap-1 rounded-full \
                 border-2 border-border bg-secondary-background px-1.5 py-0.5 text-sm font-base \
                 text-foreground has-[button]:p-0",
                side.classes(),
                align.reaction_classes(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}
