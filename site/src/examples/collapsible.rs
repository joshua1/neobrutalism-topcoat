//! The Octane examples use `Card`, `Field` and `Input`, which are ported in
//! another batch; they are plain elements with the same classes here. The
//! state the Octane examples kept in `useState` lives in the `<details>`.

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, class, component, view},
};

use crate::components::button::{ButtonSize, ButtonVariant, button, button_variants};
use crate::components::collapsible::{collapsible, collapsible_content, collapsible_trigger};

/// `Card` classes, from `card.tsrx`.
const CARD: &str = "group/card rounded-base flex flex-col shadow-shadow border-2 \
    gap-(--card-spacing) py-(--card-spacing) border-border bg-background text-foreground \
    font-base [--card-spacing:--spacing(6)]";

/// `Input` classes, from `input.tsrx`.
const INPUT: &str = "flex h-10 w-full rounded-base border-2 border-border bg-secondary-background \
    selection:bg-main selection:text-main-foreground px-3 py-2 text-sm font-base text-foreground \
    placeholder:text-foreground/50 focus-visible:outline-hidden focus-visible:ring-2 \
    focus-visible:ring-black focus-visible:ring-offset-2 disabled:cursor-not-allowed \
    disabled:opacity-50";

#[component]
pub async fn demo() -> Result<impl View> {
    // The trigger sits inside the header row while the first package stays
    // visible, so the always-visible rows are outside the `<details>` and the
    // trigger is positioned over the header's empty slot (inside its 2px
    // border and `py-2` / `px-4` padding).
    Ok(view! {
        <div class="relative w-[350px] space-y-2">
            <div class="rounded-base flex items-center justify-between space-x-4 border-2 \
                border-border text-foreground bg-background px-4 py-2">
                <h4 class="text-sm font-heading">"@octanejs published 3 packages"</h4>
                <span class="h-9 w-9"></span>
            </div>
            <div class="rounded-base border-2 border-border bg-background px-4 py-3 font-mono \
                font-base text-foreground text-sm">
                "@octanejs/base-ui"
            </div>
            collapsible(
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
                    <div class="rounded-base border-2 border-border bg-background px-4 py-3 \
                        font-mono text-sm">
                        "@octanejs/shadcn"
                    </div>
                    <div class="rounded-base border-2 border-border bg-background px-4 py-3 \
                        font-mono text-sm">
                        "@octanejs/lucide"
                    </div>
                )
            )
        </div>
    })
}

#[component]
pub async fn basic() -> Result<impl View> {
    Ok(view! {
        <div class=(class!(CARD, "mx-auto w-full max-w-sm"))>
            <div class="px-(--card-spacing)">
                collapsible(
                    collapsible_trigger(
                        attrs: attributes! {
                            // `!` overrides the button's main fill and text color.
                            class=(class!(
                                button_variants(ButtonVariant::NoShadow, ButtonSize::Default),
                                "w-full bg-secondary-background! text-foreground!",
                            ))
                        },
                        "Product details"
                        icon(
                            data: iconify_icon!("lucide:chevron-down"),
                            attrs: attributes! {
                                class="ml-auto transition-transform group-open/collapsible:rotate-180"
                            }
                        )
                    )
                    collapsible_content(
                        attrs: attributes! {
                            class="mt-2 flex flex-col items-start gap-2 rounded-base border-2 \
                                border-border bg-secondary-background p-4 text-sm"
                        },
                        <div>
                            "This panel can be expanded or collapsed to reveal additional content."
                        </div>
                        button(size: ButtonSize::Sm, "Learn More")
                    )
                )
            </div>
        </div>
    })
}

#[component]
pub async fn settings_panel() -> Result<impl View> {
    // The top two fields stay visible, so they sit outside the `<details>`;
    // the trigger is positioned beside them. In Octane the trigger is a flex
    // item next to a `w-full` field group, so it shrinks from 40px to about
    // 34px; `w-[34px]!` and the `pr-[42px]` gutter reproduce that.
    Ok(view! {
        <div class=(class!(CARD, "mx-auto w-full max-w-xs"))>
            <div class="@container/card-header grid auto-rows-min grid-rows-[auto_auto] \
                items-start gap-1.5 px-(--card-spacing)">
                <div class="font-heading leading-none">"Radius"</div>
                <div class="text-sm font-base">"Set the corner radius of the element."</div>
            </div>
            <div class="px-(--card-spacing)">
                <div class="relative pr-[42px]">
                    <div class="grid w-full grid-cols-2 gap-2">
                        <label for="radius-top-left" class="sr-only">"Radius top left"</label>
                        <input id="radius-top-left" placeholder="0" value="0" class=(INPUT)>
                        <label for="radius-top-right" class="sr-only">"Radius top right"</label>
                        <input id="radius-top-right" placeholder="0" value="0" class=(INPUT)>
                    </div>
                    collapsible(
                        collapsible_trigger(
                            attrs: attributes! {
                                // `!` overrides the button's main fill and text color.
                                class=(class!(
                                    button_variants(ButtonVariant::NoShadow, ButtonSize::Icon),
                                    "absolute top-0 right-0 w-[34px]! bg-secondary-background! \
                                     text-foreground!",
                                ))
                                aria-label="Toggle more corners"
                            },
                            icon(
                                data: iconify_icon!("lucide:maximize"),
                                attrs: attributes! { class="group-open/collapsible:hidden" }
                            )
                            icon(
                                data: iconify_icon!("lucide:minimize"),
                                attrs: attributes! { class="hidden group-open/collapsible:block" }
                            )
                        )
                        collapsible_content(attrs: attributes! { class="mt-2 grid grid-cols-2 gap-2" },
                            <label for="radius-bottom-left" class="sr-only">"Radius bottom left"</label>
                            <input id="radius-bottom-left" placeholder="0" value="0" class=(INPUT)>
                            <label for="radius-bottom-right" class="sr-only">
                                "Radius bottom right"
                            </label>
                            <input id="radius-bottom-right" placeholder="0" value="0" class=(INPUT)>
                        )
                    )
                </div>
            </div>
        </div>
    })
}
