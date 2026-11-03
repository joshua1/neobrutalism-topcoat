//! The Octane examples open the palette from React state; here a
//! `dialog_trigger` opens it, and the demo's ⌘J shortcut is a small script.

use topcoat::{
    Result,
    icon::{IconData, icon, iconify::iconify_icon},
    view::{Unescaped, View, attributes, class, component, view},
};

use crate::components::button::{ButtonSize, ButtonVariant, button_variants};
use crate::components::command::{
    command_dialog, command_empty, command_group, command_input, command_item, command_list,
    command_separator, command_shortcut,
};
use crate::components::dialog::dialog_trigger;

/// Toggles the demo's palette on ⌘J (Ctrl+J elsewhere).
const TOGGLE_ON_CMD_J: &str = r#"
document.addEventListener('keydown', (e) => {
  if (e.key === 'j' && (e.metaKey || e.ctrlKey)) {
    e.preventDefault();
    const palette = document.getElementById('command-demo');
    palette.open ? palette.close() : palette.showModal();
  }
});
"#;

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        <p class="text-foreground text-sm">
            "Press "
            <kbd class="bg-main text-main-foreground pointer-events-none inline-flex h-5 items-center gap-1 rounded-base border-2 px-1.5 font-mono text-[10px] font-heading select-none">
                <span class="text-xs">"⌘"</span>
                "J"
            </kbd>
        </p>
        // The script is a trusted constant, so it is written out unescaped.
        <script>(Unescaped::new_unchecked(TOGGLE_ON_CMD_J))</script>
        command_dialog(id: "command-demo",
            command_input(attrs: attributes! { placeholder="Type a command or search..." })
            command_list(
                command_empty("No results found.")
                command_group(heading: "Suggestions",
                    command_item(
                        icon(data: iconify_icon!("lucide:calendar"))
                        <span>"Calendar"</span>
                    )
                    command_item(
                        icon(data: iconify_icon!("lucide:smile"))
                        <span>"Search Emoji"</span>
                    )
                    command_item(
                        icon(data: iconify_icon!("lucide:calculator"))
                        <span>"Calculator"</span>
                    )
                )
                command_separator()
                command_group(heading: "Settings",
                    command_item(
                        icon(data: iconify_icon!("lucide:user"))
                        <span>"Profile"</span>
                        command_shortcut("⌘P")
                    )
                    command_item(
                        icon(data: iconify_icon!("lucide:credit-card"))
                        <span>"Billing"</span>
                        command_shortcut("⌘B")
                    )
                    command_item(
                        icon(data: iconify_icon!("lucide:settings"))
                        <span>"Settings"</span>
                        command_shortcut("⌘S")
                    )
                )
            )
        )
    })
}

#[component]
pub async fn basic() -> Result<impl View> {
    Ok(view! {
        <div class="flex flex-col gap-4">
            dialog_trigger(
                target: "command-basic",
                attrs: attributes! {
                    class=(class!(button_variants(ButtonVariant::Neutral, ButtonSize::Default), "w-fit"))
                },
                "Open Menu"
            )
            command_dialog(id: "command-basic",
                command_input(attrs: attributes! { placeholder="Type a command or search..." })
                command_list(
                    command_empty("No results found.")
                    command_group(heading: "Suggestions",
                        command_item("Calendar")
                        command_item("Search Emoji")
                        command_item("Calculator")
                    )
                )
            )
        </div>
    })
}

#[component]
pub async fn shortcuts() -> Result<impl View> {
    Ok(view! {
        <div class="flex flex-col gap-4">
            dialog_trigger(
                target: "command-shortcuts",
                attrs: attributes! {
                    class=(class!(button_variants(ButtonVariant::Neutral, ButtonSize::Default), "w-fit"))
                },
                "Open Menu"
            )
            command_dialog(id: "command-shortcuts",
                command_input(attrs: attributes! { placeholder="Type a command or search..." })
                command_list(
                    command_empty("No results found.")
                    command_group(heading: "Settings",
                        command_item(
                            icon(data: iconify_icon!("lucide:user"))
                            <span>"Profile"</span>
                            command_shortcut("⌘P")
                        )
                        command_item(
                            icon(data: iconify_icon!("lucide:credit-card"))
                            <span>"Billing"</span>
                            command_shortcut("⌘B")
                        )
                        command_item(
                            icon(data: iconify_icon!("lucide:settings"))
                            <span>"Settings"</span>
                            command_shortcut("⌘S")
                        )
                    )
                )
            )
        </div>
    })
}

#[component]
pub async fn groups() -> Result<impl View> {
    Ok(view! {
        <div class="flex flex-col gap-4">
            dialog_trigger(
                target: "command-groups",
                attrs: attributes! {
                    class=(class!(button_variants(ButtonVariant::Neutral, ButtonSize::Default), "w-fit"))
                },
                "Open Menu"
            )
            command_dialog(id: "command-groups",
                command_input(attrs: attributes! { placeholder="Type a command or search..." })
                command_list(
                    command_empty("No results found.")
                    command_group(heading: "Suggestions",
                        command_item(
                            icon(data: iconify_icon!("lucide:calendar"))
                            <span>"Calendar"</span>
                        )
                        command_item(
                            icon(data: iconify_icon!("lucide:smile"))
                            <span>"Search Emoji"</span>
                        )
                        command_item(
                            icon(data: iconify_icon!("lucide:calculator"))
                            <span>"Calculator"</span>
                        )
                    )
                    command_separator()
                    command_group(heading: "Settings",
                        command_item(
                            icon(data: iconify_icon!("lucide:user"))
                            <span>"Profile"</span>
                            command_shortcut("⌘P")
                        )
                        command_item(
                            icon(data: iconify_icon!("lucide:credit-card"))
                            <span>"Billing"</span>
                            command_shortcut("⌘B")
                        )
                        command_item(
                            icon(data: iconify_icon!("lucide:settings"))
                            <span>"Settings"</span>
                            command_shortcut("⌘S")
                        )
                    )
                )
            )
        </div>
    })
}

/// One row of the scrollable example: its icon, label and optional shortcut.
type Entry = (IconData, &'static str, Option<&'static str>);

/// The scrollable example's groups, in order.
const SCROLLABLE: [(&str, &[Entry]); 5] = [
    (
        "Navigation",
        &[
            (iconify_icon!("lucide:house"), "Home", Some("⌘H")),
            (iconify_icon!("lucide:inbox"), "Inbox", Some("⌘I")),
            (iconify_icon!("lucide:file-text"), "Documents", Some("⌘D")),
            (iconify_icon!("lucide:folder"), "Folders", Some("⌘F")),
        ],
    ),
    (
        "Actions",
        &[
            (iconify_icon!("lucide:plus"), "New File", Some("⌘N")),
            (iconify_icon!("lucide:folder-plus"), "New Folder", Some("⇧⌘N")),
            (iconify_icon!("lucide:copy"), "Copy", Some("⌘C")),
            (iconify_icon!("lucide:scissors"), "Cut", Some("⌘X")),
            (iconify_icon!("lucide:clipboard-paste"), "Paste", Some("⌘V")),
            (iconify_icon!("lucide:trash"), "Delete", Some("⌫")),
        ],
    ),
    (
        "View",
        &[
            (iconify_icon!("lucide:layout-grid"), "Grid View", None),
            (iconify_icon!("lucide:list"), "List View", None),
            (iconify_icon!("lucide:zoom-in"), "Zoom In", Some("⌘+")),
            (iconify_icon!("lucide:zoom-out"), "Zoom Out", Some("⌘-")),
        ],
    ),
    (
        "Account",
        &[
            (iconify_icon!("lucide:user"), "Profile", Some("⌘P")),
            (iconify_icon!("lucide:credit-card"), "Billing", Some("⌘B")),
            (iconify_icon!("lucide:settings"), "Settings", Some("⌘S")),
            (iconify_icon!("lucide:bell"), "Notifications", None),
            (iconify_icon!("lucide:circle-help"), "Help & Support", None),
        ],
    ),
    (
        "Tools",
        &[
            (iconify_icon!("lucide:calculator"), "Calculator", None),
            (iconify_icon!("lucide:calendar"), "Calendar", None),
            (iconify_icon!("lucide:image"), "Image Editor", None),
            (iconify_icon!("lucide:code"), "Code Editor", None),
        ],
    ),
];

#[component]
pub async fn scrollable() -> Result<impl View> {
    Ok(view! {
        <div class="flex flex-col gap-4">
            dialog_trigger(
                target: "command-scrollable",
                attrs: attributes! {
                    class=(class!(button_variants(ButtonVariant::Neutral, ButtonSize::Default), "w-fit"))
                },
                "Open Menu"
            )
            command_dialog(id: "command-scrollable",
                command_input(attrs: attributes! { placeholder="Type a command or search..." })
                command_list(
                    command_empty("No results found.")
                    for (index, (heading, entries)) in SCROLLABLE.into_iter().enumerate() {
                        if index > 0 {
                            command_separator()
                        }
                        command_group(heading: heading,
                            for (glyph, label, shortcut) in entries {
                                command_item(
                                    icon(data: glyph.clone())
                                    <span>(*label)</span>
                                    if let Some(keys) = *shortcut {
                                        command_shortcut((keys))
                                    }
                                )
                            }
                        )
                    }
                )
            )
        </div>
    })
}
