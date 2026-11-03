use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, class, component, view},
};

use crate::components::{
    button::{ButtonSize, ButtonVariant, button_variants},
    direction::{Direction, direction_provider},
};

/// The Octane example opens a `Popover` on its inline-end side. This uses a
/// native `<details>` instead, with the panel placed by logical `start-full`
/// and `ms-1` utilities (Base UI's 4px side offset), so in right-to-left text
/// it opens to the left. Octane portals the popover out of the right-to-left
/// subtree, so its English text runs left to right; `dir="ltr"` on the text
/// keeps that.
#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        direction_provider(direction: Direction::Rtl,
            <div class="flex flex-col items-start gap-4">
                <details class="relative">
                    <summary
                        class=(class!(
                            button_variants(ButtonVariant::Neutral, ButtonSize::Default),
                            "cursor-pointer list-none [&::-webkit-details-marker]:hidden",
                        ))
                    >
                        "افتح القائمة"
                        icon(data: iconify_icon!("lucide:arrow-right"))
                    </summary>
                    <div
                        class="absolute top-0 start-full z-50 ms-1 flex w-72 flex-col gap-4 \
                            rounded-base border-2 border-border bg-background p-4 text-foreground"
                    >
                        <p dir="ltr" class="text-sm font-base">
                            "This panel opens on the inline end side. In RTL that is the left, \
                             because the provider sets the direction for everything inside it."
                        </p>
                    </div>
                </details>
            </div>
        )
    })
}
