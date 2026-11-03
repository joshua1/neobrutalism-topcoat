//! The Octane examples keep the choice in React state; here the combobox
//! parts check the chosen item and copy it into the trigger in the browser.
//! `!` on a class overrides one the button or popover already sets, since
//! `class!` appends without merging.

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

use crate::components::avatar::{avatar, avatar_fallback, avatar_image};
use crate::components::combobox::{
    combobox, combobox_item, combobox_item_indicator, combobox_trigger, combobox_value,
};
use crate::components::command::{
    command, command_empty, command_group, command_input, command_item, command_list,
    command_separator,
};
use crate::components::popover::{PopoverAlign, popover_content};

/// The frameworks to choose from: `(value, label)`.
const FRAMEWORKS: [(&str, &str); 5] = [
    ("next.js", "Next.js"),
    ("sveltekit", "SvelteKit"),
    ("nuxt.js", "Nuxt.js"),
    ("remix", "Remix"),
    ("astro", "Astro"),
];

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        combobox(name: "framework",
            combobox_trigger(
                target: "combobox-demo",
                attrs: attributes! { class="w-full justify-between! md:max-w-[200px]" },
                combobox_value(placeholder: "Select framework...")
                icon(data: iconify_icon!("lucide:chevrons-up-down"))
            )
            popover_content(
                id: "combobox-demo",
                attrs: attributes! { class="w-[anchor-size(width)]! border-0! p-0!" },
                command(attrs: attributes! { class="**:data-[slot=command-input-wrapper]:h-11" },
                    command_input(attrs: attributes! { placeholder="Search framework..." autofocus="" })
                    command_list(attrs: attributes! { class="p-1" },
                        command_empty("No framework found.")
                        command_group(
                            for (value, label) in FRAMEWORKS {
                                combobox_item(value: value,
                                    (label)
                                    combobox_item_indicator(attrs: attributes! { class="ml-auto" })
                                )
                            }
                        )
                    )
                )
            )
        )
    })
}

/// The users to choose from: `(id, username)`.
const USERS: [(&str, &str); 3] = [("1", "shadcn"), ("2", "leerob"), ("3", "evilrabbit")];

#[component]
pub async fn users() -> Result<impl View> {
    Ok(view! {
        combobox(name: "user", value: "1",
            combobox_trigger(
                target: "combobox-users",
                attrs: attributes! { class="w-full justify-between! px-2! md:max-w-[200px]" },
                // The initially checked user, as its item shows it.
                combobox_value(
                    placeholder: "Select user...",
                    attrs: attributes! { class="flex items-center gap-2" },
                    avatar(attrs: attributes! { class="size-5!" },
                        avatar_image(src: "https://github.com/shadcn.png")
                        avatar_fallback("s")
                    )
                    "shadcn"
                )
                icon(
                    data: iconify_icon!("lucide:chevrons-up-down"),
                    attrs: attributes! { class="text-foreground" }
                )
            )
            popover_content(
                id: "combobox-users",
                attrs: attributes! { class="w-[anchor-size(width)]! border-0! p-0!" },
                command(attrs: attributes! { class="**:data-[slot=command-input-wrapper]:h-11" },
                    command_input(attrs: attributes! { placeholder="Search user..." autofocus="" })
                    command_list(attrs: attributes! { class="p-1" },
                        command_empty("No user found.")
                        command_group(
                            attrs: attributes! {
                                class="[&_[cmdk-group-items]]:flex [&_[cmdk-group-items]]:flex-col \
                                    [&_[cmdk-group-items]]:gap-1"
                            },
                            for (id, username) in USERS {
                                combobox_item(value: id, checked: id == "1",
                                    avatar(attrs: attributes! { class="size-5!" },
                                        avatar_image(src: format!("https://github.com/{username}.png"))
                                        avatar_fallback((&username[..1]))
                                    )
                                    (username)
                                    combobox_item_indicator(attrs: attributes! { class="ml-auto" })
                                )
                            }
                        )
                        command_separator()
                        command_group(
                            command_item(
                                icon(data: iconify_icon!("lucide:circle-plus"))
                                "Create user"
                            )
                        )
                    )
                )
            )
        )
    })
}

/// The timezones to choose from, by region: `(value, label)`.
const TIMEZONES: [(&str, [(&str, &str); 6]); 3] = [
    (
        "Americas",
        [
            ("America/New_York", "(GMT-5) New York"),
            ("America/Los_Angeles", "(GMT-8) Los Angeles"),
            ("America/Chicago", "(GMT-6) Chicago"),
            ("America/Toronto", "(GMT-5) Toronto"),
            ("America/Vancouver", "(GMT-8) Vancouver"),
            ("America/Sao_Paulo", "(GMT-3) São Paulo"),
        ],
    ),
    (
        "Europe",
        [
            ("Europe/London", "(GMT+0) London"),
            ("Europe/Paris", "(GMT+1) Paris"),
            ("Europe/Berlin", "(GMT+1) Berlin"),
            ("Europe/Rome", "(GMT+1) Rome"),
            ("Europe/Madrid", "(GMT+1) Madrid"),
            ("Europe/Amsterdam", "(GMT+1) Amsterdam"),
        ],
    ),
    (
        "Asia/Pacific",
        [
            ("Asia/Tokyo", "(GMT+9) Tokyo"),
            ("Asia/Shanghai", "(GMT+8) Shanghai"),
            ("Asia/Singapore", "(GMT+8) Singapore"),
            ("Asia/Dubai", "(GMT+4) Dubai"),
            ("Australia/Sydney", "(GMT+11) Sydney"),
            ("Asia/Seoul", "(GMT+9) Seoul"),
        ],
    ),
];

#[component]
pub async fn timezones() -> Result<impl View> {
    Ok(view! {
        combobox(name: "timezone", value: "America/New_York",
            combobox_trigger(
                target: "combobox-timezones",
                attrs: attributes! { class="h-12! w-full justify-between! px-2.5! md:max-w-[200px]" },
                combobox_value(
                    placeholder: "Select timezone",
                    attrs: attributes! { class="flex flex-col items-start gap-0.5" },
                    <span class="text-xs font-normal">"Americas"</span>
                    <span>"(GMT-5) New York"</span>
                )
                icon(data: iconify_icon!("lucide:chevron-down"))
            )
            popover_content(
                id: "combobox-timezones",
                align: PopoverAlign::Start,
                attrs: attributes! { class="p-0! border-0!" },
                command(attrs: attributes! { class="**:data-[slot=command-input-wrapper]:h-11" },
                    command_input(attrs: attributes! { placeholder="Search timezone..." autofocus="" })
                    command_list(attrs: attributes! { class="scroll-pb-12" },
                        command_empty("No timezone found.")
                        for (region, zones) in TIMEZONES {
                            command_group(heading: region, attrs: attributes! { class="p-2" },
                                for (value, label) in zones {
                                    combobox_item(value: value, checked: value == "America/New_York",
                                        // The region shows only in the trigger, above the label.
                                        <span class="hidden text-xs font-normal in-data-[slot=combobox-value]:inline">
                                            (region)
                                        </span>
                                        <span>(label)</span>
                                        combobox_item_indicator(attrs: attributes! { class="ml-auto" })
                                    )
                                }
                            )
                        }
                        command_group(
                            attrs: attributes! {
                                class="bg-background border-t-2 p-2 border-t-border sticky bottom-0"
                            },
                            command_item(
                                icon(data: iconify_icon!("lucide:circle-plus"))
                                "Create timezone"
                            )
                        )
                    )
                )
            )
        )
    })
}

#[component]
pub async fn multiselect() -> Result<impl View> {
    Ok(view! {
        combobox(name: "frameworks", multiple: true,
            combobox_trigger(
                target: "combobox-multiselect",
                attrs: attributes! { class="w-fit min-w-[280px] justify-between!" },
                combobox_value(placeholder: "Select frameworks (multi-select)...")
                icon(
                    data: iconify_icon!("lucide:chevrons-up-down"),
                    attrs: attributes! { class="text-foreground" }
                )
            )
            popover_content(
                id: "combobox-multiselect",
                align: PopoverAlign::Start,
                attrs: attributes! { class="w-[300px]! p-0! border-0!" },
                command(attrs: attributes! { class="**:data-[slot=command-input-wrapper]:h-11" },
                    command_input(attrs: attributes! { placeholder="Search framework..." autofocus="" })
                    command_list(
                        command_empty("No framework found.")
                        command_group(
                            attrs: attributes! {
                                class="p-2 [&_[cmdk-group-items]]:flex [&_[cmdk-group-items]]:flex-col \
                                    [&_[cmdk-group-items]]:gap-1"
                            },
                            for (value, label) in FRAMEWORKS {
                                combobox_item(value: value,
                                    // A checkbox-like box whose check follows the item's state.
                                    <div class="border-border pointer-events-none size-5 shrink-0 rounded-base border-2 transition-all select-none *:[svg]:opacity-0 in-data-[checked=true]:*:[svg]:opacity-100">
                                        icon(
                                            data: iconify_icon!("lucide:check"),
                                            attrs: attributes! { class="size-4 text-current" }
                                        )
                                    </div>
                                    (label)
                                )
                            }
                        )
                    )
                )
            )
        )
    })
}
