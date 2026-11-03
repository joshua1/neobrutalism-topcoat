use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

use crate::components::{
    button::{ButtonSize, ButtonVariant, button},
    kbd::{kbd, kbd_group},
};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        <div class="flex items-center gap-2">
            kbd("Ctrl")
            kbd("⌘K")
            kbd("Ctrl + B")
        </div>
    })
}

#[component]
pub async fn group() -> Result<impl View> {
    Ok(view! {
        kbd_group(kbd("Ctrl") kbd("Shift") kbd("P"))
    })
}

#[component]
pub async fn with_icon() -> Result<impl View> {
    Ok(view! {
        kbd_group(
            kbd(icon(data: iconify_icon!("lucide:circle-dashed")))
            kbd(icon(data: iconify_icon!("lucide:arrow-left")))
            kbd(icon(data: iconify_icon!("lucide:arrow-right")))
        )
    })
}

#[component]
pub async fn in_tooltip() -> Result<impl View> {
    // A stand-in for the `tooltip` component: CSS shows the content while the
    // trigger is hovered or focused. Its `data-slot="tooltip-content"` gives
    // the key its in-tooltip colors.
    Ok(view! {
        <span class="group/tooltip relative inline-flex">
            button(
                variant: ButtonVariant::Neutral,
                size: ButtonSize::Icon,
                attrs: attributes! {
                    aria-label="Save"
                    aria-describedby="kbd-tooltip"
                    class="shadow-none! hover:translate-x-0! hover:translate-y-0!"
                },
                icon(data: iconify_icon!("lucide:save"))
            )
            <div
                id="kbd-tooltip"
                role="tooltip"
                data-slot="tooltip-content"
                class="invisible absolute bottom-full left-1/2 z-50 mb-2 -translate-x-1/2 \
                       overflow-hidden whitespace-nowrap rounded-base border-2 border-border \
                       bg-secondary-background px-3 py-1.5 pr-1.5 text-sm font-base \
                       text-foreground opacity-0 transition-opacity \
                       group-hover/tooltip:visible group-hover/tooltip:opacity-100 \
                       group-focus-within/tooltip:visible group-focus-within/tooltip:opacity-100"
            >
                <div class="flex items-center gap-2">"Save Changes " kbd("S")</div>
            </div>
        </span>
    })
}
