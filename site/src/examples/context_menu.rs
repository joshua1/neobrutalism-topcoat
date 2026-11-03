use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

use crate::components::context_menu::{
    ContextMenuItemVariant, ContextMenuSide, context_menu, context_menu_checkbox_item,
    context_menu_content, context_menu_group, context_menu_item, context_menu_label,
    context_menu_radio_group, context_menu_radio_item, context_menu_separator,
    context_menu_shortcut, context_menu_sub, context_menu_sub_content, context_menu_sub_trigger,
    context_menu_trigger,
};

/// The dashed drop zone most examples right-click on.
const ZONE: &str = "flex aspect-video w-full max-w-xs items-center justify-center rounded-base \
                    border-2 border-dashed border-border text-sm font-base";

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        context_menu(
            context_menu_trigger(
                attrs: attributes! {
                    class="flex h-[150px] w-[300px] items-center justify-center rounded-base \
                        border border-border border-dashed text-sm font-base"
                },
                "Right click here"
            )
            context_menu_content(attrs: attributes! { class="w-64" },
                context_menu_item(inset: true, "Back" context_menu_shortcut("⌘["))
                context_menu_item(inset: true, attrs: attributes! { disabled="" },
                    "Forward"
                    context_menu_shortcut("⌘]")
                )
                context_menu_item(inset: true, "Reload" context_menu_shortcut("⌘R"))
                context_menu_sub(
                    context_menu_sub_trigger(inset: true, "More Tools")
                    context_menu_sub_content(attrs: attributes! { class="w-48" },
                        context_menu_item(inset: true,
                            "Save Page..."
                            context_menu_shortcut("⇧⌘S")
                        )
                        context_menu_item(
                            icon(data: iconify_icon!("lucide:plus"))
                            "Create Shortcut..."
                        )
                        context_menu_item(inset: true, "Name Window...")
                        context_menu_separator()
                        context_menu_item(
                            icon(data: iconify_icon!("lucide:code-xml"))
                            "Developer Tools"
                        )
                        context_menu_separator()
                        context_menu_item(
                            icon(data: iconify_icon!("lucide:trash"))
                            "Delete"
                        )
                    )
                )
                context_menu_separator()
                context_menu_checkbox_item(attrs: attributes! { checked="" },
                    "Show Bookmarks Bar"
                    context_menu_shortcut("⌘⇧B")
                )
                context_menu_checkbox_item("Show Full URLs")
                context_menu_separator()
                context_menu_radio_group(
                    context_menu_label(inset: true, "People")
                    context_menu_radio_item(
                        attrs: attributes! { name="person" value="pedro" checked="" },
                        "Pedro Duarte"
                    )
                    context_menu_radio_item(
                        attrs: attributes! { name="person" value="colm" },
                        "Colm Tuite"
                    )
                )
            )
        )
    })
}

/// The trigger text for pointer and touch devices, as in the Octane examples.
#[component]
async fn zone_text(#[into] fine: String, #[into] coarse: String) -> Result<impl View> {
    Ok(view! {
        <span class="hidden pointer-fine:inline-block">(fine)</span>
        <span class="hidden pointer-coarse:inline-block">(coarse)</span>
    })
}

#[component]
pub async fn basic() -> Result<impl View> {
    Ok(view! {
        context_menu(
            context_menu_trigger(attrs: attributes! { class=(ZONE) },
                zone_text(fine: "Right click here", coarse: "Long press here")
            )
            context_menu_content(
                context_menu_group(
                    context_menu_item("Back")
                    context_menu_item(attrs: attributes! { disabled="" }, "Forward")
                    context_menu_item("Reload")
                )
            )
        )
    })
}

#[component]
pub async fn checkboxes() -> Result<impl View> {
    Ok(view! {
        context_menu(
            context_menu_trigger(attrs: attributes! { class=(ZONE) },
                zone_text(fine: "Right click here", coarse: "Long press here")
            )
            context_menu_content(
                context_menu_group(
                    context_menu_checkbox_item(attrs: attributes! { checked="" },
                        "Show Bookmarks Bar"
                    )
                    context_menu_checkbox_item("Show Full URLs")
                    context_menu_checkbox_item(attrs: attributes! { checked="" },
                        "Show Developer Tools"
                    )
                )
            )
        )
    })
}

#[component]
pub async fn destructive() -> Result<impl View> {
    Ok(view! {
        context_menu(
            context_menu_trigger(attrs: attributes! { class=(ZONE) },
                zone_text(fine: "Right click here", coarse: "Long press here")
            )
            context_menu_content(
                context_menu_group(
                    context_menu_item(icon(data: iconify_icon!("lucide:pencil")) "Edit")
                    context_menu_item(icon(data: iconify_icon!("lucide:share")) "Share")
                )
                context_menu_separator()
                context_menu_group(
                    context_menu_item(variant: ContextMenuItemVariant::Destructive,
                        icon(data: iconify_icon!("lucide:trash"))
                        "Delete"
                    )
                )
            )
        )
    })
}

#[component]
pub async fn groups() -> Result<impl View> {
    Ok(view! {
        context_menu(
            context_menu_trigger(attrs: attributes! { class=(ZONE) },
                zone_text(fine: "Right click here", coarse: "Long press here")
            )
            context_menu_content(
                context_menu_group(
                    context_menu_label("File")
                    context_menu_item("New File" context_menu_shortcut("⌘N"))
                    context_menu_item("Open File" context_menu_shortcut("⌘O"))
                    context_menu_item("Save" context_menu_shortcut("⌘S"))
                )
                context_menu_separator()
                context_menu_group(
                    context_menu_label("Edit")
                    context_menu_item("Undo" context_menu_shortcut("⌘Z"))
                    context_menu_item("Redo" context_menu_shortcut("⇧⌘Z"))
                )
                context_menu_separator()
                context_menu_group(
                    context_menu_item("Cut" context_menu_shortcut("⌘X"))
                    context_menu_item("Copy" context_menu_shortcut("⌘C"))
                    context_menu_item("Paste" context_menu_shortcut("⌘V"))
                )
                context_menu_separator()
                context_menu_group(
                    context_menu_item(variant: ContextMenuItemVariant::Destructive,
                        "Delete"
                        context_menu_shortcut("⌫")
                    )
                )
            )
        )
    })
}

#[component]
pub async fn icons() -> Result<impl View> {
    Ok(view! {
        context_menu(
            context_menu_trigger(attrs: attributes! { class=(ZONE) },
                zone_text(fine: "Right click here", coarse: "Long press here")
            )
            context_menu_content(
                context_menu_group(
                    context_menu_item(icon(data: iconify_icon!("lucide:copy")) "Copy")
                    context_menu_item(icon(data: iconify_icon!("lucide:scissors")) "Cut")
                    context_menu_item(icon(data: iconify_icon!("lucide:clipboard-paste")) "Paste")
                )
                context_menu_separator()
                context_menu_group(
                    context_menu_item(variant: ContextMenuItemVariant::Destructive,
                        icon(data: iconify_icon!("lucide:trash"))
                        "Delete"
                    )
                )
            )
        )
    })
}

/// The Octane version keeps the choices in `useState`; native radio buttons
/// sharing a `name` hold the choice here.
#[component]
pub async fn radio() -> Result<impl View> {
    Ok(view! {
        context_menu(
            context_menu_trigger(attrs: attributes! { class=(ZONE) },
                zone_text(fine: "Right click here", coarse: "Long press here")
            )
            context_menu_content(
                context_menu_group(
                    context_menu_label("People")
                    context_menu_radio_group(
                        context_menu_radio_item(
                            attrs: attributes! { name="user" value="pedro" checked="" },
                            "Pedro Duarte"
                        )
                        context_menu_radio_item(
                            attrs: attributes! { name="user" value="colm" },
                            "Colm Tuite"
                        )
                    )
                )
                context_menu_separator()
                context_menu_group(
                    context_menu_label("Theme")
                    context_menu_radio_group(
                        context_menu_radio_item(
                            attrs: attributes! { name="theme" value="light" checked="" },
                            "Light"
                        )
                        context_menu_radio_item(
                            attrs: attributes! { name="theme" value="dark" },
                            "Dark"
                        )
                        context_menu_radio_item(
                            attrs: attributes! { name="theme" value="system" },
                            "System"
                        )
                    )
                )
            )
        )
    })
}

#[component]
pub async fn shortcuts() -> Result<impl View> {
    Ok(view! {
        context_menu(
            context_menu_trigger(attrs: attributes! { class=(ZONE) },
                zone_text(fine: "Right click here", coarse: "Long press here")
            )
            context_menu_content(
                context_menu_group(
                    context_menu_item("Back" context_menu_shortcut("⌘["))
                    context_menu_item(attrs: attributes! { disabled="" },
                        "Forward"
                        context_menu_shortcut("⌘]")
                    )
                    context_menu_item("Reload" context_menu_shortcut("⌘R"))
                )
                context_menu_separator()
                context_menu_group(
                    context_menu_item("Save" context_menu_shortcut("⌘S"))
                    context_menu_item("Save As..." context_menu_shortcut("⇧⌘S"))
                )
            )
        )
    })
}

#[component]
pub async fn sides() -> Result<impl View> {
    let sides = [
        (ContextMenuSide::Top, "top"),
        (ContextMenuSide::Right, "right"),
        (ContextMenuSide::Bottom, "bottom"),
        (ContextMenuSide::Left, "left"),
    ];

    Ok(view! {
        <div class="grid w-full max-w-sm grid-cols-2 gap-4">
            for (side, name) in sides {
                context_menu(
                    context_menu_trigger(attrs: attributes! { class=(ZONE) },
                        zone_text(
                            fine: format!("Right click ({name})"),
                            coarse: format!("Long press ({name})"),
                        )
                    )
                    context_menu_content(side: side,
                        context_menu_group(
                            context_menu_item("Back")
                            context_menu_item("Forward")
                            context_menu_item("Reload")
                        )
                    )
                )
            }
        </div>
    })
}

#[component]
pub async fn submenu() -> Result<impl View> {
    Ok(view! {
        context_menu(
            context_menu_trigger(attrs: attributes! { class=(ZONE) },
                zone_text(fine: "Right click here", coarse: "Long press here")
            )
            context_menu_content(
                context_menu_group(
                    context_menu_item("Copy" context_menu_shortcut("⌘C"))
                    context_menu_item("Cut" context_menu_shortcut("⌘X"))
                )
                context_menu_sub(
                    context_menu_sub_trigger("More Tools")
                    context_menu_sub_content(
                        context_menu_group(
                            context_menu_item("Save Page...")
                            context_menu_item("Create Shortcut...")
                            context_menu_item("Name Window...")
                        )
                        context_menu_separator()
                        context_menu_group(
                            context_menu_item("Developer Tools")
                        )
                        context_menu_separator()
                        context_menu_group(
                            context_menu_item(variant: ContextMenuItemVariant::Destructive,
                                "Delete"
                            )
                        )
                    )
                )
            )
        )
    })
}
