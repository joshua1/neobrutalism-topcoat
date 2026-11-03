use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

use crate::components::bubble::{
    BUBBLE_CONTENT, BubbleAlign, BubbleSide, BubbleVariant, bubble, bubble_content,
    bubble_reactions,
};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        <div class="flex w-full max-w-sm flex-col gap-4">
            bubble(variant: BubbleVariant::Muted,
                bubble_content("Hey! I just pushed the new registry build. Can you take a look?")
            )
            bubble(align: BubbleAlign::End,
                bubble_content("On it. I will run the checks and get back to you in a minute.")
            )
        </div>
    })
}

#[component]
pub async fn as_button() -> Result<impl View> {
    Ok(view! {
        <div class="flex w-full max-w-sm flex-col gap-4">
            bubble(variant: BubbleVariant::Muted,
                <button type="button" data-slot="bubble-content" class=(BUBBLE_CONTENT)>
                    "This bubble is rendered as a button. Hover or focus it to see the \
                     interactive state."
                </button>
            )
            bubble(align: BubbleAlign::End,
                <a href="#" data-slot="bubble-content" class=(BUBBLE_CONTENT)>
                    "This one is rendered as a link."
                </a>
            )
        </div>
    })
}

#[component]
pub async fn variants() -> Result<impl View> {
    Ok(view! {
        <div class="flex w-full max-w-sm flex-col gap-4">
            bubble(bubble_content("Default bubbles use the main color."))
            bubble(variant: BubbleVariant::Secondary,
                bubble_content("Secondary bubbles sit on a white surface.")
            )
            bubble(variant: BubbleVariant::Muted,
                bubble_content("Muted bubbles blend into the background.")
            )
            bubble(variant: BubbleVariant::Destructive,
                bubble_content("Destructive bubbles flag failed actions.")
            )
        </div>
    })
}

#[component]
pub async fn with_reactions() -> Result<impl View> {
    Ok(view! {
        <div class="flex w-full max-w-sm flex-col gap-8">
            bubble(variant: BubbleVariant::Muted,
                bubble_content("The new landing page is live. Thanks everyone for the hard work!")
                bubble_reactions(align: BubbleAlign::Start,
                    icon(data: iconify_icon!("lucide:heart"), attrs: attributes! { class="size-3.5" })
                    <span class="text-xs">"12"</span>
                )
            )
            bubble(align: BubbleAlign::End,
                bubble_content("Great job team, that was a big one.")
                bubble_reactions(side: BubbleSide::Top,
                    icon(data: iconify_icon!("lucide:thumbs-up"), attrs: attributes! { class="size-3.5" })
                    <span class="text-xs">"3"</span>
                )
            )
        </div>
    })
}
