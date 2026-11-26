//! The revenue page: gross and net revenue, refunds, revenue by channel and
//! by country, and the average order value.

use topcoat::{
    Result,
    context::Cx,
    view::{View, component, view},
};

use crate::components::chart::{ChartConfig, ChartSeries};

use super::{
    charts::{Row, columns, ranked_bars, trend},
    data::{
        CHANNELS, COUNTRIES, Day, Period, format_cents, format_compact_money, format_money,
        format_percent, orders_of, revenue_of, total,
    },
    shared::{chart_card, stat_card, stat_grid},
};

/// The sales channels, in [`CHANNELS`](super::data::CHANNELS) order.
pub const CHANNEL_CONFIG: ChartConfig<'static> = &[
    ChartSeries::new("online", "Online store", "var(--chart-1)"),
    ChartSeries::new("marketplace", "Marketplace", "var(--chart-3)"),
    ChartSeries::new("retail", "Retail", "var(--chart-2)"),
];

const REFUND_CONFIG: ChartConfig<'static> =
    &[ChartSeries::new("refunds", "Refunds", "var(--chart-2)")];
const ORDER_VALUE_CONFIG: ChartConfig<'static> =
    &[ChartSeries::new("value", "Average order", "var(--chart-4)")];
const COUNTRY_CONFIG: ChartConfig<'static> =
    &[ChartSeries::new("value", "Revenue", "var(--chart-5)")];

fn gross_in(days: &[Day]) -> f64 {
    total(days, revenue_of)
}

fn refunds_in(days: &[Day]) -> f64 {
    total(days, |day| day.refunds)
}

fn net_in(days: &[Day]) -> f64 {
    gross_in(days) - refunds_in(days)
}

fn refund_rate_in(days: &[Day]) -> f64 {
    refunds_in(days) / gross_in(days) * 100.0
}

#[component]
pub async fn revenue_page(cx: &Cx, period: &Period) -> Result<impl View> {
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
    let per_bucket = |measure: &dyn Fn(&[Day]) -> Vec<f64>| -> Vec<Row> {
        period
            .buckets
            .iter()
            .map(|bucket| Row::new(bucket.label.clone(), measure(bucket.days)))
            .collect()
    };
    let by_channel = per_bucket(&|days| {
        (0..CHANNELS.len())
            .map(|index| total(days, |day| day.revenue[index]))
            .collect()
    });
    let order_value = per_bucket(&|days| vec![gross_in(days) / total(days, orders_of)]);
    let refunds = per_bucket(&|days| vec![refunds_in(days)]);
    let gross = gross_in(period.days);
    let countries: Vec<Row> = COUNTRIES
        .iter()
        .map(|(name, share)| Row::new(*name, vec![gross * share]))
        .collect();

    Ok(view! {
        stat_grid(
            stat_card(
                label: "Gross revenue",
                value: format_money(gross),
                compare: compare(gross_in),
                trend: trend_of(gross_in),
            )
            stat_card(
                label: "Net revenue",
                value: format_money(net_in(period.days)),
                color: "var(--chart-4)",
                compare: compare(net_in),
                trend: trend_of(net_in),
            )
            stat_card(
                label: "Refunds",
                value: format_money(refunds_in(period.days)),
                color: "var(--chart-2)",
                invert: true,
                compare: compare(refunds_in),
                trend: trend_of(refunds_in),
            )
            stat_card(
                label: "Refund rate",
                value: format_percent(refund_rate_in(period.days)),
                color: "var(--chart-3)",
                invert: true,
                compare: compare(refund_rate_in),
                trend: trend_of(refund_rate_in),
            )
        )

        chart_card(title: "Revenue by channel", description: format!("Gross revenue per {}", period.unit),
            columns(
                config: CHANNEL_CONFIG,
                rows: by_channel,
                stacked: true,
                format: format_money,
                axis_format: format_compact_money,
                height: "h-80",
            )
        )

        <div class="grid gap-5 lg:grid-cols-2 xl:grid-cols-3">
            chart_card(title: "Average order value", description: format!("Per {}", period.unit),
                trend(
                    config: ORDER_VALUE_CONFIG,
                    rows: order_value,
                    format: format_cents,
                    axis_format: format_compact_money,
                )
            )
            chart_card(title: "Refunds", description: format!("Amount per {}", period.unit),
                columns(
                    config: REFUND_CONFIG,
                    rows: refunds,
                    format: format_money,
                    axis_format: format_compact_money,
                )
            )
            chart_card(
                title: "Revenue by country",
                description: "Where orders ship to".to_owned(),
                class: "lg:col-span-2 xl:col-span-1",
                ranked_bars(config: COUNTRY_CONFIG, rows: countries, format: format_money)
            )
        </div>
    })
}
