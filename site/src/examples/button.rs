use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

use crate::components::button::{ButtonSize, ButtonVariant, button};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! { button("Button") })
}

#[component]
pub async fn neutral() -> Result<impl View> {
    Ok(view! { button(variant: ButtonVariant::Neutral, "Button") })
}

#[component]
pub async fn no_shadow() -> Result<impl View> {
    Ok(view! { button(variant: ButtonVariant::NoShadow, "Button") })
}

#[component]
pub async fn reverse() -> Result<impl View> {
    Ok(view! { button(variant: ButtonVariant::Reverse, "Button") })
}

#[component]
pub async fn icon_button() -> Result<impl View> {
    Ok(view! {
        button(size: ButtonSize::Icon, attrs: attributes! { aria-label="More" },
            icon(data: iconify_icon!("lucide:ellipsis"))
        )
    })
}

#[component]
pub async fn with_icon() -> Result<impl View> {
    Ok(view! {
        button(icon(data: iconify_icon!("lucide:send")) " Send")
    })
}

#[component]
pub async fn size() -> Result<impl View> {
    Ok(view! {
        <div class="flex flex-col items-start gap-4">
            <div class="flex items-center gap-2">
                button(size: ButtonSize::Xs, "Extra Small")
                button(size: ButtonSize::IconXs, attrs: attributes! { aria-label="Submit" },
                    icon(data: iconify_icon!("lucide:arrow-up-right"))
                )
            </div>
            <div class="flex items-center gap-2">
                button(size: ButtonSize::Sm, "Small")
                button(size: ButtonSize::IconSm, attrs: attributes! { aria-label="Submit" },
                    icon(data: iconify_icon!("lucide:arrow-up-right"))
                )
            </div>
            <div class="flex items-center gap-2">
                button("Default")
                button(size: ButtonSize::Icon, attrs: attributes! { aria-label="Submit" },
                    icon(data: iconify_icon!("lucide:arrow-up-right"))
                )
            </div>
            <div class="flex items-center gap-2">
                button(size: ButtonSize::Lg, "Large")
                button(size: ButtonSize::IconLg, attrs: attributes! { aria-label="Submit" },
                    icon(data: iconify_icon!("lucide:arrow-up-right"))
                )
            </div>
        </div>
    })
}
