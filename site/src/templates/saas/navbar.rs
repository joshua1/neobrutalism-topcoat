//! The sticky header, with a menu that drops down on small screens.

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    runtime::{Event, signal},
    view::{View, attributes, component, view},
};

use crate::components::button::{ButtonSize, ButtonVariant, button};

const LINKS: [(&str, &str); 4] = [
    ("Features", "#features"),
    ("Product", "#product"),
    ("Pricing", "#pricing"),
    ("FAQ", "#faq"),
];

/// The product's logo mark and name, linking to the top of the page.
#[component]
pub async fn logo(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <a href="#" class="flex items-center gap-3 font-heading text-xl">
            <span class="flex size-8 items-center justify-center rounded-base border-2 border-border bg-main text-main-foreground">
                "P"
            </span>
            "Planly"
        </a>
    })
}

#[component]
pub async fn navbar(cx: &Cx) -> Result<impl View> {
    // Whether the small-screen menu is open.
    let open = signal(cx, || false);

    Ok(view! {
        <header class="sticky top-0 z-40 border-b-4 border-border bg-secondary-background">
            <div class="mx-auto flex h-16 w-full max-w-6xl items-center justify-between px-5">
                logo()

                <nav class="hidden items-center gap-8 text-sm font-base md:flex">
                    for (label, href) in LINKS {
                        <a href=(href) class="decoration-2 underline-offset-4 hover:underline">(label)</a>
                    }
                </nav>

                <div class="hidden items-center gap-3 md:flex">
                    button(variant: ButtonVariant::Neutral, size: ButtonSize::Sm, "Sign in")
                    button(size: ButtonSize::Sm, "Start free trial")
                </div>

                button(
                    variant: ButtonVariant::Neutral,
                    size: ButtonSize::IconSm,
                    attrs: attributes! {
                        type="button"
                        class="md:hidden"
                        :aria-label=$(if open.get() { "Close menu" } else { "Open menu" })
                        :aria-expanded=$(if open.get() { "true" } else { "false" })
                        @click=$(|_e: Event| open.toggle())
                    },
                    <span :hidden=$(open.get())>icon(data: iconify_icon!("lucide:menu"))</span>
                    <span :hidden=$(!open.get())>icon(data: iconify_icon!("lucide:x"))</span>
                )
            </div>

            <div class="border-t-4 border-border md:hidden" :hidden=$(!open.get())>
                <nav class="mx-auto flex w-full max-w-6xl flex-col gap-1 px-5 py-4">
                    #[key(href)]
                    for (label, href) in LINKS {
                        <a
                            href=(href)
                            @click=$(|_e: Event| open.set(false))
                            class="rounded-base px-3 py-2 font-base hover:bg-background"
                        >
                            (label)
                        </a>
                    }
                    <div class="mt-3 grid grid-cols-2 gap-3">
                        button(variant: ButtonVariant::Neutral, "Sign in")
                        button("Start free trial")
                    </div>
                </nav>
            </div>
        </header>
    })
}
