//! The Octane examples use the `avatar` component. These render the same
//! markup and classes with plain HTML (the `fallback` helper below), so the
//! examples depend only on `message`, `bubble` and `button`.

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

use crate::components::{
    bubble::{BubbleVariant, bubble, bubble_content},
    button::{ButtonSize, ButtonVariant, button},
    message::{
        MessageAlign, message, message_avatar, message_content, message_footer, message_group,
        message_header,
    },
};

/// An avatar showing initials, styled like `avatar` + `avatar_fallback`.
#[component]
async fn fallback(initials: &str) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="avatar"
            class="group/avatar relative flex size-10 shrink-0 rounded-full outline-2 outline-border"
        >
            <span
                data-slot="avatar-fallback"
                class="flex size-full items-center justify-center rounded-full \
                    bg-secondary-background text-foreground font-base"
            >
                (initials)
            </span>
        </span>
    })
}

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        <div class="flex w-full max-w-md flex-col gap-4">
            message(
                message_avatar(
                    <span
                        data-slot="avatar"
                        class="group/avatar relative flex size-10 shrink-0 rounded-full \
                            outline-2 outline-border"
                    >
                        <img
                            data-slot="avatar-image"
                            class="aspect-square size-full rounded-full"
                            src="https://github.com/ekmas.png"
                            alt="@ekmas"
                        >
                    </span>
                )
                message_content(
                    bubble(variant: BubbleVariant::Muted,
                        bubble_content(
                            "I checked the latest deployment and the build failed while \
                             installing dependencies. Can you share the error from the logs?"
                        )
                    )
                )
            )
            message(align: MessageAlign::End,
                message_avatar(fallback(initials: "ME"))
                message_content(
                    bubble(
                        bubble_content(
                            "Sure, sending it over now. It looks like a peer dependency issue."
                        )
                    )
                )
            )
        </div>
    })
}

#[component]
pub async fn group() -> Result<impl View> {
    Ok(view! {
        <div class="flex w-full max-w-md flex-col gap-6">
            message_group(
                message(
                    message_avatar(fallback(initials: "BF"))
                    message_content(
                        bubble(variant: BubbleVariant::Muted,
                            bubble_content("I checked the registry addresses.")
                        )
                    )
                )
                message(
                    message_avatar(fallback(initials: "EK"))
                    message_content(
                        bubble(variant: BubbleVariant::Muted,
                            bubble_content(
                                "The component and example JSON now live under the UI registry."
                            )
                        )
                    )
                )
            )
            message_group(
                message(align: MessageAlign::End,
                    message_content(bubble(bubble_content("Nice, that is what we wanted.")))
                )
                message(align: MessageAlign::End,
                    message_content(bubble(bubble_content("Ship it.")))
                )
            )
        </div>
    })
}

#[component]
pub async fn header_footer() -> Result<impl View> {
    Ok(view! {
        <div class="flex w-full max-w-md flex-col gap-6">
            message(
                message_avatar(fallback(initials: "EK"))
                message_content(
                    message_header("Ekmas")
                    bubble(variant: BubbleVariant::Muted,
                        bubble_content(
                            "The header shows who sent the message and the footer shows when."
                        )
                    )
                    message_footer("10:24 AM")
                )
            )
            message(align: MessageAlign::End,
                message_avatar(fallback(initials: "ME"))
                message_content(
                    message_header("You")
                    bubble(bubble_content("Got it, thanks for the explanation."))
                    message_footer("10:26 AM")
                )
            )
        </div>
    })
}

#[component]
pub async fn with_actions() -> Result<impl View> {
    let actions = [
        ("Copy", iconify_icon!("lucide:copy")),
        ("Regenerate", iconify_icon!("lucide:refresh-cw")),
        ("Like", iconify_icon!("lucide:thumbs-up")),
    ];
    Ok(view! {
        <div class="flex w-full max-w-md flex-col gap-4">
            message(
                message_avatar(fallback(initials: "AI"))
                message_content(
                    bubble(variant: BubbleVariant::Muted,
                        bubble_content(
                            "Here is a summary of the changes: the registry was rebuilt, two \
                             stale routes were removed and the docs were updated."
                        )
                    )
                    <div class="flex items-center gap-1">
                        #[key(label)]
                        for (label, data) in actions {
                            button(
                                variant: ButtonVariant::Neutral,
                                size: ButtonSize::Icon,
                                attrs: attributes! {
                                    class="size-8! shadow-none! hover:translate-x-0! hover:translate-y-0!"
                                    aria-label=(label)
                                },
                                icon(data: data)
                            )
                        }
                    </div>
                )
            )
        </div>
    })
}
