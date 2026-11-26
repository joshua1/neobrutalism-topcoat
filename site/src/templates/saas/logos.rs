//! A strip of customer logos scrolling sideways, without end.

use topcoat::{
    Result,
    context::Cx,
    view::{View, attributes, class, component, view},
};

use super::decor_star::decor_star;
use crate::components::stars::Star;

const LOGOS: [(&str, Star, &str); 8] = [
    ("Acme", Star::S25, "var(--chart-1)"),
    ("Globex", Star::S5, "var(--chart-3)"),
    ("Initech", Star::S6, "var(--chart-2)"),
    ("Umbrella", Star::S22, "var(--chart-4)"),
    ("Hooli", Star::S19, "var(--chart-1)"),
    ("Soylent", Star::S27, "var(--chart-3)"),
    ("Cyberdyne", Star::S24, "var(--chart-2)"),
    ("Pied Piper", Star::S28, "var(--chart-4)"),
];

/// One copy of the row. Two copies scroll in step (the theme's
/// `animate-marquee` and `animate-marquee2`), so the loop has no gap.
#[component]
async fn track(cx: &Cx, class: &str, #[default] hidden: bool) -> Result<impl View> {
    Ok(view! {
        <div
            aria-hidden=(hidden.then_some("true"))
            class=(class!("flex shrink-0 items-center motion-reduce:animate-none", class))
            style="animation-duration: 35s"
        >
            for (name, shape, color) in LOGOS {
                <div class="mx-8 flex shrink-0 items-center gap-6 whitespace-nowrap font-heading text-3xl">
                    decor_star(shape: shape, color: color, attrs: attributes! { class="size-9" })
                    (name)
                </div>
            }
        </div>
    })
}

#[component]
pub async fn logos(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <section class="border-b-4 border-border bg-secondary-background py-8">
            <div class="relative flex overflow-hidden">
                track(class: "animate-marquee")
                track(class: "absolute left-0 top-0 animate-marquee2", hidden: true)
            </div>
        </section>
    })
}
