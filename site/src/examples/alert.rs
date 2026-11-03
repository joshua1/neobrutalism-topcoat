use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

use crate::components::{
    alert::{AlertVariant, alert, alert_description, alert_title},
    button::{ButtonSize, ButtonVariant, button},
};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        alert(
            icon(data: iconify_icon!("lucide:circle-check"))
            alert_title("Success! Your changes have been saved")
            alert_description("This is an alert with icon, title and description.")
        )
    })
}

#[component]
pub async fn description_only() -> Result<impl View> {
    Ok(view! {
        alert(
            alert_description("This one has a description only. No title. No icon.")
        )
    })
}

#[component]
pub async fn destructive() -> Result<impl View> {
    Ok(view! {
        alert(
            variant: AlertVariant::Destructive,
            icon(data: iconify_icon!("lucide:circle-alert"))
            alert_title("Something went wrong!")
            alert_description("Your session has expired. Please log in again.")
        )
    })
}

#[component]
pub async fn icon_description() -> Result<impl View> {
    Ok(view! {
        alert(
            icon(data: iconify_icon!("lucide:bookmark-check"))
            alert_description("This one has an icon and a description only. No title.")
        )
    })
}

#[component]
pub async fn icon_title() -> Result<impl View> {
    Ok(view! {
        alert(
            icon(data: iconify_icon!("lucide:popcorn"))
            alert_title("Let's try one with icon and title.")
        )
    })
}

#[component]
pub async fn long_description() -> Result<impl View> {
    Ok(view! {
        alert(
            icon(data: iconify_icon!("lucide:gift"))
            alert_description(
                "This is a very long alert description that demonstrates how the component \
                 handles extended text content and potentially wraps across multiple lines"
            )
        )
    })
}

#[component]
pub async fn long_title_and_description() -> Result<impl View> {
    Ok(view! {
        alert(
            icon(data: iconify_icon!("lucide:circle-alert"))
            alert_title(
                "This is an extremely long alert title that spans multiple lines to \
                 demonstrate how the component handles very lengthy headings while \
                 maintaining readability and proper text wrapping behavior"
            )
            alert_description(
                "This is an equally long description that contains detailed information \
                 about the alert. It shows how the component can accommodate extensive \
                 content while preserving proper spacing, alignment, and readability across \
                 different screen sizes and viewport widths. This helps ensure the user \
                 experience remains consistent regardless of the content length."
            )
        )
    })
}

#[component]
pub async fn long_title() -> Result<impl View> {
    Ok(view! {
        alert(
            icon(data: iconify_icon!("lucide:shield-alert"))
            alert_title(
                "This is a very long alert title that demonstrates how the component \
                 handles extended text content and potentially wraps across multiple lines"
            )
        )
    })
}

#[component]
pub async fn with_button() -> Result<impl View> {
    // `!` makes the overrides beat the button's own height and colors: unlike
    // Octane's `cn`, `class!` does not drop conflicting classes.
    Ok(view! {
        alert(
            attrs: attributes! { class="sm:min-w-[500px]" },
            icon(data: iconify_icon!("lucide:circle-check"))
            alert_title(
                attrs: attributes! { class="max-w-[calc(100%-4rem)] overflow-ellipsis" },
                "The selected emails have been marked as spam."
            )
            button(
                size: ButtonSize::Sm,
                variant: ButtonVariant::NoShadow,
                attrs: attributes! {
                    class="absolute top-2.5 right-3 h-6! bg-secondary-background! text-foreground!"
                },
                "Undo"
            )
        )
    })
}
