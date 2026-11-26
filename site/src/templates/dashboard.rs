//! The store dashboard template.

use topcoat::{
    Result,
    context::Cx,
    view::{View, component, view},
};

#[component]
pub async fn dashboard(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <div class="min-h-screen bg-background font-base text-foreground">"Dashboard"</div>
    })
}
