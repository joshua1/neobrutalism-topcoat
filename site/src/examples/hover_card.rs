use topcoat::{
    Result,
    view::{View, attributes, class, component, view},
};

use crate::components::button::{ButtonSize, ButtonVariant, button_variants};
use crate::components::hover_card::{
    HoverCardSide, hover_card, hover_card_content, hover_card_trigger,
};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        hover_card(
            hover_card_trigger(
                attrs: attributes! {
                    href="#"
                    class=(button_variants(ButtonVariant::NoShadow, ButtonSize::Default))
                },
                "Hover"
            )
            hover_card_content("The compiler-first UI framework.")
        )
    })
}

#[component]
pub async fn basic() -> Result<impl View> {
    Ok(view! {
        hover_card(delay: 10, close_delay: 100,
            hover_card_trigger(
                attrs: attributes! {
                    href="#"
                    class="font-heading underline underline-offset-4"
                },
                "Hover Here"
            )
            hover_card_content(attrs: attributes! { class="flex w-64 flex-col gap-0.5" },
                <div class="font-heading">"@octanejs"</div>
                <div>"The compiler-first UI framework."</div>
            )
        )
    })
}

#[component]
pub async fn sides() -> Result<impl View> {
    // `!` overrides the button's main fill and text color.
    Ok(view! {
        <div class="flex flex-wrap justify-center gap-2">
            for (name, side) in [
                ("left", HoverCardSide::Left),
                ("top", HoverCardSide::Top),
                ("bottom", HoverCardSide::Bottom),
                ("right", HoverCardSide::Right),
            ] {
                hover_card(delay: 100, close_delay: 100,
                    hover_card_trigger(
                        attrs: attributes! {
                            href="#"
                            class=(class!(
                                button_variants(ButtonVariant::NoShadow, ButtonSize::Default),
                                "bg-secondary-background! text-foreground! capitalize",
                            ))
                        },
                        (name)
                    )
                    hover_card_content(side: side,
                        <div class="flex flex-col gap-1">
                            <h4 class="font-heading">"Hover Card"</h4>
                            <p>
                                "This hover card appears on the " (name) " side of the trigger."
                            </p>
                        </div>
                    )
                )
            }
        </div>
    })
}
