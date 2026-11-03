use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

use crate::components::menubar::{
    menubar, menubar_checkbox_item, menubar_content, menubar_group, menubar_item, menubar_menu,
    menubar_radio_group, menubar_radio_item, menubar_separator, menubar_shortcut, menubar_sub,
    menubar_sub_content, menubar_sub_trigger, menubar_trigger,
};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        menubar(
            menubar_menu(
                menubar_trigger("File")
                menubar_content(
                    menubar_item("New Tab " menubar_shortcut("⌘T"))
                    menubar_item("New Window " menubar_shortcut("⌘N"))
                    menubar_item(attrs: attributes! { disabled="" }, "New Incognito Window")
                    menubar_separator()
                    menubar_sub(
                        menubar_sub_trigger("Share")
                        menubar_sub_content(
                            menubar_item("Email link")
                            menubar_item("Messages")
                            menubar_item("Notes")
                        )
                    )
                    menubar_separator()
                    menubar_item("Print... " menubar_shortcut("⌘P"))
                )
            )
            menubar_menu(
                menubar_trigger("Edit")
                menubar_content(
                    menubar_item("Undo " menubar_shortcut("⌘Z"))
                    menubar_item("Redo " menubar_shortcut("⇧⌘Z"))
                    menubar_separator()
                    menubar_sub(
                        menubar_sub_trigger("Find")
                        menubar_sub_content(
                            menubar_item("Search the web")
                            menubar_separator()
                            menubar_item("Find...")
                            menubar_item("Find Next")
                            menubar_item("Find Previous")
                        )
                    )
                    menubar_separator()
                    menubar_item("Cut")
                    menubar_item("Copy")
                    menubar_item("Paste")
                )
            )
            menubar_menu(
                menubar_trigger("View")
                menubar_content(
                    menubar_checkbox_item("Always Show Bookmarks Bar")
                    menubar_checkbox_item(attrs: attributes! { checked="" },
                        "Always Show Full URLs"
                    )
                    menubar_separator()
                    menubar_item(inset: true, "Reload " menubar_shortcut("⌘R"))
                    menubar_item(inset: true, attrs: attributes! { disabled="" },
                        "Force Reload "
                        menubar_shortcut("⇧⌘R")
                    )
                    menubar_separator()
                    menubar_item(inset: true, "Toggle Fullscreen")
                    menubar_separator()
                    menubar_item(inset: true, "Hide Sidebar")
                )
            )
            menubar_menu(
                menubar_trigger("Profiles")
                menubar_content(
                    menubar_radio_group(
                        menubar_radio_item(attrs: attributes! { name="profile" value="andy" },
                            "Andy"
                        )
                        menubar_radio_item(
                            attrs: attributes! { name="profile" value="benoit" checked="" },
                            "Benoit"
                        )
                        menubar_radio_item(attrs: attributes! { name="profile" value="Luis" },
                            "Luis"
                        )
                    )
                    menubar_separator()
                    menubar_item(inset: true, "Edit...")
                    menubar_separator()
                    menubar_item(inset: true, "Add Profile...")
                )
            )
        )
    })
}

#[component]
pub async fn checkbox() -> Result<impl View> {
    Ok(view! {
        menubar(attrs: attributes! { class="w-72" },
            menubar_menu(
                menubar_trigger("View")
                menubar_content(attrs: attributes! { class="w-64" },
                    menubar_checkbox_item("Always Show Bookmarks Bar")
                    menubar_checkbox_item(attrs: attributes! { checked="" },
                        "Always Show Full URLs"
                    )
                    menubar_separator()
                    menubar_item(inset: true, "Reload " menubar_shortcut("⌘R"))
                    menubar_item(inset: true, attrs: attributes! { disabled="" },
                        "Force Reload "
                        menubar_shortcut("⇧⌘R")
                    )
                )
            )
            menubar_menu(
                menubar_trigger("Format")
                menubar_content(
                    menubar_checkbox_item(attrs: attributes! { checked="" }, "Strikethrough")
                    menubar_checkbox_item("Code")
                    menubar_checkbox_item("Superscript")
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
        menubar(attrs: attributes! { class="w-72" },
            menubar_menu(
                menubar_trigger("Profiles")
                menubar_content(
                    menubar_radio_group(
                        menubar_radio_item(attrs: attributes! { name="user" value="andy" },
                            "Andy"
                        )
                        menubar_radio_item(
                            attrs: attributes! { name="user" value="benoit" checked="" },
                            "Benoit"
                        )
                        menubar_radio_item(attrs: attributes! { name="user" value="luis" },
                            "Luis"
                        )
                    )
                    menubar_separator()
                    menubar_item(inset: true, "Edit...")
                    menubar_item(inset: true, "Add Profile...")
                )
            )
            menubar_menu(
                menubar_trigger("Theme")
                menubar_content(
                    menubar_radio_group(
                        menubar_radio_item(attrs: attributes! { name="theme" value="light" },
                            "Light"
                        )
                        menubar_radio_item(attrs: attributes! { name="theme" value="dark" },
                            "Dark"
                        )
                        menubar_radio_item(
                            attrs: attributes! { name="theme" value="system" checked="" },
                            "System"
                        )
                    )
                )
            )
        )
    })
}

#[component]
pub async fn submenu() -> Result<impl View> {
    Ok(view! {
        menubar(attrs: attributes! { class="w-72" },
            menubar_menu(
                menubar_trigger("File")
                menubar_content(
                    menubar_sub(
                        menubar_sub_trigger("Share")
                        menubar_sub_content(
                            menubar_item("Email link")
                            menubar_item("Messages")
                            menubar_item("Notes")
                        )
                    )
                    menubar_separator()
                    menubar_item("Print... " menubar_shortcut("⌘P"))
                )
            )
            menubar_menu(
                menubar_trigger("Edit")
                menubar_content(
                    menubar_item("Undo " menubar_shortcut("⌘Z"))
                    menubar_item("Redo " menubar_shortcut("⇧⌘Z"))
                    menubar_separator()
                    menubar_sub(
                        menubar_sub_trigger("Find")
                        menubar_sub_content(
                            menubar_item("Find...")
                            menubar_item("Find Next")
                            menubar_item("Find Previous")
                        )
                    )
                    menubar_separator()
                    menubar_item("Cut")
                    menubar_item("Copy")
                    menubar_item("Paste")
                )
            )
        )
    })
}

#[component]
pub async fn with_icons() -> Result<impl View> {
    Ok(view! {
        menubar(attrs: attributes! { class="w-72" },
            menubar_menu(
                menubar_trigger("File")
                menubar_content(
                    menubar_item(
                        icon(data: iconify_icon!("lucide:file"))
                        "New File "
                        menubar_shortcut("⌘N")
                    )
                    menubar_item(icon(data: iconify_icon!("lucide:folder")) "Open Folder")
                    menubar_separator()
                    menubar_item(
                        icon(data: iconify_icon!("lucide:save"))
                        "Save "
                        menubar_shortcut("⌘S")
                    )
                )
            )
            menubar_menu(
                menubar_trigger("More")
                menubar_content(
                    menubar_group(
                        menubar_item(icon(data: iconify_icon!("lucide:settings")) "Settings")
                        menubar_item(icon(data: iconify_icon!("lucide:circle-help")) "Help")
                        menubar_separator()
                        menubar_item(icon(data: iconify_icon!("lucide:trash")) "Delete")
                    )
                )
            )
        )
    })
}
