//! The dashboard's sidebar: the store, the pages, and the user's menu. It
//! collapses to icons on desktop and opens as a sheet on phones.

use topcoat::{
    Result,
    context::Cx,
    icon::{IconData, icon, iconify::iconify_icon},
    view::{StaticClass, View, attributes, class, component, view},
};

use crate::components::{
    avatar::{AvatarSize, avatar, avatar_fallback},
    dropdown_menu::{
        DropdownMenuAlign, DropdownMenuSide, dropdown_menu, dropdown_menu_content,
        dropdown_menu_group, dropdown_menu_item, dropdown_menu_label, dropdown_menu_separator,
        dropdown_menu_trigger,
    },
    sidebar::{
        SidebarCollapsible, SidebarMenuButtonSize, sidebar, sidebar_content, sidebar_footer,
        sidebar_group, sidebar_group_content, sidebar_group_label, sidebar_header, sidebar_menu,
        sidebar_menu_badge, sidebar_menu_button, sidebar_menu_button_variants, sidebar_menu_item,
    },
};

use super::{
    Page,
    data::{PRODUCTS, Status, USER, orders},
};

/// A dropdown menu item's classes, for the items that are links (the
/// `dropdown_menu_item` component renders a `<button>`).
const MENU_LINK: StaticClass = class!(
    "relative gap-2 flex cursor-default select-none items-center rounded-base border-2 \
     border-transparent px-2 py-1.5 text-sm font-base outline-hidden transition-colors \
     hover:bg-main hover:text-main-foreground hover:border-border \
     focus-visible:bg-main focus-visible:text-main-foreground focus-visible:border-border w-full",
);

/// One page in the sidebar, with an optional count beside it.
#[component]
async fn nav_item(
    cx: &Cx,
    page: Page,
    glyph: IconData,
    #[default] badge: Option<usize>,
    active: Page,
    range: usize,
) -> Result<impl View> {
    let current = page == active;
    let href = page.href(range);
    Ok(view! {
        sidebar_menu_item(
            sidebar_menu_button(
                href: href.as_str(),
                active: current,
                tooltip: page.title(),
                attrs: attributes! { aria-current=(current.then_some("page")) },
                icon(data: glyph)
                <span>(page.title())</span>
            )
            if let Some(count) = badge {
                sidebar_menu_badge((count))
            }
        )
    })
}

#[component]
pub async fn dashboard_sidebar(
    cx: &Cx,
    active: Page,
    range: usize,
    open: bool,
) -> Result<impl View> {
    let processing = orders()
        .iter()
        .filter(|order| order.status == Status::Processing)
        .count();
    let low_stock = PRODUCTS
        .iter()
        .filter(|product| product.stock < product.reorder)
        .count();

    Ok(view! {
        // The 15px side padding centres 32px icons in the collapsed sidebar,
        // so they stay put while it opens and closes.
        sidebar(collapsible: SidebarCollapsible::Icon, default_open: open,
            sidebar_header(attrs: attributes! { class="h-16 justify-center px-3.75" },
                <a
                    href=(Page::Overview.href(range))
                    class="flex items-center gap-3 overflow-hidden font-heading text-xl"
                >
                    <span class="flex size-8 shrink-0 items-center justify-center rounded-base border-2 border-border bg-main text-main-foreground">
                        icon(data: iconify_icon!("lucide:store"), attrs: attributes! { class="size-4" })
                    </span>
                    <span>"Tally"</span>
                </a>
            )

            sidebar_content(attrs: attributes! { class="overflow-x-hidden" },
                sidebar_group(attrs: attributes! { class="px-3.75" },
                    sidebar_group_label("Analytics")
                    sidebar_group_content(
                        sidebar_menu(attrs: attributes! { class="gap-1.5" },
                            nav_item(page: Page::Overview, glyph: iconify_icon!("lucide:layout-dashboard"), active: active, range: range)
                            nav_item(page: Page::Revenue, glyph: iconify_icon!("lucide:chart-no-axes-column"), active: active, range: range)
                            nav_item(page: Page::Marketing, glyph: iconify_icon!("lucide:megaphone"), active: active, range: range)
                        )
                    )
                )
                sidebar_group(attrs: attributes! { class="px-3.75" },
                    sidebar_group_label("Store")
                    sidebar_group_content(
                        sidebar_menu(attrs: attributes! { class="gap-1.5" },
                            nav_item(page: Page::Orders, glyph: iconify_icon!("lucide:shopping-cart"), badge: Some(processing), active: active, range: range)
                            nav_item(page: Page::Customers, glyph: iconify_icon!("lucide:users"), active: active, range: range)
                            nav_item(page: Page::Inventory, glyph: iconify_icon!("lucide:package"), badge: Some(low_stock), active: active, range: range)
                        )
                    )
                )
            )

            sidebar_footer(attrs: attributes! { class="px-3.75" },
                sidebar_menu(
                    sidebar_menu_item(
                        // `!` replaces the menu's own `inline-block`.
                        dropdown_menu(attrs: attributes! { class="block! w-full" },
                            dropdown_menu_trigger(
                                attrs: attributes! {
                                    data-sidebar="menu-button"
                                    data-size="lg"
                                    class=(sidebar_menu_button_variants(SidebarMenuButtonSize::Lg))
                                },
                                avatar(size: AvatarSize::Sm, attrs: attributes! { class="shrink-0 -outline-offset-2" },
                                    // `!` replaces the fallback's own background.
                                    avatar_fallback(attrs: attributes! { class="bg-chart-3!" }, (USER.initials))
                                )
                                <span class="grid min-w-0 flex-1 leading-tight">
                                    <span class="truncate font-heading">(USER.name)</span>
                                    <span class="truncate text-xs">(USER.email)</span>
                                </span>
                                icon(data: iconify_icon!("lucide:chevrons-up-down"), attrs: attributes! { class="ml-auto" })
                            )
                            dropdown_menu_content(
                                side: DropdownMenuSide::Top,
                                align: DropdownMenuAlign::Start,
                                attrs: attributes! { class="w-52" },
                                dropdown_menu_group(
                                    dropdown_menu_label("My account")
                                    dropdown_menu_separator()
                                    <a href=(Page::Profile.href(range)) role="menuitem" class=(MENU_LINK)>"Profile"</a>
                                    <a href=(Page::Billing.href(range)) role="menuitem" class=(MENU_LINK)>"Billing"</a>
                                    dropdown_menu_item("Log out")
                                )
                            )
                        )
                    )
                )
            )
        )
    })
}
