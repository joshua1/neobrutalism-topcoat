//! `/templates/saas`: the SaaS landing page template.

use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

#[page]
async fn saas(cx: &Cx) -> Result<impl View> {
    Ok(view! { crate::templates::saas::saas_landing() })
}
