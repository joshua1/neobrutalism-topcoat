//! `with_keyboard_shortcut` uses `Kbd`, which is ported in another batch; it
//! is a plain `<kbd>` with the same classes here.

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, class, component, view},
};

use crate::components::button::{ButtonSize, ButtonVariant, button, button_variants};
use crate::components::tooltip::{
    TooltipSide, tooltip, tooltip_content, tooltip_provider, tooltip_trigger,
};

/// `Kbd` classes, from `kbd.tsrx`. Inside a tooltip it takes the main fill.
const KBD: &str = "pointer-events-none inline-flex h-5 w-fit min-w-5 items-center justify-center \
    gap-1 rounded-base border-2 border-border bg-secondary-background px-1 font-sans text-xs \
    font-heading text-foreground select-none \
    in-data-[slot=tooltip-content]:border-main-foreground/30 in-data-[slot=tooltip-content]:bg-main \
    in-data-[slot=tooltip-content]:text-main-foreground [&_svg:not([class*='size-'])]:size-3";

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        tooltip_provider(
            tooltip(
                tooltip_trigger(
                    attrs: attributes! {
                        class=(button_variants(ButtonVariant::NoShadow, ButtonSize::Default))
                    },
                    "Hover"
                )
                tooltip_content(<p>"Add to library"</p>)
            )
        )
    })
}

#[component]
pub async fn disabled_button() -> Result<impl View> {
    // A disabled button gets no pointer events, so the hover lands on the
    // wrapper span, which shows the tooltip.
    Ok(view! {
        tooltip_provider(
            tooltip(
                <span class="inline-block w-fit" data-slot="tooltip-trigger">
                    button(
                        variant: ButtonVariant::NoShadow,
                        attrs: attributes! { disabled="" },
                        "Disabled"
                    )
                </span>
                tooltip_content(<p>"This feature is currently unavailable"</p>)
            )
        )
    })
}

#[component]
pub async fn side() -> Result<impl View> {
    Ok(view! {
        tooltip_provider(
            <div class="flex flex-wrap justify-center gap-2">
                for (name, side) in [
                    ("left", TooltipSide::Left),
                    ("top", TooltipSide::Top),
                    ("bottom", TooltipSide::Bottom),
                    ("right", TooltipSide::Right),
                ] {
                    tooltip(
                        tooltip_trigger(
                            attrs: attributes! {
                                class=(class!(
                                    button_variants(ButtonVariant::NoShadow, ButtonSize::Default),
                                    "w-fit capitalize",
                                ))
                            },
                            (name)
                        )
                        tooltip_content(side: side, <p>"Add to library"</p>)
                    )
                }
            </div>
        )
    })
}

#[component]
pub async fn with_keyboard_shortcut() -> Result<impl View> {
    Ok(view! {
        tooltip_provider(
            tooltip(
                tooltip_trigger(
                    attrs: attributes! {
                        class=(button_variants(ButtonVariant::NoShadow, ButtonSize::Icon))
                        aria-label="Save Changes"
                    },
                    icon(data: iconify_icon!("lucide:save"))
                )
                tooltip_content(
                    "Save Changes "
                    <kbd data-slot="kbd" class=(KBD)>"S"</kbd>
                )
            )
        )
    })
}
