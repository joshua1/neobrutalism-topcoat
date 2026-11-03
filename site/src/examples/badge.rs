use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, component, view},
};

use crate::components::badge::{BadgeVariant, badge};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! { badge("Badge") })
}

#[component]
pub async fn neutral() -> Result<impl View> {
    Ok(view! { badge(variant: BadgeVariant::Neutral, "Neutral") })
}

#[component]
pub async fn with_icon() -> Result<impl View> {
    Ok(view! {
        badge(icon(data: iconify_icon!("lucide:check")) "Badge")
    })
}
