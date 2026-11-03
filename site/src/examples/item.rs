use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, class, component, view},
};

use crate::components::{
    button::{ButtonSize, ButtonVariant, button},
    item::{
        ItemMediaVariant, ItemSize, item, item_actions, item_content, item_description,
        item_group, item_media, item_title, item_variants,
    },
};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        item(attrs: attributes! { class="max-w-md" },
            item_media(variant: ItemMediaVariant::Icon,
                icon(data: iconify_icon!("lucide:badge-check"))
            )
            item_content(
                item_title("Basic Item")
                item_description("A simple item with title and description.")
            )
            item_actions(
                button(variant: ButtonVariant::NoShadow, size: ButtonSize::Sm, "Action")
            )
        )
    })
}

#[component]
pub async fn as_link() -> Result<impl View> {
    Ok(view! {
        <a
            href="#"
            data-slot="item"
            data-size=(ItemSize::Default.as_str())
            class=(class!(item_variants(ItemSize::Default), "max-w-md"))
        >
            item_media(variant: ItemMediaVariant::Icon,
                icon(data: iconify_icon!("lucide:external-link"))
            )
            item_content(
                item_title("Visit our documentation")
                item_description("Learn how to get started with our components.")
            )
            item_actions(
                icon(
                    data: iconify_icon!("lucide:chevron-right"),
                    attrs: attributes! { class="size-4" }
                )
            )
        </a>
    })
}

#[component]
pub async fn group() -> Result<impl View> {
    Ok(view! {
        item_group(attrs: attributes! { class="max-w-md" },
            item(
                item_media(variant: ItemMediaVariant::Icon, icon(data: iconify_icon!("lucide:inbox")))
                item_content(
                    item_title("Inbox")
                    item_description("View all incoming messages.")
                )
            )
            item(
                item_media(variant: ItemMediaVariant::Icon, icon(data: iconify_icon!("lucide:send")))
                item_content(
                    item_title("Sent")
                    item_description("View all sent messages.")
                )
            )
            item(
                item_media(variant: ItemMediaVariant::Icon, icon(data: iconify_icon!("lucide:archive")))
                item_content(
                    item_title("Archive")
                    item_description("View archived messages.")
                )
            )
        )
    })
}

#[component]
pub async fn with_image() -> Result<impl View> {
    Ok(view! {
        item(attrs: attributes! { class="max-w-md" },
            item_media(variant: ItemMediaVariant::Image,
                <img
                    src="https://avatars.githubusercontent.com/u/124599?v=4"
                    alt="shadcn"
                    width="40"
                    height="40"
                >
            )
            item_content(
                item_title("shadcn")
                item_description("Building things for the web.")
            )
            item_actions(
                button(variant: ButtonVariant::NoShadow, size: ButtonSize::Sm, "Follow")
            )
        )
    })
}
