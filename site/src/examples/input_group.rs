use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

use crate::components::{
    button::ButtonVariant,
    input_group::{
        InputGroupAddonAlign, InputGroupButtonSize, input_group, input_group_addon,
        input_group_button, input_group_input, input_group_text, input_group_textarea,
    },
    kbd::kbd,
};

// `class!` appends without merging, so a class that replaces one the
// component already sets carries Tailwind's `!` (important) modifier.

#[component]
pub async fn block_start() -> Result<impl View> {
    Ok(view! {
        input_group(attrs: attributes! { class="max-w-sm" },
            input_group_input(attrs: attributes! { placeholder="John" })
            input_group_addon(align: InputGroupAddonAlign::BlockStart,
                input_group_text(attrs: attributes! { class="font-heading! text-foreground" },
                    "First Name"
                )
                icon(data: iconify_icon!("lucide:info"), attrs: attributes! { class="ml-auto" })
            )
        )
    })
}

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        input_group(attrs: attributes! { class="max-w-sm" },
            input_group_input(attrs: attributes! { placeholder="Search..." })
            input_group_addon(icon(data: iconify_icon!("lucide:search")))
            input_group_addon(align: InputGroupAddonAlign::InlineEnd, kbd("⌘K"))
        )
    })
}

#[component]
pub async fn textarea() -> Result<impl View> {
    Ok(view! {
        input_group(attrs: attributes! { class="max-w-md" },
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

#[component]
pub async fn with_button() -> Result<impl View> {
    Ok(view! {
        <div class="flex w-full max-w-sm flex-col gap-4">
            input_group(
                input_group_input(attrs: attributes! { placeholder="Search the docs..." })
                input_group_addon(align: InputGroupAddonAlign::InlineEnd,
                    input_group_button(
                        variant: Some(ButtonVariant::Default),
                        size: InputGroupButtonSize::Sm,
                        "Search"
                    )
                )
            )
            input_group(
                input_group_input(attrs: attributes! {
                    readonly="" value="https://neobrutalism.dev"
                })
                input_group_addon(align: InputGroupAddonAlign::InlineEnd,
                    input_group_button(
                        variant: Some(ButtonVariant::Neutral),
                        size: InputGroupButtonSize::IconSm,
                        attrs: attributes! { aria-label="Copy" },
                        icon(data: iconify_icon!("lucide:copy"))
                    )
                )
            )
        </div>
    })
}
