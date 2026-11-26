//! The marketing page: sessions, conversion, traffic sources, the purchase
//! funnel and campaign results.

use topcoat::{
    Result,
    context::Cx,
    view::{View, attributes, component, view},
};

use crate::components::{
    badge::{BadgeVariant, badge},
    chart::{ChartConfig, ChartSeries},
    table::{table, table_body, table_cell, table_head, table_header, table_row},
};

use super::{
    charts::{Row, columns, ranked_bars, stacked_area},
    data::{
        CAMPAIGNS, Day, FUNNEL, Period, SOURCES, format_compact_money, format_money, format_number,
        format_percent, round, sessions_of, sum, to_fixed, total,
    },
    shared::{chart_card, stat_card, stat_grid},
};

const SOURCE_CONFIG: ChartConfig<'static> = &[
    ChartSeries::new("organic", "Organic", "var(--chart-1)"),
    ChartSeries::new("paid", "Paid", "var(--chart-3)"),
    ChartSeries::new("social", "Social", "var(--chart-2)"),
    ChartSeries::new("email", "Email", "var(--chart-4)"),
];

const CAMPAIGN_CONFIG: ChartConfig<'static> = &[
    ChartSeries::new("spend", "Spend", "var(--chart-2)"),
    ChartSeries::new("revenue", "Revenue", "var(--chart-4)"),
];

const FUNNEL_CONFIG: ChartConfig<'static> =
    &[ChartSeries::new("value", "Visitors", "var(--chart-5)")];

/// A campaign's spend and attributed revenue over some days.
struct CampaignResult {
    name: &'static str,
    channel: &'static str,
    spend: f64,
    revenue: f64,
}

fn campaigns_in(days: &[Day]) -> Vec<CampaignResult> {
    let online = total(days, |day| day.revenue[0]);
    CAMPAIGNS
        .iter()
        .map(|campaign| CampaignResult {
            name: campaign.name,
            channel: campaign.channel,
            spend: campaign.spend * days.len() as f64,
            revenue: round(online * campaign.share),
        })
        .collect()
}

fn sessions_in(days: &[Day]) -> f64 {
    total(days, sessions_of)
}

fn orders_in(days: &[Day]) -> f64 {
    total(days, |day| day.orders[0])
}

fn conversion_in(days: &[Day]) -> f64 {
    orders_in(days) / sessions_in(days) * 100.0
}

fn attributed_in(days: &[Day]) -> f64 {
    sum(campaigns_in(days).iter().map(|campaign| campaign.revenue))
}

fn roas_in(days: &[Day]) -> f64 {
    attributed_in(days) / sum(campaigns_in(days).iter().map(|campaign| campaign.spend))
}

#[component]
pub async fn marketing_page(cx: &Cx, period: &Period) -> Result<impl View> {
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
    let results = campaigns_in(period.days);
    let sources: Vec<Row> = period
        .buckets
        .iter()
        .map(|bucket| {
            Row::new(
                bucket.label.clone(),
                (0..SOURCES.len())
                    .map(|index| total(bucket.days, |day| day.sessions[index]))
                    .collect(),
            )
        })
        .collect();
    let sessions = sessions_in(period.days);
    let mut funnel: Vec<Row> = FUNNEL
        .iter()
        .map(|(label, share)| Row::new(*label, vec![round(sessions * share)]))
        .collect();
    funnel.push(Row::new("Ordered", vec![orders_in(period.days)]));
    let spend_rows: Vec<Row> = results
        .iter()
        .map(|campaign| Row::new(campaign.name, vec![campaign.spend, campaign.revenue]))
        .collect();

    Ok(view! {
        stat_grid(
            stat_card(
                label: "Sessions",
                value: format_number(sessions),
                compare: compare(sessions_in),
                trend: trend_of(sessions_in),
            )
            stat_card(
                label: "Conversion",
                value: format_percent(conversion_in(period.days)),
                color: "var(--chart-5)",
                compare: compare(conversion_in),
                trend: trend_of(conversion_in),
            )
            stat_card(
                label: "Campaign revenue",
                value: format_money(attributed_in(period.days)),
                color: "var(--chart-4)",
                compare: compare(attributed_in),
                trend: trend_of(attributed_in),
            )
            stat_card(
                label: "Return on ad spend",
                value: format!("{}×", to_fixed(roas_in(period.days), 2)),
                color: "var(--chart-3)",
                compare: compare(roas_in),
                trend: trend_of(roas_in),
            )
        )

        chart_card(title: "Traffic sources", description: format!("Online store sessions per {}", period.unit),
            stacked_area(config: SOURCE_CONFIG, rows: sources, height: "h-80")
        )

        <div class="grid gap-5 xl:grid-cols-2">
            chart_card(title: "Purchase funnel", description: "Online store visitors".to_owned(),
                ranked_bars(config: FUNNEL_CONFIG, rows: funnel)
            )
            chart_card(title: "Spend and revenue", description: "By campaign".to_owned(),
                columns(
                    config: CAMPAIGN_CONFIG,
                    rows: spend_rows,
                    all_labels: true,
                    format: format_money,
                    axis_format: format_compact_money,
                )
            )
        </div>

        chart_card(title: "Campaigns", description: "Spend and attributed revenue".to_owned(),
            table(
                table_header(
                    table_row(
                        table_head("Campaign")
                        table_head(attrs: attributes! { class="hidden sm:table-cell" }, "Channel")
                        table_head(attrs: attributes! { class="text-right" }, "Spend")
                        table_head(attrs: attributes! { class="text-right" }, "Revenue")
                        table_head(attrs: attributes! { class="text-right" }, "ROAS")
                    )
                )
                table_body(
                    for campaign in &results {
                        table_row(attrs: attributes! { class="bg-secondary-background!" },
                            table_cell(attrs: attributes! { class="py-3 font-heading" }, (campaign.name))
                            table_cell(attrs: attributes! { class="hidden py-3 sm:table-cell" },
                                badge(variant: BadgeVariant::Neutral, (campaign.channel))
                            )
                            table_cell(attrs: attributes! { class="py-3 text-right tabular-nums" },
                                (format_money(campaign.spend))
                            )
                            table_cell(attrs: attributes! { class="py-3 text-right tabular-nums" },
                                (format_money(campaign.revenue))
                            )
                            table_cell(attrs: attributes! { class="py-3 text-right font-heading tabular-nums" },
                                (format!("{}×", to_fixed(campaign.revenue / campaign.spend, 2)))
                            )
                        )
                    }
                )
            )
        )
    })
}
