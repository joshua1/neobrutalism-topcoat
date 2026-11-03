//! `/docs/{slug}`: a component's documentation page.

use topcoat::{
    Result,
    context::Cx,
    router::{error::RouterErrorExt, module_param, page, path_param},
    view::{View, class, component, view},
};

use crate::catalog::{DOCS, Doc, Section, component_source, example_source, render_example};

module_param!(slug);

#[page]
async fn page(cx: &Cx) -> Result<impl View> {
    let slug = path_param::<Slug>(cx);
    let doc = DOCS.iter().find(|doc| doc.slug == slug).ok_or_not_found()?;

    Ok(view! {
        <div class="min-h-dvh w-full pt-[70px]">
            super::sidebar(current: doc.slug)
            toc(doc: doc)
            <main class="py-12 sm:py-16 lg:ml-[250px] lg:py-20 xl:mr-[250px]">
                <article class="mx-auto w-full max-w-[750px] px-5 leading-relaxed">
                    <h1 class="font-heading text-2xl sm:text-3xl">(doc.title)</h1>
                    <p class="mt-4 text-base sm:text-lg">(doc.description)</p>

                    // Examples can hold signals, so each iteration needs its own
                    // identity: the example's function name is unique per page.
                    #[key(section.example)]
                    for (index, section) in doc.sections.iter().enumerate() {
                        example_section(doc: doc, section: section, first: index == 0)
                    }

                    installation(doc: doc)
                </article>
            </main>
        </div>
    })
}

/// "On this page": the page's example headings, then Installation.
#[component]
async fn toc(doc: &Doc) -> Result<impl View> {
    Ok(view! {
        <aside class="fixed right-0 top-[70px] hidden h-[calc(100svh-70px)] w-[250px] flex-col overflow-y-auto border-l-4 border-border bg-secondary-background xl:flex">
            <p class="border-b-4 border-border p-4 font-heading text-xl">"On this page"</p>
            for section in doc.sections.iter().skip(1) {
                <a href=(format!("#{}", section.example)) class="border-b-2 border-border px-4 py-2 font-base hover:bg-main/50">
                    (section.heading)
                </a>
            }
            <a href="#installation" class="border-b-2 border-border px-4 py-2 font-base hover:bg-main/50">
                "Installation"
            </a>
        </aside>
    })
}

/// Components whose examples are whole page layouts, shown edge to edge.
const FLUSH_PREVIEWS: &[&str] = &["sidebar"];

/// One example: its heading and prose, then Preview / Code tabs.
///
/// The tabs are CSS only: two radio inputs share a name per example, and
/// `group-has-[…:checked]` shows the matching panel.
#[component]
async fn example_section(doc: &Doc, section: &Section, first: bool) -> Result<impl View> {
    let name = format!("{}-{}", doc.module, section.example);
    let source = example_source(doc.module, section.example);

    Ok(view! {
        if !first {
            <h3 id=(section.example) class="mt-10 mb-4 scroll-mt-28 font-heading text-lg sm:text-xl">
                (section.heading)
            </h3>
        }
        if !section.body.is_empty() {
            <p class="mb-4 text-sm leading-7 sm:text-base">prose(text: section.body)</p>
        }
        <div class=(class!("group w-full shadow-shadow", "mt-8" if first))>
            <div class="grid h-10 grid-cols-2 border-2 border-b-0 border-border bg-secondary-background sm:h-12">
                <label class="flex cursor-pointer items-center justify-center border-r-2 border-border has-checked:bg-main has-checked:text-main-foreground has-focus-visible:underline sm:text-base">
                    <input type="radio" name=(name.clone()) class="sr-only" checked="">
                    "Preview"
                </label>
                <label class="flex cursor-pointer items-center justify-center has-checked:bg-main has-checked:text-main-foreground has-focus-visible:underline sm:text-base">
                    <input type="radio" name=(name) class="sr-only" data-tab="code">
                    "Code"
                </label>
            </div>
            // `leading-normal`: previews use the body's line height, not the
            // article's. Full-layout examples (the sidebar) fill the box.
            <div class=(class!(
                "relative flex min-h-[200px] w-full items-center justify-center border-2 border-border \
                 bg-secondary-background bg-[linear-gradient(to_right,#8080804D_1px,transparent_1px),linear-gradient(to_bottom,#80808090_1px,transparent_1px)] \
                 bg-size-[40px_40px] leading-normal group-has-[[data-tab=code]:checked]:hidden",
                "p-0" if FLUSH_PREVIEWS.contains(&doc.module) else "px-5 py-10 sm:px-10 sm:py-20",
            ))>
                render_example(module: doc.module, example: section.example)
            </div>
            <div class="hidden group-has-[[data-tab=code]:checked]:block">
                code_block(code: source)
            </div>
        </div>
    })
}

/// How to install the component, and its full source.
#[component]
async fn installation(doc: &Doc) -> Result<impl View> {
    let command = format!("topcoat ui add {} --registry neobrutalism", doc.module);
    let source = component_source(doc.module);

    Ok(view! {
        <h2 id="installation" class="mt-14 mb-6 scroll-mt-28 font-heading text-xl sm:text-2xl">"Installation"</h2>
        code_block(code: &command)
        if let Some(source) = source {
            <details class="mt-5 shadow-shadow">
                <summary class="cursor-pointer border-2 border-border bg-secondary-background px-4 py-3 font-heading">
                    "Component source (src/components/" (doc.module) ".rs)"
                </summary>
                <div class="-mt-0.5">code_block(code: source)</div>
            </details>
        }
    })
}

/// Docs prose with `backtick` spans rendered as inline code. Splitting on the
/// backtick puts code at every odd index.
#[component]
async fn prose(text: &str) -> Result<impl View> {
    Ok(view! {
        for (index, part) in text.split('`').enumerate() {
            if index % 2 == 1 {
                <code class="mx-0.5 whitespace-nowrap rounded-base border-2 border-border bg-secondary-background px-[5px] py-[2px] text-sm">
                    (part)
                </code>
            } else {
                (part)
            }
        }
    })
}

#[component]
async fn code_block(code: &str) -> Result<impl View> {
    Ok(view! {
        <pre class="max-h-[600px] overflow-auto border-2 border-border bg-[#272933] p-4 text-sm leading-6 text-white"><code>(code)</code></pre>
    })
}
