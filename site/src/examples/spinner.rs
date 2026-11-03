use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use crate::components::{
    badge::{BadgeVariant, badge},
    button::{ButtonVariant, button},
    spinner::spinner,
};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! { spinner() })
}

#[component]
pub async fn size() -> Result<impl View> {
    Ok(view! {
        <div class="flex items-center gap-6">
            spinner(attrs: attributes! { class="size-4" })
            // `!` beats the spinner's own `size-4` (class! doesn't merge).
            spinner(attrs: attributes! { class="size-6!" })
            spinner(attrs: attributes! { class="size-8!" })
        </div>
    })
}

#[component]
pub async fn in_button() -> Result<impl View> {
    Ok(view! {
        <div class="flex flex-wrap items-center gap-4">
            button(attrs: attributes! { disabled="" }, spinner() "Please wait")
            button(
                variant: ButtonVariant::Neutral,
                attrs: attributes! { disabled="" },
                spinner()
                "Please wait"
            )
        </div>
    })
}

#[component]
pub async fn with_badge() -> Result<impl View> {
    Ok(view! {
        <div class="flex flex-wrap items-center gap-4">
            badge(spinner() "Syncing")
            badge(variant: BadgeVariant::Neutral, spinner() "Syncing")
        </div>
    })
}
