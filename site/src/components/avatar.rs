// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

/// The size of an [`avatar`].
///
/// [`Default`] is `AvatarSize::Default`, used when no size is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AvatarSize {
    /// A compact avatar, with smaller fallback text and a dot-only badge.
    Sm,
    /// The standard avatar size.
    #[default]
    Default,
    /// A prominent avatar, with larger fallback text.
    Lg,
}

impl AvatarSize {
    /// Classes for the avatar dimensions.
    fn classes(self) -> StaticClass {
        match self {
            Self::Sm => class!("size-8"),
            Self::Default => class!("size-10"),
            Self::Lg => class!("size-12"),
        }
    }

    /// The value of the avatar's `data-size` attribute. The fallback and badge
    /// read it through `group-data-[size=..]/avatar:` classes.
    const fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Default => "default",
            Self::Lg => "lg",
        }
    }
}

/// A circular image with a fallback, outlined in the border color.
///
/// Add an [`avatar_image`], an [`avatar_fallback`], or both, and optionally an
/// [`avatar_badge`]. The image is layered over the fallback, so the fallback
/// shows while the image loads and stays visible if it fails. `size` defaults
/// to `Default`. `attrs` are forwarded to the `<span>`, with extra classes
/// appended to its classes.
///
/// ```ignore
/// view! {
///     avatar(
///         avatar_image(src: "https://github.com/shadcn.png", alt: "@shadcn")
///         avatar_fallback("CN")
///     )
/// }
/// ```
#[component]
pub async fn avatar(
    #[default] size: AvatarSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="avatar"
            data-size=(size.as_str())
            class=(class!(
                "group/avatar relative flex shrink-0 rounded-full outline-2 outline-border",
                size.classes(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </span>
    })
}

/// An image that fills its [`avatar`] and covers the fallback.
///
/// `src` is required. `alt` defaults to empty text, which suits an avatar
/// next to the person's name. The image is positioned over the fallback
/// (`absolute inset-0`) and removes itself if it fails to load, uncovering
/// the fallback. `attrs` are forwarded to the `<img>`, with extra classes
/// appended to its classes.
#[component]
pub async fn avatar_image(
    /// The image URL.
    #[into]
    src: String,
    /// Alternative text for the image.
    #[into]
    #[default]
    alt: String,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <img
            data-slot="avatar-image"
            src=(src)
            alt=(alt)
            onerror="this.remove()"
            class=(class!(
                "absolute inset-0",
                "aspect-square size-full rounded-full",
                attrs.remove("class"),
            ))
            (attrs)
        >
    })
}

/// Content shown behind the [`avatar_image`], such as initials. Its text size
/// follows the avatar's size.
#[component]
pub async fn avatar_fallback(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="avatar-fallback"
            class=(class!(
                "flex size-full items-center justify-center rounded-full \
                 bg-secondary-background text-foreground font-base \
                 group-data-[size=sm]/avatar:text-xs group-data-[size=lg]/avatar:text-lg",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </span>
    })
}

/// Classes that size the badge, and any icon in it, from the avatar's size.
const BADGE_SIZES: StaticClass = class!(
    "group-data-[size=sm]/avatar:size-3 group-data-[size=sm]/avatar:[&>svg]:hidden \
     group-data-[size=default]/avatar:size-4 group-data-[size=default]/avatar:[&>svg]:size-2 \
     group-data-[size=lg]/avatar:size-5 group-data-[size=lg]/avatar:[&>svg]:size-2.5",
);

/// A status dot in the bottom-right corner of an [`avatar`]. Pass an icon as
/// children to show it inside the dot (hidden on small avatars).
#[component]
pub async fn avatar_badge(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="avatar-badge"
            class=(class!(
                "absolute right-0 bottom-0 z-10 inline-flex items-center justify-center \
                 rounded-full border-2 border-border bg-main text-main-foreground select-none \
                 [&>svg]:shrink-0",
                BADGE_SIZES,
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </span>
    })
}

/// A row of overlapping [`avatar`]s. End it with an [`avatar_group_count`] to
/// show how many more there are.
#[component]
pub async fn avatar_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="avatar-group"
            class=(class!("group/avatar-group flex -space-x-2", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A circle at the end of an [`avatar_group`] holding a count such as `"+3"`
/// or an icon.
#[component]
pub async fn avatar_group_count(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="avatar-group-count"
            class=(class!(
                "relative flex size-10 shrink-0 items-center justify-center rounded-full \
                 bg-secondary-background text-sm font-base text-foreground outline-2 \
                 outline-border [&>svg]:size-4",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}
