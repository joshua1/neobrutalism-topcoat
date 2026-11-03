//! A short hint shown while its trigger is hovered or focused, done in CSS.
//!
//! [`tooltip`] is a `group/tooltip` wrapper; [`tooltip_content`] shows on
//! `group-hover` and `group-focus-within`, positioned absolutely against the
//! wrapper. No runtime is needed.
//!
//! Not matched from Base UI: the hint does not move to stay inside the
//! viewport or escape `overflow: hidden` ancestors, Escape does not hide it,
//! and there is no open delay (the Octane provider used none either).

// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

/// Renders only its children. Base UI's provider shares open delays between
/// tooltips; the CSS tooltips here open instantly, so there is nothing to
/// share. It exists so markup ported from Octane keeps its shape.
#[component]
pub async fn tooltip_provider(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! { (child) })
}

/// Wraps a trigger and its [`tooltip_content`]. Hovering or focusing anything
/// inside it shows the hint.
///
/// Give the trigger its own text or `aria-label`; the tooltip must not be the
/// only way to learn what it does. To link them for screen readers, give the
/// content an `id` and the trigger a matching `aria-describedby`.
///
/// ```ignore
/// view! {
///     tooltip(
///         tooltip_trigger(
///             attrs: attributes! {
///                 class=(button_variants(ButtonVariant::NoShadow, ButtonSize::Default))
///                 aria-describedby="library-hint"
///             },
///             "Hover"
///         )
///         tooltip_content(attrs: attributes! { id="library-hint" }, "Add to library")
///     )
/// }
/// ```
#[component]
pub async fn tooltip(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="tooltip"
            class=(class!("group/tooltip relative inline-flex w-fit", attrs.remove("class")))
            (attrs)
        >
            (child)
        </span>
    })
}

/// A button that shows the [`tooltip`] it is inside. It is unstyled; give it
/// button styling with `class=(button_variants(..))`. Any focusable element
/// inside the wrapper works as a trigger too.
#[component]
pub async fn tooltip_trigger(
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button type="button" data-slot="tooltip-trigger" (attrs)>(child)</button>
    })
}

/// The side of the trigger a [`tooltip_content`] shows on.
///
/// [`Default`] is `TooltipSide::Top`, used when no side is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TooltipSide {
    /// Above the trigger.
    #[default]
    Top,
    /// To the right of the trigger.
    Right,
    /// Below the trigger.
    Bottom,
    /// To the left of the trigger.
    Left,
}

impl TooltipSide {
    /// The value of the content's `data-side` attribute.
    fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }

    /// Classes that place the hint 4px off the side and slide it in toward it.
    fn classes(self) -> StaticClass {
        match self {
            Self::Top => class!(
                "bottom-full mb-1 origin-bottom translate-y-2 \
                 group-hover/tooltip:translate-y-0 group-focus-within/tooltip:translate-y-0",
            ),
            Self::Bottom => class!(
                "top-full mt-1 origin-top -translate-y-2 \
                 group-hover/tooltip:translate-y-0 group-focus-within/tooltip:translate-y-0",
            ),
            Self::Left => class!(
                "right-full mr-1 origin-right translate-x-2 \
                 group-hover/tooltip:translate-x-0 group-focus-within/tooltip:translate-x-0",
            ),
            Self::Right => class!(
                "left-full ml-1 origin-left -translate-x-2 \
                 group-hover/tooltip:translate-x-0 group-focus-within/tooltip:translate-x-0",
            ),
        }
    }
}

/// How a [`tooltip_content`] lines up with its trigger along its side.
///
/// [`Default`] is `TooltipAlign::Center`, used when no alignment is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TooltipAlign {
    /// Lines up the starting edges (left, or top on the left and right sides).
    Start,
    /// Centers the hint on the trigger.
    #[default]
    Center,
    /// Lines up the ending edges (right, or bottom on the left and right sides).
    End,
}

/// Classes that line the hint up with the trigger along `side`.
fn alignment(side: TooltipSide, align: TooltipAlign) -> StaticClass {
    use TooltipAlign::{Center, End, Start};
    use TooltipSide::{Bottom, Left, Right, Top};

    match (side, align) {
        (Top | Bottom, Start) => class!("left-0"),
        (Top | Bottom, Center) => class!("left-1/2 -translate-x-1/2"),
        (Top | Bottom, End) => class!("right-0"),
        (Left | Right, Start) => class!("top-0"),
        (Left | Right, Center) => class!("top-1/2 -translate-y-1/2"),
        (Left | Right, End) => class!("bottom-0"),
    }
}

/// Classes for the hint bubble and its hidden state. It ignores the pointer, so
/// it never covers what the user is reaching for.
const CONTENT: StaticClass = class!(
    "z-50 overflow-hidden rounded-base border-2 border-border bg-secondary-background px-3 \
     py-1.5 text-sm font-base text-foreground \
     pointer-events-none invisible absolute w-max opacity-0 scale-95 \
     transition-[opacity,scale,translate,visibility] duration-150 ease-out \
     group-hover/tooltip:visible group-hover/tooltip:opacity-100 group-hover/tooltip:scale-100 \
     group-focus-within/tooltip:visible group-focus-within/tooltip:opacity-100 \
     group-focus-within/tooltip:scale-100",
);

/// The hint a [`tooltip`] shows.
///
/// `side` defaults to `Top` and `align` to `Center`.
#[component]
pub async fn tooltip_content(
    /// The side of the trigger to show on.
    #[default]
    side: TooltipSide,
    /// How to line up with the trigger.
    #[default]
    align: TooltipAlign,
    /// Extra attributes for the bubble.
    #[default]
    mut attrs: Attributes,
    /// The hint.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="tooltip-content"
            data-side=(side.as_str())
            role="tooltip"
            class=(class!(CONTENT, side.classes(), alignment(side, align), attrs.remove("class")))
            (attrs)
        >
            (child)
        </span>
    })
}
