//! The orders page: orders by channel, volume, status, when people buy, time
//! to dispatch, and a searchable list of the latest orders.

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    runtime::{Event, shard, signal},
    view::{View, attributes, component, view},
};

use crate::components::{
    button::{ButtonSize, ButtonVariant, button},
    chart::{ChartConfig, ChartSeries},
    input::input,
    select::{select, select_content, select_group, select_item},
};

use super::{
    charts::{Row, Slice, columns, donut},
    data::{
        DISPATCH, Day, HOURS, Period, STATUSES, Status, format_number, order_heatmap, orders,
        orders_of, split, total,
    },
    order_table::order_table,
    shared::{HeatmapRow, chart_card, heatmap, stat_card, stat_grid},
};

const VOLUME_CONFIG: ChartConfig<'static> =
    &[ChartSeries::new("orders", "Orders", "var(--chart-1)")];
const DISPATCH_CONFIG: ChartConfig<'static> =
    &[ChartSeries::new("orders", "Orders", "var(--chart-5)")];

/// Orders per page of the latest orders.
const PAGE_SIZE: usize = 8;
/// The longest search the list accepts; the browser can send anything.
const MAX_QUERY: usize = 100;

/// The latest orders, filtered by a search and a status, eight per page.
///
/// A shard: the search, the status and the page are signals read here on the
/// server, so changing one re-renders only this card.
#[shard]
async fn latest_orders(cx: &Cx) -> Result<impl View> {
    let query = signal(cx, String::new);
    let status = signal(cx, || "All".to_owned());
    let page = signal(cx, || 0usize);

    // The signals come back from the browser: check them.
    let needle: String = query
        .get()
        .trim()
        .chars()
        .take(MAX_QUERY)
        .collect::<String>()
        .to_lowercase();
    let wanted = Status::from_label(&status.get());
    let all = orders();
    let matching: Vec<_> = all
        .iter()
        .filter(|order| wanted.is_none_or(|wanted| order.status == wanted))
        .filter(|order| {
            [order.id.as_str(), order.customer, order.product.as_str()]
                .iter()
                .any(|text| text.to_lowercase().contains(&needle))
        })
        .collect();
    let pages = matching.len().div_ceil(PAGE_SIZE).max(1);
    let current = page.get().min(pages - 1);
    let visible: Vec<_> = matching
        .iter()
        .skip(current * PAGE_SIZE)
        .take(PAGE_SIZE)
        .map(|order| (*order).clone())
        .collect();
    let previous = current.saturating_sub(1);
    let next = (current + 1).min(pages - 1);
    let summary = format!(
        "{} {}, page {} of {}",
        matching.len(),
        if matching.len() == 1 {
            "order"
        } else {
            "orders"
        },
        current + 1,
        pages,
    );
    let selected = wanted.map_or("All", Status::label);

    Ok(view! {
        chart_card(title: "Latest orders", description: format!("The {} most recent orders", all.len()),
            <div class="mb-4 flex flex-col gap-3 sm:flex-row">
                input(attrs: attributes! {
                    aria-label="Search orders"
                    placeholder="Search orders"
                    class="sm:max-w-xs"
                    :value=$(query.get())
                    @input=$(|e: Event| {
                        query.set(e.target.value);
                        page.set(0);
                    })
                })
                select(attrs: attributes! {
                    aria-label="Filter by status"
                    class="sm:w-44"
                    @change=$(|e: Event| {
                        status.set(e.target.value);
                        page.set(0);
                    })
                },
                    select_content(
                        select_group(
                            select_item(attrs: attributes! { value="All" selected=(selected == "All") }, "All statuses")
                            for status_option in STATUSES {
                                select_item(
                                    attrs: attributes! {
                                        value=(status_option.label())
                                        selected=(selected == status_option.label())
                                    },
                                    (status_option.label())
                                )
                            }
                        )
                    )
                )
            </div>
            if visible.is_empty() {
                <p class="rounded-base border-2 border-border p-8 text-center text-sm">
                    "No orders match these filters."
                </p>
            } else {
                order_table(orders: &visible)
            }
            <div class="mt-4 flex items-center justify-between text-sm">
                <p aria-live="polite">(summary)</p>
                <div class="flex gap-3">
                    button(
                        size: ButtonSize::IconSm,
                        variant: ButtonVariant::Neutral,
                        attrs: attributes! {
                            aria-label="Previous page"
                            disabled=(current == 0)
                            @click=$(|_e: Event| page.set(previous))
                        },
                        icon(data: iconify_icon!("lucide:chevron-left"))
                    )
                    button(
                        size: ButtonSize::IconSm,
                        variant: ButtonVariant::Neutral,
                        attrs: attributes! {
                            aria-label="Next page"
                            disabled=(current + 1 >= pages)
                            @click=$(|_e: Event| page.set(next))
                        },
                        icon(data: iconify_icon!("lucide:chevron-right"))
                    )
                </div>
            </div>
        )
    })
}

#[component]
pub async fn orders_page(cx: &Cx, period: &Period) -> Result<impl View> {
    let count = total(period.days, orders_of);
    let shares: Vec<f64> = STATUSES.iter().map(|status| status.share()).collect();
    let by_status = split(count, &shares);
    let dispatch_shares: Vec<f64> = DISPATCH.iter().map(|(_, share)| *share).collect();
    let by_dispatch = split(count, &dispatch_shares);
    let heat = order_heatmap(period.days);
    let heat_max = heat
        .iter()
        .flat_map(|(_, values)| values.iter().copied())
        .fold(0.0, f64::max);

    // A figure's value, change and sparkline from a daily count.
    let stat = |pick: &dyn Fn(&Day) -> f64| {
        let current = total(period.days, pick);
        (
            format_number(current),
            Some((current, total(period.previous, pick))),
            Some(
                period
                    .buckets
                    .iter()
                    .map(|bucket| total(bucket.days, pick))
                    .collect::<Vec<_>>(),
            ),
        )
    };
    let (all_value, all_compare, all_trend) = stat(&orders_of);
    let (online_value, online_compare, online_trend) = stat(&|day| day.orders[0]);
    let (market_value, market_compare, market_trend) = stat(&|day| day.orders[1]);
    let (retail_value, retail_compare, retail_trend) = stat(&|day| day.orders[2]);

    let volume: Vec<Row> = period
        .buckets
        .iter()
        .map(|bucket| Row::new(bucket.label.clone(), vec![total(bucket.days, orders_of)]))
        .collect();
    let statuses: Vec<Slice> = STATUSES
        .iter()
        .zip(&by_status)
        .map(|(status, value)| Slice {
            label: status.label(),
            value: *value,
            color: status.color(),
        })
        .collect();
    let dispatch: Vec<Row> = DISPATCH
        .iter()
        .zip(&by_dispatch)
        .map(|((label, _), value)| Row::new(*label, vec![*value]))
        .collect();
    let hours: Vec<String> = HOURS.iter().map(|hour| format!("{hour:02}h")).collect();
    let heat_rows: Vec<HeatmapRow> = heat
        .into_iter()
        .map(|(label, values)| HeatmapRow { label, values })
        .collect();

    Ok(view! {
        stat_grid(
            stat_card(label: "All orders", color: "var(--chart-4)", value: all_value, compare: all_compare, trend: all_trend)
            stat_card(label: "Online store", value: online_value, compare: online_compare, trend: online_trend)
            stat_card(label: "Marketplace", color: "var(--chart-3)", value: market_value, compare: market_compare, trend: market_trend)
            stat_card(label: "Retail", color: "var(--chart-2)", value: retail_value, compare: retail_compare, trend: retail_trend)
        )

        <div class="grid gap-5 xl:grid-cols-3">
            chart_card(
                title: "Order volume",
                description: format!("Orders per {}", period.unit),
                class: "xl:col-span-2",
                columns(config: VOLUME_CONFIG, rows: volume, height: "h-80")
            )
            chart_card(title: "Order status", description: "Orders in this period".to_owned(),
                donut(slices: statuses, caption: "orders")
            )
        </div>

        <div class="grid gap-5 xl:grid-cols-3">
            chart_card(
                title: "When people buy",
                description: "Orders by weekday and time of day".to_owned(),
                class: "xl:col-span-2",
                heatmap(corner: "Day", columns: hours, rows: heat_rows, max: heat_max)
            )
            chart_card(title: "Time to dispatch", description: "Payment to pickup".to_owned(),
                columns(config: DISPATCH_CONFIG, rows: dispatch)
            )
        </div>

        latest_orders()
    })
}
