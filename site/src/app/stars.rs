//! `/stars`: every star shape, with the code to draw it.

use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, component, view},
};

use crate::components::{
    button::{ButtonSize, ButtonVariant, button},
    stars::{Star, star},
};

/// The gallery of the 40 star shapes.
#[page]
async fn stars(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <main class="min-h-dvh bg-background bg-[linear-gradient(to_right,#80808033_1px,transparent_1px),linear-gradient(to_bottom,#80808033_1px,transparent_1px)] bg-size-[70px_70px] px-5 pt-[70px]">
            <div class="mx-auto w-[1300px] max-w-full py-16 text-foreground">
                <div class="mb-[50px] flex flex-col gap-8 text-center">
                    <h1 class="font-heading text-2xl xl:text-3xl 2xl:text-4xl">"Stars"</h1>
                    <p class="mx-auto max-w-[800px] font-base text-base sm:text-lg xl:text-xl">
                        "40 geometric shapes often used in neobrutalism layouts, all drawn by one component. "
                        "Install it with "
                        <code class="rounded-base border-2 border-border bg-secondary-background px-1.5 py-0.5 text-base">
                            "topcoat ui add stars --registry neobrutalism"
                        </code>
                        " and see the "
                        <a href="/docs/stars" class="font-heading underline decoration-2 underline-offset-2">"docs"</a>
                        " for its props."
                    </p>
                </div>
                <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 xl:gap-[50px]">
                    for shape in Star::ALL {
                        star_card(shape: shape)
                    }
                </div>
            </div>
        </main>
    })
}

/// One shape: its preview, name, and a button that copies the call.
#[component]
async fn star_card(cx: &Cx, shape: Star) -> Result<impl View> {
    let n = shape.number();
    let code = format!("star(shape: Star::S{n})");
    let copy = format!(
        "navigator.clipboard.writeText('{code}'); \
         this.textContent = 'Copied!'; setTimeout(() => this.textContent = 'Copy', 1500)"
    );
    Ok(view! {
        <div class="flex flex-col items-center justify-center gap-4 rounded-base border-2 border-border bg-secondary-background p-5 shadow-shadow">
            <div class="size-[120px] md:size-[160px] xl:size-[200px]">
                star(
                    shape: shape,
                    color: "var(--main)",
                    path_class: "stroke-black stroke-3",
                    attrs: attributes! { class="size-full" aria-hidden="true" }
                )
            </div>
            <h2 class="font-heading">(format!("Star {n}"))</h2>
            <code class="text-xs">(&code)</code>
            button(
                variant: ButtonVariant::Neutral,
                size: ButtonSize::Sm,
                attrs: attributes! { type="button" onclick=(copy) aria-label=(format!("Copy the code for star {n}")) },
                "Copy"
            )
        </div>
    })
}
