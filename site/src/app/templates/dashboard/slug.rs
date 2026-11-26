//! `/templates/dashboard/{slug}`: a page of the dashboard template other than
//! the overview. Unknown slugs are a 404.

use topcoat::{
    Result,
    context::Cx,
    router::{error::RouterErrorExt, module_param, page},
    view::{View, view},
};

use crate::templates::dashboard::Page;

module_param!(slug);

#[page]
async fn dashboard_page(cx: &Cx) -> Result<impl View> {
    let page = Page::from_slug(slug(cx)).ok_or_not_found()?;
    Ok(view! { crate::templates::dashboard::dashboard(page: page) })
}
