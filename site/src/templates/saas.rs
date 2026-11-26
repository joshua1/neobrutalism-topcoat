//! The SaaS landing page template: Planly, a project tracker. A port of the
//! neobrutalism.dev SaaS template.
//!
//! Each section is its own module. Interactivity uses native elements where
//! they fit (the FAQ is `<details>`), runtime signals for state that stays in
//! one place (the menu, the pricing switch, the product views, the email
//! form), and two small inline scripts for what the runtime can't express:
//! the hero's task board and the product tabs' autoplay.

mod cta;
mod decor_star;
mod email_capture;
mod faq;
mod features;
mod footer;
mod hero;
mod how_it_works;
mod logos;
mod navbar;
mod pricing;
mod product_tabs;
mod section_heading;
mod stats;
mod testimonials;

use topcoat::{
    Result,
    context::Cx,
    view::{View, ViewExt, component, view},
};

/// The whole page: header, sections and footer.
#[component]
pub async fn saas_landing(cx: &Cx) -> Result<impl View> {
    // The original loads DM Sans without its optical-size axis, which pins it
    // at the text size (14) even in headings. Pinning the axis here matches
    // its letterforms and line breaks where the font has the axis.
    //
    // Each section is boxed: inline, the whole page's render state overflowed
    // a Tokio worker's stack in debug builds.
    Ok(view! {
        <div class="min-h-screen bg-background font-base text-foreground [font-variation-settings:'opsz'_14]">
            (view! { navbar::navbar() }.boxed())
            <main>
                (view! { hero::hero() }.boxed())
                (view! { logos::logos() }.boxed())
                (view! { features::features() }.boxed())
                (view! { product_tabs::product_tabs() }.boxed())
                (view! { how_it_works::how_it_works() }.boxed())
                (view! { stats::stats() }.boxed())
                (view! { pricing::pricing() }.boxed())
                (view! { testimonials::testimonials() }.boxed())
                (view! { faq::faq() }.boxed())
                (view! { cta::cta() }.boxed())
            </main>
            (view! { footer::footer() }.boxed())
        </div>
    })
}
