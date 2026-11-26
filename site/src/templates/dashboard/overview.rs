//! The overview page: headline figures, the revenue trend, sales channels,
//! top products and the latest orders.

use topcoat::{
    Result,
    context::Cx,
    view::{View, attributes, component, view},
};

use crate::components::{
    button::{ButtonSize, ButtonVariant, button_variants},
    card::{card, card_action, card_content, card_header, card_title},
    chart::{ChartConfig, ChartSeries},
};

use super::{
    Page,
    charts::{Row, Slice, comparison, donut, ranked_bars},
    data::{
        Day, PRODUCTS, Period, format_cents, format_compact_money, format_money, format_number,
        format_percent, orders, orders_of, revenue_of, sessions_of, total,
    },
    order_table::order_table,
    revenue::CHANNEL_CONFIG,
    shared::{chart_card, stat_card, stat_grid},
};

const PRODUCT_REVENUE: ChartConfig<'static> =
    &[ChartSeries::new("value", "Revenue", "var(--chart-3)")];

fn revenue_in(days: &[Day]) -> f64 {
    total(days, revenue_of)
}

fn orders_in(days: &[Day]) -> f64 {
    total(days, orders_of)
}

fn average_in(days: &[Day]) -> f64 {
    revenue_in(days) / orders_in(days)
}

fn conversion_in(days: &[Day]) -> f64 {
    total(days, |day| day.orders[0]) / total(days, sessions_of) * 100.0
}

#[component]
pub async fn overview_page(cx: &Cx, period: &Period, range: usize) -> Result<impl View> {
    let compare =
        |measure: fn(&[Day]) -> f64| Some((measure(period.days), measure(period.previous)));
    let trend = |measure: fn(&[Day]) -> f64| {
        Some(
            period
                .buckets
                .iter()
                .map(|bucket| measure(bucket.days))
                .collect::<Vec<_>>(),
        )
    };
    let revenue_rows: Vec<Row> = period
        .buckets
        .iter()
        .zip(&period.previous_buckets)
        .map(|(bucket, previous)| {
            Row::new(
                bucket.label.clone(),
                vec![revenue_in(bucket.days), revenue_in(previous.days)],
            )
        })
        .collect();
    let channels: Vec<Slice> = CHANNEL_CONFIG
        .iter()
        .enumerate()
        .map(|(index, series)| Slice {
            label: series.label,
            value: total(period.days, |day| day.revenue[index]),
            color: series.color,
        })
        .collect();
    let revenue = revenue_in(period.days);
    let products: Vec<Row> = PRODUCTS[..6]
        .iter()
        .map(|product| Row::new(product.name, vec![revenue * product.share]))
        .collect();

    Ok(view! {
        stat_grid(
            stat_card(
                label: "Revenue",
                value: format_money(revenue),
                compare: compare(revenue_in),
                trend: trend(revenue_in),
            )
            stat_card(
                label: "Orders",
                value: format_number(orders_in(period.days)),
                color: "var(--chart-3)",
                compare: compare(orders_in),
                trend: trend(orders_in),
            )
            stat_card(
                label: "Average order",
                value: format_cents(average_in(period.days)),
                color: "var(--chart-4)",
                compare: compare(average_in),
                trend: trend(average_in),
            )
            stat_card(
                label: "Online conversion",
                value: format_percent(conversion_in(period.days)),
                color: "var(--chart-5)",
                compare: compare(conversion_in),
                trend: trend(conversion_in),
            )
        )

        <div class="grid gap-5 xl:grid-cols-3">
            chart_card(
                title: "Revenue trend",
                description: format!("Per {}, against the previous period", period.unit),
                class: "xl:col-span-2",
                comparison(
                    rows: revenue_rows,
                    format: format_money,
                    axis_format: format_compact_money,
                    height: "h-80",
                )
            )
            chart_card(title: "Sales channels", description: "Share of revenue".to_owned(),
                donut(slices: channels, caption: "revenue", format: format_money)
            )
        </div>

        <div class="grid gap-5 xl:grid-cols-3">
            chart_card(title: "Top products", description: "By revenue".to_owned(),
                ranked_bars(config: PRODUCT_REVENUE, rows: products, format: format_money)
            )
            // A chart card with an action in its header.
            card(attrs: attributes! { class="min-w-0 bg-secondary-background! xl:col-span-2" },
                card_header(
                    card_title("Recent orders")
                    card_action(
                        <a
                            href=(Page::Orders.href(range))
                            class=(button_variants(ButtonVariant::Neutral, ButtonSize::Sm))
                        >
                            "View all"
                        </a>
                    )
                )
                card_content(attrs: attributes! { class="flex min-w-0 flex-1 flex-col justify-center" },
                    order_table(orders: &orders()[..5])
                )
            )
        </div>
    })
}
