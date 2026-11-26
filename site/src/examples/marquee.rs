use topcoat::{
    Result,
    context::Cx,
    view::{View, component, view},
};

use crate::components::marquee::marquee;

#[component]
pub async fn demo(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        marquee(items: &["Item 1", "Item 2", "Item 3", "Item 4", "Item 5"])
    })
}
