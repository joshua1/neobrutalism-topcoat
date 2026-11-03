//! `class!` does not drop conflicting classes the way Octane's `cn` does, so
//! classes that replace a part's own (`basis-1/2!` over `basis-full`, `-ml-1!`
//! over `-ml-4`, `p-0!` over the card's padding) carry Tailwind's `!`.
//! `shadow-none` stays plain: tailwind-merge does not treat the card's
//! `shadow-shadow` as a conflict, so in Octane both classes apply and the
//! card's shadow wins, and the slides keep it here too.

use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use crate::components::{
    card::{card, card_content},
    carousel::{
        CarouselOrientation, carousel, carousel_content, carousel_item, carousel_next,
        carousel_previous,
    },
};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        <div class="w-full flex-col items-center gap-4 flex">
            carousel(attrs: attributes! { class="w-full max-w-[200px]" },
                carousel_content(
                    #[key(index)]
                    for index in 1..=5 {
                        carousel_item(
                            <div class="p-[10px]">
                                card(attrs: attributes! { class="shadow-none p-0! bg-background text-foreground" },
                                    card_content(attrs: attributes! { class="flex aspect-square items-center justify-center p-4!" },
                                        <span class="text-3xl font-base">(index)</span>
                                    )
                                )
                            </div>
                        )
                    }
                )
                carousel_previous()
                carousel_next()
            )
        </div>
    })
}

#[component]
pub async fn sizes() -> Result<impl View> {
    Ok(view! {
        carousel(attrs: attributes! { class="w-full max-w-[12rem] sm:max-w-xs md:max-w-sm" },
            carousel_content(
                #[key(index)]
                for index in 1..=5 {
                    carousel_item(attrs: attributes! { class="basis-1/2! lg:basis-1/3!" },
                        <div class="p-1">
                            card(attrs: attributes! { class="p-0! shadow-none" },
                                card_content(attrs: attributes! { class="flex aspect-square items-center justify-center p-6!" },
                                    <span class="text-3xl font-heading">(index)</span>
                                )
                            )
                        </div>
                    )
                }
            )
            carousel_previous()
            carousel_next()
        )
    })
}

#[component]
pub async fn spacing() -> Result<impl View> {
    Ok(view! {
        carousel(attrs: attributes! { class="w-full max-w-[12rem] sm:max-w-xs md:max-w-sm" },
            carousel_content(attrs: attributes! { class="-ml-1!" },
                #[key(index)]
                for index in 1..=5 {
                    carousel_item(attrs: attributes! { class="basis-1/2! pl-1! lg:basis-1/3!" },
                        <div class="p-1">
                            card(attrs: attributes! { class="p-0! shadow-none" },
                                card_content(attrs: attributes! { class="flex aspect-square items-center justify-center p-6!" },
                                    <span class="text-2xl font-heading">(index)</span>
                                )
                            )
                        </div>
                    )
                }
            )
            carousel_previous()
            carousel_next()
        )
    })
}

#[component]
pub async fn orientation() -> Result<impl View> {
    Ok(view! {
        carousel(
            orientation: CarouselOrientation::Vertical,
            attrs: attributes! { class="w-full max-w-xs" },
            carousel_content(attrs: attributes! { class="-mt-1! h-[270px]" },
                #[key(index)]
                for index in 1..=5 {
                    carousel_item(attrs: attributes! { class="basis-1/2! pt-1!" },
                        <div class="p-1">
                            card(attrs: attributes! { class="p-0! shadow-none" },
                                card_content(attrs: attributes! { class="flex items-center justify-center p-6!" },
                                    <span class="text-3xl font-heading">(index)</span>
                                )
                            )
                        </div>
                    )
                }
            )
            carousel_previous()
            carousel_next()
        )
    })
}

/// Writes the current slide into the status line below the carousel, the way
/// Octane's example does from `api.on("select", ..)`.
const SHOW_SLIDE: &str =
    "document.getElementById('carousel-api-status').textContent=`Slide ${index+1} of ${count}`";

#[component]
pub async fn api() -> Result<impl View> {
    Ok(view! {
        <div class="mx-auto max-w-[10rem] sm:max-w-xs">
            carousel(on_select: Some(SHOW_SLIDE), attrs: attributes! { class="w-full max-w-xs" },
                carousel_content(
                    #[key(index)]
                    for index in 1..=5 {
                        carousel_item(
                            <div class="p-1">
                                card(attrs: attributes! { class="p-0! shadow-none" },
                                    card_content(attrs: attributes! { class="flex aspect-square items-center justify-center p-6!" },
                                        <span class="text-4xl font-heading">(index)</span>
                                    )
                                )
                            </div>
                        )
                    }
                )
                carousel_previous()
                carousel_next()
            )
            <div id="carousel-api-status" class="py-2 text-center text-sm text-foreground">
                "Slide 1 of 5"
            </div>
        </div>
    })
}
