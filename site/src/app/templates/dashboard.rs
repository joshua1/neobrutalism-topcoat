//! `/templates/dashboard`: the store dashboard template.

use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

#[page]
async fn dashboard(cx: &Cx) -> Result<impl View> {
    Ok(view! { crate::templates::dashboard::dashboard() })
}
