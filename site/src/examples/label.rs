use topcoat::{
    Result,
    context::Cx,
    view::{View, attributes, component, view},
};

use crate::components::{checkbox::checkbox, label::label};

#[component]
pub async fn demo(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <div>
            <div class="flex items-center space-x-2">
                checkbox(attrs: attributes! { id="terms" })
                label(attrs: attributes! { for="terms" }, "Accept terms and conditions")
            </div>
        </div>
    })
}
