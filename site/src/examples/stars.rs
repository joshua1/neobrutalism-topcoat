use topcoat::{
    Result,
    context::Cx,
    view::{View, attributes, component, view},
};

use crate::components::stars::{Star, star};

#[component]
pub async fn demo(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        star(shape: Star::S8, color: "red", size: 200)
    })
}

#[component]
pub async fn with_stroke(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        star(shape: Star::S8, color: "red", size: 200, stroke: "black", stroke_width: 2.0)
    })
}

#[component]
pub async fn custom_width_height(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        star(
            shape: Star::S8,
            color: "red",
            stroke: "black",
            stroke_width: 2.0,
            attrs: attributes! { width="180" height="210" }
        )
    })
}

#[component]
pub async fn sticker(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        // A badge on a spinning star; `[vector-effect:non-scaling-stroke]`
        // keeps the outline 2px at any size.
        <div class="group relative flex size-40 items-center justify-center font-heading text-lg">
            star(
                shape: Star::S14,
                color: "var(--chart-3)",
                stroke: "var(--border)",
                stroke_width: 2.0,
                path_class: "[vector-effect:non-scaling-stroke]",
                attrs: attributes! {
                    aria-hidden="true"
                    class="absolute inset-0 size-full overflow-visible transition-transform duration-700 group-hover:rotate-45"
                }
            )
            <span class="relative -rotate-6">"New!"</span>
        </div>
    })
}
