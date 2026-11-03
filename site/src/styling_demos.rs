//! The example components on the Styling page: ports of Octane's
//! `src/app/styling/example-components.tsrx` and the demos in
//! `src/app/styling/demos/`, one private function per demo file (named after
//! its Octane export, `SettingsDemo` -> `settings_demo`, so no demo shares a
//! name with a component it uses).
//!
//! `class!` does not drop conflicting classes the way Octane's `cn` does, so
//! a class that replaces one the component already sets carries Tailwind's
//! `!` (important) modifier.

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, class, component, view},
};

use crate::components::{
    alert::{alert, alert_description, alert_title},
    avatar::{avatar, avatar_fallback},
    bubble::{BubbleVariant, bubble, bubble_content},
    button::{ButtonSize, ButtonVariant, button, button_variants},
    checkbox::checkbox,
    collapsible::{collapsible, collapsible_content, collapsible_trigger},
    field::{FieldOrientation, field, field_content, field_description, field_group, field_label},
    input_group::{
        InputGroupAddonAlign, InputGroupButtonSize, input_group, input_group_addon,
        input_group_button, input_group_input, input_group_text, input_group_textarea,
    },
    item::{
        ItemMediaVariant, item, item_actions, item_content, item_description, item_group,
        item_media, item_title,
    },
    kbd::kbd,
    label::label,
    message::{
        MessageAlign, message, message_avatar, message_content, message_footer, message_header,
    },
    scroll_area::scroll_area,
    slider::slider,
    switch::switch,
};

/// The Styling page's grid of example components: three columns on large
/// screens, two on small ones (the chat and items column is hidden), one on
/// phones (the chart moves to the top and the scroll area is hidden).
#[component]
pub async fn styling_examples() -> Result<impl View> {
    Ok(view! {
        <div class="w-full grid lg:grid-cols-3 sm:grid-cols-2 grid-cols-1 mt-10 gap-5 not-prose">
            <div class="flex flex-col gap-5">
                settings_demo()
                input_group_demo()
                collapsible_demo()
                slider_demo()
                alert_demo()
                input_group_textarea_demo()
            </div>
            <div class="lg:flex hidden flex-col gap-5">
                chat_demo()
                items_demo()
            </div>
            <div class="flex flex-col gap-5 max-sm:order-first">
                crate::examples::chart::chart_bar_multiple()
                <div class="max-sm:hidden">
                    scroll_area_demo()
                </div>
            </div>
        </div>
    })
}

#[component]
async fn settings_demo() -> Result<impl View> {
    Ok(view! {
        field_group(
            attrs: attributes! {
                class="w-full rounded-base border-2 border-border bg-secondary-background p-4 \
                    text-foreground"
            },
            field(orientation: FieldOrientation::Horizontal,
                field_content(
                    field_label(attrs: attributes! { for="styling-notifications" }, "Notifications")
                    field_description("Get notified when someone mentions you.")
                )
                switch(attrs: attributes! { id="styling-notifications" checked="" })
            )
            field(orientation: FieldOrientation::Horizontal,
                field_content(
                    field_label(attrs: attributes! { for="styling-sounds" }, "Sounds")
                    field_description("Play a sound for new messages.")
                )
                switch(attrs: attributes! { id="styling-sounds" })
            )
            field(orientation: FieldOrientation::Horizontal,
                checkbox(attrs: attributes! { id="styling-newsletter" checked="" })
                field_content(
                    field_label(attrs: attributes! { for="styling-newsletter" },
                        "Subscribe to newsletter"
                    )
                    field_description("Receive weekly updates about new components.")
                )
            )
        )
    })
}

#[component]
async fn input_group_demo() -> Result<impl View> {
    Ok(view! {
        input_group(attrs: attributes! { class="w-full" },
            input_group_input(attrs: attributes! { placeholder="Search components..." })
            input_group_addon(icon(data: iconify_icon!("lucide:search")))
            input_group_addon(align: InputGroupAddonAlign::InlineEnd, kbd("⌘K"))
        )
    })
}

#[component]
async fn collapsible_demo() -> Result<impl View> {
    // The `<details>` trigger must be its direct child and hides everything
    // else in it while closed, so the always-visible rows sit outside it and
    // the trigger is positioned over the header's empty slot (inside its 2px
    // border and `py-2` / `px-4` padding). Base UI unmounts the closed panel,
    // so `not-open:-mt-2` cancels the `space-y-2` gap the base-ui row keeps
    // below it.
    Ok(view! {
        <div class="relative w-full space-y-2">
            <div class="rounded-base flex items-center justify-between space-x-4 border-2 \
                border-border text-foreground bg-secondary-background px-4 py-2">
                <h4 class="text-sm font-heading">"@octanejs published 3 packages"</h4>
                <span class="h-9 w-9"></span>
            </div>
            <div class="rounded-base border-2 border-border bg-secondary-background px-4 py-3 \
                font-mono font-base text-foreground text-sm">
                "@octanejs/base-ui"
            </div>
            collapsible(attrs: attributes! { class="not-open:-mt-2" },
                collapsible_trigger(
                    attrs: attributes! {
                        // `!` overrides the button's fill, text color and padding.
                        class=(class!(
                            button_variants(ButtonVariant::NoShadow, ButtonSize::Sm),
                            "absolute top-2.5 right-4.5 w-9 bg-secondary-background! \
                             text-foreground! p-0!",
                        ))
                    },
                    icon(
                        data: iconify_icon!("lucide:chevrons-up-down"),
                        attrs: attributes! { class="size-4" }
                    )
                    <span class="sr-only">"Toggle"</span>
                )
                collapsible_content(attrs: attributes! { class="space-y-2 text-foreground font-base" },
                    <div class="rounded-base border-2 border-border bg-secondary-background px-4 \
                        py-3 font-mono text-sm">
                        "@octanejs/shadcn"
                    </div>
                    <div class="rounded-base border-2 border-border bg-secondary-background px-4 \
                        py-3 font-mono text-sm">
                        "@octanejs/lucide"
                    </div>
                )
            )
        </div>
    })
}

#[component]
async fn slider_demo() -> Result<impl View> {
    Ok(view! {
        <div class="grid w-full gap-3 text-foreground">
            <div class="flex items-center justify-between">
                label("Volume")
                <span class="text-sm font-base">"40%"</span>
            </div>
            slider(value: vec![40.0], max: 100.0, step: 1.0)
        </div>
    })
}

#[component]
async fn alert_demo() -> Result<impl View> {
    Ok(view! {
        // `!` overrides the alert's `bg-background`.
        alert(attrs: attributes! { class="bg-secondary-background!" },
            icon(data: iconify_icon!("lucide:circle-check"))
            alert_title("Success! Your changes have been saved")
            alert_description("This is an alert with icon, title and description.")
        )
    })
}

#[component]
async fn input_group_textarea_demo() -> Result<impl View> {
    Ok(view! {
        input_group(attrs: attributes! { class="w-full" },
            input_group_textarea(attrs: attributes! { placeholder="Ask anything..." })
            input_group_addon(
                align: InputGroupAddonAlign::BlockEnd,
                attrs: attributes! { class="border-t-2 border-border" },
                input_group_text("Markdown supported")
                input_group_button(
                    variant: Some(ButtonVariant::Default),
                    size: InputGroupButtonSize::IconSm,
                    attrs: attributes! { class="ml-auto" aria-label="Send" },
                    icon(data: iconify_icon!("lucide:send"))
                )
            )
        )
    })
}

/// One message in the chat demo.
struct ChatMessage {
    author: &'static str,
    time: &'static str,
    fallback: &'static str,
    align: MessageAlign,
    text: &'static str,
}

impl ChatMessage {
    /// Your own messages use the main color, the others the muted one.
    fn bubble_variant(&self) -> BubbleVariant {
        match self.align {
            MessageAlign::End => BubbleVariant::Default,
            MessageAlign::Start => BubbleVariant::Muted,
        }
    }
}

const THREAD: [ChatMessage; 3] = [
    ChatMessage {
        author: "Mia",
        time: "10:24 AM",
        fallback: "MI",
        align: MessageAlign::Start,
        text: "I checked the latest deployment and the build failed while installing \
               dependencies. Can you share the error from the logs?",
    },
    ChatMessage {
        author: "You",
        time: "10:26 AM",
        fallback: "ME",
        align: MessageAlign::End,
        text: "Sure, sending it over now. It looks like a peer dependency issue.",
    },
    ChatMessage {
        author: "Mia",
        time: "10:31 AM",
        fallback: "MI",
        align: MessageAlign::Start,
        text: "Got it. Pinning the version should fix it, I will push a patch in a minute.",
    },
];

#[component]
async fn chat_demo() -> Result<impl View> {
    // A loop binding named `message` would match the `message` component's
    // unit struct, so each one is an `entry`.
    Ok(view! {
        <div class="flex w-full flex-col gap-5 rounded-base border-2 border-border \
            bg-secondary-background p-4 shadow-shadow">
            for entry in &THREAD {
                message(align: entry.align,
                    message_avatar(avatar(avatar_fallback((entry.fallback))))
                    message_content(
                        message_header((entry.author))
                        bubble(variant: entry.bubble_variant(), bubble_content((entry.text)))
                        message_footer((entry.time))
                    )
                )
            }
        </div>
    })
}

#[component]
async fn items_demo() -> Result<impl View> {
    let items = [
        (
            iconify_icon!("lucide:git-pull-request"),
            "Pull request opened",
            "Mia opened #128: pin peer dependency versions",
        ),
        (iconify_icon!("lucide:star"), "New star", "Your repository reached 1,000 stars"),
        (iconify_icon!("lucide:bell"), "Reminder", "Release notes are due tomorrow"),
    ];
    Ok(view! {
        // `!` overrides the group's `gap-4` and the item's `bg-background`.
        item_group(attrs: attributes! { class="w-full gap-3!" },
            for (data, title, description) in items {
                item(attrs: attributes! { class="bg-secondary-background!" },
                    item_media(variant: ItemMediaVariant::Icon, icon(data: data))
                    item_content(
                        item_title((title))
                        item_description((description))
                    )
                    item_actions(
                        button(variant: ButtonVariant::NoShadow, size: ButtonSize::Sm, "View")
                    )
                )
            }
        )
    })
}

#[component]
async fn scroll_area_demo() -> Result<impl View> {
    Ok(view! {
        scroll_area(
            attrs: attributes! {
                class="rounded-base h-[300px] text-foreground border-2 border-border \
                    bg-secondary-background p-4 shadow-shadow"
            },
            "Jokester began sneaking into the castle in the middle of the night and leaving \
             jokes all over the place: under the king's pillow, in his soup, even in the royal \
             toilet. The king was furious, but he couldn't seem to stop Jokester. And then, one \
             day, the people of the kingdom discovered that the jokes left by Jokester were so \
             funny that they couldn't help but laugh. And once they started laughing, they \
             couldn't stop. Jokester began sneaking into the castle in the middle of the night \
             and leaving jokes all over the place: under the king's pillow, in his soup, even in \
             the royal toilet. The king was furious, but he couldn't seem to stop Jokester. And \
             then, one day, the people of the kingdom discovered that the jokes left by Jokester \
             were so funny that they couldn't help but laugh. And once they started laughing, \
             they couldn't stop."
        )
    })
}
