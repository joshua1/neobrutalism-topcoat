//! The customers page: new and returning customers, segments, retention by
//! cohort and the top customers.

use topcoat::{
    Result,
    context::Cx,
    view::{View, attributes, component, view},
};

use crate::components::{
    avatar::{AvatarSize, avatar, avatar_fallback},
    chart::{ChartConfig, ChartSeries},
    table::{table, table_body, table_cell, table_head, table_header, table_row},
};

use super::{
    charts::{Row, Slice, columns, donut, stacked_area},
    data::{
        COHORTS, Day, Period, SEGMENTS, TOP_CUSTOMERS, format_money, format_number, format_percent,
        orders_of, revenue_of, split, total,
    },
    shared::{HeatmapRow, chart_card, heatmap, stat_card, stat_grid},
};

const MIX_CONFIG: ChartConfig<'static> = &[
    ChartSeries::new("returning", "Returning", "var(--chart-1)"),
    ChartSeries::new("first", "New", "var(--chart-3)"),
];

const SIZE_CONFIG: ChartConfig<'static> =
    &[ChartSeries::new("size", "New customers", "var(--chart-5)")];

fn new_in(days: &[Day]) -> f64 {
    total(days, |day| day.new_customers)
}

fn returning_in(days: &[Day]) -> f64 {
    total(days, |day| day.returning)
}

fn repeat_in(days: &[Day]) -> f64 {
    returning_in(days) / total(days, orders_of) * 100.0
}

fn per_new_in(days: &[Day]) -> f64 {
    total(days, revenue_of) / new_in(days)
}

/// The initials of a name, such as `MT` for Mei Tanaka.
fn initials(name: &str) -> String {
    name.split(' ')
        .filter_map(|part| part.chars().next())
        .collect()
}

#[component]
pub async fn customers_page(cx: &Cx, period: &Period) -> Result<impl View> {
    let compare =
        |measure: fn(&[Day]) -> f64| Some((measure(period.days), measure(period.previous)));
    let trend_of = |measure: fn(&[Day]) -> f64| {
        Some(
            period
                .buckets
                .iter()
                .map(|bucket| measure(bucket.days))
                .collect::<Vec<_>>(),
        )
    };
    let shares: Vec<f64> = SEGMENTS.iter().map(|segment| segment.share).collect();
    let segment_counts = split(total(period.days, orders_of), &shares);
    let segments: Vec<Slice> = SEGMENTS
        .iter()
        .zip(&segment_counts)
        .map(|(segment, value)| Slice {
            label: segment.label,
            value: *value,
            color: segment.color,
        })
        .collect();
    let mix: Vec<Row> = period
        .buckets
        .iter()
        .map(|bucket| {
            Row::new(
                bucket.label.clone(),
                vec![returning_in(bucket.days), new_in(bucket.days)],
            )
        })
        .collect();
    let months: Vec<String> = (0..COHORTS.len())
        .map(|month| format!("Month {month}"))
        .collect();
    let retention: Vec<HeatmapRow> = COHORTS
        .iter()
        .map(|cohort| HeatmapRow {
            label: cohort.month,
            values: cohort.retention.to_vec(),
        })
        .collect();
    let sizes: Vec<Row> = COHORTS
        .iter()
        .map(|cohort| Row::new(cohort.month, vec![cohort.size]))
        .collect();

    Ok(view! {
        stat_grid(
            stat_card(
                label: "New customers",
                value: format_number(new_in(period.days)),
                color: "var(--chart-3)",
                compare: compare(new_in),
                trend: trend_of(new_in),
            )
            stat_card(
                label: "Returning orders",
                value: format_number(returning_in(period.days)),
                compare: compare(returning_in),
                trend: trend_of(returning_in),
            )
            stat_card(
                label: "Repeat rate",
                value: format_percent(repeat_in(period.days)),
                color: "var(--chart-4)",
                compare: compare(repeat_in),
                trend: trend_of(repeat_in),
            )
            stat_card(
                label: "Revenue per new",
                value: format_money(per_new_in(period.days)),
                color: "var(--chart-5)",
                compare: compare(per_new_in),
                trend: trend_of(per_new_in),
            )
        )

        <div class="grid gap-5 xl:grid-cols-3">
            chart_card(
                title: "New and returning",
                description: format!("Orders per {}", period.unit),
                class: "xl:col-span-2",
                stacked_area(config: MIX_CONFIG, rows: mix, height: "h-80")
            )
            chart_card(title: "Segments", description: "Orders by customer type".to_owned(),
                donut(slices: segments, caption: "orders")
            )
        </div>

        <div class="grid gap-5 xl:grid-cols-3">
            chart_card(
                title: "Retention",
                description: "Customers who ordered again, by month of first order".to_owned(),
                class: "xl:col-span-2",
                heatmap(corner: "Cohort", columns: months, rows: retention, max: 100.0, suffix: "%")
            )
            chart_card(title: "Cohort size", description: "First orders per month".to_owned(),
                columns(config: SIZE_CONFIG, rows: sizes)
            )
        </div>

        chart_card(title: "Top customers", description: "By lifetime spend".to_owned(),
            table(
                table_header(
                    table_row(
                        table_head("Customer")
                        table_head(attrs: attributes! { class="hidden sm:table-cell" }, "Country")
                        table_head(attrs: attributes! { class="text-right" }, "Orders")
                        table_head(attrs: attributes! { class="text-right" }, "Spent")
                    )
                )
                table_body(
                    for customer in &TOP_CUSTOMERS {
                        table_row(attrs: attributes! { class="bg-secondary-background!" },
                            table_cell(attrs: attributes! { class="py-3" },
                                <div class="flex items-center gap-3">
                                    avatar(size: AvatarSize::Sm,
                                        // `!` replaces the fallback's own background.
                                        avatar_fallback(attrs: attributes! { class="bg-chart-3!" }, (initials(customer.name)))
                                    )
                                    (customer.name)
                                </div>
                            )
                            table_cell(attrs: attributes! { class="hidden py-3 sm:table-cell" }, (customer.country))
                            table_cell(attrs: attributes! { class="py-3 text-right tabular-nums" }, (customer.orders))
                            table_cell(attrs: attributes! { class="py-3 text-right font-heading tabular-nums" },
                                (format_money(customer.spent))
                            )
                        )
                    }
                )
            )
        )
    })
}
