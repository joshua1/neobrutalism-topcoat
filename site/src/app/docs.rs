//! `/docs`: the getting-started page, and one page per component at
//! `/docs/{slug}`. The sidebar is shared by both.

mod slug;

use topcoat::{
    Result,
    router::page,
    view::{View, class, component, view},
};

use crate::catalog::{DOCS, doc_href};

/// How to set up a Topcoat app and install the components.
#[page]
async fn getting_started() -> Result<impl View> {
    Ok(view! {
        <div class="min-h-dvh w-full pt-[70px]">
            sidebar(current: "")
            <main class="py-12 sm:py-16 lg:ml-[250px] lg:py-20">
                <article class="mx-auto w-full max-w-[750px] px-5 leading-relaxed">
                    <h1 class="font-heading text-2xl sm:text-3xl">"Getting started"</h1>
                    <p class="mt-4 text-base sm:text-lg">
                        "These components are a Topcoat UI registry. "
                        <code class=(INLINE_CODE)>"topcoat ui add"</code>
                        " copies a component's Rust source into your app, where it is yours to edit."
                    </p>

                    step(number: 1, title: "Add the dependencies",
                        <p>"Depend on the registry crate directly (path or git) and enable Topcoat's "
                            <code class=(INLINE_CODE)>"ui"</code> ", "
                            <code class=(INLINE_CODE)>"tailwind"</code> " and "
                            <code class=(INLINE_CODE)>"icon-iconify"</code> " features."</p>
                        code(text: CARGO_TOML)
                    )
                    step(number: 2, title: "Build the stylesheet and stage the icons",
                        <p>"Add a " <code class=(INLINE_CODE)>"build.rs"</code>
                            " next to Cargo.toml. Several components use Lucide icons."</p>
                        code(text: BUILD_RS)
                    )
                    step(number: 3, title: "Install the theme",
                        <p>"Run " <code class=(INLINE_CODE)>"topcoat ui init"</code>
                            " once, then replace the " <code class=(INLINE_CODE)>"styles.css"</code>
                            " it writes with the registry's theme, "
                            <code class=(INLINE_CODE)>"themes/neobrutalism.css"</code>
                            ". Topcoat 0.10 only installs themes from its built-in registry, so this step is a copy."</p>
                        code(text: THEME)
                        <p class="mt-4">"Want your own colors? Build a theme on the "
                            <a href="/styling" class="font-heading underline decoration-2 underline-offset-2">"Styling"</a>
                            " page and copy or download its " <code class=(INLINE_CODE)>"styles.css"</code>
                            " instead."</p>
                        <p class="mt-4">"Restyle everything by editing the variables in "
                            <code class=(INLINE_CODE)>":root"</code> ": "
                            <code class=(INLINE_CODE)>"--main"</code> ", "
                            <code class=(INLINE_CODE)>"--border-radius"</code> ", "
                            <code class=(INLINE_CODE)>"--box-shadow-x"</code> ", and so on."</p>
                    )
                    step(number: 4, title: "Load the stylesheet, font and runtime",
                        <p>"Components that change state in the browser (calendar, controlled examples) use Topcoat's runtime; the rest work with plain HTML and small inline scripts."</p>
                        code(text: LAYOUT_RS)
                    )
                    step(number: 5, title: "Add components",
                        code(text: ADD)
                        <p class="mt-4">"Then use them like any Rust function component:"</p>
                        code(text: USAGE_RS)
                    )

                    <h2 class="mt-14 mb-6 font-heading text-xl sm:text-2xl">"Good to know"</h2>
                    <ul class="ml-5 list-disc space-y-2">
                        <li>"There is no tailwind-merge: to override a class a component already sets, use Tailwind's important suffix, e.g. "
                            <code class=(INLINE_CODE)>"size-6!"</code> "."</li>
                        <li>"Overlays use modern platform features: " <code class=(INLINE_CODE)>"<dialog>"</code>
                            ", the popover API, CSS anchor positioning and customizable "
                            <code class=(INLINE_CODE)>"<select>"</code>
                            ". Each component's doc comment says what happens in browsers without them."</li>
                        <li>"Some components use small inline event handlers, so a Content-Security-Policy must allow inline scripts."</li>
                    </ul>
                </article>
            </main>
        </div>
    })
}

const INLINE_CODE: &str = "mx-0.5 whitespace-nowrap rounded-base border-2 border-border bg-secondary-background px-[5px] py-[2px] text-sm";

const CARGO_TOML: &str = r#"[dependencies]
topcoat = { version = "0.10.0", features = ["ui", "tailwind", "icon-iconify"] }
neobrutalism = { path = "../neo-octane/topcoat" }
# or: neobrutalism = { git = "https://github.com/<you>/neo-octane" }

[build-dependencies]
topcoat = { version = "0.10.0", default-features = false, features = ["tailwind", "icon-iconify"] }"#;

const BUILD_RS: &str = r#"fn main() {
    for path in ["src", "styles.css", "build.rs"] {
        println!("cargo:rerun-if-changed={path}");
    }
    topcoat::tailwind::BuildConfig::new()
        .input("styles.css")
        .render()
        .unwrap();
    topcoat::icon::iconify::BuildConfig::new()
        .icon_set("lucide")
        .stage()
        .unwrap();
}"#;

const THEME: &str = r#"topcoat ui init
cp ../neo-octane/topcoat/themes/neobrutalism.css styles.css"#;

const LAYOUT_RS: &str = r#"use topcoat::{Result, router::{Slot, layout}, tailwind, view::{View, view}};

#[layout]
async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html>
            <head>
                // The theme's font; or set --base-font-family to your own.
                <link
                    rel="stylesheet"
                    href="https://fonts.googleapis.com/css2?family=DM+Sans:opsz,wght@9..40,100..1000&display=swap"
                >
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
                topcoat::runtime::script()
            </head>
            <body>(slot)</body>
        </html>
    })
}

// And on the router: .assets(AssetBundle::load().unwrap()).runtime()"#;

const ADD: &str = r#"topcoat ui add button card dialog --registry neobrutalism
# or everything:
topcoat ui add --all --registry neobrutalism"#;

const USAGE_RS: &str = r#"use crate::components::button::{ButtonVariant, button};

view! {
    button(variant: ButtonVariant::Reverse, "Save")
}"#;

/// A numbered setup step.
#[component]
async fn step(number: u8, title: &str, child: topcoat::view::Child<'_>) -> Result<impl View> {
    Ok(view! {
        <section class="mt-12">
            <h2 class="mb-4 flex items-center gap-3 font-heading text-xl sm:text-2xl">
                <span class="flex size-8 shrink-0 items-center justify-center rounded-base border-2 border-border bg-main text-base text-main-foreground">
                    (number)
                </span>
                (title)
            </h2>
            <div class="space-y-4 text-sm leading-7 sm:text-base">(child)</div>
        </section>
    })
}

#[component]
async fn code(text: &str) -> Result<impl View> {
    Ok(view! {
        <pre class="overflow-auto border-2 border-border bg-[#272933] p-4 text-sm leading-6 text-white shadow-shadow"><code>(text)</code></pre>
    })
}

/// The fixed navigation on the left: getting started, then every component.
#[component]
async fn sidebar(current: &str) -> Result<impl View> {
    Ok(view! {
        <aside class="fixed top-[70px] hidden h-[calc(100svh-70px)] w-[250px] overflow-y-auto border-r-4 border-border bg-secondary-background lg:block">
            <p class="border-b-4 border-border p-4 font-heading text-xl">"Getting started"</p>
            <a
                href="/docs"
                class=(class!(
                    "block border-b-4 border-border px-4 py-2 font-base hover:bg-main/50",
                    "bg-main text-main-foreground" if current.is_empty(),
                ))
            >
                "Installation"
            </a>
            <p class="border-b-4 border-border p-4 font-heading text-xl">"Components"</p>
            for doc in DOCS {
                <a
                    href=(doc_href(doc))
                    class=(class!(
                        "block border-b-4 border-border px-4 py-2 font-base hover:bg-main/50",
                        "bg-main text-main-foreground" if doc.slug == current,
                    ))
                >
                    (doc.title)
                </a>
            }
        </aside>
    })
}
