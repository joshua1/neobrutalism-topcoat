use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

use crate::components::toggle::{ToggleSize, ToggleVariant, toggle};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        toggle(attrs: attributes! { aria-label="Toggle bold" checked="" },
            icon(data: iconify_icon!("lucide:bold"))
        )
    })
}

#[component]
pub async fn disabled() -> Result<impl View> {
    Ok(view! {
        toggle(attrs: attributes! { aria-label="Toggle underline" disabled="" },
            icon(data: iconify_icon!("lucide:underline"))
        )
    })
}

#[component]
pub async fn outline() -> Result<impl View> {
    Ok(view! {
        <div class="flex items-center gap-2">
            toggle(
                variant: ToggleVariant::Outline,
                attrs: attributes! { aria-label="Toggle bold" checked="" },
                icon(data: iconify_icon!("lucide:bold"))
                "Bold"
            )
            toggle(
                variant: ToggleVariant::Outline,
                attrs: attributes! { aria-label="Toggle italic" },
                icon(data: iconify_icon!("lucide:italic"))
                "Italic"
            )
        </div>
    })
}

#[component]
pub async fn size() -> Result<impl View> {
    Ok(view! {
        <div class="flex flex-wrap items-center gap-2">
            toggle(
                variant: ToggleVariant::Outline,
                size: ToggleSize::Sm,
                attrs: attributes! { aria-label="Toggle small" },
                "Small"
            )
            toggle(
                variant: ToggleVariant::Outline,
                attrs: attributes! { aria-label="Toggle default" },
                "Default"
            )
            toggle(
                variant: ToggleVariant::Outline,
                size: ToggleSize::Lg,
                attrs: attributes! { aria-label="Toggle large" },
                "Large"
            )
        </div>
    })
}

#[component]
pub async fn with_text() -> Result<impl View> {
    Ok(view! {
        toggle(attrs: attributes! { aria-label="Toggle italic" },
            icon(data: iconify_icon!("lucide:italic"))
            "Italic"
        )
    })
}
