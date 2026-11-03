use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, class, component, view},
};

use crate::components::button::{ButtonSize, ButtonVariant, button, button_variants};
use crate::components::button_group::{ButtonGroupOrientation, button_group, button_group_text};
use crate::components::dropdown_menu::{
    DropdownMenuAlign, dropdown_menu, dropdown_menu_content, dropdown_menu_item,
    dropdown_menu_trigger,
};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        button_group(
            button(variant: ButtonVariant::Neutral, "Archive")
            button(variant: ButtonVariant::Neutral, "Report")
            button(variant: ButtonVariant::Neutral, "Snooze")
        )
    })
}

#[component]
pub async fn vertical() -> Result<impl View> {
    Ok(view! {
        button_group(
            orientation: ButtonGroupOrientation::Vertical,
            attrs: attributes! { aria-label="Zoom controls" },
            button(
                variant: ButtonVariant::Neutral,
                size: ButtonSize::Icon,
                attrs: attributes! { aria-label="Zoom in" },
                icon(data: iconify_icon!("lucide:plus"))
            )
            button(
                variant: ButtonVariant::Neutral,
                size: ButtonSize::Icon,
                attrs: attributes! { aria-label="Zoom out" },
                icon(data: iconify_icon!("lucide:minus"))
            )
        )
    })
}

/// The dropdown's `<details>` is the group's child, so the group's joining
/// classes land on it rather than on the trigger button inside. The trigger
/// joins the group with its own classes instead.
#[component]
pub async fn with_dropdown() -> Result<impl View> {
    Ok(view! {
        button_group(
            button(variant: ButtonVariant::Neutral, "Update")
            dropdown_menu(
                dropdown_menu_trigger(
                    attrs: attributes! {
                        // The `!` versions override the neutral button's shadow
                        // and hover shift, as the group does for its direct
                        // children.
                        class=(class!(
                            button_variants(ButtonVariant::Neutral, ButtonSize::Icon),
                            "rounded-l-none border-l-0 shadow-none! hover:translate-x-0! \
                             hover:translate-y-0!",
                        ))
                        aria-label="More options"
                    },
                    icon(data: iconify_icon!("lucide:chevron-down"))
                )
                dropdown_menu_content(align: DropdownMenuAlign::End,
                    dropdown_menu_item("Disable")
                    dropdown_menu_item("Uninstall")
                )
            )
        )
    })
}

/// Octane's `Input` belongs to another component, so a plain `<input>` with
/// the same classes stands in for it.
#[component]
pub async fn with_input() -> Result<impl View> {
    Ok(view! {
        button_group(attrs: attributes! { class="w-full max-w-sm" },
            <input
                data-slot="input"
                placeholder="Search..."
                class="flex h-10 w-full rounded-base border-2 border-border \
                    bg-secondary-background selection:bg-main selection:text-main-foreground \
                    px-3 py-2 text-sm font-base text-foreground placeholder:text-foreground/50 \
                    focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-black \
                    focus-visible:ring-offset-2"
            >
            button(
                variant: ButtonVariant::Neutral,
                size: ButtonSize::Icon,
                attrs: attributes! { aria-label="Search" },
                icon(data: iconify_icon!("lucide:search"))
            )
        )
    })
}

#[component]
pub async fn with_text() -> Result<impl View> {
    Ok(view! {
        button_group(
            button_group_text("Text")
            button(variant: ButtonVariant::Neutral, "Another Button")
        )
    })
}
