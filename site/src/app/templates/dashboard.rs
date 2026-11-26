//! `/templates/dashboard`: the store dashboard template, on its overview
//! page. `slug` serves its other pages at `/templates/dashboard/{slug}`, and
//! `export.csv` the chosen range's figures as a download.

mod slug;

use topcoat::{
    Result,
    context::Cx,
    router::{page, route},
    view::{View, view},
};

use crate::templates::dashboard::{Page, export_csv};

#[page]
async fn dashboard(cx: &Cx) -> Result<impl View> {
    Ok(view! { crate::templates::dashboard::dashboard(page: Page::Overview) })
}

#[route(GET "./export.csv")]
async fn export(cx: &Cx) -> Result<([(&'static str, &'static str); 2], String)> {
    Ok(export_csv(cx))
}
