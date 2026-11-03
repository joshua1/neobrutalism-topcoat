use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{StaticClass, View, attributes, class, component, view},
};

// The group examples share their names with the group parts, so those parts
// are called through the module path, `ui::avatar_group(..)`.
use crate::components::{
    avatar::{self as ui, AvatarSize, avatar, avatar_badge, avatar_fallback, avatar_image},
    button::{ButtonSize, ButtonVariant, button_variants},
};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        avatar(
            avatar_image(src: "https://github.com/shadcn.png", alt: "@shadcn")
            avatar_fallback("CN")
        )
    })
}

#[component]
pub async fn basic() -> Result<impl View> {
    Ok(view! {
        avatar(
            avatar_image(
                src: "https://github.com/shadcn.png",
                alt: "@shadcn",
                attrs: attributes! { class="grayscale" },
            )
            avatar_fallback("CN")
        )
    })
}

#[component]
pub async fn fallback() -> Result<impl View> {
    Ok(view! {
        avatar(avatar_fallback("CN"))
    })
}

#[component]
pub async fn sizes() -> Result<impl View> {
    Ok(view! {
        <div class="flex flex-wrap items-center gap-2 grayscale">
            avatar(
                size: AvatarSize::Sm,
                avatar_image(src: "https://github.com/shadcn.png", alt: "@shadcn")
                avatar_fallback("CN")
            )
            avatar(
                avatar_image(src: "https://github.com/shadcn.png", alt: "@shadcn")
                avatar_fallback("CN")
            )
            avatar(
                size: AvatarSize::Lg,
                avatar_image(src: "https://github.com/shadcn.png", alt: "@shadcn")
                avatar_fallback("CN")
            )
        </div>
    })
}

#[component]
pub async fn badge() -> Result<impl View> {
    // `!` beats the badge's own `bg-main`: `class!` does not drop conflicting
    // classes the way Octane's `cn` does.
    Ok(view! {
        avatar(
            avatar_image(src: "https://github.com/shadcn.png", alt: "@shadcn")
            avatar_fallback("CN")
            avatar_badge(attrs: attributes! { class="bg-green-600! dark:bg-green-800!" })
        )
    })
}

#[component]
pub async fn badge_with_icon() -> Result<impl View> {
    Ok(view! {
        avatar(
            attrs: attributes! { class="grayscale" },
            avatar_image(src: "https://github.com/pranathip.png", alt: "@pranathip")
            avatar_fallback("PP")
            avatar_badge(icon(data: iconify_icon!("lucide:plus")))
        )
    })
}

/// The people in the group examples: image URL, alt text and initials.
const PEOPLE: [(&str, &str, &str); 3] = [
    ("https://github.com/shadcn.png", "@shadcn", "CN"),
    ("https://github.com/maxleiter.png", "@maxleiter", "LR"),
    ("https://github.com/evilrabbit.png", "@evilrabbit", "ER"),
];

#[component]
pub async fn avatar_group() -> Result<impl View> {
    Ok(view! {
        ui::avatar_group(
            attrs: attributes! { class="grayscale" },
            #[key(alt)]
            for (src, alt, initials) in PEOPLE {
                avatar(
                    avatar_image(src: src, alt: alt)
                    avatar_fallback((initials))
                )
            }
        )
    })
}

#[component]
pub async fn avatar_group_count() -> Result<impl View> {
    Ok(view! {
        ui::avatar_group(
            attrs: attributes! { class="grayscale" },
            #[key(alt)]
            for (src, alt, initials) in PEOPLE {
                avatar(
                    avatar_image(src: src, alt: alt)
                    avatar_fallback((initials))
                )
            }
            ui::avatar_group_count("+3")
        )
    })
}

#[component]
pub async fn avatar_group_with_icon() -> Result<impl View> {
    Ok(view! {
        ui::avatar_group(
            attrs: attributes! { class="grayscale" },
            #[key(alt)]
            for (src, alt, initials) in PEOPLE {
                avatar(
                    avatar_image(src: src, alt: alt)
                    avatar_fallback((initials))
                )
            }
            ui::avatar_group_count(icon(data: iconify_icon!("lucide:plus")))
        )
    })
}

// A stand-in for the `dropdown-menu` component: a native popover anchored
// under its trigger, using that component's classes.
const MENU: StaticClass = class!(
    "z-50 min-w-[8rem] overflow-hidden rounded-base border-2 border-border bg-background p-1 \
     font-base text-foreground outline-none inset-auto m-0 mt-1 [position-area:bottom]",
);
const MENU_ITEM: StaticClass = class!(
    "relative flex w-full cursor-default select-none items-center gap-2 rounded-base border-2 \
     border-transparent px-2 py-1.5 text-sm font-base outline-hidden transition-colors \
     hover:bg-main hover:text-main-foreground hover:border-border focus:bg-main \
     focus:text-main-foreground focus:border-border",
);
const MENU_SEPARATOR: StaticClass = class!("-mx-1 my-1 h-0.5 bg-border");

#[component]
pub async fn dropdown() -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            popovertarget="avatar-dropdown-menu"
            style="anchor-name: --avatar-dropdown"
            class=(class!(
                button_variants(ButtonVariant::NoShadow, ButtonSize::Icon),
                "rounded-full",
            ))
        >
            avatar(
                avatar_image(src: "https://github.com/shadcn.png", alt: "shadcn")
                avatar_fallback("CN")
            )
        </button>
        <div
            id="avatar-dropdown-menu"
            popover=""
            role="menu"
            style="position-anchor: --avatar-dropdown"
            class=(class!(MENU, "w-32"))
        >
            <div role="group">
                for item in ["Profile", "Billing", "Settings"] {
                    <button
                        type="button"
                        role="menuitem"
                        popovertarget="avatar-dropdown-menu"
                        popovertargetaction="hide"
                        class=(MENU_ITEM)
                    >
                        (item)
                    </button>
                }
            </div>
            <div role="separator" class=(MENU_SEPARATOR)></div>
            <div role="group">
                <button
                    type="button"
                    role="menuitem"
                    popovertarget="avatar-dropdown-menu"
                    popovertargetaction="hide"
                    class=(MENU_ITEM)
                >
                    "Log out"
                </button>
            </div>
        </div>
    })
}
