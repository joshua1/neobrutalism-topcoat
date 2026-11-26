//! `/templates`: full-page templates built with the components. Each one is
//! previewed at `/templates/{slug}` without the site's navbar, with a toolbar
//! instead (see [`toolbar`]). Their sources live in `site/src/templates/`.

mod dashboard;
mod saas;

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    router::page,
    view::{Unescaped, View, attributes, class, component, view},
};

use crate::components::{
    button::{ButtonSize, ButtonVariant, button_variants},
    dialog::{dialog, dialog_content, dialog_description, dialog_header, dialog_title, dialog_trigger},
};

/// A template: where it is previewed and what installing it takes.
pub struct Template {
    pub slug: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    /// The registry components it uses, for `topcoat ui add`.
    pub components: &'static [&'static str],
    /// What to call from a `#[page]` once the sources are copied.
    pub entry: &'static str,
    /// Anything else an app has to add, or "" when the entry is enough.
    pub setup: &'static str,
}

impl Template {
    pub fn href(&self) -> String {
        format!("/templates/{}", self.slug)
    }

    fn source(&self) -> String {
        format!(
            "https://github.com/joshua1/neobrutalism-topcoat/tree/main/site/src/templates/{}",
            self.slug
        )
    }

    fn install(&self) -> String {
        format!("topcoat ui add {} --registry neobrutalism", self.components.join(" "))
    }

    fn dialog_id(&self) -> String {
        format!("install-{}", self.slug)
    }
}

pub const TEMPLATES: &[Template] = &[
    Template {
        slug: "saas",
        name: "SaaS Landing Page",
        description: "A complete SaaS landing page with a product-board hero, bento features, tabbed product views, pricing toggle, testimonial marquee and more. Built entirely with neobrutalism components.",
        components: &[
            "accordion",
            "avatar",
            "badge",
            "button",
            "card",
            "input",
            "kbd",
            "stars",
            "switch",
            "tabs",
        ],
        entry: "templates::saas::saas_landing()",
        setup: "",
    },
    Template {
        slug: "dashboard",
        name: "Dashboard",
        description: "A store analytics dashboard with a collapsible sidebar and six pages: overview, revenue, marketing, orders, customers and inventory. Date ranges, charts, heatmaps and searchable tables, all built with neobrutalism components.",
        components: &[
            "avatar", "badge", "button", "card", "chart", "dropdown_menu", "input", "label",
            "progress", "select", "sidebar", "switch", "table",
        ],
        entry: "templates::dashboard::dashboard(page: Page::Overview)",
        setup: "Each page has its own URL and the export is a route: copy site/src/app/templates/dashboard.rs and dashboard/slug.rs as your routes, set BASE in the template to where you mount it, and keep .discover() on the router for its two shards. The module docs have the details.",
    },
];

/// The template whose preview `path` is in, if any.
pub fn template_for(path: &str) -> Option<&'static Template> {
    TEMPLATES.iter().find(|template| {
        let href = template.href();
        path == href || path.starts_with(&format!("{href}/"))
    })
}

/// The list of templates, each with a live preview.
#[page]
async fn templates(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <main class="min-h-dvh bg-background bg-[linear-gradient(to_right,#80808033_1px,transparent_1px),linear-gradient(to_bottom,#80808033_1px,transparent_1px)] bg-size-[70px_70px] px-5 pt-[70px]">
            <div class="mx-auto w-[1300px] max-w-full py-16 text-foreground">
                <div class="mb-[50px] flex flex-col gap-8 text-center">
                    <h1 class="font-heading text-2xl xl:text-3xl 2xl:text-4xl">"Templates"</h1>
                    <p class="mx-auto max-w-[800px] font-base text-base sm:text-lg xl:text-xl">
                        "Full page templates built with neobrutalism components."
                    </p>
                </div>
                <div class="grid gap-8 lg:grid-cols-2">
                    for template in TEMPLATES {
                        template_card(template: template)
                    }
                </div>
            </div>
            <script>(Unescaped::new_unchecked(PREVIEW_SCRIPT))</script>
        </main>
    })
}

/// Scales each 1440px-wide preview page down to its card.
const PREVIEW_SCRIPT: &str = r#"
for (const box of document.querySelectorAll("[data-template-preview]")) {
  const frame = box.querySelector("iframe");
  new ResizeObserver(() => { frame.style.scale = box.clientWidth / 1440; }).observe(box);
}
"#;

#[component]
async fn template_card(cx: &Cx, template: &Template) -> Result<impl View> {
    let href = template.href();
    let dialog_id = template.dialog_id();
    Ok(view! {
        <div class="min-w-0 overflow-hidden rounded-base border-2 border-border bg-secondary-background shadow-shadow">
            <div data-template-preview="" class="relative aspect-[16/10] overflow-hidden border-b-2 border-border bg-background">
                <iframe
                    src=(&href)
                    title=(format!("{} preview", template.name))
                    loading="lazy"
                    tabindex="-1"
                    aria-hidden="true"
                    class="pointer-events-none absolute top-0 left-0 h-[900px] w-[1440px] origin-top-left"
                    style="scale: 0.43"
                ></iframe>
                <a
                    href=(&href)
                    target="_blank"
                    class="absolute inset-0 z-10"
                    aria-label=(format!("Preview {}", template.name))
                ></a>
            </div>
            <div class="p-6">
                <h2 class="font-heading text-xl">(template.name)</h2>
                <p class="mt-1 font-base text-sm text-foreground">(template.description)</p>
                <div class="mt-5 grid grid-cols-2 gap-4">
                    <a
                        href=(&href)
                        target="_blank"
                        class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))
                    >
                        "Preview"
                        icon(data: iconify_icon!("lucide:eye"))
                    </a>
                    dialog_trigger(
                        target: &dialog_id,
                        attrs: attributes! { class=(button_variants(ButtonVariant::Default, ButtonSize::Default)) },
                        "Install"
                        icon(data: iconify_icon!("lucide:download"))
                    )
                </div>
            </div>
            install_dialog(template: template)
        </div>
    })
}

/// How to add a template to an app: its components, its sources, and the call.
#[component]
async fn install_dialog(cx: &Cx, template: &Template) -> Result<impl View> {
    let id = template.dialog_id();
    let title = format!("{id}-title");
    Ok(view! {
        dialog(
            id: id.as_str(),
            attrs: attributes! { aria-labelledby=(&title) },
            dialog_content(
                attrs: attributes! { class="max-w-2xl!" },
                dialog_header(
                    dialog_title(attrs: attributes! { id=(&title) }, (format!("Install the {}", template.name)))
                    dialog_description("Templates are Rust source you copy into your app, like the components.")
                )
                <ol class="ml-5 list-decimal space-y-4 text-sm leading-6">
                    <li>
                        "Set up the registry (see " <a href="/docs" class="underline">"Getting started"</a>
                        "), then add the components it uses:"
                        <pre class="mt-2 overflow-x-auto rounded-base border-2 border-border bg-[#272933] p-3 text-xs text-white"><code>(template.install())</code></pre>
                    </li>
                    <li>
                        "Copy " <a href=(template.source()) class="font-heading underline" target="_blank">
                            (format!("site/src/templates/{}", template.slug))
                        </a>
                        (format!(" (and {}.rs next to it) into src/templates/ and declare ", template.slug))
                        <code>"mod templates;"</code> "."
                    </li>
                    <li>
                        "Render it from a page: " <code>(template.entry)</code> "."
                    </li>
                    if !template.setup.is_empty() {
                        <li>(template.setup)</li>
                    }
                </ol>
            )
        )
    })
}

const HIDE_IN_PREVIEW: &str =
    "if (window.self !== window.top) document.currentScript.parentElement.remove();";

/// The floating dock on a template preview: back to the list, the theme
/// generator, and how to install the template. Removed inside the previews on
/// `/templates`, which are iframes.
#[component]
pub async fn toolbar(cx: &Cx, template: &Template) -> Result<impl View> {
    let dialog_id = template.dialog_id();
    let plain = || {
        class!(
            button_variants(ButtonVariant::NoShadow, ButtonSize::Default),
            "bg-secondary-background px-3 text-foreground sm:px-4",
        )
    };
    Ok(view! {
        <div
            role="toolbar"
            aria-label="Template tools"
            class="fixed bottom-4 left-1/2 z-50 flex -translate-x-1/2 items-center gap-1.5 rounded-base border-2 border-border bg-secondary-background p-1.5 shadow-shadow sm:bottom-6 sm:gap-2 sm:p-2"
        >
            <script>(Unescaped::new_unchecked(HIDE_IN_PREVIEW))</script>
            <a href="/templates" aria-label="Back to templates" class=(plain())>
                icon(data: iconify_icon!("lucide:arrow-left"))
                <span class="hidden min-[400px]:inline">"Templates"</span>
            </a>
            <a href="/styling" class=(plain())>
                "Customize"
                icon(data: iconify_icon!("lucide:paintbrush"))
            </a>
            dialog_trigger(
                target: &dialog_id,
                attrs: attributes! {
                    class=(class!(button_variants(ButtonVariant::NoShadow, ButtonSize::Default), "px-3 sm:px-4"))
                },
                "Install"
                icon(data: iconify_icon!("lucide:download"))
            )
        </div>
        install_dialog(template: template)
    })
}
