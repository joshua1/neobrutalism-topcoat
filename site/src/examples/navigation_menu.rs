use topcoat::{
    Result,
    view::{Child, View, attributes, component, view},
};

use crate::components::navigation_menu::{
    navigation_menu, navigation_menu_content, navigation_menu_item, navigation_menu_link,
    navigation_menu_list, navigation_menu_trigger, navigation_menu_trigger_style,
};

/// The entries of the "Components" panel: title, link and description.
const COMPONENTS: [(&str, &str, &str); 6] = [
    (
        "Alert Dialog",
        "https://ui.shadcn.com/docs/primitives/alert-dialog",
        "A modal dialog that interrupts the user with important content and expects a response.",
    ),
    (
        "Hover Card",
        "https://ui.shadcn.com/docs/primitives/hover-card",
        "For sighted users to preview content available behind a link.",
    ),
    (
        "Progress",
        "https://ui.shadcn.com/docs/primitives/progress",
        "Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.",
    ),
    (
        "Scroll-area",
        "https://ui.shadcn.com/docs/primitives/scroll-area",
        "Visually or semantically separates content.",
    ),
    (
        "Tabs",
        "https://ui.shadcn.com/docs/primitives/tabs",
        "A set of layered sections of content—known as tab panels—that are displayed one at a time.",
    ),
    (
        "Tooltip",
        "https://ui.shadcn.com/docs/primitives/tooltip",
        "A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.",
    ),
];

/// A titled link with a short description, as one entry of a panel.
#[component]
async fn list_item(title: &str, href: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <li>
            navigation_menu_link(
                attrs: attributes! {
                    href=(href)
                    // `p-3!` overrides the link's own `p-2`.
                    class="block text-foreground select-none space-y-1 rounded-base border-2 \
                        border-transparent p-3! leading-none no-underline outline-hidden \
                        transition-colors hover:bg-main hover:text-main-foreground \
                        hover:border-border"
                },
                <div class="text-base font-heading leading-none">(title)</div>
                <p class="font-base line-clamp-2 text-sm leading-snug">(child)</p>
            )
        </li>
    })
}

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        navigation_menu(attrs: attributes! { class="z-5" },
            navigation_menu_list(
                navigation_menu_item(attrs: attributes! { class="sm:block hidden" },
                    navigation_menu_trigger("Getting started")
                    navigation_menu_content(
                        <ul class="grid w-[500px] gap-3 p-2 lg:grid-cols-[.75fr_1fr]">
                            <li class="row-span-3">
                                navigation_menu_link(
                                    attrs: attributes! {
                                        href="https://ui.shadcn.com"
                                        // `flex!` and `p-6!` override the link's own
                                        // `block` and `p-2`.
                                        class="flex! h-full w-full select-none flex-col \
                                            justify-end rounded-base border-2 border-transparent \
                                            p-6! no-underline outline-hidden hover:border-border"
                                    },
                                    <div class="mb-2 mt-4 text-lg font-heading">"shadcn/ui"</div>
                                    <p class="text-sm font-base leading-tight">
                                        "Beautifully designed components that you can copy and \
                                         paste into your apps. Accessible. Customizable. Open \
                                         Source."
                                    </p>
                                )
                            </li>
                            list_item(href: "https://ui.shadcn.com/docs", title: "Introduction",
                                "Re-usable components built using Base UI and Tailwind CSS."
                            )
                            list_item(
                                href: "https://ui.shadcn.com/docs/installation",
                                title: "Installation",
                                "How to install dependencies and structure your app."
                            )
                            list_item(
                                href: "https://ui.shadcn.com/docs/primitives/typography",
                                title: "Typography",
                                "Styles for headings, paragraphs, lists...etc"
                            )
                        </ul>
                    )
                )
                navigation_menu_item(
                    navigation_menu_trigger("Components")
                    navigation_menu_content(
                        <ul class="grid w-[400px] gap-3 p-2 md:w-[500px] md:grid-cols-2 lg:w-[600px]">
                            #[key(title)]
                            for (title, href, description) in COMPONENTS {
                                list_item(title: title, href: href, (description))
                            }
                        </ul>
                    )
                )
                navigation_menu_item(
                    // Octane renders a `NavigationMenuLink` as an `<a>` with the
                    // trigger classes; a plain link avoids mixing both class lists.
                    <a
                        href="https://ui.shadcn.com/docs"
                        data-slot="navigation-menu-link"
                        class=(navigation_menu_trigger_style())
                    >
                        "Documentation"
                    </a>
                )
            )
        )
    })
}
