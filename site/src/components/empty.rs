// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

/// A dashed panel for an empty state, such as a list with no items yet.
///
/// Put an [`empty_header`] (holding [`empty_media`], [`empty_title`] and
/// [`empty_description`]) and an [`empty_content`] for actions inside.
/// `attrs` are forwarded to the `<div>`, with extra classes appended to its
/// classes.
///
/// ```ignore
/// view! {
///     empty(
///         empty_header(
///             empty_media(
///                 variant: EmptyMediaVariant::Icon,
///                 icon(data: iconify_icon!("lucide:folder"))
///             )
///             empty_title("No projects yet")
///             empty_description("Get started by creating your first project.")
///         )
///         empty_content(button(size: ButtonSize::Sm, "Create project"))
///     )
/// }
/// ```
#[component]
pub async fn empty(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="empty"
            class=(class!(
                "flex w-full min-w-0 flex-1 flex-col items-center justify-center gap-4 \
                 rounded-base border-2 border-dashed border-border bg-background p-6 \
                 text-center font-base md:p-12 text-foreground text-balance",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The centered heading block of an [`empty`] state.
#[component]
pub async fn empty_header(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="empty-header"
            class=(class!("flex max-w-sm flex-col items-center gap-2", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// How an [`empty_media`] frames its content.
///
/// [`Default`] is `EmptyMediaVariant::Default`, used when no variant is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum EmptyMediaVariant {
    /// No frame, for an illustration or avatar.
    #[default]
    Default,
    /// A small bordered tile for a single icon.
    Icon,
}

impl EmptyMediaVariant {
    /// Classes for the variant's frame.
    fn classes(self) -> StaticClass {
        match self {
            Self::Default => class!("bg-transparent"),
            Self::Icon => class!(
                "flex size-10 shrink-0 items-center justify-center rounded-base border-2 \
                 border-border bg-secondary-background text-foreground \
                 [&_svg:not([class*='size-'])]:size-5",
            ),
        }
    }

    /// The value of the media's `data-variant` attribute.
    const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Icon => "icon",
        }
    }
}

/// Classes shared by every media variant.
const MEDIA: StaticClass = class!(
    "mb-2 flex shrink-0 items-center justify-center [&_svg]:pointer-events-none \
     [&_svg]:shrink-0",
);

/// An icon or image above the [`empty_title`]. `variant` defaults to
/// `Default`; use `Icon` to frame a single icon.
#[component]
pub async fn empty_media(
    #[default] variant: EmptyMediaVariant,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="empty-icon"
            data-variant=(variant.as_str())
            class=(class!(MEDIA, variant.classes(), attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The heading of an [`empty`] state.
#[component]
pub async fn empty_title(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="empty-title"
            class=(class!("font-heading text-sm tracking-tight", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Text explaining an [`empty`] state. Links inside it are underlined.
#[component]
pub async fn empty_description(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="empty-description"
            class=(class!(
                "text-sm/relaxed font-base text-foreground [&>a]:underline \
                 [&>a]:underline-offset-4 [&>a:hover]:text-foreground",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The actions of an [`empty`] state, such as a button or a search field.
#[component]
pub async fn empty_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="empty-content"
            class=(class!(
                "flex w-full max-w-sm min-w-0 flex-col items-center gap-2.5 text-sm text-balance",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}
