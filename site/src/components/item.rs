// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, Class, StaticClass, View, class, component, view},
};

/// The padding and gap of an [`item`].
///
/// [`Default`] is `ItemSize::Default`, used when no size is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ItemSize {
    /// The standard item size.
    #[default]
    Default,
    /// A compact item.
    Sm,
    /// An extra-compact item, with no padding inside a dropdown menu.
    Xs,
}

impl ItemSize {
    /// Classes for the item's padding and gap.
    fn classes(self) -> StaticClass {
        match self {
            Self::Default => class!("gap-3.5 px-4 py-3.5"),
            Self::Sm => class!("gap-2.5 px-3 py-2.5"),
            Self::Xs => class!("gap-2 px-2.5 py-2 in-data-[slot=dropdown-menu-content]:p-0"),
        }
    }

    /// The `data-size` value the item's parts style themselves by.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
            Self::Xs => "xs",
        }
    }
}

/// Classes shared by every item size.
const ITEM: StaticClass = class!(
    "group/item flex w-full flex-wrap items-center rounded-base border-2 border-border \
     bg-background text-sm font-base text-foreground transition-colors duration-100 \
     outline-none ring-offset-white focus-visible:outline-hidden focus-visible:ring-2 \
     focus-visible:ring-black focus-visible:ring-offset-2 [a]:transition-colors \
     [a]:hover:bg-main [a]:hover:text-main-foreground [button]:transition-colors \
     [button]:hover:bg-main [button]:hover:text-main-foreground",
);

/// Builds the full class list for an item of the given `size`.
///
/// Use it to render an item as a link or button. Add `data-slot="item"` and
/// the matching `data-size` so the item's parts size themselves:
///
/// ```ignore
/// view! {
///     <a
///         href="/docs"
///         data-slot="item"
///         data-size=(ItemSize::Default.as_str())
///         class=(item_variants(ItemSize::Default))
///     >
///         item_content(item_title("Visit our documentation"))
///     </a>
/// }
/// ```
#[must_use]
#[allow(dead_code)]
pub fn item_variants(size: ItemSize) -> Class<(StaticClass, StaticClass)> {
    class!(ITEM, size.classes())
}

/// A bordered row of media, content and actions.
///
/// `size` defaults to `Default`. `attrs` are forwarded to the `<div>`, with
/// extra classes appended to its classes. Use [`item_variants`] to render an
/// item as another element, such as a link.
///
/// ```ignore
/// view! {
///     item(
///         item_media(variant: ItemMediaVariant::Icon, icon(data: iconify_icon!("lucide:inbox")))
///         item_content(
///             item_title("Inbox")
///             item_description("View all incoming messages.")
///         )
///         item_actions(button(size: ButtonSize::Sm, "Open"))
///     )
/// }
/// ```
#[component]
pub async fn item(
    #[default] size: ItemSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="item"
            data-size=(size.as_str())
            class=(class!(ITEM, size.classes(), attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A list of [`item`]s, stacked with a gap that shrinks for small items.
#[component]
#[allow(dead_code)]
pub async fn item_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="list"
            data-slot="item-group"
            class=(class!(
                "group/item-group flex w-full flex-col gap-4 has-data-[size=sm]:gap-2.5 \
                 has-data-[size=xs]:gap-2",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// What an [`item_media`] holds.
///
/// [`Default`] is `ItemMediaVariant::Default`, used when no variant is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ItemMediaVariant {
    /// Any content, on a transparent background.
    #[default]
    Default,
    /// An icon, sized up to `size-5`.
    Icon,
    /// An image, cropped into a bordered square.
    Image,
}

impl ItemMediaVariant {
    /// Classes for the media variant.
    fn classes(self) -> StaticClass {
        match self {
            Self::Default => class!("bg-transparent"),
            Self::Icon => class!("[&_svg:not([class*='size-'])]:size-5"),
            Self::Image => class!(
                "size-10 overflow-hidden rounded-base border-2 border-border \
                 group-data-[size=sm]/item:size-8 group-data-[size=xs]/item:size-6 \
                 [&_img]:size-full [&_img]:object-cover",
            ),
        }
    }

    /// The `data-variant` value.
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Icon => "icon",
            Self::Image => "image",
        }
    }
}

/// An icon or image at the start of an [`item`].
///
/// `variant` defaults to `Default`. It aligns with the title when the item
/// has a description.
#[component]
#[allow(dead_code)]
pub async fn item_media(
    #[default] variant: ItemMediaVariant,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="item-media"
            data-variant=(variant.as_str())
            class=(class!(
                "flex shrink-0 items-center justify-center gap-2 \
                 group-has-data-[slot=item-description]/item:translate-y-0.5 \
                 group-has-data-[slot=item-description]/item:self-start \
                 [&_svg]:pointer-events-none",
                variant.classes(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The title and description column of an [`item`]. It grows to fill the row.
#[component]
#[allow(dead_code)]
pub async fn item_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="item-content"
            class=(class!(
                "flex flex-1 flex-col gap-1 group-data-[size=xs]/item:gap-0 \
                 [&+[data-slot=item-content]]:flex-none",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The one-line title of an [`item`].
#[component]
#[allow(dead_code)]
pub async fn item_title(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="item-title"
            class=(class!(
                "line-clamp-1 flex w-fit items-center gap-2 text-sm leading-snug font-base \
                 underline-offset-4",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The description under an [`item_title`], clamped to two lines.
#[component]
#[allow(dead_code)]
pub async fn item_description(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p
            data-slot="item-description"
            class=(class!(
                "line-clamp-2 text-left text-sm leading-normal font-base text-foreground \
                 group-data-[size=xs]/item:text-xs [&>a]:underline [&>a]:underline-offset-4 \
                 [&>a:hover]:text-foreground",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </p>
    })
}

/// Buttons or other controls at the end of an [`item`].
#[component]
#[allow(dead_code)]
pub async fn item_actions(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="item-actions"
            class=(class!("flex items-center gap-2", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A full-width row above the rest of an [`item`]'s content.
#[component]
#[allow(dead_code)]
pub async fn item_header(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="item-header"
            class=(class!(
                "flex basis-full items-center justify-between gap-2",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A full-width row below the rest of an [`item`]'s content.
#[component]
#[allow(dead_code)]
pub async fn item_footer(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="item-footer"
            class=(class!(
                "flex basis-full items-center justify-between gap-2",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}
