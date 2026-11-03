use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use crate::components::field::{FieldOrientation, field, field_label};
use crate::components::pagination::{
    pagination, pagination_content, pagination_ellipsis, pagination_item, pagination_link,
    pagination_next, pagination_previous,
};
use crate::components::select::{select, select_content, select_group, select_item};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        pagination(
            pagination_content(
                pagination_item(pagination_previous(attrs: attributes! { href="#" }))
                pagination_item(pagination_link(attrs: attributes! { href="#" }, "1"))
                pagination_item(pagination_link(active: true, attrs: attributes! { href="#" }, "2"))
                <div class="items-center md:flex hidden">
                    pagination_item(pagination_link(attrs: attributes! { href="#" }, "3"))
                    pagination_item(pagination_ellipsis())
                </div>
                pagination_item(pagination_next(attrs: attributes! { href="#" }))
            )
        )
    })
}

#[component]
pub async fn icons_only() -> Result<impl View> {
    Ok(view! {
        <div class="flex items-center justify-between gap-4">
            // `w-fit!` overrides the field's `w-full`.
            field(orientation: FieldOrientation::Horizontal, attrs: attributes! { class="w-fit!" },
                field_label(attrs: attributes! { for="select-rows-per-page" }, "Rows per page")
                select(attrs: attributes! { id="select-rows-per-page" name="rows" class="w-20" },
                    select_content(
                        select_group(
                            #[key(rows)]
                            for rows in ["10", "25", "50", "100"] {
                                select_item(
                                    attrs: attributes! { value=(rows) selected=(rows == "25") },
                                    (rows)
                                )
                            }
                        )
                    )
                )
            )
            // `!` overrides the navigation's own `mx-auto w-full`.
            pagination(attrs: attributes! { class="mx-0! w-auto!" },
                pagination_content(
                    pagination_item(pagination_previous(attrs: attributes! { href="#" }))
                    pagination_item(pagination_next(attrs: attributes! { href="#" }))
                )
            )
        </div>
    })
}

#[component]
pub async fn simple() -> Result<impl View> {
    Ok(view! {
        pagination(
            pagination_content(
                #[key(page)]
                for page in 1..=5 {
                    pagination_item(
                        pagination_link(
                            active: page == 2,
                            attrs: attributes! { href="#" },
                            (page)
                        )
                    )
                }
            )
        )
    })
}
