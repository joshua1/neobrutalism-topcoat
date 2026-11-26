//! The table of orders on the overview and orders pages.

use topcoat::{
    Result,
    context::Cx,
    view::{View, attributes, component, view},
};

use crate::components::table::{
    table, table_body, table_cell, table_head, table_header, table_row,
};

use super::{
    data::{Order, format_money},
    shared::status_badge,
};

#[component]
pub async fn order_table(cx: &Cx, orders: &[Order]) -> Result<impl View> {
    Ok(view! {
        table(
            table_header(
                table_row(
                    table_head("Order")
                    table_head("Customer")
                    table_head(attrs: attributes! { class="hidden md:table-cell" }, "Product")
                    table_head(attrs: attributes! { class="hidden sm:table-cell" }, "Date")
                    table_head("Status")
                    table_head(attrs: attributes! { class="text-right" }, "Total")
                )
            )
            table_body(
                for order in orders {
                    // `!` replaces the row's own background.
                    table_row(attrs: attributes! { class="bg-secondary-background!" },
                        table_cell(attrs: attributes! { class="py-3 font-heading" }, (order.id.as_str()))
                        table_cell(attrs: attributes! { class="py-3 whitespace-nowrap" }, (order.customer))
                        table_cell(attrs: attributes! { class="hidden py-3 whitespace-nowrap md:table-cell" },
                            (order.product.as_str())
                        )
                        table_cell(attrs: attributes! { class="hidden py-3 whitespace-nowrap sm:table-cell" },
                            (order.date.short())
                        )
                        table_cell(attrs: attributes! { class="py-3" }, status_badge(status: order.status))
                        table_cell(attrs: attributes! { class="py-3 text-right font-heading tabular-nums" },
                            (format_money(order.total))
                        )
                    )
                }
            )
        )
    })
}
