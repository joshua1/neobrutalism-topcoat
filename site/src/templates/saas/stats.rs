//! A yellow band of four big numbers.

use topcoat::{
    Result,
    context::Cx,
    view::{View, class, component, view},
};

/// Value and label.
const STATS: [(&str, &str); 4] = [
    ("2,000+", "teams shipping with Planly"),
    ("14M", "issues closed last year"),
    ("99.99%", "uptime over 12 months"),
    ("4.9/5", "average review rating"),
];

#[component]
pub async fn stats(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <section class="border-b-4 border-border bg-chart-3 text-foreground">
            <dl class="mx-auto grid w-full max-w-6xl grid-cols-1 sm:grid-cols-2 md:grid-cols-4">
                for (i, (value, label)) in STATS.into_iter().enumerate() {
                    // One column below sm, 2×2 from sm, one row from md.
                    <div class=(class!(
                        "flex flex-col-reverse justify-end gap-1 border-border px-5 py-10 text-center",
                        "border-b-4 md:border-b-0" if i < 3,
                        "sm:border-b-0" if i == 2,
                        "sm:border-l-4" if i % 2 == 1,
                        "md:border-l-4" if i == 0 || i == 2,
                        "md:border-r-4" if i == STATS.len() - 1,
                    ))>
                        <dt class="text-sm font-base">(label)</dt>
                        <dd class="font-heading text-4xl sm:text-5xl md:text-4xl lg:text-5xl">(value)</dd>
                    </div>
                }
            </dl>
        </section>
    })
}
