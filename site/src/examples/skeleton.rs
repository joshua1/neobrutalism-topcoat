use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use crate::components::skeleton::skeleton;

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        <div class="flex items-center space-x-4">
            skeleton(attrs: attributes! { class="h-12 w-12 rounded-full" })
            <div class="space-y-2">
                skeleton(attrs: attributes! { class="h-4 sm:w-[250px] w-[100px]" })
                skeleton(attrs: attributes! { class="h-4 sm:w-[200px] w-[100px]" })
            </div>
        </div>
    })
}
