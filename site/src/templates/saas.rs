//! The SaaS landing page template.

use topcoat::{
    Result,
    context::Cx,
    view::{View, component, view},
};

#[component]
pub async fn saas_landing(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <div class="min-h-screen bg-background font-base text-foreground">"SaaS landing page"</div>
    })
}
