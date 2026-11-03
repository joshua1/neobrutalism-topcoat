use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

use crate::components::marker::{MarkerVariant, marker, marker_content, marker_icon};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        <div class="w-full max-w-sm">
            marker(
                marker_icon(icon(data: iconify_icon!("lucide:globe")))
                marker_content("Searched the web")
            )
        </div>
    })
}

#[component]
pub async fn separator() -> Result<impl View> {
    Ok(view! {
        <div class="w-full max-w-sm">
            marker(variant: MarkerVariant::Separator, marker_content("Yesterday"))
        </div>
    })
}

/// The Octane example uses `Spinner`; this inlines the same spinning
/// `loader-circle` icon so the example doesn't depend on that component.
#[component]
pub async fn with_spinner() -> Result<impl View> {
    Ok(view! {
        <div class="w-full max-w-sm">
            marker(attrs: attributes! { role="status" },
                marker_icon(
                    icon(
                        data: iconify_icon!("lucide:loader-circle"),
                        attrs: attributes! {
                            data-slot="spinner"
                            aria-label="Loading"
                            class="size-4 animate-spin"
                        }
                    )
                )
                marker_content("Thinking...")
            )
        </div>
    })
}
