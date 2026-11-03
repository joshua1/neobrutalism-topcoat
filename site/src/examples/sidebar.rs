//! A port of `page.tsrx` (an application layout, the docs preview) with
//! `sidebar.tsrx` (the `AppSidebar` it renders). `app_sidebar` is a plain
//! function after `page` rather than a second example, since it only makes
//! sense inside the layout.
//!
//! - The layout is shown in a 640px box: the provider is `relative` with a
//!   height, and the sidebar is `contained` (Octane's preview passes
//!   `class="static"` to the sidebar for the same reason).
//! - Base UI's `render` props become class builders: the dropdown and
//!   collapsible triggers get `sidebar_menu_button_variants(..)` or
//!   `sidebar_menu_action_variants(..)`, and a collapsible item's tooltip is a
//!   `sidebar_menu_tooltip` beside the `<details>`.
//! - `data-popup-open:` / `data-panel-open:` become `group-open/dropdown-menu:`
//!   and `group-open/collapsible:`. The team switcher keeps the active team in
//!   a signal. The menus open to the right; there is no `isMobile` to flip
//!   them below the trigger, but they flip on their own when out of room.

use topcoat::{
    Result,
    context::Cx,
    icon::{IconData, icon, iconify::iconify_icon},
    runtime::{Event, signal},
    view::{Child, View, attributes, class, component, view},
};

use crate::components::avatar::{avatar, avatar_fallback, avatar_image};
use crate::components::breadcrumb::{
    breadcrumb, breadcrumb_item, breadcrumb_link, breadcrumb_list, breadcrumb_page,
    breadcrumb_separator,
};
use crate::components::collapsible::{collapsible, collapsible_content, collapsible_trigger};
use crate::components::dropdown_menu::{
    DropdownMenuAlign, DropdownMenuSide, dropdown_menu, dropdown_menu_content, dropdown_menu_group,
    dropdown_menu_item, dropdown_menu_label, dropdown_menu_separator, dropdown_menu_shortcut,
    dropdown_menu_trigger,
};
use crate::components::sidebar::{
    SidebarCollapsible, SidebarMenuButtonSize, sidebar, sidebar_content, sidebar_footer,
    sidebar_group, sidebar_group_label, sidebar_header, sidebar_inset, sidebar_menu,
    sidebar_menu_action_variants, sidebar_menu_button, sidebar_menu_button_variants,
    sidebar_menu_item, sidebar_menu_sub, sidebar_menu_sub_button, sidebar_menu_sub_item,
    sidebar_menu_tooltip, sidebar_provider, sidebar_rail, sidebar_state, sidebar_trigger,
};

#[component]
pub async fn page(cx: &Cx) -> Result<impl View> {
    // The sidebar renders in the state the user last left it in.
    let open = sidebar_state(cx).unwrap_or(true);

    Ok(view! {
        // `min-h-0!` drops the provider's viewport-height minimum.
        sidebar_provider(
            attrs: attributes! {
                class="relative h-[640px] min-h-0! overflow-hidden rounded-base border-2 border-border"
            },
            (app_sidebar(cx, open))
            sidebar_inset(attrs: attributes! { class="overflow-y-auto" },
                <header class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12">
                    <div class="flex items-center gap-2 px-4">
                        sidebar_trigger(attrs: attributes! { class="-ml-1" })
                        breadcrumb(
                            breadcrumb_list(
                                // `!` replaces the item's own `inline-flex`.
                                breadcrumb_item(attrs: attributes! { class="hidden! md:block!" },
                                    breadcrumb_link(attrs: attributes! { href="#" },
                                        "Building Your Application"
                                    )
                                )
                                breadcrumb_separator(attrs: attributes! { class="hidden md:block" })
                                breadcrumb_item(breadcrumb_page("Data Fetching"))
                            )
                        )
                    </div>
                </header>
                <div class="flex flex-1 flex-col gap-4 p-4 pt-0">
                    <div class="grid auto-rows-min gap-4 md:grid-cols-3">
                        <div class="aspect-video rounded-base bg-background/50 border-2 border-border"></div>
                        <div class="aspect-video rounded-base bg-background/50 border-2 border-border"></div>
                        <div class="aspect-video rounded-base bg-background/50 border-2 border-border"></div>
                    </div>
                    <div class="min-h-[100vh] flex-1 rounded-base bg-background/50 border-2 border-border md:min-h-min"></div>
                </div>
            )
        )
    })
}

/// The signed-in user shown in the footer.
struct User {
    name: &'static str,
    email: &'static str,
    avatar: &'static str,
}

/// A team in the header's switcher.
struct Team {
    name: &'static str,
    logo: IconData,
    plan: &'static str,
}

/// A link in a collapsible section's submenu.
struct SubItem {
    title: &'static str,
    url: &'static str,
}

/// A collapsible section of the "Platform" group.
struct NavItem {
    title: &'static str,
    icon: IconData,
    /// Whether the section starts open.
    active: bool,
    items: &'static [SubItem],
}

/// A row of the "Projects" group.
struct Project {
    name: &'static str,
    url: &'static str,
    icon: IconData,
}

const USER: User = User {
    name: "shadcn",
    email: "m@example.com",
    avatar: "https://github.com/shadcn.png?size=40",
};

const TEAMS: &[Team] = &[
    Team {
        name: "Acme Inc",
        logo: iconify_icon!("lucide:gallery-vertical-end"),
        plan: "Enterprise",
    },
    Team {
        name: "Acme Corp.",
        logo: iconify_icon!("lucide:audio-waveform"),
        plan: "Startup",
    },
    Team {
        name: "Evil Corp.",
        logo: iconify_icon!("lucide:command"),
        plan: "Free",
    },
];

const NAV_MAIN: &[NavItem] = &[
    NavItem {
        title: "Playground",
        icon: iconify_icon!("lucide:square-terminal"),
        active: true,
        items: &[
            SubItem {
                title: "History",
                url: "#",
            },
            SubItem {
                title: "Starred",
                url: "#",
            },
            SubItem {
                title: "Settings",
                url: "#",
            },
        ],
    },
    NavItem {
        title: "Models",
        icon: iconify_icon!("lucide:bot"),
        active: false,
        items: &[
            SubItem {
                title: "Genesis",
                url: "#",
            },
            SubItem {
                title: "Explorer",
                url: "#",
            },
            SubItem {
                title: "Quantum",
                url: "#",
            },
        ],
    },
    NavItem {
        title: "Documentation",
        icon: iconify_icon!("lucide:book-open"),
        active: false,
        items: &[
            SubItem {
                title: "Introduction",
                url: "#",
            },
            SubItem {
                title: "Get Started",
                url: "#",
            },
            SubItem {
                title: "Tutorials",
                url: "#",
            },
            SubItem {
                title: "Changelog",
                url: "#",
            },
        ],
    },
    NavItem {
        title: "Settings",
        icon: iconify_icon!("lucide:settings-2"),
        active: false,
        items: &[
            SubItem {
                title: "General",
                url: "#",
            },
            SubItem {
                title: "Team",
                url: "#",
            },
            SubItem {
                title: "Billing",
                url: "#",
            },
            SubItem {
                title: "Limits",
                url: "#",
            },
        ],
    },
];

const PROJECTS: &[Project] = &[
    Project {
        name: "Design Engineering",
        url: "#",
        icon: iconify_icon!("lucide:frame"),
    },
    Project {
        name: "Sales & Marketing",
        url: "#",
        icon: iconify_icon!("lucide:chart-pie"),
    },
    Project {
        name: "Travel",
        url: "#",
        icon: iconify_icon!("lucide:map"),
    },
];

/// Octane's `AppSidebar`. A plain function can build a view too: it names
/// the request context the view renders against (`cx =>`), which a
/// `#[component]` does for you, and returns it as a `Child` so the caller
/// can interpolate it.
fn app_sidebar(cx: &Cx, default_open: bool) -> Child<'_> {
    let active_team = signal(cx, || 0usize);
    // Owned strings, so runtime expressions can index them in the browser.
    let team_names: Vec<String> = TEAMS.iter().map(|team| team.name.to_owned()).collect();
    let team_plans: Vec<String> = TEAMS.iter().map(|team| team.plan.to_owned()).collect();

    view! { cx =>
        sidebar(collapsible: SidebarCollapsible::Icon, default_open: default_open, contained: true,
            sidebar_header(
                sidebar_menu(
                    sidebar_menu_item(
                        dropdown_menu(attrs: attributes! { class="block! w-full" },
                            dropdown_menu_trigger(
                                attrs: attributes! {
                                    data-sidebar="menu-button"
                                    data-size="lg"
                                    class=(class!(
                                        sidebar_menu_button_variants(SidebarMenuButtonSize::Lg),
                                        "focus-visible:ring-0 group-open/dropdown-menu:bg-main \
                                         group-open/dropdown-menu:text-main-foreground \
                                         group-open/dropdown-menu:outline-border \
                                         group-open/dropdown-menu:outline-2",
                                    ))
                                },
                                <div class="flex aspect-square size-8 items-center justify-center rounded-base">
                                    // Every logo is rendered; the active team's shows.
                                    #[key(index)]
                                    for (index, team) in TEAMS.iter().enumerate() {
                                        icon(
                                            data: team.logo.clone(),
                                            attrs: attributes! {
                                                class="size-4"
                                                :hidden=$(active_team.get() != index)
                                            }
                                        )
                                    }
                                </div>
                                <div class="grid flex-1 text-left text-sm leading-tight">
                                    <span class="truncate font-heading">$(team_names[active_team.get()].to_owned())</span>
                                    <span class="truncate font-base text-xs">$(team_plans[active_team.get()].to_owned())</span>
                                </div>
                                icon(
                                    data: iconify_icon!("lucide:chevrons-up-down"),
                                    attrs: attributes! { class="ml-auto" }
                                )
                            )
                            // `w-[anchor-size(width)]` is Base UI's `w-(--anchor-width)`
                            // (the trigger's width); `min-w-56!` beats the panel's own
                            // `min-w-[8rem]`.
                            dropdown_menu_content(
                                side: DropdownMenuSide::Right,
                                align: DropdownMenuAlign::Start,
                                attrs: attributes! { class="w-[anchor-size(width)] min-w-56! rounded-base" },
                                dropdown_menu_label(attrs: attributes! { class="text-sm font-heading" }, "Teams")
                                #[key(index)]
                                for (index, team) in TEAMS.iter().enumerate() {
                                    // `p-1.5!` beats the item's own `px-2 py-1.5`.
                                    dropdown_menu_item(
                                        attrs: attributes! {
                                            class="gap-2 p-1.5!"
                                            @click=$(|_e: Event| active_team.set(index))
                                        },
                                        <div class="flex size-6 items-center justify-center">
                                            icon(data: team.logo.clone(), attrs: attributes! { class="size-4 shrink-0" })
                                        </div>
                                        (team.name)
                                        dropdown_menu_shortcut((format!("⌘{}", index + 1)))
                                    )
                                }
                                dropdown_menu_separator()
                                dropdown_menu_item(attrs: attributes! { class="gap-2 p-1.5!" },
                                    <div class="flex size-6 items-center justify-center">
                                        icon(data: iconify_icon!("lucide:plus"), attrs: attributes! { class="size-4" })
                                    </div>
                                    <div class="font-base">"Add team"</div>
                                )
                            )
                        )
                    )
                )
            )
            sidebar_content(
                sidebar_group(
                    sidebar_group_label("Platform")
                    sidebar_menu(
                        for item in NAV_MAIN {
                            sidebar_menu_item(
                                collapsible(attrs: attributes! { open=(item.active) },
                                    collapsible_trigger(
                                        attrs: attributes! {
                                            data-sidebar="menu-button"
                                            data-size="default"
                                            class=(class!(
                                                sidebar_menu_button_variants(SidebarMenuButtonSize::Default),
                                                "group-open/collapsible:bg-main \
                                                 group-open/collapsible:outline-border \
                                                 group-open/collapsible:text-main-foreground",
                                            ))
                                        },
                                        icon(data: item.icon.clone())
                                        <span>(item.title)</span>
                                        icon(
                                            data: iconify_icon!("lucide:chevron-right"),
                                            attrs: attributes! {
                                                class="ml-auto transition-transform duration-200 \
                                                       group-open/collapsible:rotate-90"
                                            }
                                        )
                                    )
                                    collapsible_content(
                                        sidebar_menu_sub(
                                            for sub_item in item.items {
                                                sidebar_menu_sub_item(
                                                    sidebar_menu_sub_button(
                                                        attrs: attributes! { href=(sub_item.url) },
                                                        <span>(sub_item.title)</span>
                                                    )
                                                )
                                            }
                                        )
                                    )
                                )
                                // Outside the `<details>`, which hides its
                                // content while closed.
                                sidebar_menu_tooltip((item.title))
                            )
                        }
                    )
                )
                sidebar_group(attrs: attributes! { class="group-data-[collapsible=icon]:hidden" },
                    sidebar_group_label("Projects")
                    sidebar_menu(
                        for project in PROJECTS {
                            sidebar_menu_item(
                                sidebar_menu_button(href: project.url,
                                    icon(data: project.icon.clone())
                                    <span>(project.name)</span>
                                )
                                // A zero-size box in the item's corner, so the
                                // action places itself against the item.
                                dropdown_menu(attrs: attributes! { class="absolute! top-0 right-0" },
                                    dropdown_menu_trigger(
                                        attrs: attributes! {
                                            data-sidebar="menu-action"
                                            class=(sidebar_menu_action_variants(false))
                                        },
                                        icon(
                                            data: iconify_icon!("lucide:ellipsis"),
                                            attrs: attributes! { class="group-hover/menu-item:text-main-foreground" }
                                        )
                                        <span class="sr-only">"More"</span>
                                    )
                                    dropdown_menu_content(
                                        side: DropdownMenuSide::Right,
                                        align: DropdownMenuAlign::Start,
                                        attrs: attributes! { class="w-48" },
                                        dropdown_menu_item(
                                            icon(data: iconify_icon!("lucide:folder"))
                                            <span>"View Project"</span>
                                        )
                                        dropdown_menu_item(
                                            icon(data: iconify_icon!("lucide:forward"))
                                            <span>"Share Project"</span>
                                        )
                                        dropdown_menu_separator()
                                        dropdown_menu_item(
                                            icon(data: iconify_icon!("lucide:trash-2"))
                                            <span>"Delete Project"</span>
                                        )
                                    )
                                )
                            )
                        }
                        sidebar_menu_item(
                            sidebar_menu_button(
                                icon(data: iconify_icon!("lucide:ellipsis"))
                                <span>"More"</span>
                            )
                        )
                    )
                )
            )
            sidebar_footer(
                sidebar_menu(
                    sidebar_menu_item(
                        dropdown_menu(attrs: attributes! { class="block! w-full" },
                            dropdown_menu_trigger(
                                attrs: attributes! {
                                    data-sidebar="menu-button"
                                    data-size="lg"
                                    // `!` replaces the button's own `overflow-hidden`.
                                    class=(class!(
                                        sidebar_menu_button_variants(SidebarMenuButtonSize::Lg),
                                        "group-data-[state=collapsed]:hover:outline-0 \
                                         group-data-[state=collapsed]:hover:bg-transparent \
                                         overflow-visible!",
                                    ))
                                },
                                (user_summary(cx))
                                // Hidden when collapsed: the button lets its
                                // content overflow (for the avatar's outline),
                                // and the panel sits above the page content.
                                icon(
                                    data: iconify_icon!("lucide:chevrons-up-down"),
                                    attrs: attributes! {
                                        class="ml-auto size-4 group-data-[collapsible=icon]:hidden"
                                    }
                                )
                            )
                            dropdown_menu_content(
                                side: DropdownMenuSide::Right,
                                align: DropdownMenuAlign::End,
                                attrs: attributes! { class="w-[anchor-size(width)] min-w-56!" },
                                // `!` beats the label's own `px-2 py-1.5` and `font-heading`.
                                dropdown_menu_label(attrs: attributes! { class="p-0! font-base!" },
                                    <div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
                                        (user_summary(cx))
                                    </div>
                                )
                                dropdown_menu_separator()
                                dropdown_menu_group(
                                    dropdown_menu_item(
                                        icon(data: iconify_icon!("lucide:sparkles"))
                                        "Upgrade to Pro"
                                    )
                                )
                                dropdown_menu_separator()
                                dropdown_menu_group(
                                    dropdown_menu_item(
                                        icon(data: iconify_icon!("lucide:badge-check"))
                                        "Account"
                                    )
                                    dropdown_menu_item(
                                        icon(data: iconify_icon!("lucide:credit-card"))
                                        "Billing"
                                    )
                                    dropdown_menu_item(
                                        icon(data: iconify_icon!("lucide:bell"))
                                        "Notifications"
                                    )
                                )
                                dropdown_menu_separator()
                                dropdown_menu_item(
                                    icon(data: iconify_icon!("lucide:log-out"))
                                    "Log out"
                                )
                            )
                        )
                    )
                )
            )
            sidebar_rail()
        )
    }
    .into()
}

/// The avatar, name and email of the user, in the footer button and its menu.
fn user_summary(cx: &Cx) -> Child<'_> {
    view! { cx =>
        // `!` replaces the avatar's own `size-10`.
        avatar(
            attrs: attributes! { class="h-8! w-8!" },
            avatar_image(src: USER.avatar, alt: "CN")
            avatar_fallback("CN")
        )
        <div class="grid flex-1 text-left text-sm leading-tight">
            <span class="truncate font-heading">(USER.name)</span>
            <span class="truncate text-xs">(USER.email)</span>
        </div>
    }
    .into()
}
