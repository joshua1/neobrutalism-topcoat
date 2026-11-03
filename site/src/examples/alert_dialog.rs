use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

use crate::components::alert_dialog::{
    AlertDialogSize, alert_dialog, alert_dialog_action, alert_dialog_cancel, alert_dialog_content,
    alert_dialog_description, alert_dialog_footer, alert_dialog_header, alert_dialog_media,
    alert_dialog_title, alert_dialog_trigger,
};
use crate::components::button::{ButtonSize, ButtonVariant, button_variants};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        alert_dialog_trigger(
            target: "alert-dialog-demo",
            attrs: attributes! {
                class=(button_variants(ButtonVariant::Default, ButtonSize::Default))
            },
            "Open"
        )
        alert_dialog(
            id: "alert-dialog-demo",
            attrs: attributes! {
                aria-labelledby="alert-dialog-demo-title"
                aria-describedby="alert-dialog-demo-description"
            },
            alert_dialog_content(
                alert_dialog_header(
                    alert_dialog_title(
                        attrs: attributes! { id="alert-dialog-demo-title" },
                        "Are you absolutely sure?"
                    )
                    alert_dialog_description(
                        attrs: attributes! { id="alert-dialog-demo-description" },
                        "This action cannot be undone. This will permanently delete your account \
                         and remove your data from our servers."
                    )
                )
                alert_dialog_footer(
                    alert_dialog_cancel("Cancel")
                    alert_dialog_action("Continue")
                )
            )
        )
    })
}

#[component]
pub async fn basic() -> Result<impl View> {
    Ok(view! {
        alert_dialog_trigger(
            target: "alert-dialog-basic",
            attrs: attributes! {
                class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))
            },
            "Show Dialog"
        )
        alert_dialog(
            id: "alert-dialog-basic",
            attrs: attributes! { aria-labelledby="alert-dialog-basic-title" },
            alert_dialog_content(
                alert_dialog_header(
                    alert_dialog_title(
                        attrs: attributes! { id="alert-dialog-basic-title" },
                        "Are you absolutely sure?"
                    )
                    alert_dialog_description(
                        "This action cannot be undone. This will permanently delete your account \
                         and remove your data from our servers."
                    )
                )
                alert_dialog_footer(
                    alert_dialog_cancel("Cancel")
                    alert_dialog_action("Continue")
                )
            )
        )
    })
}

#[component]
pub async fn destructive() -> Result<impl View> {
    Ok(view! {
        alert_dialog_trigger(
            target: "alert-dialog-destructive",
            attrs: attributes! {
                class=(button_variants(ButtonVariant::Default, ButtonSize::Default))
            },
            "Delete Chat"
        )
        alert_dialog(
            id: "alert-dialog-destructive",
            attrs: attributes! { aria-labelledby="alert-dialog-destructive-title" },
            alert_dialog_content(size: AlertDialogSize::Sm,
                alert_dialog_header(
                    // `!` overrides the media box's `bg-main text-main-foreground`.
                    alert_dialog_media(attrs: attributes! { class="bg-black! text-white!" },
                        icon(data: iconify_icon!("lucide:trash-2"))
                    )
                    alert_dialog_title(
                        attrs: attributes! { id="alert-dialog-destructive-title" },
                        "Delete chat?"
                    )
                    alert_dialog_description(
                        "This will permanently delete this chat conversation. View "
                        <a href="#" class="underline underline-offset-4">"Settings"</a>
                        " delete any memories saved during this chat."
                    )
                )
                alert_dialog_footer(
                    alert_dialog_cancel("Cancel")
                    alert_dialog_action("Delete")
                )
            )
        )
    })
}

#[component]
pub async fn media() -> Result<impl View> {
    Ok(view! {
        alert_dialog_trigger(
            target: "alert-dialog-media",
            attrs: attributes! {
                class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))
            },
            "Share Project"
        )
        alert_dialog(
            id: "alert-dialog-media",
            attrs: attributes! { aria-labelledby="alert-dialog-media-title" },
            alert_dialog_content(
                alert_dialog_header(
                    alert_dialog_media(icon(data: iconify_icon!("lucide:circle-fading-plus")))
                    alert_dialog_title(
                        attrs: attributes! { id="alert-dialog-media-title" },
                        "Share this project?"
                    )
                    alert_dialog_description(
                        "Anyone with the link will be able to view and edit this project."
                    )
                )
                alert_dialog_footer(
                    alert_dialog_cancel("Cancel")
                    alert_dialog_action("Share")
                )
            )
        )
    })
}

#[component]
pub async fn small_with_media() -> Result<impl View> {
    Ok(view! {
        alert_dialog_trigger(
            target: "alert-dialog-small-with-media",
            attrs: attributes! {
                class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))
            },
            "Show Dialog"
        )
        alert_dialog(
            id: "alert-dialog-small-with-media",
            attrs: attributes! { aria-labelledby="alert-dialog-small-with-media-title" },
            alert_dialog_content(size: AlertDialogSize::Sm,
                alert_dialog_header(
                    alert_dialog_media(icon(data: iconify_icon!("lucide:bluetooth")))
                    alert_dialog_title(
                        attrs: attributes! { id="alert-dialog-small-with-media-title" },
                        "Allow accessory to connect?"
                    )
                    alert_dialog_description(
                        "Do you want to allow the USB accessory to connect to this device?"
                    )
                )
                alert_dialog_footer(
                    alert_dialog_cancel(size: ButtonSize::Sm, "Don't allow")
                    alert_dialog_action(size: ButtonSize::Sm, "Allow")
                )
            )
        )
    })
}

#[component]
pub async fn small() -> Result<impl View> {
    Ok(view! {
        alert_dialog_trigger(
            target: "alert-dialog-small",
            attrs: attributes! {
                class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))
            },
            "Show Dialog"
        )
        alert_dialog(
            id: "alert-dialog-small",
            attrs: attributes! { aria-labelledby="alert-dialog-small-title" },
            alert_dialog_content(size: AlertDialogSize::Sm,
                alert_dialog_header(
                    alert_dialog_title(
                        attrs: attributes! { id="alert-dialog-small-title" },
                        "Allow accessory to connect?"
                    )
                    alert_dialog_description(
                        "Do you want to allow the USB accessory to connect to this device?"
                    )
                )
                alert_dialog_footer(
                    alert_dialog_cancel(size: ButtonSize::Sm, "Don't allow")
                    alert_dialog_action(size: ButtonSize::Sm, "Allow")
                )
            )
        )
    })
}
