//! Routes. This module is `/`; `docs` serves `/docs/{slug}`, `styling` the
//! theme generator at `/styling`.

mod docs;
mod styling;

use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    cookie::{Cookies, RouterBuilderCookieExt, cookies},
    router::{Router, RouterBuilderDiscoverExt, Slot, layout, module_router, page},
    runtime::RouterBuilderRuntimeExt,
    tailwind,
    view::{Unescaped, View, component, view},
};

use crate::{
    catalog::{DOCS, doc_href},
    theme::{self, FONTS, Theme},
};

pub fn router() -> Router {
    module_router!()
        .discover()
        .assets(AssetBundle::load().unwrap())
        .cookies()
        .runtime()
        .build()
}

/// The theme saved by the styling page, if the visitor has one.
pub fn visitor_theme(cx: &Cx) -> Option<Theme> {
    cookies(cx)
        .get(theme::COOKIE)
        .map(|cookie| Theme::from_cookie(cookie.value()))
}

/// The HTML document and the fixed navbar around every page.
///
/// A saved theme is applied here, on the server, so every page renders in
/// it from the first paint.
#[layout]
async fn root_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    let theme = visitor_theme(cx);
    // Built only from validated values (see `Theme::from_cookie`), so it is
    // safe to embed unescaped.
    let overrides = theme
        .as_ref()
        .map(|theme| Unescaped::new_unchecked(theme.root_block()));
    let font_href = theme
        .as_ref()
        .filter(|theme| theme.font.name != FONTS[0].name)
        .map(|theme| theme.font.href());

    Ok(view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"Neobrutalism components for Topcoat"</title>
                <link rel="preconnect" href="https://fonts.googleapis.com">
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="">
                <link
                    rel="stylesheet"
                    href="https://fonts.googleapis.com/css2?family=DM+Sans:opsz,wght@9..40,100..1000&display=swap"
                >
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
                // The styling page swaps this link's href when the font changes.
                <link id="neo-font" rel="stylesheet" href=(font_href)>
                if let Some(css) = overrides {
                    <style id="neo-theme">(css)</style>
                }
                topcoat::runtime::script()
                topcoat::dev::script()
            </head>
            <body class="min-h-dvh bg-background">
                navbar()
                (slot)
            </body>
        </html>
    })
}

#[component]
async fn navbar() -> Result<impl View> {
    Ok(view! {
        <nav class="fixed left-0 top-0 z-20 flex h-[70px] w-full items-center border-b-4 border-border bg-secondary-background px-5">
            <div class="mx-auto flex w-[1300px] max-w-full items-center justify-between text-foreground">
                <a href="/" class="flex items-center gap-3 font-heading text-xl">
                    <span class="flex size-8 items-center justify-center rounded-base border-2 border-black bg-main text-[22px] text-main-foreground">
                        "N"
                    </span>
                    <span class="hidden sm:inline">"neobrutalism"</span>
                    <span class="bg-chart-3 px-1.5 text-[10px] uppercase text-main-foreground">"topcoat"</span>
                </a>
                <div class="flex items-center gap-8 font-base">
                    <a href="/docs" class="hover:underline">"Docs"</a>
                    <a href="/docs/button" class="hover:underline">"Components"</a>
                    <a href="/styling" class="flex items-center gap-2 hover:underline">
                        "Styling"
                        <span class="bg-chart-3 px-1.5 text-[10px] uppercase text-main-foreground">"new"</span>
                    </a>
                    <a
                        href="https://topcoat.rs"
                        class="flex h-9 items-center rounded-base border-2 border-border px-3 shadow-shadow transition-all hover:translate-x-boxShadowX hover:translate-y-boxShadowY hover:shadow-none"
                    >
                        "Topcoat"
                    </a>
                </div>
            </div>
        </nav>
    })
}

/// The landing page: what this is, how to install, and every component.
#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <main class="bg-[linear-gradient(to_right,#80808033_1px,transparent_1px),linear-gradient(to_bottom,#80808033_1px,transparent_1px)] bg-size-[70px_70px] px-5 pt-[70px]">
            <section class="mx-auto max-w-[900px] py-20 text-center">
                <h1 class="font-heading text-4xl sm:text-6xl">"Neobrutalism components"</h1>
                <p class="mx-auto mt-6 max-w-[650px] text-lg">
                    "A collection of neobrutalism-styled components for Topcoat, the Rust web framework. "
                    "Install them into your app with the topcoat CLI, then edit them like your own code."
                </p>
                <pre class="mx-auto mt-10 w-fit max-w-full overflow-x-auto rounded-base border-2 border-border bg-secondary-background px-5 py-3 text-left text-sm shadow-shadow"><code>"topcoat ui add button --registry neobrutalism"</code></pre>
            </section>
            <section class="mx-auto grid max-w-[1300px] grid-cols-2 gap-5 pb-24 sm:grid-cols-3 lg:grid-cols-4">
                for doc in DOCS {
                    <a
                        href=(doc_href(doc))
                        class="rounded-base border-2 border-border bg-secondary-background p-5 shadow-shadow transition-all hover:translate-x-boxShadowX hover:translate-y-boxShadowY hover:shadow-none"
                    >
                        <h2 class="font-heading text-lg">(doc.title)</h2>
                        <p class="mt-2 line-clamp-2 text-sm">(doc.description)</p>
                    </a>
                }
            </section>
        </main>
    })
}
