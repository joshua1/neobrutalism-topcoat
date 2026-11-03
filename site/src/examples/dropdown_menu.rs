use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

use crate::components::button::{ButtonSize, ButtonVariant, button_variants};
use crate::components::dropdown_menu::{
    DropdownMenuAlign, dropdown_menu, dropdown_menu_checkbox_item, dropdown_menu_content,
    dropdown_menu_group, dropdown_menu_item, dropdown_menu_label, dropdown_menu_radio_group,
    dropdown_menu_radio_item, dropdown_menu_separator, dropdown_menu_shortcut, dropdown_menu_sub,
    dropdown_menu_sub_content, dropdown_menu_sub_trigger, dropdown_menu_trigger,
};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        dropdown_menu(
            dropdown_menu_trigger(
                attrs: attributes! {
                    class=(button_variants(ButtonVariant::NoShadow, ButtonSize::Default))
                },
                "Open"
            )
            dropdown_menu_content(attrs: attributes! { class="w-56" },
                dropdown_menu_label("My Account")
                dropdown_menu_separator()
                dropdown_menu_group(
                    dropdown_menu_item(
                        icon(data: iconify_icon!("lucide:user"))
                        <span>"Profile"</span>
                        dropdown_menu_shortcut("⇧⌘P")
                    )
                    dropdown_menu_item(
                        icon(data: iconify_icon!("lucide:credit-card"))
                        <span>"Billing"</span>
                        dropdown_menu_shortcut("⌘B")
                    )
                    dropdown_menu_item(
                        icon(data: iconify_icon!("lucide:settings"))
                        <span>"Settings"</span>
                        dropdown_menu_shortcut("⌘S")
                    )
                    dropdown_menu_item(
                        icon(data: iconify_icon!("lucide:keyboard"))
                        <span>"Keyboard shortcuts"</span>
                        dropdown_menu_shortcut("⌘K")
                    )
                )
                dropdown_menu_separator()
                dropdown_menu_group(
                    dropdown_menu_item(
                        icon(data: iconify_icon!("lucide:users"))
                        <span>"Team"</span>
                    )
                    dropdown_menu_sub(
                        dropdown_menu_sub_trigger(
                            icon(data: iconify_icon!("lucide:user-plus"))
                            <span>"Invite users"</span>
                        )
                        dropdown_menu_sub_content(
                            dropdown_menu_item(
                                icon(data: iconify_icon!("lucide:mail"))
                                <span>"Email"</span>
                            )
                            dropdown_menu_item(
                                icon(data: iconify_icon!("lucide:message-square"))
                                <span>"Message"</span>
                            )
                            dropdown_menu_separator()
                            dropdown_menu_item(
                                icon(data: iconify_icon!("lucide:circle-plus"))
                                <span>"More..."</span>
                            )
                        )
                    )
                    dropdown_menu_item(
                        icon(data: iconify_icon!("lucide:plus"))
                        <span>"New Team"</span>
                        dropdown_menu_shortcut("⌘+T")
                    )
                )
                dropdown_menu_separator()
                dropdown_menu_item(
                    icon(data: iconify_icon!("lucide:git-branch"))
                    <span>"GitHub"</span>
                )
                dropdown_menu_item(
                    icon(data: iconify_icon!("lucide:life-buoy"))
                    <span>"Support"</span>
                )
                dropdown_menu_item(attrs: attributes! { disabled="" },
                    icon(data: iconify_icon!("lucide:cloud"))
                    <span>"API"</span>
                )
                dropdown_menu_separator()
                dropdown_menu_item(
                    icon(data: iconify_icon!("lucide:log-out"))
                    <span>"Log out"</span>
                    dropdown_menu_shortcut("⇧⌘Q")
                )
            )
        )
    })
}

/// The Octane version keeps each checkbox in `useState`; here each native
/// checkbox holds its own state.
#[component]
pub async fn checkboxes() -> Result<impl View> {
    Ok(view! {
        dropdown_menu(
            dropdown_menu_trigger(
                attrs: attributes! {
                    class=(button_variants(ButtonVariant::NoShadow, ButtonSize::Default))
                },
                "Checkboxes"
            )
            dropdown_menu_content(
                align: DropdownMenuAlign::Start,
                attrs: attributes! { class="w-56" },
                dropdown_menu_group(
                    dropdown_menu_label("Account")
                    dropdown_menu_item(icon(data: iconify_icon!("lucide:user")) " Profile")
                    dropdown_menu_item(icon(data: iconify_icon!("lucide:credit-card")) " Billing")
                    dropdown_menu_item(icon(data: iconify_icon!("lucide:settings-2")) " Settings")
                )
                dropdown_menu_separator()
                dropdown_menu_group(
                    dropdown_menu_label("Appearance")
                    dropdown_menu_checkbox_item(
                        attrs: attributes! { name="status_bar" checked="" },
                        "Status Bar"
                    )
                    dropdown_menu_checkbox_item(
                        attrs: attributes! { name="activity_bar" disabled="" },
                        "Activity Bar"
                    )
                    dropdown_menu_checkbox_item(attrs: attributes! { name="panel" }, "Panel")
                )
                dropdown_menu_separator()
                dropdown_menu_group(
                    dropdown_menu_item(icon(data: iconify_icon!("lucide:log-out")) " Sign Out")
                )
            )
        )
    })
}

/// The radio buttons share the `position` name, so checking one unchecks the
/// others without any state of our own.
#[component]
pub async fn radio() -> Result<impl View> {
    Ok(view! {
        dropdown_menu(
            dropdown_menu_trigger(
                attrs: attributes! {
                    class=(button_variants(ButtonVariant::NoShadow, ButtonSize::Default))
                },
                "Radio Group"
            )
            dropdown_menu_content(
                align: DropdownMenuAlign::Start,
                attrs: attributes! { class="w-56" },
                dropdown_menu_label(inset: true, "Panel Position")
                dropdown_menu_group(
                    dropdown_menu_radio_group(
                        dropdown_menu_radio_item(
                            attrs: attributes! { name="position" value="top" },
                            "Top"
                        )
                        dropdown_menu_radio_item(
                            attrs: attributes! { name="position" value="bottom" checked="" },
                            "Bottom"
                        )
                        dropdown_menu_radio_item(
                            attrs: attributes! { name="position" value="right" disabled="" },
                            "Right"
                        )
                    )
                )
            )
        )
    })
}
