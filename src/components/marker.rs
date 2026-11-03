// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, Class, StaticClass, View, class, component, view},
};

/// The visual style of a [`marker`].
///
/// [`Default`] is `MarkerVariant::Default`, used when no variant is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum MarkerVariant {
    /// Icon and text on one line.
    #[default]
    Default,
    /// Centered text between two horizontal rules, like a date divider.
    Separator,
    /// A line with a bottom border.
    Border,
}

impl MarkerVariant {
    /// Classes for the variant, or `None` for the plain default.
    fn classes(self) -> Option<StaticClass> {
        match self {
            Self::Default => None,
            Self::Separator => Some(class!(
                "before:mr-0 before:h-0.5 before:min-w-0 before:flex-1 before:bg-border \
                 after:ml-0 after:h-0.5 after:min-w-0 after:flex-1 after:bg-border",
            )),
            Self::Border => Some(class!("border-b-2 border-border pb-2")),
        }
    }

    /// The `data-variant` value [`marker_content`] styles itself by.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Separator => "separator",
            Self::Border => "border",
        }
    }
}

/// Classes shared by every variant.
const BASE: StaticClass = class!(
    "group/marker relative flex min-h-4 w-full items-center gap-2 text-left text-sm font-base \
     text-foreground [&_svg:not([class*='size-'])]:size-4 [a]:underline [a]:underline-offset-3 \
     [a]:hover:text-foreground",
);

/// Builds the full class list for a marker of the given `variant`.
///
/// Use it to render a marker as another element, such as a link. Add
/// `data-slot="marker"` and the matching `data-variant`:
///
/// ```ignore
/// view! {
///     <a
///         href="/search"
///         data-slot="marker"
///         data-variant=(MarkerVariant::Default.as_str())
///         class=(marker_variants(MarkerVariant::Default))
///     >
///         marker_content("Searched the web")
///     </a>
/// }
/// ```
#[must_use]
#[allow(dead_code)]
pub fn marker_variants(variant: MarkerVariant) -> Class<(StaticClass, Option<StaticClass>)> {
    class!(BASE, variant.classes())
}

/// A small status line in a conversation, such as "Searched the web" or a
/// date divider.
///
/// `variant` defaults to `Default`. `attrs` are forwarded to the `<div>`, with
/// extra classes appended to its classes. Pass `role="status"` for a line
/// that announces progress.
///
/// ```ignore
/// view! {
///     marker(
///         marker_icon(icon(data: iconify_icon!("lucide:globe")))
///         marker_content("Searched the web")
///     )
/// }
/// ```
#[component]
pub async fn marker(
    #[default] variant: MarkerVariant,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="marker"
            data-variant=(variant.as_str())
            class=(class!(BASE, variant.classes(), attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A decorative icon at the start of a [`marker`], hidden from screen readers.
#[component]
#[allow(dead_code)]
pub async fn marker_icon(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="marker-icon"
            aria-hidden="true"
            class=(class!(
                "size-4 shrink-0 [&_svg:not([class*='size-'])]:size-4",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </span>
    })
}

/// The text of a [`marker`]. Centered between the rules of a separator marker.
#[component]
#[allow(dead_code)]
pub async fn marker_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="marker-content"
            class=(class!(
                "min-w-0 wrap-break-word group-data-[variant=separator]/marker:flex-none \
                 group-data-[variant=separator]/marker:text-center *:[a]:underline \
                 *:[a]:underline-offset-3 *:[a]:hover:text-foreground",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </span>
    })
}
