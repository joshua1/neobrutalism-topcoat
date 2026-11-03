use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

use crate::components::attachment::{
    AttachmentMediaVariant, AttachmentOrientation, AttachmentState, attachment,
    attachment_action, attachment_actions, attachment_content, attachment_description,
    attachment_media, attachment_title,
};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        attachment(attrs: attributes! { class="w-full max-w-xs" },
            attachment_media(icon(data: iconify_icon!("lucide:file-text")))
            attachment_content(
                attachment_title("sales-dashboard.pdf")
                attachment_description("PDF · 2.4 MB")
            )
            attachment_actions(
                attachment_action(attrs: attributes! { aria-label="Remove" },
                    icon(data: iconify_icon!("lucide:x"))
                )
            )
        )
    })
}

#[component]
pub async fn image() -> Result<impl View> {
    Ok(view! {
        attachment(attrs: attributes! { class="w-full max-w-xs" },
            attachment_media(variant: AttachmentMediaVariant::Image,
                <img
                    src="https://images.unsplash.com/photo-1518791841217-8f162f1e1131?w=200&q=80"
                    alt="A cat"
                >
            )
            attachment_content(
                attachment_title("cat.jpg")
                attachment_description("JPG · 340 KB")
            )
            attachment_actions(
                attachment_action(attrs: attributes! { aria-label="Remove" },
                    icon(data: iconify_icon!("lucide:x"))
                )
            )
        )
    })
}

/// The Octane example uses `Spinner` for the uploading state; this inlines the
/// same spinning `loader-circle` icon so the example doesn't depend on it.
#[component]
pub async fn states() -> Result<impl View> {
    Ok(view! {
        <div class="flex w-full max-w-xs flex-col gap-4">
            attachment(state: AttachmentState::Idle, attrs: attributes! { class="w-full" },
                attachment_media(icon(data: iconify_icon!("lucide:upload")))
                attachment_content(
                    attachment_title("Drop a file")
                    attachment_description("Waiting for a file")
                )
            )
            attachment(state: AttachmentState::Uploading, attrs: attributes! { class="w-full" },
                attachment_media(
                    icon(
                        data: iconify_icon!("lucide:loader-circle"),
                        attrs: attributes! {
                            data-slot="spinner"
                            role="status"
                            aria-label="Loading"
                            class="size-4 animate-spin"
                        }
                    )
                )
                attachment_content(
                    attachment_title("quarterly-report.pdf")
                    attachment_description("Uploading · 45%")
                )
                attachment_actions(
                    attachment_action(attrs: attributes! { aria-label="Cancel" },
                        icon(data: iconify_icon!("lucide:x"))
                    )
                )
            )
            attachment(state: AttachmentState::Error, attrs: attributes! { class="w-full" },
                attachment_media(icon(data: iconify_icon!("lucide:circle-alert")))
                attachment_content(
                    attachment_title("large-video.mp4")
                    attachment_description("File is too large")
                )
                attachment_actions(
                    attachment_action(attrs: attributes! { aria-label="Remove" },
                        icon(data: iconify_icon!("lucide:x"))
                    )
                )
            )
            attachment(state: AttachmentState::Done, attrs: attributes! { class="w-full" },
                attachment_media(icon(data: iconify_icon!("lucide:file-text")))
                attachment_content(
                    attachment_title("notes.md")
                    attachment_description("Markdown · 12 KB")
                )
                attachment_actions(
                    attachment_action(attrs: attributes! { aria-label="Remove" },
                        icon(data: iconify_icon!("lucide:x"))
                    )
                )
            )
        </div>
    })
}

#[component]
pub async fn vertical() -> Result<impl View> {
    Ok(view! {
        <div class="flex gap-4">
            attachment(orientation: AttachmentOrientation::Vertical,
                attachment_media(variant: AttachmentMediaVariant::Image,
                    <img
                        src="https://images.unsplash.com/photo-1518791841217-8f162f1e1131?w=200&q=80"
                        alt="A cat"
                    >
                )
                attachment_content(
                    attachment_title("cat.jpg")
                    attachment_description("340 KB")
                )
            )
            attachment(orientation: AttachmentOrientation::Vertical,
                attachment_media(icon(data: iconify_icon!("lucide:file-text")))
                attachment_content(
                    attachment_title("report.pdf")
                    attachment_description("2.4 MB")
                )
            )
        </div>
    })
}
