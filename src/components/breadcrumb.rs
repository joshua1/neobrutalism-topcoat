// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Attributes, Child, View, attributes, class, component, view},
};

/// Navigation links showing the path to the current page.
///
/// Put a [`breadcrumb_list`] inside, with a [`breadcrumb_item`] per step and
/// [`breadcrumb_separator`]s between them. `attrs` are forwarded to the
/// `<nav>`, with extra classes added to its (otherwise empty) class list.
///
/// ```ignore
/// view! {
///     breadcrumb(
///         breadcrumb_list(
///             breadcrumb_item(
///                 breadcrumb_link(attrs: attributes! { href="/" }, "Home")
///             )
///             breadcrumb_separator()
///             breadcrumb_item(breadcrumb_page("Breadcrumb"))
///         )
///     )
/// }
/// ```
#[component]
pub async fn breadcrumb(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <nav
            data-slot="breadcrumb"
            aria-label="breadcrumb"
            class=(attrs.remove("class"))
            (attrs)
        >
            (child)
        </nav>
    })
}

/// The ordered list of steps in a [`breadcrumb`]. Steps wrap onto another
/// line when the trail outgrows its container.
#[component]
pub async fn breadcrumb_list(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <ol
            data-slot="breadcrumb-list"
            class=(class!(
                "flex flex-wrap items-center gap-1.5 text-sm font-base break-words \
                 text-foreground sm:gap-2.5",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </ol>
    })
}

/// One step of a [`breadcrumb_list`], holding a [`breadcrumb_link`], a
/// [`breadcrumb_page`] or a [`breadcrumb_ellipsis`].
#[component]
pub async fn breadcrumb_item(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <li
            data-slot="breadcrumb-item"
            class=(class!("inline-flex items-center gap-1.5", attrs.remove("class")))
            (attrs)
        >
            (child)
        </li>
    })
}

/// A link to an ancestor page, rendered as an `<a>`.
///
/// Pass `href` in `attrs`. For a link that navigates without a full reload,
/// pass Topcoat's runtime link attributes instead:
/// `attrs: link_attrs(cx, "/docs", prefetch_mode(cx))`.
#[component]
pub async fn breadcrumb_link(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <a data-slot="breadcrumb-link" class=(attrs.remove("class")) (attrs)>
            (child)
        </a>
    })
}

/// The current page's label. It is marked `aria-current="page"` and does not
/// navigate.
#[component]
pub async fn breadcrumb_page(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="breadcrumb-page"
            role="link"
            aria-disabled="true"
            aria-current="page"
            class=(attrs.remove("class"))
            (attrs)
        >
            (child)
        </span>
    })
}

/// A decorative divider between [`breadcrumb_item`]s, hidden from assistive
/// technology.
///
/// Shows a chevron unless you pass something else as children, such as
/// `icon(data: iconify_icon!("lucide:dot"))`.
#[component]
pub async fn breadcrumb_separator(
    #[default] mut attrs: Attributes,
    #[into]
    #[default]
    child: Option<Child<'_>>,
) -> Result<impl View> {
    Ok(view! {
        <li
            data-slot="breadcrumb-separator"
            role="presentation"
            aria-hidden="true"
            class=(class!("[&>svg]:size-3.5", attrs.remove("class")))
            (attrs)
        >
            match child {
                Some(child) => (child),
                None => icon(data: iconify_icon!("lucide:chevron-right")),
            }
        </li>
    })
}

/// An ellipsis standing in for collapsed [`breadcrumb_item`]s, with a
/// screen-reader-only "More" label.
#[component]
pub async fn breadcrumb_ellipsis(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="breadcrumb-ellipsis"
            role="presentation"
            aria-hidden="true"
            class=(class!("flex size-9 items-center justify-center", attrs.remove("class")))
            (attrs)
        >
            icon(
                data: iconify_icon!("lucide:ellipsis"),
                attrs: attributes! { class="size-4" }
            )
            <span class="sr-only">"More"</span>
        </span>
    })
}
