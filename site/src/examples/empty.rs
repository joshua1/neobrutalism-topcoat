use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

use crate::components::{
    button::{ButtonSize, ButtonVariant, button},
    empty::{
        EmptyMediaVariant, empty, empty_content, empty_description, empty_header, empty_media,
        empty_title,
    },
};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        empty(
            attrs: attributes! { class="max-w-md" },
            empty_header(
                empty_media(
                    variant: EmptyMediaVariant::Icon,
                    icon(data: iconify_icon!("lucide:folder"))
                )
                empty_title("No projects yet")
                empty_description(
                    "You haven't created any projects yet. Get started by creating your \
                     first project."
                )
            )
            empty_content(
                button(size: ButtonSize::Sm,
                    icon(data: iconify_icon!("lucide:plus"))
                    "Create project"
                )
            )
        )
    })
}

#[component]
pub async fn with_input() -> Result<impl View> {
    // The search field is a native `<input>` with the `input` component's
    // classes, standing in for that component.
    Ok(view! {
        empty(
            attrs: attributes! { class="max-w-md" },
            empty_header(
                empty_title("404 - Not Found")
                empty_description(
                    "The page you're looking for doesn't exist. Try searching for what you \
                     need below."
                )
            )
            empty_content(
                <div class="flex w-full items-center gap-2">
                    <input
                        placeholder="Try searching for pages..."
                        aria-label="Search pages"
                        class="flex h-10 w-full rounded-base border-2 border-border \
                               bg-secondary-background selection:bg-main \
                               selection:text-main-foreground px-3 py-2 text-sm font-base \
                               text-foreground placeholder:text-foreground/50 \
                               focus-visible:outline-hidden focus-visible:ring-2 \
                               focus-visible:ring-black focus-visible:ring-offset-2"
                    >
                    button(variant: ButtonVariant::NoShadow, size: ButtonSize::Sm, "Search")
                </div>
                empty_description("Need help? " <a href="#">"Contact support"</a>)
            )
        )
    })
}
