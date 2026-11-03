use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    runtime::{link_attrs, prefetch_mode},
    view::{StaticClass, View, attributes, class, component, view},
};

use crate::components::breadcrumb::{
    breadcrumb, breadcrumb_ellipsis, breadcrumb_item, breadcrumb_link, breadcrumb_list,
    breadcrumb_page, breadcrumb_separator,
};

// A stand-in for the `dropdown-menu` component: a native popover anchored
// under its trigger, using that component's classes.
const MENU: StaticClass = class!(
    "z-50 min-w-[8rem] overflow-hidden rounded-base border-2 border-border bg-background p-1 \
     font-base text-foreground outline-none inset-auto m-0 mt-1 \
     [position-area:bottom_span-right]",
);
const MENU_ITEM: StaticClass = class!(
    "relative flex w-full cursor-default select-none items-center gap-2 rounded-base border-2 \
     border-transparent px-2 py-1.5 text-sm font-base outline-hidden transition-colors \
     hover:bg-main hover:text-main-foreground hover:border-border focus:bg-main \
     focus:text-main-foreground focus:border-border",
);

#[component]
pub async fn demo() -> Result<impl View> {
    // `size-4!` beats the ellipsis's own `size-9`: `class!` does not drop
    // conflicting classes the way Octane's `cn` does.
    Ok(view! {
        breadcrumb(
            breadcrumb_list(
                breadcrumb_item(breadcrumb_link(attrs: attributes! { href="/" }, "Home"))
                breadcrumb_separator()
                breadcrumb_item(
                    <button
                        type="button"
                        popovertarget="breadcrumb-demo-menu"
                        style="anchor-name: --breadcrumb-demo"
                        class="flex items-center gap-1"
                    >
                        breadcrumb_ellipsis(attrs: attributes! { class="size-4!" })
                        <span class="sr-only">"Toggle menu"</span>
                    </button>
                    <div
                        id="breadcrumb-demo-menu"
                        popover=""
                        role="menu"
                        style="position-anchor: --breadcrumb-demo"
                        class=(MENU)
                    >
                        for item in ["Documentation", "Themes", "GitHub"] {
                            <button
                                type="button"
                                role="menuitem"
                                popovertarget="breadcrumb-demo-menu"
                                popovertargetaction="hide"
                                class=(MENU_ITEM)
                            >
                                (item)
                            </button>
                        }
                    </div>
                )
                breadcrumb_separator()
                breadcrumb_item(
                    breadcrumb_link(attrs: attributes! { href="/components" }, "Components")
                )
                breadcrumb_separator()
                breadcrumb_item(breadcrumb_page("Breadcrumb"))
            )
        )
    })
}

#[component]
pub async fn basic() -> Result<impl View> {
    Ok(view! {
        breadcrumb(
            breadcrumb_list(
                breadcrumb_item(breadcrumb_link(attrs: attributes! { href="#" }, "Home"))
                breadcrumb_separator()
                breadcrumb_item(breadcrumb_link(attrs: attributes! { href="#" }, "Components"))
                breadcrumb_separator()
                breadcrumb_item(breadcrumb_page("Breadcrumb"))
            )
        )
    })
}

#[component]
pub async fn collapsed(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        breadcrumb(
            breadcrumb_list(
                breadcrumb_item(
                    breadcrumb_link(attrs: link_attrs(cx, "/", prefetch_mode(cx)), "Home")
                )
                breadcrumb_separator()
                breadcrumb_item(breadcrumb_ellipsis())
                breadcrumb_separator()
                breadcrumb_item(
                    breadcrumb_link(
                        attrs: link_attrs(cx, "/docs/components", prefetch_mode(cx)),
                        "Components"
                    )
                )
                breadcrumb_separator()
                breadcrumb_item(breadcrumb_page("Breadcrumb"))
            )
        )
    })
}

#[component]
pub async fn custom_separator(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        breadcrumb(
            breadcrumb_list(
                breadcrumb_item(
                    breadcrumb_link(attrs: link_attrs(cx, "/", prefetch_mode(cx)), "Home")
                )
                breadcrumb_separator(icon(data: iconify_icon!("lucide:dot")))
                breadcrumb_item(
                    breadcrumb_link(
                        attrs: link_attrs(cx, "/components", prefetch_mode(cx)),
                        "Components"
                    )
                )
                breadcrumb_separator(icon(data: iconify_icon!("lucide:dot")))
                breadcrumb_item(breadcrumb_page("Breadcrumb"))
            )
        )
    })
}

#[component]
pub async fn dropdown(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        breadcrumb(
            breadcrumb_list(
                breadcrumb_item(
                    breadcrumb_link(attrs: link_attrs(cx, "/", prefetch_mode(cx)), "Home")
                )
                breadcrumb_separator(icon(data: iconify_icon!("lucide:dot")))
                breadcrumb_item(
                    <button
                        type="button"
                        popovertarget="breadcrumb-dropdown-menu"
                        style="anchor-name: --breadcrumb-dropdown"
                        class="flex items-center gap-1"
                    >
                        "Components"
                        icon(
                            data: iconify_icon!("lucide:chevron-down"),
                            attrs: attributes! { class="size-3.5" }
                        )
                    </button>
                    <div
                        id="breadcrumb-dropdown-menu"
                        popover=""
                        role="menu"
                        style="position-anchor: --breadcrumb-dropdown"
                        class=(MENU)
                    >
                        <div role="group">
                            for item in ["Documentation", "Themes", "GitHub"] {
                                <button
                                    type="button"
                                    role="menuitem"
                                    popovertarget="breadcrumb-dropdown-menu"
                                    popovertargetaction="hide"
                                    class=(MENU_ITEM)
                                >
                                    (item)
                                </button>
                            }
                        </div>
                    </div>
                )
                breadcrumb_separator(icon(data: iconify_icon!("lucide:dot")))
                breadcrumb_item(breadcrumb_page("Breadcrumb"))
            )
        )
    })
}

#[component]
pub async fn link_component(cx: &Cx) -> Result<impl View> {
    // `link_attrs` gives a plain `<a>` Topcoat's client-side navigation, the
    // way Octane passes `render={<Link />}`.
    Ok(view! {
        breadcrumb(
            breadcrumb_list(
                breadcrumb_item(
                    breadcrumb_link(
                        attrs: link_attrs(cx, "#link-component", prefetch_mode(cx)),
                        "Home"
                    )
                )
                breadcrumb_separator()
                breadcrumb_item(
                    breadcrumb_link(
                        attrs: link_attrs(cx, "#link-component", prefetch_mode(cx)),
                        "Components"
                    )
                )
                breadcrumb_separator()
                breadcrumb_item(breadcrumb_page("Breadcrumb"))
            )
        )
    })
}
