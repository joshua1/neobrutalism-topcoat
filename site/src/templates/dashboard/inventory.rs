//! The inventory page: stock value, stock levels against reorder points,
//! units sold and days of cover.

use topcoat::{
    Result,
    context::Cx,
    view::{View, attributes, component, view},
};

use crate::components::{
    badge::badge,
    chart::{ChartConfig, ChartSeries},
    progress::progress,
    table::{table, table_body, table_cell, table_head, table_header, table_row},
};

use super::{
    charts::{Row, Slice, columns, donut, ranked_bars},
    data::{
        PRODUCTS, Period, format_money, format_number, revenue_of, round, sum, to_fixed, total,
    },
    shared::{chart_card, stat_card, stat_grid},
};

const STOCK_CONFIG: ChartConfig<'static> = &[
    ChartSeries::new("stock", "In stock", "var(--chart-1)"),
    ChartSeries::new("reorder", "Reorder point", "var(--chart-3)"),
];

const SOLD_CONFIG: ChartConfig<'static> = &[ChartSeries::new("value", "Units", "var(--chart-5)")];

const CATEGORY_COLORS: [(&str, &str); 4] = [
    ("Bags", "var(--chart-1)"),
    ("Apparel", "var(--chart-3)"),
    ("Stationery", "var(--chart-2)"),
    ("Home", "var(--chart-4)"),
];

/// A product's stock figures over a period.
#[derive(Clone)]
struct StockRow {
    name: &'static str,
    category: &'static str,
    stock: f64,
    reorder: f64,
    value: f64,
    sold: f64,
    /// Days the stock lasts at the period's pace.
    cover: f64,
    low: bool,
}

#[component]
pub async fn inventory_page(cx: &Cx, period: &Period) -> Result<impl View> {
    let days = period.days.len() as f64;
    let per_day = total(period.days, revenue_of) / days;
    let rows: Vec<StockRow> = PRODUCTS
        .iter()
        .map(|product| {
            let sold_per_day = (per_day * product.share) / product.price;
            StockRow {
                name: product.name,
                category: product.category,
                stock: product.stock,
                reorder: product.reorder,
                value: product.stock * product.price,
                sold: round(sold_per_day * days),
                cover: product.stock / sold_per_day,
                low: product.stock < product.reorder,
            }
        })
        .collect();
    let low = rows.iter().filter(|row| row.low).count();
    let levels: Vec<Row> = rows
        .iter()
        .map(|row| Row::new(row.name, vec![row.stock, row.reorder]))
        .collect();
    let categories: Vec<Slice> = CATEGORY_COLORS
        .iter()
        .map(|(category, color)| Slice {
            label: category,
            value: sum(rows
                .iter()
                .filter(|row| row.category == *category)
                .map(|row| row.value)),
            color,
        })
        .collect();
    let mut by_sold: Vec<&StockRow> = rows.iter().collect();
    by_sold.sort_by(|a, b| b.sold.total_cmp(&a.sold));
    let sold: Vec<Row> = by_sold
        .iter()
        .map(|row| Row::new(row.name, vec![row.sold]))
        .collect();
    let mut by_cover = rows.clone();
    by_cover.sort_by(|a, b| a.cover.total_cmp(&b.cover));

    Ok(view! {
        stat_grid(
            stat_card(label: "Stock value", value: format_money(sum(rows.iter().map(|row| row.value))))
            stat_card(label: "Units on hand", value: format_number(sum(rows.iter().map(|row| row.stock))))
            stat_card(label: "Units sold", value: format_number(sum(rows.iter().map(|row| row.sold))))
            stat_card(label: "Below reorder point", value: format!("{low} of {}", rows.len()))
        )

        <div class="grid gap-5 xl:grid-cols-3">
            chart_card(
                title: "Stock levels",
                description: "Units on hand against the reorder point".to_owned(),
                class: "xl:col-span-2",
                columns(config: STOCK_CONFIG, rows: levels, all_labels: true, height: "h-80")
            )
            chart_card(title: "Stock value", description: "By category".to_owned(),
                donut(slices: categories, caption: "on hand", format: format_money)
            )
        </div>

        <div class="grid gap-5 xl:grid-cols-3">
            chart_card(title: "Units sold", description: "In this period".to_owned(),
                ranked_bars(config: SOLD_CONFIG, rows: sold, height: "h-96")
            )
            chart_card(
                title: "Days of cover",
                description: "How long stock lasts at the current pace".to_owned(),
                class: "xl:col-span-2",
                table(
                    table_header(
                        table_row(
                            table_head("Product")
                            table_head(attrs: attributes! { class="hidden w-1/3 md:table-cell" }, "Stock")
                            table_head(attrs: attributes! { class="text-right" }, "Days")
                            table_head("Status")
                        )
                    )
                    table_body(
                        for row in by_cover {
                            table_row(attrs: attributes! { class="bg-secondary-background!" },
                                table_cell(attrs: attributes! { class="py-2.5 font-heading" }, (row.name))
                                table_cell(attrs: attributes! { class="hidden py-2.5 md:table-cell" },
                                    progress(
                                        value: (row.stock / (row.reorder * 2.0) * 100.0).min(100.0),
                                        attrs: attributes! {
                                            aria-label=(format!("{} stock against twice its reorder point", row.name))
                                            class=(if row.low {
                                                "**:data-[slot=progress-indicator]:bg-chart-2"
                                            } else {
                                                "**:data-[slot=progress-indicator]:bg-chart-4"
                                            })
                                        },
                                    )
                                )
                                table_cell(attrs: attributes! { class="py-2.5 text-right tabular-nums" },
                                    (to_fixed(row.cover, 1))
                                )
                                table_cell(attrs: attributes! { class="py-2.5" },
                                    badge(
                                        attrs: attributes! { class=(if row.low { "bg-chart-2!" } else { "bg-chart-4!" }) },
                                        (if row.low { "Reorder" } else { "Healthy" })
                                    )
                                )
                            )
                        }
                    )
                )
            )
        </div>
    })
}
