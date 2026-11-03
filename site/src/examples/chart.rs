//! Ports of Octane's chart examples (`src/examples/ui/chart/`), one function
//! per file.
//!
//! The data and configs are shared constants below. A config with icons is a
//! `static`: an icon is not copied into each use the way a constant is.
//! Recharts' custom renderers become props: label lists are `end_label` /
//! `inside_label` / `point_label` / `label`, custom dots are `dots` with
//! `colors` or `dot_icon`, and the tooltip `formatter` examples use the
//! tooltip's `unit` and `total`.
//!
//! The interactive charts are `#[shard]`s: their buttons and select set a
//! signal, and the shard re-renders the chart on the server.
//!
//! `class!` does not drop conflicting classes the way Octane's `cn` does, so
//! classes that replace a component's own (`bg-secondary-background!` over
//! the card's `bg-background`, `aspect-square!` over the chart's
//! `aspect-video`) carry Tailwind's `!`.

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    runtime::{Event, shard, signal},
    view::{Child, View, attributes, component, view},
};

use crate::components::{
    card::{card, card_content, card_description, card_footer, card_header, card_title},
    chart::{
        ChartConfig, ChartCurve, ChartLabel, ChartRow, ChartSeries, ChartTooltip, PieLabel,
        PieRing, TooltipIndicator, area_chart, bar_chart, format_number, line_chart, pie_chart,
    },
    select::{select, select_content, select_item},
};

// Series.

const DESKTOP: ChartSeries = ChartSeries::new("desktop", "Desktop", "var(--chart-1)");
const MOBILE: ChartSeries = ChartSeries::new("mobile", "Mobile", "var(--chart-2)");
const OTHER: ChartSeries = ChartSeries::new("other", "Other", "var(--chart-3)");
const RUNNING: ChartSeries = ChartSeries::new("running", "Running", "var(--chart-1)");
const SWIMMING: ChartSeries = ChartSeries::new("swimming", "Swimming", "var(--chart-2)");

/// Desktop visitors.
const DESKTOP_ONLY: ChartConfig = &[DESKTOP];
/// Desktop and mobile visitors, side by side.
const DESKTOP_MOBILE: ChartConfig = &[DESKTOP, MOBILE];
/// Mobile and desktop visitors, mobile at the bottom of the stack (Octane
/// declares that `<Area>` first).
const MOBILE_DESKTOP: ChartConfig = &[MOBILE, DESKTOP];
/// Mobile visitors.
const MOBILE_ONLY: ChartConfig = &[MOBILE];
/// Running and swimming, for the tooltip examples.
const ACTIVITIES: ChartConfig = &[RUNNING, SWIMMING];

/// Visitors by browser, colored per bar or point.
const VISITORS: ChartConfig = &[ChartSeries::new("visitors", "Visitors", "var(--chart-2)")];
/// One slice per browser.
const BROWSERS: ChartConfig = &[
    ChartSeries::new("chrome", "Chrome", "var(--chart-1)"),
    ChartSeries::new("safari", "Safari", "var(--chart-2)"),
    ChartSeries::new("firefox", "Firefox", "var(--chart-3)"),
    ChartSeries::new("edge", "Edge", "var(--chart-4)"),
    ChartSeries::new("other", "Other", "var(--chart-5)"),
];
/// The browsers' colors, in `BROWSERS` order, for per-category fills.
const BROWSER_COLORS: &[&str] = &[
    "var(--chart-1)",
    "var(--chart-2)",
    "var(--chart-3)",
    "var(--chart-4)",
    "var(--chart-5)",
];
/// One slice per month, for the stacked pie.
const MONTH_SLICES: ChartConfig = &[
    ChartSeries::new("january", "January", "var(--chart-1)"),
    ChartSeries::new("february", "February", "var(--chart-2)"),
    ChartSeries::new("march", "March", "var(--chart-3)"),
    ChartSeries::new("april", "April", "var(--chart-4)"),
    ChartSeries::new("may", "May", "var(--chart-5)"),
];

/// Desktop with the Activity icon, shown in the tooltip in place of a
/// marker.
static DESKTOP_ACTIVITY: [ChartSeries; 1] = [ChartSeries::with_icon(
    "desktop",
    "Desktop",
    "var(--chart-1)",
    iconify_icon!("lucide:activity"),
)];
/// Mobile and desktop with trend icons, shown in the legend.
static MOBILE_DESKTOP_ICONS: [ChartSeries; 2] = [
    ChartSeries::with_icon(
        "mobile",
        "Mobile",
        "var(--chart-2)",
        iconify_icon!("lucide:trending-up"),
    ),
    ChartSeries::with_icon(
        "desktop",
        "Desktop",
        "var(--chart-1)",
        iconify_icon!("lucide:trending-down"),
    ),
];
/// Running and swimming with icons, shown in the tooltip.
static ACTIVITY_ICONS: [ChartSeries; 2] = [
    ChartSeries::with_icon(
        "running",
        "Running",
        "var(--chart-1)",
        iconify_icon!("lucide:footprints"),
    ),
    ChartSeries::with_icon(
        "swimming",
        "Swimming",
        "var(--chart-2)",
        iconify_icon!("lucide:waves"),
    ),
];

// Data.

/// Desktop and mobile visitors per month.
const MONTHS: &[ChartRow] = &[
    ("January", &[186.0, 80.0]),
    ("February", &[305.0, 200.0]),
    ("March", &[237.0, 120.0]),
    ("April", &[73.0, 190.0]),
    ("May", &[209.0, 130.0]),
    ("June", &[214.0, 140.0]),
];
/// `MONTHS` with mobile first, for `MOBILE_DESKTOP`.
const MONTHS_STACKED: &[ChartRow] = &[
    ("January", &[80.0, 186.0]),
    ("February", &[200.0, 305.0]),
    ("March", &[120.0, 237.0]),
    ("April", &[190.0, 73.0]),
    ("May", &[130.0, 209.0]),
    ("June", &[140.0, 214.0]),
];
/// Desktop visitors per month.
const DESKTOP_MONTHS: &[ChartRow] = &[
    ("January", &[186.0]),
    ("February", &[305.0]),
    ("March", &[237.0]),
    ("April", &[73.0]),
    ("May", &[209.0]),
    ("June", &[214.0]),
];
/// Visitors per browser, in `BROWSERS` order.
const BROWSER_VISITS: &[ChartRow] = &[
    ("Chrome", &[275.0]),
    ("Safari", &[200.0]),
    ("Firefox", &[187.0]),
    ("Edge", &[173.0]),
    ("Other", &[90.0]),
];
/// Visitors per browser, for the pie charts.
const BROWSER_SHARES: &[f64] = &[275.0, 200.0, 187.0, 173.0, 90.0];
/// Daily mobile and desktop visitors, April to June 2024 (91 days). Mobile
/// comes first, to stack under desktop in the area chart.
const VISITS: [ChartRow; 91] = [
    ("2024-04-01", &[150.0, 222.0]),
    ("2024-04-02", &[180.0, 97.0]),
    ("2024-04-03", &[120.0, 167.0]),
    ("2024-04-04", &[260.0, 242.0]),
    ("2024-04-05", &[290.0, 373.0]),
    ("2024-04-06", &[340.0, 301.0]),
    ("2024-04-07", &[180.0, 245.0]),
    ("2024-04-08", &[320.0, 409.0]),
    ("2024-04-09", &[110.0, 59.0]),
    ("2024-04-10", &[190.0, 261.0]),
    ("2024-04-11", &[350.0, 327.0]),
    ("2024-04-12", &[210.0, 292.0]),
    ("2024-04-13", &[380.0, 342.0]),
    ("2024-04-14", &[220.0, 137.0]),
    ("2024-04-15", &[170.0, 120.0]),
    ("2024-04-16", &[190.0, 138.0]),
    ("2024-04-17", &[360.0, 446.0]),
    ("2024-04-18", &[410.0, 364.0]),
    ("2024-04-19", &[180.0, 243.0]),
    ("2024-04-20", &[150.0, 89.0]),
    ("2024-04-21", &[200.0, 137.0]),
    ("2024-04-22", &[170.0, 224.0]),
    ("2024-04-23", &[230.0, 138.0]),
    ("2024-04-24", &[290.0, 387.0]),
    ("2024-04-25", &[250.0, 215.0]),
    ("2024-04-26", &[130.0, 75.0]),
    ("2024-04-27", &[420.0, 383.0]),
    ("2024-04-28", &[180.0, 122.0]),
    ("2024-04-29", &[240.0, 315.0]),
    ("2024-04-30", &[380.0, 454.0]),
    ("2024-05-01", &[220.0, 165.0]),
    ("2024-05-02", &[310.0, 293.0]),
    ("2024-05-03", &[190.0, 247.0]),
    ("2024-05-04", &[420.0, 385.0]),
    ("2024-05-05", &[390.0, 481.0]),
    ("2024-05-06", &[520.0, 498.0]),
    ("2024-05-07", &[300.0, 388.0]),
    ("2024-05-08", &[210.0, 149.0]),
    ("2024-05-09", &[180.0, 227.0]),
    ("2024-05-10", &[330.0, 293.0]),
    ("2024-05-11", &[270.0, 335.0]),
    ("2024-05-12", &[240.0, 197.0]),
    ("2024-05-13", &[160.0, 197.0]),
    ("2024-05-14", &[490.0, 448.0]),
    ("2024-05-15", &[380.0, 473.0]),
    ("2024-05-16", &[400.0, 338.0]),
    ("2024-05-17", &[420.0, 499.0]),
    ("2024-05-18", &[350.0, 315.0]),
    ("2024-05-19", &[180.0, 235.0]),
    ("2024-05-20", &[230.0, 177.0]),
    ("2024-05-21", &[140.0, 82.0]),
    ("2024-05-22", &[120.0, 81.0]),
    ("2024-05-23", &[290.0, 252.0]),
    ("2024-05-24", &[220.0, 294.0]),
    ("2024-05-25", &[250.0, 201.0]),
    ("2024-05-26", &[170.0, 213.0]),
    ("2024-05-27", &[460.0, 420.0]),
    ("2024-05-28", &[190.0, 233.0]),
    ("2024-05-29", &[130.0, 78.0]),
    ("2024-05-30", &[280.0, 340.0]),
    ("2024-05-31", &[230.0, 178.0]),
    ("2024-06-01", &[200.0, 178.0]),
    ("2024-06-02", &[410.0, 470.0]),
    ("2024-06-03", &[160.0, 103.0]),
    ("2024-06-04", &[380.0, 439.0]),
    ("2024-06-05", &[140.0, 88.0]),
    ("2024-06-06", &[250.0, 294.0]),
    ("2024-06-07", &[370.0, 323.0]),
    ("2024-06-08", &[320.0, 385.0]),
    ("2024-06-09", &[480.0, 438.0]),
    ("2024-06-10", &[200.0, 155.0]),
    ("2024-06-11", &[150.0, 92.0]),
    ("2024-06-12", &[420.0, 492.0]),
    ("2024-06-13", &[130.0, 81.0]),
    ("2024-06-14", &[380.0, 426.0]),
    ("2024-06-15", &[350.0, 307.0]),
    ("2024-06-16", &[310.0, 371.0]),
    ("2024-06-17", &[520.0, 475.0]),
    ("2024-06-18", &[170.0, 107.0]),
    ("2024-06-19", &[290.0, 341.0]),
    ("2024-06-20", &[450.0, 408.0]),
    ("2024-06-21", &[210.0, 169.0]),
    ("2024-06-22", &[270.0, 317.0]),
    ("2024-06-23", &[530.0, 480.0]),
    ("2024-06-24", &[180.0, 132.0]),
    ("2024-06-25", &[190.0, 141.0]),
    ("2024-06-26", &[380.0, 434.0]),
    ("2024-06-27", &[490.0, 448.0]),
    ("2024-06-28", &[200.0, 149.0]),
    ("2024-06-29", &[160.0, 103.0]),
    ("2024-06-30", &[400.0, 446.0]),
];

/// Calories burned per day, running then swimming.
const WORKOUTS: &[ChartRow] = &[
    ("2024-07-15", &[450.0, 300.0]),
    ("2024-07-16", &[380.0, 420.0]),
    ("2024-07-17", &[520.0, 120.0]),
    ("2024-07-18", &[140.0, 550.0]),
    ("2024-07-19", &[600.0, 350.0]),
    ("2024-07-20", &[480.0, 400.0]),
];

// Formatting.

/// `January` as `Jan`, Octane's `value.slice(0, 3)`.
fn short_month(month: &str) -> String {
    month.chars().take(3).collect()
}

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// The year, month (1 to 12) and day of a `YYYY-MM-DD` date.
fn parse_date(date: &str) -> Option<(i32, usize, u32)> {
    let mut parts = date.splitn(3, '-');
    let year = parts.next()?.parse().ok()?;
    let month: usize = parts.next()?.parse().ok()?;
    let day = parts.next()?.parse().ok()?;
    (1..=12).contains(&month).then_some((year, month, day))
}

/// `2024-04-01` as `Apr 1`.
fn month_day(date: &str) -> String {
    parse_date(date).map_or_else(
        || date.to_owned(),
        |(_, month, day)| format!("{} {day}", &MONTH_NAMES[month - 1][..3]),
    )
}

/// `2024-04-01` as `Apr 1, 2024`.
fn month_day_year(date: &str) -> String {
    parse_date(date).map_or_else(
        || date.to_owned(),
        |(year, month, day)| format!("{} {day}, {year}", &MONTH_NAMES[month - 1][..3]),
    )
}

/// `2024-07-16` as `July 16, 2024`.
fn long_date(date: &str) -> String {
    parse_date(date).map_or_else(
        || date.to_owned(),
        |(year, month, day)| format!("{} {day}, {year}", MONTH_NAMES[month - 1]),
    )
}

/// `2024-07-15` as `Mon`, with Sakamoto's day-of-week formula.
fn short_weekday(date: &str) -> String {
    const OFFSETS: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let Some((year, month, day)) = parse_date(date) else {
        return date.to_owned();
    };
    // January and February count as months of the previous year.
    let year = if month < 3 { year - 1 } else { year };
    let weekday = (year + year / 4 - year / 100 + year / 400 + OFFSETS[month - 1] + day as i32) % 7;
    ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"][weekday as usize].to_owned()
}

// The card around each chart.

/// The footer of a chart card.
#[derive(Clone, Copy)]
enum Footer {
    /// No footer.
    None,
    /// A trend line over `note`, in a grid (the area charts).
    Trend(&'static str),
    /// A trend line over the visitors note, left-aligned (the bar and line
    /// charts).
    Column,
    /// A trend line over the visitors note, centered (the pie charts).
    Centered,
}

/// "Trending up by 5.2% this month", with its icon.
#[component]
async fn trend() -> Result<impl View> {
    Ok(view! {
        "Trending up by 5.2% this month "
        icon(data: iconify_icon!("lucide:trending-up"), attrs: attributes! { class="h-4 w-4" })
    })
}

/// A card with a chart: a title, a description, the chart and a footer.
#[component]
async fn chart_card(
    title: &str,
    description: &str,
    footer: Footer,
    #[default] pie: bool,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    const NOTE: &str = "Showing total visitors for the last 6 months";
    Ok(view! {
        card(attrs: attributes! { class="bg-secondary-background! text-foreground" },
            card_header(attrs: attributes! { class=(pie.then_some("items-center! pb-0")) },
                card_title((title))
                card_description((description))
            )
            card_content(attrs: attributes! { class=(pie.then_some("flex-1 pb-0")) },
                (child)
            )
            match footer {
                Footer::None => "",
                Footer::Trend(note) => card_footer(
                    <div class="flex w-full items-start gap-2 text-sm">
                        <div class="grid gap-2">
                            <div class="flex items-center gap-2 leading-none font-medium">trend()</div>
                            <div class="flex items-center gap-2 leading-none">(note)</div>
                        </div>
                    </div>
                ),
                Footer::Column => card_footer(attrs: attributes! { class="flex-col items-start! gap-2 text-sm" },
                    <div class="flex gap-2 leading-none font-medium">trend()</div>
                    <div class="leading-none">(NOTE)</div>
                ),
                Footer::Centered => card_footer(attrs: attributes! { class="flex-col gap-2 text-sm" },
                    <div class="flex items-center gap-2 leading-none font-medium">trend()</div>
                    <div class="leading-none">(NOTE)</div>
                ),
            }
        )
    })
}

/// A tooltip with a line marker.
const LINE_TOOLTIP: ChartTooltip = ChartTooltip {
    indicator: TooltipIndicator::Line,
    ..ChartTooltip::DEFAULT
};

/// A tooltip without its heading.
const NO_LABEL: ChartTooltip = ChartTooltip {
    hide_label: true,
    ..ChartTooltip::DEFAULT
};

// Area charts.

#[component]
pub async fn chart_area_default() -> Result<impl View> {
    Ok(view! {
        chart_card(
            title: "Area Chart",
            description: "Showing total visitors for the last 6 months",
            footer: Footer::Trend("January - June 2024"),
            area_chart(config: DESKTOP_ONLY, data: DESKTOP_MONTHS, curve: ChartCurve::Natural, category_format: short_month)
        )
    })
}

#[component]
pub async fn chart_area_linear() -> Result<impl View> {
    Ok(view! {
        chart_card(
            title: "Area Chart - Linear",
            description: "Showing total visitors for the last 6 months",
            footer: Footer::Trend("January - June 2024"),
            area_chart(
                config: DESKTOP_ONLY,
                data: DESKTOP_MONTHS,
                category_format: short_month,
                tooltip: Some(NO_LABEL),
            )
        )
    })
}

#[component]
pub async fn chart_area_step() -> Result<impl View> {
    Ok(view! {
        chart_card(
            title: "Area Chart - Step",
            description: "Showing total visitors for the last 6 months",
            footer: Footer::Trend("January - June 2024"),
            area_chart(
                config: &DESKTOP_ACTIVITY,
                data: DESKTOP_MONTHS,
                curve: ChartCurve::Step,
                category_format: short_month,
                tooltip: Some(NO_LABEL),
            )
        )
    })
}

#[component]
pub async fn chart_area_stacked() -> Result<impl View> {
    Ok(view! {
        chart_card(
            title: "Area Chart - Stacked",
            description: "Showing total visitors for the last 6 months",
            footer: Footer::Trend("January - June 2024"),
            area_chart(
                config: MOBILE_DESKTOP,
                data: MONTHS_STACKED,
                curve: ChartCurve::Natural,
                stacked: true,
                category_format: short_month,
            )
        )
    })
}

#[component]
pub async fn chart_area_stacked_expand() -> Result<impl View> {
    // Other, mobile and desktop, bottom to top, as Octane declares them.
    const CONFIG: ChartConfig = &[OTHER, MOBILE, DESKTOP];
    const DATA: &[ChartRow] = &[
        ("January", &[45.0, 80.0, 186.0]),
        ("February", &[100.0, 200.0, 305.0]),
        ("March", &[150.0, 120.0, 237.0]),
        ("April", &[50.0, 190.0, 73.0]),
        ("May", &[100.0, 130.0, 209.0]),
        ("June", &[160.0, 140.0, 214.0]),
    ];
    Ok(view! {
        chart_card(
            title: "Area Chart - Stacked Expanded",
            description: "Showing total visitors for the last 6months",
            footer: Footer::Trend("January - June 2024"),
            area_chart(
                config: CONFIG,
                data: DATA,
                curve: ChartCurve::Natural,
                expand: true,
                category_format: short_month,
                tooltip: Some(LINE_TOOLTIP),
            )
        )
    })
}

#[component]
pub async fn chart_area_legend() -> Result<impl View> {
    Ok(view! {
        chart_card(
            title: "Area Chart - Legend",
            description: "Showing total visitors for the last 6 months",
            footer: Footer::Trend("January - June 2024"),
            area_chart(
                config: MOBILE_DESKTOP,
                data: MONTHS_STACKED,
                curve: ChartCurve::Natural,
                stacked: true,
                category_format: short_month,
                tooltip: Some(LINE_TOOLTIP),
                legend: true,
            )
        )
    })
}

#[component]
pub async fn chart_area_icons() -> Result<impl View> {
    Ok(view! {
        chart_card(
            title: "Area Chart - Icons",
            description: "Showing total visitors for the last 6 months",
            footer: Footer::Trend("January - June 2024"),
            area_chart(
                config: &MOBILE_DESKTOP_ICONS,
                data: MONTHS_STACKED,
                curve: ChartCurve::Natural,
                stacked: true,
                category_format: short_month,
                tooltip: Some(LINE_TOOLTIP),
                legend: true,
            )
        )
    })
}

#[component]
pub async fn chart_area_axes() -> Result<impl View> {
    Ok(view! {
        chart_card(
            title: "Area Chart - Axes",
            description: "Showing total visitors for the last 6 months",
            footer: Footer::Trend("January - June 2024"),
            area_chart(
                config: MOBILE_DESKTOP,
                data: MONTHS_STACKED,
                curve: ChartCurve::Natural,
                stacked: true,
                value_axis: true,
                value_ticks: 3,
                category_format: short_month,
            )
        )
    })
}

/// The interactive area chart. Picking a range sets `range`, and the shard
/// re-renders the chart with that many days.
#[shard]
async fn area_interactive_card(cx: &Cx) -> Result<impl View> {
    let range = signal(cx, || "90d".to_owned());
    // The browser sends `range` back: anything unknown is the default.
    let (shown, start) = match range.get().as_str() {
        "30d" => ("30d", "2024-05-31"),
        "7d" => ("7d", "2024-06-23"),
        _ => ("90d", "2024-04-01"),
    };
    // The days from `start` on.
    let rows: Vec<ChartRow> = VISITS
        .iter()
        .filter(|(date, _)| *date >= start)
        .copied()
        .collect();
    let tooltip = ChartTooltip {
        label_format: Some(month_day),
        ..ChartTooltip::DEFAULT
    };

    // `flex!` and friends beat the card header's grid layout and spacing.
    Ok(view! {
        card(attrs: attributes! { class="bg-secondary-background! text-foreground" },
            card_header(attrs: attributes! {
                class="flex! items-center! sm:gap-2! gap-4! space-y-0 sm:flex-row flex-col"
            },
                <div class="grid flex-1 gap-1 text-center sm:text-left">
                    card_title("Area Chart - Interactive")
                    card_description("Showing total visitors for the last 3 months")
                </div>
                select(attrs: attributes! {
                    class="w-[160px] sm:ml-auto"
                    aria-label="Select a value"
                    @change=$(|e: Event| range.set(e.target.value))
                },
                    select_content(
                        select_item(attrs: attributes! { value="90d" selected=(shown == "90d") }, "Last 3 months")
                        select_item(attrs: attributes! { value="30d" selected=(shown == "30d") }, "Last 30 days")
                        select_item(attrs: attributes! { value="7d" selected=(shown == "7d") }, "Last 7 days")
                    )
                )
            )
            card_content(attrs: attributes! { class="px-2! pt-4 sm:px-6! sm:pt-6" },
                area_chart(
                    config: MOBILE_DESKTOP,
                    data: &rows,
                    curve: ChartCurve::Natural,
                    stacked: true,
                    category_format: month_day,
                    max_category_ticks: Some(8),
                    tooltip: Some(tooltip),
                    legend: true,
                    attrs: attributes! { class="aspect-auto! h-[250px] w-full" },
                )
            )
        )
    })
}

#[component]
pub async fn chart_area_interactive() -> Result<impl View> {
    Ok(view! { area_interactive_card() })
}

// Bar charts.

#[component]
pub async fn chart_bar_default() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Bar Chart", description: "January - June 2024", footer: Footer::Column,
            bar_chart(
                config: DESKTOP_ONLY,
                data: DESKTOP_MONTHS,
                radius: 8.0,
                category_format: short_month,
                tooltip: Some(NO_LABEL),
            )
        )
    })
}

#[component]
pub async fn chart_bar_horizontal() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Bar Chart - Horizontal", description: "January - June 2024", footer: Footer::Column,
            bar_chart(
                config: DESKTOP_ONLY,
                data: DESKTOP_MONTHS,
                horizontal: true,
                radius: 5.0,
                grid: false,
                category_format: short_month,
                tooltip: Some(NO_LABEL),
            )
        )
    })
}

#[component]
pub async fn chart_bar_multiple() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Bar Chart - Multiple", description: "January - June 2024", footer: Footer::Column,
            bar_chart(config: DESKTOP_MOBILE, data: MONTHS, radius: 4.0, category_format: short_month)
        )
    })
}

#[component]
pub async fn chart_bar_stacked() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Bar Chart - Stacked + Legend", description: "January - June 2024", footer: Footer::Column,
            bar_chart(
                config: DESKTOP_MOBILE,
                data: MONTHS,
                stacked: true,
                radius: 4.0,
                category_format: short_month,
                tooltip: Some(ChartTooltip { cursor: true, ..NO_LABEL }),
                legend: true,
            )
        )
    })
}

#[component]
pub async fn chart_bar_label() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Bar Chart - Label", description: "January - June 2024", footer: Footer::Column,
            bar_chart(
                config: DESKTOP_ONLY,
                data: DESKTOP_MONTHS,
                radius: 8.0,
                end_label: ChartLabel::Value,
                category_format: short_month,
                tooltip: Some(NO_LABEL),
            )
        )
    })
}

#[component]
pub async fn chart_bar_label_custom() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Bar Chart - Custom Label", description: "January - June 2024", footer: Footer::Column,
            bar_chart(
                config: DESKTOP_ONLY,
                data: DESKTOP_MONTHS,
                horizontal: true,
                radius: 4.0,
                inside_label: ChartLabel::Category,
                end_label: ChartLabel::Value,
                category_axis: false,
                tooltip: Some(LINE_TOOLTIP),
            )
        )
    })
}

#[component]
pub async fn chart_bar_mixed() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Bar Chart - Mixed", description: "January - June 2024", footer: Footer::Column,
            bar_chart(
                config: VISITORS,
                data: BROWSER_VISITS,
                colors: BROWSER_COLORS,
                horizontal: true,
                radius: 5.0,
                grid: false,
                tooltip: Some(NO_LABEL),
            )
        )
    })
}

#[component]
pub async fn chart_bar_active() -> Result<impl View> {
    const DATA: &[ChartRow] = &[
        ("Chrome", &[187.0]),
        ("Safari", &[200.0]),
        ("Firefox", &[275.0]),
        ("Edge", &[173.0]),
        ("Other", &[90.0]),
    ];
    Ok(view! {
        chart_card(title: "Bar Chart - Active", description: "January - June 2024", footer: Footer::Column,
            bar_chart(
                config: VISITORS,
                data: DATA,
                colors: BROWSER_COLORS,
                radius: 8.0,
                active_index: Some(2),
                tooltip: Some(NO_LABEL),
            )
        )
    })
}

#[component]
pub async fn chart_bar_negative() -> Result<impl View> {
    const DATA: &[ChartRow] = &[
        ("January", &[186.0]),
        ("February", &[205.0]),
        ("March", &[-207.0]),
        ("April", &[173.0]),
        ("May", &[-209.0]),
        ("June", &[214.0]),
    ];
    // Positive months in the first chart color, negative ones in the second.
    let colors: Vec<&str> = DATA
        .iter()
        .map(|(_, values)| {
            if values[0] > 0.0 {
                "var(--chart-1)"
            } else {
                "var(--chart-2)"
            }
        })
        .collect();
    Ok(view! {
        chart_card(title: "Bar Chart - Negative", description: "January - June 2024", footer: Footer::Column,
            bar_chart(
                config: VISITORS,
                data: DATA,
                colors: &colors,
                end_label: ChartLabel::Category,
                category_axis: false,
                tooltip: Some(ChartTooltip { hide_indicator: true, ..NO_LABEL }),
            )
        )
    })
}

/// The card header of the interactive bar and line charts: a title and a
/// button per series showing its total, which sets `active`.
#[component]
async fn interactive_header(
    title: &str,
    active: topcoat::runtime::Signal<String>,
    shown: &str,
) -> Result<impl View> {
    let totals = [("desktop", "Desktop", 1), ("mobile", "Mobile", 0)].map(|(key, label, index)| {
        let total: f64 = VISITS.iter().map(|(_, values)| values[index]).sum();
        (key, label, format_number(total))
    });
    Ok(view! {
        card_header(attrs: attributes! {
            class="flex! flex-col items-stretch! space-y-0 p-0! gap-0! border-b-2 border-b-border sm:flex-row"
        },
            <div class="flex flex-1 flex-col justify-center gap-1 sm:py-0 py-4 px-6">
                card_title((title))
                card_description("Showing total visitors for the last 3 months")
            </div>
            <div class="flex">
                for (key, label, total) in totals {
                    <button
                        data-active=(if key == shown { "true" } else { "false" })
                        class="data-[active=true]:bg-[var(--chart-1)] data-[active=true]:text-main-foreground \
                            text-foreground even:data-[active=true]:bg-[var(--chart-2)] relative z-10 flex \
                            flex-1 flex-col justify-center gap-1 px-6 py-4 text-left sm:border-t-0 \
                            border-t-border border-t-2 even:border-l-2 sm:border-l-2 border-l-border \
                            sm:px-8 sm:py-6"
                        @click=$(|_e: Event| active.set(key.to_owned()))
                    >
                        <span class="text-xs">(label)</span>
                        <span class="text-lg leading-none font-heading sm:text-3xl">(total)</span>
                    </button>
                }
            </div>
        )
    })
}

/// The series an interactive chart shows: its config and its column in
/// `VISITS`. The browser sends `active` back, so anything unknown is
/// desktop.
fn visits_series(active: &str) -> (&'static str, ChartConfig<'static>, usize) {
    if active == "mobile" {
        ("mobile", MOBILE_ONLY, 0)
    } else {
        ("desktop", DESKTOP_ONLY, 1)
    }
}

/// The interactive bar chart. Its header buttons set `active`, and the
/// shard re-renders the chart with that series.
#[shard]
async fn bar_interactive_card(cx: &Cx) -> Result<impl View> {
    let active = signal(cx, || "desktop".to_owned());
    let (shown, config, column) = visits_series(&active.get());
    let rows: Vec<ChartRow> = VISITS
        .iter()
        .map(|(date, values)| (*date, &values[column..=column]))
        .collect();
    let tooltip = ChartTooltip {
        name: Some("Page Views"),
        label_format: Some(month_day_year),
        cursor: true,
        class: Some("w-[150px]"),
        ..ChartTooltip::DEFAULT
    };

    // `[&_[data-slot=chart-bar]]:border!` is Octane's 1px bar outline.
    Ok(view! {
        card(attrs: attributes! { class="bg-secondary-background! py-0! text-foreground" },
            interactive_header(title: "Bar Chart - Interactive", active: active, shown: shown)
            card_content(attrs: attributes! { class="p-4! sm:p-6!" },
                bar_chart(
                    config: config,
                    data: &rows,
                    category_format: month_day,
                    max_category_ticks: Some(8),
                    tooltip: Some(tooltip),
                    attrs: attributes! {
                        class="aspect-auto! h-[250px] w-full [&_[data-slot=chart-bar]]:border!"
                    },
                )
            )
        )
    })
}

#[component]
pub async fn chart_bar_interactive() -> Result<impl View> {
    Ok(view! { bar_interactive_card() })
}

// Line charts.

#[component]
pub async fn chart_line_default() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Line Chart", description: "January - June 2024", footer: Footer::Column,
            line_chart(
                config: DESKTOP_ONLY,
                data: DESKTOP_MONTHS,
                curve: ChartCurve::Natural,
                category_format: short_month,
                tooltip: Some(NO_LABEL),
            )
        )
    })
}

#[component]
pub async fn chart_line_linear() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Line Chart - Linear", description: "January - June 2024", footer: Footer::Column,
            line_chart(
                config: DESKTOP_ONLY,
                data: DESKTOP_MONTHS,
                category_format: short_month,
                tooltip: Some(NO_LABEL),
            )
        )
    })
}

#[component]
pub async fn chart_line_step() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Line Chart - Step", description: "January - June 2024", footer: Footer::Column,
            line_chart(
                config: DESKTOP_ONLY,
                data: DESKTOP_MONTHS,
                curve: ChartCurve::Step,
                category_format: short_month,
                tooltip: Some(NO_LABEL),
            )
        )
    })
}

#[component]
pub async fn chart_line_multiple() -> Result<impl View> {
    Ok(view! {
        chart_card(
            title: "Line Chart - Multiple",
            description: "January - June 2024",
            footer: Footer::Trend("Showing total visitors for the last 6 months"),
            line_chart(
                config: DESKTOP_MOBILE,
                data: MONTHS,
                curve: ChartCurve::Monotone,
                category_format: short_month,
            )
        )
    })
}

#[component]
pub async fn chart_line_dots() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Line Chart - Dots", description: "January - June 2024", footer: Footer::Column,
            line_chart(
                config: DESKTOP_ONLY,
                data: DESKTOP_MONTHS,
                curve: ChartCurve::Natural,
                dots: true,
                active_dot_radius: 6.0,
                category_format: short_month,
                tooltip: Some(NO_LABEL),
            )
        )
    })
}

#[component]
pub async fn chart_line_dots_custom() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Line Chart - Custom Dots", description: "January - June 2024", footer: Footer::Column,
            line_chart(
                config: DESKTOP_ONLY,
                data: DESKTOP_MONTHS,
                curve: ChartCurve::Natural,
                dot_icon: Some(iconify_icon!("lucide:git-commit-vertical")),
                category_format: short_month,
                tooltip: Some(NO_LABEL),
            )
        )
    })
}

#[component]
pub async fn chart_line_dots_colors() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Line Chart - Dots Colors", description: "January - June 2024", footer: Footer::Column,
            line_chart(
                config: VISITORS,
                data: BROWSER_VISITS,
                colors: BROWSER_COLORS,
                curve: ChartCurve::Natural,
                dots: true,
                dot_radius: 5.0,
                category_axis: false,
                tooltip: Some(ChartTooltip { name: Some("Visitors"), ..LINE_TOOLTIP }),
                attrs: attributes! { class="px-3" },
            )
        )
    })
}

#[component]
pub async fn chart_line_label() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Line Chart - Label", description: "January - June 2024", footer: Footer::Column,
            line_chart(
                config: DESKTOP_ONLY,
                data: DESKTOP_MONTHS,
                curve: ChartCurve::Natural,
                dots: true,
                active_dot_radius: 6.0,
                point_label: ChartLabel::Value,
                category_format: short_month,
                tooltip: Some(LINE_TOOLTIP),
            )
        )
    })
}

#[component]
pub async fn chart_line_label_custom() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Line Chart - Custom Label", description: "January - June 2024", footer: Footer::Column,
            line_chart(
                config: VISITORS,
                data: BROWSER_VISITS,
                curve: ChartCurve::Natural,
                dots: true,
                active_dot_radius: 6.0,
                point_label: ChartLabel::Category,
                category_axis: false,
                tooltip: Some(ChartTooltip { name: Some("Visitors"), hide_label: true, ..LINE_TOOLTIP }),
                attrs: attributes! { class="px-3" },
            )
        )
    })
}

/// The interactive line chart, in the series' own color (Octane's
/// `stroke-[var(--color-desktop)]` override).
#[shard]
async fn line_interactive_card(cx: &Cx) -> Result<impl View> {
    let active = signal(cx, || "desktop".to_owned());
    let (shown, config, column) = visits_series(&active.get());
    let rows: Vec<ChartRow> = VISITS
        .iter()
        .map(|(date, values)| (*date, &values[column..=column]))
        .collect();
    let tooltip = ChartTooltip {
        name: Some("Page Views"),
        label_format: Some(month_day_year),
        cursor: true,
        class: Some("w-[150px]"),
        ..ChartTooltip::DEFAULT
    };

    Ok(view! {
        card(attrs: attributes! { class="bg-secondary-background! py-0! text-foreground" },
            interactive_header(title: "Line Chart - Interactive", active: active, shown: shown)
            card_content(attrs: attributes! { class="p-4! sm:p-6!" },
                line_chart(
                    config: config,
                    data: &rows,
                    curve: ChartCurve::Monotone,
                    colored_lines: true,
                    active_dot_color: Some("var(--chart-active-dot)"),
                    category_format: month_day,
                    max_category_ticks: Some(8),
                    tooltip: Some(tooltip),
                    attrs: attributes! { class="aspect-auto! h-[250px] w-full" },
                )
            )
        )
    })
}

#[component]
pub async fn chart_line_interactive() -> Result<impl View> {
    Ok(view! { line_interactive_card() })
}

// Pie charts. Each sits in a square box; `aspect-square!` beats the chart's
// `aspect-video`.

/// The browsers as a full pie.
const PIE: &[PieRing] = &[PieRing::new(BROWSER_SHARES)];

/// The browsers as a donut, with a hole of radius 60.
const DONUT: &[PieRing] = &[PieRing {
    inner_radius: 60.0,
    ..PieRing::new(BROWSER_SHARES)
}];

#[component]
pub async fn chart_pie_simple() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Pie Chart", description: "January - June 2024", footer: Footer::Centered, pie: true,
            pie_chart(
                config: BROWSERS,
                rings: PIE,
                attrs: attributes! { class="mx-auto aspect-square! max-h-[250px]" },
            )
        )
    })
}

#[component]
pub async fn chart_pie_label() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Pie Chart - Label", description: "January - June 2024", footer: Footer::Centered, pie: true,
            pie_chart(
                config: BROWSERS,
                rings: PIE,
                label: PieLabel::Value,
                attrs: attributes! { class="mx-auto aspect-square! max-h-[250px] pb-0" },
            )
        )
    })
}

#[component]
pub async fn chart_pie_label_custom() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Pie Chart - Custom Label", description: "January - June 2024", footer: Footer::Centered, pie: true,
            pie_chart(
                config: BROWSERS,
                rings: PIE,
                label: PieLabel::ValueOnly,
                tooltip: Some(ChartTooltip { name: Some("Visitors"), ..NO_LABEL }),
                attrs: attributes! { class="mx-auto aspect-square! max-h-[250px] px-0" },
            )
        )
    })
}

#[component]
pub async fn chart_pie_label_list() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Pie Chart - Label List", description: "January - June 2024", footer: Footer::Centered, pie: true,
            pie_chart(
                config: BROWSERS,
                rings: PIE,
                label: PieLabel::Name,
                tooltip: Some(ChartTooltip { name: Some("Visitors"), ..NO_LABEL }),
                attrs: attributes! { class="mx-auto aspect-square! max-h-[250px]" },
            )
        )
    })
}

#[component]
pub async fn chart_pie_legend() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Pie Chart - Legend", description: "January - June 2024", footer: Footer::None, pie: true,
            pie_chart(
                config: BROWSERS,
                rings: PIE,
                tooltip: None,
                legend: true,
                legend_class: Some("-translate-y-2 flex-wrap gap-2 *:basis-1/4 *:justify-center"),
                attrs: attributes! { class="mx-auto aspect-square! max-h-[300px]" },
            )
        )
    })
}

#[component]
pub async fn chart_pie_donut() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Pie Chart - Donut", description: "January - June 2024", footer: Footer::Centered, pie: true,
            pie_chart(
                config: BROWSERS,
                rings: DONUT,
                attrs: attributes! { class="mx-auto aspect-square! max-h-[250px]" },
            )
        )
    })
}

#[component]
pub async fn chart_pie_donut_active() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Pie Chart - Donut Active", description: "January - June 2024", footer: Footer::Centered, pie: true,
            pie_chart(
                config: BROWSERS,
                rings: DONUT,
                active_index: Some(0),
                attrs: attributes! { class="mx-auto aspect-square! max-h-[250px]" },
            )
        )
    })
}

#[component]
pub async fn chart_pie_donut_text() -> Result<impl View> {
    // This example has its own figures for Firefox and Other.
    const SHARES: &[f64] = &[275.0, 200.0, 287.0, 173.0, 190.0];
    const RINGS: &[PieRing] = &[PieRing {
        inner_radius: 60.0,
        ..PieRing::new(SHARES)
    }];
    let total = format_number(SHARES.iter().sum());
    Ok(view! {
        chart_card(title: "Pie Chart - Donut with Text", description: "January - June 2024", footer: Footer::Centered, pie: true,
            pie_chart(
                config: BROWSERS,
                rings: RINGS,
                center_label: Some((&total, "Visitors")),
                attrs: attributes! { class="mx-auto aspect-square! max-h-[250px]" },
            )
        )
    })
}

#[component]
pub async fn chart_pie_stacked() -> Result<impl View> {
    // Desktop inside, mobile in a ring around it; each slice is a month.
    const RINGS: &[PieRing] = &[
        PieRing {
            label: "Desktop",
            outer_radius: 60.0,
            ..PieRing::new(&[186.0, 305.0, 237.0, 173.0, 209.0])
        },
        PieRing {
            label: "Mobile",
            inner_radius: 70.0,
            outer_radius: 90.0,
            ..PieRing::new(&[80.0, 200.0, 120.0, 190.0, 130.0])
        },
    ];
    Ok(view! {
        chart_card(title: "Pie Chart - Stacked", description: "January - June 2024", footer: Footer::Centered, pie: true,
            pie_chart(
                config: MONTH_SLICES,
                rings: RINGS,
                tooltip: Some(LINE_TOOLTIP),
                attrs: attributes! { class="mx-auto aspect-square! max-h-[250px]" },
            )
        )
    })
}

// Tooltips. Each opens on Tuesday (`default_index: Some(1)`) until another
// day is hovered.

/// The stacked workout bar chart shared by the tooltip examples.
#[component]
async fn workout_chart(
    #[default] config: Option<ChartConfig<'static>>,
    tooltip: ChartTooltip<'static>,
) -> Result<impl View> {
    Ok(view! {
        bar_chart(
            config: config.unwrap_or(ACTIVITIES),
            data: WORKOUTS,
            stacked: true,
            radius: 4.0,
            grid: false,
            category_format: short_weekday,
            tooltip: Some(ChartTooltip { default_index: Some(1), ..tooltip }),
        )
    })
}

#[component]
pub async fn chart_tooltip_default() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Tooltip - Default", description: "Default tooltip with ChartTooltipContent.", footer: Footer::None,
            workout_chart(tooltip: ChartTooltip::DEFAULT)
        )
    })
}

#[component]
pub async fn chart_tooltip_indicator_line() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Tooltip - Line Indicator", description: "Tooltip with line indicator.", footer: Footer::None,
            workout_chart(tooltip: LINE_TOOLTIP)
        )
    })
}

#[component]
pub async fn chart_tooltip_indicator_none() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Tooltip - No Indicator", description: "Tooltip with no indicator.", footer: Footer::None,
            workout_chart(tooltip: ChartTooltip { hide_indicator: true, ..ChartTooltip::DEFAULT })
        )
    })
}

#[component]
pub async fn chart_tooltip_label_custom() -> Result<impl View> {
    Ok(view! {
        chart_card(
            title: "Tooltip - Custom label",
            description: "Tooltip with custom label from chartConfig.",
            footer: Footer::None,
            workout_chart(tooltip: ChartTooltip { label: Some("Activities"), ..LINE_TOOLTIP })
        )
    })
}

#[component]
pub async fn chart_tooltip_label_formatter() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Tooltip - Label Formatter", description: "Tooltip with label formatter.", footer: Footer::None,
            workout_chart(tooltip: ChartTooltip { label_format: Some(long_date), ..ChartTooltip::DEFAULT })
        )
    })
}

#[component]
pub async fn chart_tooltip_label_none() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Tooltip - No Label", description: "Tooltip with no label.", footer: Footer::None,
            workout_chart(tooltip: ChartTooltip { hide_indicator: true, ..NO_LABEL })
        )
    })
}

#[component]
pub async fn chart_tooltip_formatter() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Tooltip - Formatter", description: "Tooltip with custom formatter .", footer: Footer::None,
            workout_chart(tooltip: ChartTooltip { hide_indicator: true, unit: Some("kcal"), ..NO_LABEL })
        )
    })
}

#[component]
pub async fn chart_tooltip_icons() -> Result<impl View> {
    Ok(view! {
        chart_card(title: "Tooltip - Icons", description: "Tooltip with icons.", footer: Footer::None,
            workout_chart(config: Some(&ACTIVITY_ICONS), tooltip: NO_LABEL)
        )
    })
}

#[component]
pub async fn chart_tooltip_advanced() -> Result<impl View> {
    Ok(view! {
        chart_card(
            title: "Tooltip - Advanced",
            description: "Tooltip with custom formatter and total.",
            footer: Footer::None,
            workout_chart(tooltip: ChartTooltip {
                unit: Some("kcal"),
                total: true,
                class: Some("w-[180px]"),
                ..NO_LABEL
            })
        )
    })
}
