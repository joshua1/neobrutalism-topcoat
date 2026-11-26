//! Pieces shared by the dashboard's pages: stat cards, chart cards, status
//! badges and heatmaps.

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    view::{Child, View, attributes, class, component, view},
};

use crate::components::{
    badge::badge,
    card::{CardSize, card, card_content, card_description, card_header, card_title},
};

use super::{
    charts::sparkline,
    data::{Status, round, to_fixed},
};

/// The change against the previous period. `invert` is for figures where
/// lower is better, such as refunds.
#[component]
async fn delta(cx: &Cx, current: f64, previous: f64, invert: bool) -> Result<impl View> {
    let change = if previous == 0.0 {
        0.0
    } else {
        (current - previous) / previous * 100.0
    };
    let up = change >= 0.0;
    let text = format!(
        "{}{}%",
        if up { "+" } else { "−" },
        to_fixed(change.abs(), 1)
    );
    // `!` replaces the badge's own background.
    let fill = if up != invert {
        "bg-chart-4!"
    } else {
        "bg-chart-2!"
    };
    Ok(view! {
        badge(attrs: attributes! { class=(fill) },
            if up {
                icon(data: iconify_icon!("lucide:arrow-up-right"))
            } else {
                icon(data: iconify_icon!("lucide:arrow-down-right"))
            }
            (text)
        )
    })
}

/// Four stat cards in a row on wide screens, two on tablets, one on phones.
#[component]
pub async fn stat_grid(cx: &Cx, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 xl:grid-cols-4">(child)</div>
    })
}

/// A figure with its label. `compare` (current, previous) adds the change
/// badge and `trend` the sparkline, in `color`.
#[component]
pub async fn stat_card(
    cx: &Cx,
    label: &str,
    value: String,
    #[default] compare: Option<(f64, f64)>,
    #[default] invert: bool,
    #[default] trend: Option<Vec<f64>>,
    #[default("var(--chart-1)")] color: &str,
) -> Result<impl View> {
    Ok(view! {
        // `!` replaces the card's own background.
        card(size: CardSize::Sm, attrs: attributes! { class="min-w-0 bg-secondary-background!" },
            card_content(attrs: attributes! { class="flex flex-col gap-2" },
                <div class="flex min-h-6 items-center justify-between gap-2">
                    <p class="text-sm">(label)</p>
                    if let Some((current, previous)) = compare {
                        delta(current: current, previous: previous, invert: invert)
                    }
                </div>
                <div class="flex items-end justify-between gap-4">
                    <p class="font-heading text-3xl tabular-nums">(value)</p>
                    if let Some(values) = trend {
                        sparkline(values: values, color: color)
                    }
                </div>
            )
        )
    })
}

/// A card with a title, an optional description and its content (usually a
/// chart) filling the rest. `class` adds classes to the card, such as a grid
/// span.
#[component]
pub async fn chart_card(
    cx: &Cx,
    title: &str,
    #[into]
    #[default]
    description: Option<String>,
    #[default] class: &str,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        card(attrs: attributes! { class=(class!("min-w-0 bg-secondary-background!", class)) },
            card_header(
                card_title((title))
                if let Some(description) = description {
                    card_description((description))
                }
            )
            card_content(attrs: attributes! { class="flex min-w-0 flex-1 flex-col justify-center" },
                (child)
            )
        )
    })
}

/// An order's status, filled with its color.
#[component]
pub async fn status_badge(cx: &Cx, status: Status) -> Result<impl View> {
    Ok(view! {
        badge(attrs: attributes! { style=(format!("background: {}", status.color())) }, (status.label()))
    })
}

/// A row of a [`heatmap`]: its label and a value per column. Missing values
/// leave their cells blank.
pub struct HeatmapRow {
    pub label: &'static str,
    pub values: Vec<f64>,
}

/// A grid of values shaded on a single-hue scale, `max` being full color.
#[component]
pub async fn heatmap(
    cx: &Cx,
    corner: &str,
    columns: Vec<String>,
    rows: Vec<HeatmapRow>,
    max: f64,
    #[default] suffix: &str,
) -> Result<impl View> {
    Ok(view! {
        <div class="overflow-x-auto">
            <table class="w-full min-w-120 border-separate border-spacing-1 text-center text-xs tabular-nums">
                <thead>
                    <tr>
                        <th scope="col" class="pb-1 text-left font-base">(corner)</th>
                        for column in &columns {
                            <th scope="col" class="pb-1 font-base">(column)</th>
                        }
                    </tr>
                </thead>
                <tbody>
                    for row in &rows {
                        <tr>
                            <th scope="row" class="pr-2 text-left font-heading">(row.label)</th>
                            for (index, column) in columns.iter().enumerate() {
                                match row.values.get(index) {
                                    Some(value) => <td
                                        title=(format!("{}, {column}: {value}{suffix}", row.label))
                                        class="h-9 rounded-base border-2 border-border"
                                        style=(format!(
                                            "background: color-mix(in oklab, var(--chart-4) {}%, var(--secondary-background))",
                                            round(value / max * 100.0),
                                        ))
                                    >
                                        (format!("{value}{suffix}"))
                                    </td>,
                                    None => <td></td>,
                                }
                            }
                        </tr>
                    }
                </tbody>
            </table>
        </div>
    })
}
