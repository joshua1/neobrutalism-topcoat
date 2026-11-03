// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

use super::button::{ButtonSize, ButtonVariant, button_variants};

/// Navigation between the pages of a long list.
///
/// Place links inside [`pagination_content`] and [`pagination_item`]s. Each
/// link supplies its own `href`. `attrs` are forwarded to the `<nav>`, with
/// extra classes appended to its classes.
///
/// ```ignore
/// view! {
///     pagination(
///         pagination_content(
///             pagination_item(pagination_previous(attrs: attributes! { href="?page=1" }))
///             pagination_item(pagination_link(attrs: attributes! { href="?page=1" }, "1"))
///             pagination_item(
///                 pagination_link(active: true, attrs: attributes! { href="?page=2" }, "2")
///             )
///             pagination_item(pagination_ellipsis())
///             pagination_item(pagination_next(attrs: attributes! { href="?page=3" }))
///         )
///     )
/// }
/// ```
#[component]
pub async fn pagination(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <nav
            data-slot="pagination"
            role="navigation"
            aria-label="pagination"
            class=(class!("mx-auto flex w-full justify-center", attrs.remove("class")))
            (attrs)
        >
            (child)
        </nav>
    })
}

/// The list of steps in a [`pagination`], rendered as a `<ul>`.
#[component]
pub async fn pagination_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <ul
            data-slot="pagination-content"
            class=(class!("flex flex-row items-center gap-1", attrs.remove("class")))
            (attrs)
        >
            (child)
        </ul>
    })
}

/// One step of a [`pagination_content`], holding a link or an ellipsis.
#[component]
pub async fn pagination_item(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <li data-slot="pagination-item" class=(attrs.remove("class")) (attrs)>(child)</li>
    })
}

/// Classes that repaint an inactive link on the page background. The
/// no-shadow button variant fills with `bg-main`, and `class!` does not drop
/// conflicting classes, so these use the `!` (important) suffix to win.
const INACTIVE: StaticClass = class!("bg-background! text-foreground!");

/// A link to one page, styled as a no-shadow button.
///
/// Pass its destination as `href` in `attrs`. Set `active` for the current
/// page: it is filled with the main color and gets `aria-current="page"`.
/// `size` defaults to `ButtonSize::Icon`, a square for a page number.
#[component]
pub async fn pagination_link(
    /// Whether this link points at the page being shown.
    #[default]
    active: bool,
    /// The button size of the link.
    #[default(ButtonSize::Icon)]
    size: ButtonSize,
    /// Extra attributes for the `<a>` element.
    #[default]
    mut attrs: Attributes,
    /// The link's label.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <a
            data-slot="pagination-link"
            aria-current=(active.then_some("page"))
            class=(class!(
                button_variants(ButtonVariant::NoShadow, size),
                INACTIVE if !active,
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </a>
    })
}

/// The link to the page before the one being shown.
///
/// Pass its destination as `href` in `attrs`.
#[component]
pub async fn pagination_previous(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <a
            data-slot="pagination-previous"
            aria-label="Go to previous page"
            class=(class!(
                button_variants(ButtonVariant::NoShadow, ButtonSize::Default),
                INACTIVE,
                // `gap-1!` beats the button's `gap-2`.
                "gap-1! pl-2.5",
                attrs.remove("class"),
            ))
            (attrs)
        >
            icon(
                data: iconify_icon!("lucide:chevron-left"),
                attrs: attributes! { class="size-4" }
            )
            <span>"Previous"</span>
        </a>
    })
}

/// The link to the page after the one being shown.
///
/// Pass its destination as `href` in `attrs`.
#[component]
pub async fn pagination_next(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <a
            data-slot="pagination-next"
            aria-label="Go to next page"
            class=(class!(
                button_variants(ButtonVariant::NoShadow, ButtonSize::Default),
                INACTIVE,
                // `gap-1!` beats the button's `gap-2`.
                "gap-1! pr-2.5",
                attrs.remove("class"),
            ))
            (attrs)
        >
            <span>"Next"</span>
            icon(
                data: iconify_icon!("lucide:chevron-right"),
                attrs: attributes! { class="size-4" }
            )
        </a>
    })
}

/// An ellipsis standing in for the page links left out.
#[component]
pub async fn pagination_ellipsis(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="pagination-ellipsis"
            aria-hidden="true"
            class=(class!("flex size-9 items-center justify-center", attrs.remove("class")))
            (attrs)
        >
            icon(
                data: iconify_icon!("lucide:ellipsis"),
                attrs: attributes! { class="size-4" }
            )
            <span class="sr-only">"More pages"</span>
        </span>
    })
}
