// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

/// A bar of site navigation links, some of which reveal a panel of more links.
///
/// Pass a [`navigation_menu_list`] of [`navigation_menu_item`]s. An item holding
/// a [`navigation_menu_trigger`] and a [`navigation_menu_content`] shows its
/// panel while the item is hovered or has keyboard focus, with CSS alone.
/// Unlike Base UI, there is no shared viewport that animates between panels:
/// each panel opens below its own item, aligned to its left edge. Escape does
/// not close a panel, and arrow keys do not move between items; use Tab.
///
/// `attrs` are forwarded to the `<nav>`, with extra classes appended to its
/// classes.
///
/// ```ignore
/// view! {
///     navigation_menu(
///         navigation_menu_list(
///             navigation_menu_item(
///                 navigation_menu_trigger("Components")
///                 navigation_menu_content(
///                     <ul class="grid w-[400px] gap-3 p-2">
///                         <li>navigation_menu_link(attrs: attributes! { href="/docs" }, "Docs")</li>
///                     </ul>
///                 )
///             )
///         )
///     )
/// }
/// ```
#[component]
pub async fn navigation_menu(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <nav
            data-slot="navigation-menu"
            class=(class!(
                "group/navigation-menu relative z-10 flex max-w-max rounded-base font-heading \
                 border-border border-2 p-1 bg-background flex-1 items-center justify-center",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </nav>
    })
}

/// The row of items in a [`navigation_menu`], rendered as a `<ul>`.
#[component]
pub async fn navigation_menu_list(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <ul
            data-slot="navigation-menu-list"
            class=(class!(
                "group flex flex-1 list-none items-center font-heading justify-center space-x-1",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </ul>
    })
}

/// One entry of a [`navigation_menu_list`], rendered as an `<li>`.
///
/// Holds either a link, or a trigger with its panel. Hovering or focusing
/// anything inside it opens its panel.
#[component]
pub async fn navigation_menu_item(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <li
            data-slot="navigation-menu-item"
            class=(class!("group/navigation-menu-item relative", attrs.remove("class")))
            (attrs)
        >
            (child)
        </li>
    })
}

/// Classes for a trigger, and for a top-level link that should look like one.
/// The open state follows the item's hover and focus.
const TRIGGER_STYLE: StaticClass = class!(
    "group inline-flex h-10 w-max items-center justify-center text-foreground rounded-base \
     border-2 border-transparent bg-background px-4 py-2 text-sm font-heading transition-colors \
     focus:outline-none hover:bg-main hover:text-main-foreground hover:border-border \
     focus:bg-main focus:text-main-foreground focus:border-border \
     group-hover/navigation-menu-item:bg-main group-hover/navigation-menu-item:text-main-foreground \
     group-hover/navigation-menu-item:border-border \
     group-focus-within/navigation-menu-item:bg-main \
     group-focus-within/navigation-menu-item:text-main-foreground \
     group-focus-within/navigation-menu-item:border-border \
     aria-[current=page]:bg-main aria-[current=page]:text-main-foreground \
     aria-[current=page]:border-border disabled:pointer-events-none disabled:opacity-50",
);

/// The classes of a [`navigation_menu_trigger`].
///
/// Use them to make a top-level link look like the triggers next to it:
///
/// ```ignore
/// view! {
///     navigation_menu_item(
///         <a href="/docs" class=(navigation_menu_trigger_style())>"Documentation"</a>
///     )
/// }
/// ```
#[must_use]
#[allow(dead_code)]
pub fn navigation_menu_trigger_style() -> StaticClass {
    TRIGGER_STYLE
}

/// The button of a [`navigation_menu_item`] that reveals its panel, with a
/// chevron that flips while the panel is open.
#[component]
pub async fn navigation_menu_trigger(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            data-slot="navigation-menu-trigger"
            class=(class!(TRIGGER_STYLE, "group", attrs.remove("class")))
            (attrs)
        >
            (child)
            " "
            icon(
                data: iconify_icon!("lucide:chevron-down"),
                attrs: attributes! {
                    class="relative top-[1px] ml-2 size-4 font-heading transition duration-200 \
                        group-hover/navigation-menu-item:rotate-180 \
                        group-focus-within/navigation-menu-item:rotate-180"
                }
            )
        </button>
    })
}

/// The panel of a [`navigation_menu_item`], shown below it while the item is
/// hovered or focused.
///
/// It wraps its children in a [`navigation_menu_viewport`]. `attrs` are
/// forwarded to the inner content `<div>`, with extra classes appended to its
/// classes. Size the panel through its children, for example a
/// `<ul class="grid w-[500px]">`.
#[component]
pub async fn navigation_menu_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        navigation_menu_viewport(
            <div
                data-slot="navigation-menu-content"
                class=(class!(
                    "h-full w-auto p-2 pr-2.5 \
                     **:data-[slot=navigation-menu-link]:focus:ring-0 \
                     **:data-[slot=navigation-menu-link]:focus:outline-none",
                    attrs.remove("class"),
                ))
                (attrs)
            >
                (child)
            </div>
        )
    })
}

/// Classes for the transparent box below the item. Its top padding is the gap
/// to the trigger, kept inside the item so the pointer can cross it without
/// closing the panel. Visibility flips after the fade, so the panel stays
/// out of the tab order while closed.
const POSITIONER: StaticClass = class!(
    "absolute top-full left-0 z-50 pt-1.5 invisible opacity-0 \
     [transition:opacity_200ms_ease-out,visibility_200ms] \
     group-hover/navigation-menu-item:visible group-hover/navigation-menu-item:opacity-100 \
     group-focus-within/navigation-menu-item:visible \
     group-focus-within/navigation-menu-item:opacity-100",
);

/// Classes for the bordered panel, which grows from 90% as it opens.
const POPUP: StaticClass = class!(
    "relative origin-top-left overflow-hidden rounded-base border-2 border-border bg-background \
     text-foreground outline-none transition-[opacity,scale] duration-300 \
     ease-[cubic-bezier(0.22,1,0.36,1)] scale-90 \
     group-hover/navigation-menu-item:scale-100 group-focus-within/navigation-menu-item:scale-100",
);

/// The bordered, positioned panel that [`navigation_menu_content`] renders
/// into.
///
/// Use it directly only for a panel with custom inner markup. `attrs` are
/// forwarded to the innermost `<div>`, with extra classes appended to its
/// classes.
#[component]
pub async fn navigation_menu_viewport(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(POSITIONER)>
            <div data-slot="navigation-menu-popup" class=(POPUP)>
                <div
                    data-slot="navigation-menu-viewport"
                    class=(class!("relative size-full overflow-hidden", attrs.remove("class")))
                    (attrs)
                >
                    (child)
                </div>
            </div>
        </div>
    })
}

/// A link inside a [`navigation_menu_content`] panel.
///
/// Pass the destination as `href` in `attrs`. Set `active` for the link to
/// the current page, which adds `aria-current="page"`.
#[component]
pub async fn navigation_menu_link(
    /// Whether the link points at the current page.
    #[default]
    active: bool,
    /// Extra attributes for the `<a>` element.
    #[default]
    mut attrs: Attributes,
    /// The link's content.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <a
            data-slot="navigation-menu-link"
            aria-current=(active.then_some("page"))
            class=(class!(
                "block select-none space-y-1 rounded-base border-2 border-transparent p-2 \
                 leading-none no-underline outline-none transition-colors \
                 hover:bg-main hover:text-main-foreground hover:border-border \
                 focus:bg-main focus:text-main-foreground focus:border-border \
                 focus-visible:ring-4 focus-visible:outline-1 \
                 [&_svg:not([class*='size-'])]:size-4",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </a>
    })
}

/// A decoration, such as an arrow, that flips while its item's panel is open.
///
/// Pass the icon as children. It is hidden from assistive technology.
#[component]
#[allow(dead_code)]
pub async fn navigation_menu_indicator(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="navigation-menu-indicator"
            aria-hidden="true"
            class=(class!(
                "flex items-center justify-center transition-transform duration-200 \
                 group-hover/navigation-menu-item:rotate-180 \
                 group-focus-within/navigation-menu-item:rotate-180",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </span>
    })
}
