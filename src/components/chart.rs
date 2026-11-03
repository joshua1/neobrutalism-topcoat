//! Charts rendered on the server as SVG and HTML, replacing Recharts:
//! [`bar_chart`], [`line_chart`], [`area_chart`] and [`pie_chart`].
//!
//! A chart takes its series as a [`ChartConfig`] (a slice of
//! [`ChartSeries`]: key, label, color and an optional icon) and its data as
//! [`ChartRow`]s, `(category, &[value per series])`, in config order:
//!
//! ```ignore
//! const CONFIG: ChartConfig = &[
//!     ChartSeries::new("desktop", "Desktop", "var(--chart-1)"),
//!     ChartSeries::new("mobile", "Mobile", "var(--chart-2)"),
//! ];
//! const DATA: &[ChartRow] = &[("January", &[186.0, 80.0]), ("February", &[305.0, 200.0])];
//!
//! view! { bar_chart(config: CONFIG, data: DATA, radius: 4.0) }
//! ```
//!
//! Like Octane's `ChartContainer`, the chart's root sets a `--color-<key>`
//! custom property for each series, and the marks are filled with
//! `var(--color-<key>)`, so a stylesheet can recolor a series. Keys must be
//! CSS identifiers.
//!
//! Each chart sits in an `aspect-video` box (override it with `aspect-*!` or
//! `aspect-auto h-[250px]` in `attrs`). Inside, the plot area is an SVG whose
//! view box (`0 0 100 100`, `preserveAspectRatio="none"`) stretches to fill
//! it, so every position is a percentage. Lines keep a 2px stroke with
//! `vector-effect: non-scaling-stroke`. Bars, dots, labels and axis ticks are
//! HTML laid over the SVG at the same percentages, so their borders, corners
//! and text don't stretch.
//!
//! Tooltips need no script: each category has a hover column holding its
//! tooltip, shown with `group-hover`. A pie chart draws each slice in its own
//! `<svg>` followed by its tooltip, shown with `[:hover+&]:visible`.
//!
//! Differences from Recharts: no animation, no keyboard navigation of the
//! tooltip (`accessibilityLayer`), tick labels are thinned by count
//! (`max_category_ticks`) rather than measured, and margins are fixed
//! defaults rather than props. Custom label, dot and tooltip renderers become
//! the options of [`ChartLabel`], [`PieLabel`], `dot_icon` and
//! [`ChartTooltip`].

// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{IconData, icon},
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

/// A function that turns a category into display text, such as a tick label.
pub type Format = fn(&str) -> String;

/// The default [`Format`]: the category as it is.
fn same(category: &str) -> String {
    category.to_owned()
}

/// One series of a chart (or one slice of a [`pie_chart`]).
///
/// Build it with [`ChartSeries::new`], or [`ChartSeries::with_icon`]. A
/// config holding icons must be a `static`, not a `const`: a reference to a
/// constant is only made `'static` when its value has nothing to drop, and
/// an icon does.
///
/// ```ignore
/// static CONFIG: [ChartSeries; 1] = [ChartSeries::with_icon(
///     "running",
///     "Running",
///     "var(--chart-1)",
///     iconify_icon!("lucide:footprints"),
/// )];
/// ```
#[derive(Clone, Debug)]
pub struct ChartSeries<'a> {
    /// The name of the series' `--color-<key>` custom property.
    pub key: &'a str,
    /// The name shown in the legend and tooltip.
    pub label: &'a str,
    /// Any CSS color, usually a theme variable such as `var(--chart-1)`.
    pub color: &'a str,
    /// Shown in place of the color swatch in the legend and tooltip.
    pub icon: Option<IconData>,
}

#[allow(dead_code)]
impl<'a> ChartSeries<'a> {
    /// A series without an icon.
    #[must_use]
    pub const fn new(key: &'a str, label: &'a str, color: &'a str) -> Self {
        Self {
            key,
            label,
            color,
            icon: None,
        }
    }

    /// A series with an icon.
    #[must_use]
    pub const fn with_icon(key: &'a str, label: &'a str, color: &'a str, data: IconData) -> Self {
        Self {
            key,
            label,
            color,
            icon: Some(data),
        }
    }

    /// The series' fill, `var(--color-<key>)`.
    fn fill(&self) -> String {
        format!("var(--color-{})", self.key)
    }
}

/// The series of a chart, in the order they stack and are listed.
pub type ChartConfig<'a> = &'a [ChartSeries<'a>];

/// One category of a bar, line or area chart: its name and one value per
/// series, in config order. A missing value counts as zero.
pub type ChartRow<'a> = (&'a str, &'a [f64]);

/// Formats a number the way `toLocaleString()` does in English: thousands
/// separators and at most three decimals (`1234.5` is `"1,234.5"`).
#[must_use]
#[allow(dead_code)]
pub fn format_number(value: f64) -> String {
    let text = format!("{:.3}", value.abs());
    let (whole, fraction) = text.split_once('.').unwrap_or((&text, ""));
    let fraction = fraction.trim_end_matches('0');
    let mut grouped = String::new();
    for (index, digit) in whole.chars().enumerate() {
        if index > 0 && (whole.len() - index) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    // No sign on a value that rounds to zero.
    let sign = if value < 0.0 && (whole != "0" || !fraction.is_empty()) {
        "-"
    } else {
        ""
    };
    if fraction.is_empty() {
        format!("{sign}{grouped}")
    } else {
        format!("{sign}{grouped}.{fraction}")
    }
}

/// The marker beside each value in a [`ChartTooltip`].
///
/// [`Default`] is `TooltipIndicator::Dot`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TooltipIndicator {
    /// A filled square.
    #[default]
    Dot,
    /// A thin filled bar.
    Line,
    /// A dashed line.
    Dashed,
}

/// What a chart's tooltip shows. Every field has a default, so set the ones
/// you need with struct update syntax:
///
/// ```ignore
/// tooltip: Some(ChartTooltip { indicator: TooltipIndicator::Line, ..ChartTooltip::DEFAULT })
/// ```
///
/// These cover Octane's `ChartTooltipContent` props: `label` is a fixed
/// label (`labelKey`), `label_format` formats the category (`labelFormatter`),
/// `name` replaces every value's series name (`nameKey`), and `unit` and
/// `total` stand in for the custom `formatter` examples.
#[derive(Clone, Copy, Debug, Default)]
pub struct ChartTooltip<'a> {
    /// The marker beside each value.
    pub indicator: TooltipIndicator,
    /// Hides the heading.
    pub hide_label: bool,
    /// Hides the markers.
    pub hide_indicator: bool,
    /// A fixed heading, in place of the category.
    pub label: Option<&'a str>,
    /// Formats the category for the heading.
    pub label_format: Option<Format>,
    /// A name for every value, in place of the series names.
    pub name: Option<&'a str>,
    /// A unit shown after each value, such as `"kcal"`.
    pub unit: Option<&'a str>,
    /// Adds a row with the sum of the values.
    pub total: bool,
    /// Highlights the hovered category: a band behind bars, a line through
    /// points.
    pub cursor: bool,
    /// The category whose tooltip shows while nothing is hovered.
    pub default_index: Option<usize>,
    /// Extra classes for the tooltip box, such as a width.
    pub class: Option<&'a str>,
}

impl<'a> ChartTooltip<'a> {
    /// The default tooltip, as a constant to build others from in a `const`,
    /// where `ChartTooltip::default()` can't be called.
    pub const DEFAULT: Self = Self {
        indicator: TooltipIndicator::Dot,
        hide_label: false,
        hide_indicator: false,
        label: None,
        label_format: None,
        name: None,
        unit: None,
        total: false,
        cursor: false,
        default_index: None,
        class: None,
    };

    /// The heading for `category`, or `None` when it is hidden.
    fn heading(&self, category: &str) -> Option<String> {
        if self.hide_label {
            return None;
        }
        Some(match (self.label, self.label_format) {
            (Some(label), _) => label.to_owned(),
            (None, Some(format)) => format(category),
            (None, None) => category.to_owned(),
        })
    }
}

/// The text drawn by a bar or a point.
///
/// [`Default`] is `ChartLabel::None`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ChartLabel {
    /// No label.
    #[default]
    None,
    /// The value.
    Value,
    /// The category, through the chart's `category_format`.
    Category,
}

impl ChartLabel {
    /// The label's text for a mark of `category` with `value`.
    fn text(self, category: &str, value: f64, format: Format) -> Option<String> {
        match self {
            Self::None => None,
            Self::Value => Some(format_number(value)),
            Self::Category => Some(format(category)),
        }
    }
}

/// The shape of a line or area between its points (Recharts' `type`).
///
/// [`Default`] is `ChartCurve::Linear`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ChartCurve {
    /// Straight segments.
    #[default]
    Linear,
    /// A natural cubic spline through every point (`natural`).
    Natural,
    /// A smooth curve that never overshoots a point (`monotone`).
    Monotone,
    /// Steps that change value halfway between points (`step`).
    Step,
}

/// Classes for the chart's root: the box the chart fills.
const CONTAINER: StaticClass = class!("flex aspect-video justify-center text-xs");

/// Classes for the tooltip box.
const TOOLTIP: StaticClass = class!(
    "border-border bg-secondary-background grid min-w-[8rem] items-start gap-1.5 rounded-lg \
     border-2 px-2.5 py-1.5 text-xs shadow-xl",
);

/// Classes that show a column's tooltip while the column is hovered.
const TOOLTIP_ON_HOVER: StaticClass =
    class!("pointer-events-none absolute z-10 w-max invisible group-hover/column:visible");

/// Classes for the default tooltip: shown until another column is hovered.
const TOOLTIP_DEFAULT: StaticClass = class!(
    "pointer-events-none absolute z-10 w-max visible \
     [[data-slot=chart-plot]:has([data-slot=chart-column]:hover:not([data-default]))_&]:invisible",
);

/// Where a column's tooltip sits.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TooltipSide {
    /// To the right of a column in the left half of the chart.
    Right,
    /// To the left of a column in the right half.
    Left,
    /// Under a row of a horizontal bar chart.
    Below,
}

impl TooltipSide {
    /// The side for a column whose middle is at `middle` percent.
    fn beside(middle: f64) -> Self {
        if middle > 50.0 {
            Self::Left
        } else {
            Self::Right
        }
    }

    /// Classes that place the tooltip.
    fn classes(self) -> StaticClass {
        match self {
            Self::Right => class!("top-0 left-[calc(100%+8px)]"),
            Self::Left => class!("top-0 right-[calc(100%+8px)]"),
            Self::Below => class!("top-full right-2 mt-1"),
        }
    }
}

/// The grid line color, from Octane's `stroke-[#80808080]` override.
const GRID_STROKE: &str = "#80808080";

/// The root of a chart: it sets a `--color-<key>` custom property for each
/// series of `config` and lays out its child. The chart components render it
/// themselves; use it to wrap your own content.
///
/// `attrs` are forwarded to the `<div>`, with extra classes appended.
#[component]
pub async fn chart_container(
    config: ChartConfig<'_>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let colors: Vec<String> = config
        .iter()
        .map(|series| format!("--color-{}: {}", series.key, series.color))
        .collect();
    Ok(view! {
        <div
            data-slot="chart"
            style=(colors.join("; "))
            class=(class!(CONTAINER, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A legend: a swatch (or icon) and label for each series of `config`.
///
/// The chart components render it when `legend` is set. `attrs` are
/// forwarded to the `<div>`, with extra classes appended.
#[component]
pub async fn chart_legend_content(
    config: ChartConfig<'_>,
    /// Lists the series by key, A to Z, as the bar, line and area charts do
    /// (Recharts sorts their legend by value). Otherwise config order.
    #[default]
    sorted: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let mut config: Vec<&ChartSeries<'_>> = config.iter().collect();
    if sorted {
        config.sort_by_key(|series| series.key);
    }
    Ok(view! {
        <div
            data-slot="chart-legend"
            class=(class!("flex items-center justify-center gap-4 pt-3", attrs.remove("class")))
            (attrs)
        >
            #[key(series.key)]
            for series in config {
                <div class="flex items-center gap-1.5 [&>svg]:h-3 [&>svg]:w-3 [&>svg]:text-foreground">
                    match series.icon.clone() {
                        Some(data) => icon(data: data),
                        None => <div
                            class="h-3 w-3 shrink-0 rounded-[2px] border-2 border-border"
                            style=(format!("background-color: {}", series.fill()))
                        ></div>,
                    }
                    (series.label)
                </div>
            }
        </div>
    })
}

/// One value in a tooltip.
struct TooltipItem {
    label: String,
    value: f64,
    color: String,
    icon: Option<IconData>,
}

/// The tooltip box (Octane's `ChartTooltipContent`). `attrs` place it.
#[component]
async fn chart_tooltip_content(
    tooltip: ChartTooltip<'_>,
    heading: Option<String>,
    items: Vec<TooltipItem>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let indicator = tooltip.indicator;
    // A single value nests the heading beside it, unless it has a dot.
    let nest = items.len() == 1 && indicator != TooltipIndicator::Dot;
    let heading_class = if tooltip.label_format.is_some() {
        "font-heading"
    } else {
        "font-base"
    };
    let total = format_number(items.iter().map(|item| item.value).sum());
    let unit = tooltip.unit;
    Ok(view! {
        <div
            data-slot="chart-tooltip"
            class=(class!(TOOLTIP, tooltip.class, attrs.remove("class")))
            (attrs)
        >
            match (&heading, nest) {
                (Some(text), false) => <div class=(heading_class)>(text.clone())</div>,
                _ => "",
            }
            <div class="grid gap-1.5">
                #[key(index)]
                for (index, item) in items.into_iter().enumerate() {
                    <div class=(class!(
                        "flex w-full flex-wrap items-stretch gap-2 [&>svg]:h-2.5 [&>svg]:w-2.5 \
                         [&>svg]:text-foreground",
                        "items-center" if indicator == TooltipIndicator::Dot,
                    ))>
                        match item.icon {
                            Some(data) => icon(data: data),
                            None => {
                                if !tooltip.hide_indicator {
                                    <div
                                        class=(class!(
                                            "shrink-0 rounded-[2px] bg-(--color-bg)",
                                            "size-3.5 border border-border" if indicator == TooltipIndicator::Dot,
                                            "w-2 border border-border" if indicator == TooltipIndicator::Line,
                                            "w-0 border-[1.5px] border-dashed bg-transparent" if indicator == TooltipIndicator::Dashed,
                                            "my-0.5" if nest && indicator == TooltipIndicator::Dashed,
                                        ))
                                        style=(if indicator == TooltipIndicator::Dashed {
                                            format!("--color-bg: {0}; --color-border: {0}", item.color)
                                        } else {
                                            format!("--color-bg: {}", item.color)
                                        })
                                    ></div>
                                }
                            },
                        }
                        <div class=(class!(
                            "flex flex-1 justify-between leading-none",
                            "items-end" if nest,
                            "items-center" if !nest,
                        ))>
                            <div class="grid gap-1.5">
                                match (&heading, nest) {
                                    (Some(text), true) => <div class=(heading_class)>(text.clone())</div>,
                                    _ => "",
                                }
                                <span class="text-foreground">(item.label)</span>
                            </div>
                            <span class="ml-2 flex items-baseline gap-0.5 font-mono font-medium text-foreground tabular-nums">
                                (format_number(item.value))
                                match unit {
                                    Some(unit) => <span class="font-normal">(unit)</span>,
                                    None => "",
                                }
                            </span>
                        </div>
                    </div>
                }
                if tooltip.total {
                    <div class="text-foreground mt-1.5 flex basis-full items-center border-t pt-1.5 text-xs font-medium">
                        "Total"
                        <div class="text-foreground ml-auto flex items-baseline gap-0.5 font-mono font-medium tabular-nums">
                            (total)
                            match unit {
                                Some(unit) => <span class="font-normal">(unit)</span>,
                                None => "",
                            }
                        </div>
                    </div>
                }
            </div>
        </div>
    })
}

/// A value axis: its range and its tick values.
struct ValueScale {
    min: f64,
    max: f64,
    ticks: Vec<f64>,
}

impl ValueScale {
    /// Recharts' "nice" ticks: `count` evenly spaced values covering `low` to
    /// `high`, with a step of 1, 1.05, 1.1, ... 2, ... 9.95 or 10 times a
    /// power of ten.
    fn new(low: f64, high: f64, count: usize) -> Self {
        let count = count.max(2);
        let (low, high) = if high > low {
            (low, high)
        } else {
            (low, low + 1.0)
        };
        let rough = (high - low) / (count - 1) as f64;
        let digits = rough.log10().floor() + 1.0;
        let magnitude = 10f64.powf(digits);
        let scale = if digits == 1.0 { 0.1 } else { 0.05 };
        // Round away float noise before `ceil`, so 15.000000001 stays 15.
        let ratio = ((rough / magnitude / scale) * 1e9).round() / 1e9;
        let step = ratio.ceil() * scale * magnitude;
        let first = (low / step).floor() as i64;
        let last = (high / step).ceil() as i64;
        let ticks: Vec<f64> = (first..=last).map(|index| index as f64 * step).collect();
        Self {
            min: first as f64 * step,
            max: last as f64 * step,
            ticks,
        }
    }

    /// Where `value` falls from the low end, in percent.
    fn percent(&self, value: f64) -> f64 {
        (value - self.min) / (self.max - self.min) * 100.0
    }
}

/// A grid line from `(x1, y1)` to `(x2, y2)` in the plot's 0 to 100 view
/// box.
type GridLine = (f64, f64, f64, f64);

/// An axis or mark label, positioned with an inline style.
struct Placed {
    style: String,
    text: String,
}

/// The kind of an HTML mark drawn over the plot.
#[derive(Clone, Copy, PartialEq, Eq)]
enum MarkKind {
    Bar,
    Dot,
}

/// A mark shown while its hover column is hovered: an active dot, or the
/// crosshair line.
struct HoverMark {
    style: String,
    dot: bool,
}

/// One category's hover column: its place, its tooltip and the marks shown
/// while it is hovered.
struct Column {
    style: String,
    side: TooltipSide,
    is_default: bool,
    heading: Option<String>,
    items: Vec<TooltipItem>,
    /// Active dots and the crosshair, positioned within the column.
    marks: Vec<HoverMark>,
}

/// The value of series `index` in `values`, or zero.
fn value_at(values: &[f64], index: usize) -> f64 {
    values.get(index).copied().unwrap_or(0.0)
}

/// The tooltip values of one category.
fn tooltip_items(
    config: ChartConfig<'_>,
    values: &[f64],
    row_color: Option<&str>,
    tooltip: &ChartTooltip<'_>,
) -> Vec<TooltipItem> {
    config
        .iter()
        .enumerate()
        .map(|(index, series)| TooltipItem {
            label: tooltip.name.unwrap_or(series.label).to_owned(),
            value: value_at(values, index),
            color: row_color.map_or_else(|| series.fill(), str::to_owned),
            icon: series.icon.clone(),
        })
        .collect()
}

/// The space around the plot, for the axes and labels, as `(top, right,
/// bottom, left)` pixels.
type Margins = (f64, f64, f64, f64);

/// The width of a column of axis labels, estimated from the longest one.
fn label_width<'a>(labels: impl Iterator<Item = &'a str>) -> f64 {
    let longest = labels.map(|label| label.chars().count()).max().unwrap_or(0);
    longest as f64 * 7.0 + 10.0
}

/// Which categories get a tick label: every one, or every `step`-th to keep
/// at most `max` labels.
fn shows_tick(index: usize, count: usize, max: Option<usize>) -> bool {
    match max {
        Some(max) if max > 0 && count > max => index.is_multiple_of(count.div_ceil(max)),
        _ => true,
    }
}

/// The plot of a cartesian chart, shared by the bar, line and area charts:
/// the grid and SVG marks, then the HTML marks, labels, ticks and hover
/// columns over them, then the legend.
#[component]
async fn cartesian_plot(
    config: ChartConfig<'_>,
    margins: Margins,
    grid: Vec<GridLine>,
    /// SVG paths: `(d, fill, stroke)`.
    paths: Vec<(String, String, String)>,
    /// Bars and dots, each with its inline style.
    marks: Vec<(MarkKind, String)>,
    labels: Vec<Placed>,
    ticks: Vec<Placed>,
    columns: Vec<Column>,
    /// The `class` of every bar.
    bar_class: &str,
    /// Whether a hovered column is shaded (the bar chart's cursor).
    shade_column: bool,
    /// The icon drawn at each dot, in place of a circle.
    dot_icon: Option<IconData>,
    tooltip: Option<ChartTooltip<'_>>,
    legend: bool,
) -> Result<impl View> {
    let (top, right, bottom, left) = margins;
    let tooltip = tooltip.unwrap_or_default();
    Ok(view! {
        <div class="flex size-full flex-col">
            <div
                data-slot="chart-plot"
                class="relative min-h-0 flex-1"
                style=(format!("margin: {top}px {right}px {bottom}px {left}px"))
            >
                <svg
                    class="absolute inset-0 size-full overflow-visible"
                    viewBox="0 0 100 100"
                    preserveAspectRatio="none"
                    aria-hidden="true"
                >
                    for (x1, y1, x2, y2) in grid {
                        <line
                            x1=(x1) y1=(y1) x2=(x2) y2=(y2)
                            stroke=(GRID_STROKE)
                            stroke-width="1"
                            vector-effect="non-scaling-stroke"
                        ></line>
                    }
                    for (d, fill, stroke) in paths {
                        <path
                            d=(d)
                            fill=(fill)
                            stroke=(stroke)
                            stroke-width="2"
                            stroke-linejoin="round"
                            vector-effect="non-scaling-stroke"
                        ></path>
                    }
                </svg>
                #[key(index)]
                for (index, (kind, style)) in marks.into_iter().enumerate() {
                    match (kind, dot_icon.clone()) {
                        (MarkKind::Bar, _) => <div data-slot="chart-bar" class=(bar_class) style=(style)></div>,
                        (MarkKind::Dot, Some(data)) => <span
                            data-slot="chart-dot"
                            class="pointer-events-none absolute flex size-6 -translate-1/2 items-center justify-center [&>svg]:size-6 [&_path]:stroke-(--color-border)"
                            style=(style)
                        >
                            icon(data: data)
                        </span>,
                        (MarkKind::Dot, None) => <span
                            data-slot="chart-dot"
                            class="pointer-events-none absolute -translate-1/2 rounded-full"
                            style=(style)
                        ></span>,
                    }
                }
                for label in labels {
                    <span
                        class="pointer-events-none absolute whitespace-nowrap text-xs text-foreground"
                        style=(label.style)
                    >
                        (label.text)
                    </span>
                }
                for tick in ticks {
                    <span
                        class="pointer-events-none absolute whitespace-nowrap text-foreground"
                        style=(tick.style)
                    >
                        (tick.text)
                    </span>
                }
                #[key(index)]
                for (index, column) in columns.into_iter().enumerate() {
                    <div
                        data-slot="chart-column"
                        data-default=(column.is_default)
                        class=(class!(
                            "group/column absolute",
                            "hover:bg-[#8080804D]" if shade_column,
                        ))
                        style=(column.style)
                    >
                        for mark in column.marks {
                            <span
                                class=(class!(
                                    "pointer-events-none absolute invisible group-hover/column:visible",
                                    "-translate-1/2 rounded-full" if mark.dot,
                                ))
                                style=(mark.style)
                            ></span>
                        }
                        if !column.items.is_empty() {
                            chart_tooltip_content(
                                tooltip: tooltip,
                                heading: column.heading,
                                items: column.items,
                                attrs: attributes! {
                                    class=(class!(
                                        TOOLTIP_ON_HOVER if !column.is_default,
                                        TOOLTIP_DEFAULT if column.is_default,
                                        column.side.classes(),
                                    ))
                                },
                            )
                        }
                    </div>
                }
            </div>
            if legend {
                chart_legend_content(config: config, sorted: true)
            }
        </div>
    })
}

/// The hover column at `index`, with its tooltip on `side`.
fn column(
    style: String,
    side: TooltipSide,
    index: usize,
    heading: Option<String>,
    items: Vec<TooltipItem>,
    tooltip: Option<&ChartTooltip<'_>>,
) -> Column {
    Column {
        style,
        side,
        is_default: tooltip.and_then(|tooltip| tooltip.default_index) == Some(index),
        heading,
        items: if tooltip.is_some() { items } else { Vec::new() },
        marks: Vec::new(),
    }
}

/// A bar chart: one bar per series in each category, side by side or
/// `stacked`, and vertical unless `horizontal`.
///
/// - `radius` rounds the bars' corners in pixels; stacked bars round only
///   the ends of the stack.
/// - `colors` gives each category its own fill, in place of the series
///   color (Octane's per-row `fill`).
/// - `active_index` draws that category's bar with a thick border and a
///   lighter fill.
/// - `end_label` draws text past the end of each bar, and `inside_label`
///   inside its start.
/// - `category_axis` (default on) labels the categories, through
///   `category_format`, at most `max_category_ticks` of them; `value_axis`
///   (default off) labels `value_ticks` values; `grid` (default on) draws a
///   line at each value tick.
/// - `tooltip` (default on) and `legend` (default off) add those parts.
///
/// `attrs` are forwarded to the chart's root, with extra classes appended;
/// bars carry `data-slot="chart-bar"` for styling them from there.
#[component]
pub async fn bar_chart(
    config: ChartConfig<'_>,
    data: &[ChartRow<'_>],
    #[default] horizontal: bool,
    #[default] stacked: bool,
    #[default] radius: f64,
    #[default] colors: &[&str],
    #[default] active_index: Option<usize>,
    #[default] end_label: ChartLabel,
    #[default] inside_label: ChartLabel,
    #[default(true)] category_axis: bool,
    #[default] value_axis: bool,
    #[default(5)] value_ticks: usize,
    #[default(true)] grid: bool,
    #[default(same)] category_format: Format,
    #[default] max_category_ticks: Option<usize>,
    #[default(Some(ChartTooltip::default()))] tooltip: Option<ChartTooltip<'_>>,
    #[default] legend: bool,
    #[default] attrs: Attributes,
) -> Result<impl View> {
    let count = data.len().max(1);
    // The value range: each stack's totals when stacked, else every value.
    let (mut low, mut high) = (0.0f64, 0.0f64);
    for (_, values) in data {
        if stacked {
            let positive: f64 = values.iter().filter(|value| **value > 0.0).sum();
            let negative: f64 = values.iter().filter(|value| **value < 0.0).sum();
            high = high.max(positive);
            low = low.min(negative);
        } else {
            for value in values.iter().take(config.len()) {
                high = high.max(*value);
                low = low.min(*value);
            }
        }
    }
    let scale = ValueScale::new(low, high, value_ticks);
    let zero = scale.percent(0.0);
    let band = 100.0 / count as f64;
    // Bars in a category share 80% of its band, 4px apart.
    let slots = if stacked { 1 } else { config.len().max(1) };
    let gaps = (slots - 1) * 4;

    let mut marks = Vec::new();
    let mut labels = Vec::new();
    let mut columns = Vec::new();
    for (index, (category, values)) in data.iter().enumerate() {
        let start = index as f64 * band;
        let row_color = colors.get(index).copied();
        let (mut above, mut below) = (0.0, 0.0);
        for (series_index, series) in config.iter().enumerate() {
            let value = value_at(values, series_index);
            let (from, to) = if stacked {
                let base = if value >= 0.0 { &mut above } else { &mut below };
                let from = *base;
                *base += value;
                (from, *base)
            } else {
                (0.0, value)
            };
            let slot = if stacked { 0 } else { series_index };
            let cross_start = format!(
                "calc({:.4}% + {slot} * (({:.4}% - {gaps}px) / {slots} + 4px))",
                start + band * 0.1,
                band * 0.8,
            );
            let cross_size = format!("calc(({:.4}% - {gaps}px) / {slots})", band * 0.8);
            let (a, b) = (scale.percent(from.min(to)), scale.percent(from.max(to)));
            // A stacked segment reaches 2px back over the one below it, so
            // the two share one border, as Recharts' centered strokes do.
            let overlap = if stacked && from != 0.0 { 2 } else { 0 };
            let (back, back_size) = match (from < to, overlap) {
                (_, 0) => (format!("{a:.4}%"), format!("{:.4}%", b - a)),
                (true, _) => (
                    format!("calc({a:.4}% - 2px)"),
                    format!("calc({:.4}% + 2px)", b - a),
                ),
                (false, _) => (format!("{a:.4}%"), format!("calc({:.4}% + 2px)", b - a)),
            };
            // Rounded corners: all of them, or only the ends of a stack.
            let first = series_index == 0;
            let last = series_index + 1 == config.len();
            let (start_corners, end_corners) = if stacked { (first, last) } else { (true, true) };
            let (r0, r1) = (
                if start_corners { radius } else { 0.0 },
                if end_corners { radius } else { 0.0 },
            );
            let fill = row_color.map_or_else(|| series.fill(), str::to_owned);
            let active = active_index == Some(index);
            let background = if active {
                format!("color-mix(in srgb, {fill} 80%, transparent)")
            } else {
                fill
            };
            let border = if active { "border-width: 4px; " } else { "" };
            let style = if horizontal {
                format!(
                    "top: {cross_start}; height: {cross_size}; left: {back}; width: {back_size}; \
                     border-radius: {r0}px {r1}px {r1}px {r0}px; {border}background: {background}",
                )
            } else {
                format!(
                    "left: {cross_start}; width: {cross_size}; bottom: {back}; height: {back_size}; \
                     border-radius: {r1}px {r1}px {r0}px {r0}px; {border}background: {background}",
                )
            };
            if from != to {
                marks.push((MarkKind::Bar, style));
            }

            // Labels: past the bar's end (the top of a stack), and inside its
            // start.
            let center = format!("calc({cross_start} + {cross_size} / 2)");
            if (!stacked || last)
                && let Some(text) = end_label.text(category, value, category_format)
            {
                let end = scale.percent(to);
                let style = match (horizontal, to >= 0.0) {
                    (false, true) => format!(
                        "left: {center}; bottom: calc({end:.4}% + 6px); transform: translateX(-50%)"
                    ),
                    (false, false) => format!(
                        "left: {center}; top: calc({:.4}% + 6px); transform: translateX(-50%)",
                        100.0 - end,
                    ),
                    (true, true) => format!(
                        "top: {center}; left: calc({end:.4}% + 8px); transform: translateY(-50%)"
                    ),
                    (true, false) => format!(
                        "top: {center}; right: calc({:.4}% + 8px); transform: translateY(-50%)",
                        100.0 - end,
                    ),
                };
                labels.push(Placed { style, text });
            }
            if let Some(text) = inside_label.text(category, value, category_format) {
                let style = if horizontal {
                    format!(
                        "top: {center}; left: calc({zero:.4}% + 8px); transform: translateY(-50%)"
                    )
                } else {
                    format!(
                        "left: {center}; bottom: calc({zero:.4}% + 8px); transform: translateX(-50%)"
                    )
                };
                labels.push(Placed {
                    style: format!("{style}; color: var(--color-main-foreground)"),
                    text,
                });
            }
        }

        let style = if horizontal {
            format!("left: 0; right: 0; top: {start:.4}%; height: {band:.4}%")
        } else {
            format!("top: 0; bottom: 0; left: {start:.4}%; width: {band:.4}%")
        };
        let heading = tooltip
            .as_ref()
            .and_then(|tooltip| tooltip.heading(category));
        let items = match &tooltip {
            Some(tooltip) => tooltip_items(config, values, row_color, tooltip),
            None => Vec::new(),
        };
        let side = if horizontal {
            TooltipSide::Below
        } else {
            TooltipSide::beside(start + band / 2.0)
        };
        columns.push(column(style, side, index, heading, items, tooltip.as_ref()));
    }

    // Axes: categories along the band axis, values along the other.
    let mut ticks = Vec::new();
    let category_labels: Vec<String> = data
        .iter()
        .map(|(category, _)| category_format(category))
        .collect();
    if category_axis {
        for (index, text) in category_labels.iter().enumerate() {
            if !shows_tick(index, data.len(), max_category_ticks) {
                continue;
            }
            let middle = index as f64 * band + band / 2.0;
            let style = if horizontal {
                format!("right: calc(100% + 10px); top: {middle:.4}%; transform: translateY(-50%)")
            } else {
                format!("top: calc(100% + 10px); left: {middle:.4}%; transform: translateX(-50%)")
            };
            ticks.push(Placed {
                style,
                text: text.clone(),
            });
        }
    }
    let value_labels: Vec<String> = scale
        .ticks
        .iter()
        .map(|tick| format_number(*tick))
        .collect();
    if value_axis {
        for (tick, text) in scale.ticks.iter().zip(&value_labels) {
            let at = scale.percent(*tick);
            let style = if horizontal {
                format!("top: calc(100% + 10px); left: {at:.4}%; transform: translateX(-50%)")
            } else {
                format!("right: calc(100% + 8px); bottom: {at:.4}%; transform: translateY(50%)")
            };
            ticks.push(Placed {
                style,
                text: text.clone(),
            });
        }
    }
    let grid_lines = if grid {
        scale
            .ticks
            .iter()
            .map(|tick| {
                let at = scale.percent(*tick);
                if horizontal {
                    (at, 0.0, at, 100.0)
                } else {
                    (0.0, 100.0 - at, 100.0, 100.0 - at)
                }
            })
            .collect()
    } else {
        Vec::new()
    };

    // Room for the axes and the labels past the bars.
    let ends = end_label != ChartLabel::None;
    let side_axis = if horizontal {
        category_axis
    } else {
        value_axis
    };
    let side_width = match (horizontal, side_axis) {
        (true, true) => label_width(category_labels.iter().map(String::as_str)),
        (false, true) => label_width(value_labels.iter().map(String::as_str)),
        _ => 0.0,
    };
    let bottom_axis = if horizontal {
        value_axis
    } else {
        category_axis
    };
    let margins = (
        5.0 + if ends && !horizontal { 20.0 } else { 0.0 },
        5.0 + if ends && horizontal { 40.0 } else { 0.0 },
        5.0 + if bottom_axis { 25.0 } else { 0.0 } + if ends && low < 0.0 { 20.0 } else { 0.0 },
        5.0 + side_width,
    );

    Ok(view! {
        chart_container(config: config, attrs: attrs,
            cartesian_plot(
                config: config,
                margins: margins,
                grid: grid_lines,
                paths: Vec::new(),
                marks: marks,
                labels: labels,
                ticks: ticks,
                columns: columns,
                bar_class: "absolute border-2 border-border",
                shade_column: tooltip.is_some_and(|tooltip| tooltip.cursor),
                dot_icon: None,
                tooltip: tooltip,
                legend: legend,
            )
        )
    })
}

/// The x position, in percent, of point `index` of `count` evenly spaced
/// points, from edge to edge.
fn point_x(index: usize, count: usize) -> f64 {
    if count <= 1 {
        50.0
    } else {
        index as f64 * 100.0 / (count - 1) as f64
    }
}

/// The hover column around point `index` of `count`: from halfway to the
/// previous point to halfway to the next, as `(left, width)` percentages.
fn point_column(index: usize, count: usize) -> (f64, f64) {
    if count <= 1 {
        return (0.0, 100.0);
    }
    let step = 100.0 / (count - 1) as f64;
    let x = point_x(index, count);
    let left = (x - step / 2.0).max(0.0);
    let right = (x + step / 2.0).min(100.0);
    (left, right - left)
}

/// Appends SVG path commands that draw `curve` through `points`, starting
/// from the first point (the pen is already there).
fn curve_to(path: &mut String, points: &[(f64, f64)], curve: ChartCurve) {
    use std::fmt::Write;

    let count = points.len();
    if count < 2 {
        return;
    }
    match curve {
        ChartCurve::Natural | ChartCurve::Monotone if count > 2 => {
            for (index, ((c1x, c1y), (c2x, c2y))) in
                control_points(points, curve).into_iter().enumerate()
            {
                let (x, y) = points[index + 1];
                let _ = write!(path, " C{c1x:.3},{c1y:.3} {c2x:.3},{c2y:.3} {x:.3},{y:.3}");
            }
        }
        ChartCurve::Step => {
            for pair in points.windows(2) {
                let ((x0, y0), (x1, y1)) = (pair[0], pair[1]);
                let middle = (x0 + x1) / 2.0;
                let _ = write!(
                    path,
                    " L{middle:.3},{y0:.3} L{middle:.3},{y1:.3} L{x1:.3},{y1:.3}"
                );
            }
        }
        _ => {
            for (x, y) in &points[1..] {
                let _ = write!(path, " L{x:.3},{y:.3}");
            }
        }
    }
}

/// The two Bézier control points of each segment of a natural or monotone
/// curve through `points` (at least three).
fn control_points(points: &[(f64, f64)], curve: ChartCurve) -> Vec<((f64, f64), (f64, f64))> {
    if curve == ChartCurve::Natural {
        // d3's `curveNatural`: solve each coordinate's spline separately.
        let xs: Vec<f64> = points.iter().map(|point| point.0).collect();
        let ys: Vec<f64> = points.iter().map(|point| point.1).collect();
        let (ax, bx) = natural_controls(&xs);
        let (ay, by) = natural_controls(&ys);
        return (0..points.len() - 1)
            .map(|i| ((ax[i], ay[i]), (bx[i], by[i])))
            .collect();
    }

    // d3's `curveMonotoneX`: a tangent at each point that keeps every
    // segment monotone (Steffen's method), then cubic Hermite segments.
    let slope = |a: (f64, f64), b: (f64, f64)| (b.1 - a.1) / (b.0 - a.0);
    let count = points.len();
    let mut tangents = vec![0.0; count];
    for i in 1..count - 1 {
        let (h0, h1) = (points[i].0 - points[i - 1].0, points[i + 1].0 - points[i].0);
        let (s0, s1) = (
            slope(points[i - 1], points[i]),
            slope(points[i], points[i + 1]),
        );
        let p = (s0 * h1 + s1 * h0) / (h0 + h1);
        let sign = |value: f64| if value < 0.0 { -1.0 } else { 1.0 };
        let tangent = (sign(s0) + sign(s1)) * s0.abs().min(s1.abs()).min(0.5 * p.abs());
        tangents[i] = if tangent.is_finite() { tangent } else { 0.0 };
    }
    tangents[0] = (3.0 * slope(points[0], points[1]) - tangents[1]) / 2.0;
    tangents[count - 1] =
        (3.0 * slope(points[count - 2], points[count - 1]) - tangents[count - 2]) / 2.0;
    points
        .windows(2)
        .enumerate()
        .map(|(i, pair)| {
            let ((x0, y0), (x1, y1)) = (pair[0], pair[1]);
            let dx = (x1 - x0) / 3.0;
            (
                (x0 + dx, y0 + dx * tangents[i]),
                (x1 - dx, y1 - dx * tangents[i + 1]),
            )
        })
        .collect()
}

/// The first and second control coordinates of each segment of a natural
/// cubic spline through `values` (d3's `controlPoints`, a tridiagonal solve).
fn natural_controls(values: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let n = values.len() - 1;
    let mut a = vec![0.0; n];
    let mut b = vec![0.0; n];
    let mut r = vec![0.0; n];
    b[0] = 2.0;
    r[0] = values[0] + 2.0 * values[1];
    for i in 1..n - 1 {
        a[i] = 1.0;
        b[i] = 4.0;
        r[i] = 4.0 * values[i] + 2.0 * values[i + 1];
    }
    a[n - 1] = 2.0;
    b[n - 1] = 7.0;
    r[n - 1] = 8.0 * values[n - 1] + values[n];
    for i in 1..n {
        let m = a[i] / b[i - 1];
        b[i] -= m;
        r[i] -= m * r[i - 1];
    }
    a[n - 1] = r[n - 1] / b[n - 1];
    for i in (0..n - 1).rev() {
        a[i] = (r[i] - a[i + 1]) / b[i];
    }
    b[n - 1] = (values[n] + a[n - 1]) / 2.0;
    for i in 0..n - 1 {
        b[i] = 2.0 * values[i + 1] - a[i + 1];
    }
    (a, b)
}

/// The value ticks, axis labels and grid of a line or area chart.
fn point_axes(
    data: &[ChartRow<'_>],
    scale: &ValueScale,
    category_axis: bool,
    value_axis: bool,
    grid: bool,
    category_format: Format,
    max_category_ticks: Option<usize>,
) -> (Vec<Placed>, Vec<GridLine>, f64) {
    let mut ticks = Vec::new();
    if category_axis {
        for (index, (category, _)) in data.iter().enumerate() {
            if shows_tick(index, data.len(), max_category_ticks) {
                ticks.push(Placed {
                    style: format!(
                        "top: calc(100% + 8px); left: {:.4}%; transform: translateX(-50%)",
                        point_x(index, data.len()),
                    ),
                    text: category_format(category),
                });
            }
        }
    }
    let value_labels: Vec<String> = scale
        .ticks
        .iter()
        .map(|tick| format_number(*tick))
        .collect();
    if value_axis {
        for (tick, text) in scale.ticks.iter().zip(&value_labels) {
            ticks.push(Placed {
                style: format!(
                    "right: calc(100% + 8px); bottom: {:.4}%; transform: translateY(50%)",
                    scale.percent(*tick),
                ),
                text: text.clone(),
            });
        }
    }
    let grid = if grid {
        scale
            .ticks
            .iter()
            .map(|tick| {
                let y = 100.0 - scale.percent(*tick);
                (0.0, y, 100.0, y)
            })
            .collect()
    } else {
        Vec::new()
    };
    let width = if value_axis {
        label_width(value_labels.iter().map(String::as_str))
    } else {
        0.0
    };
    (ticks, grid, width)
}

/// A line chart: a line per series through its value in each category.
///
/// - `curve` shapes the lines (default `Linear`). The lines are drawn in the
///   border color, as Octane's neobrutalism override does, unless
///   `colored_lines` uses each series' color.
/// - `dots` marks each point with a dot `dot_radius` pixels wide in the
///   series color (or the category's, from `colors`); `dot_icon` draws an
///   icon there instead. `active_dot_radius` and `active_dot_color` style the
///   dots shown on hover (default: 4px, the series color).
/// - `point_label` draws text above each point.
/// - The axes, grid, tooltip and legend take the same props as
///   [`bar_chart`]'s.
///
/// `attrs` are forwarded to the chart's root, with extra classes appended.
#[component]
pub async fn line_chart(
    config: ChartConfig<'_>,
    data: &[ChartRow<'_>],
    #[default] curve: ChartCurve,
    #[default] colored_lines: bool,
    #[default] dots: bool,
    #[default(3.0)] dot_radius: f64,
    #[default] dot_icon: Option<IconData>,
    #[default] colors: &[&str],
    #[default(4.0)] active_dot_radius: f64,
    #[default] active_dot_color: Option<&str>,
    #[default] point_label: ChartLabel,
    #[default(true)] category_axis: bool,
    #[default] value_axis: bool,
    #[default(5)] value_ticks: usize,
    #[default(true)] grid: bool,
    #[default(same)] category_format: Format,
    #[default] max_category_ticks: Option<usize>,
    #[default(Some(ChartTooltip::default()))] tooltip: Option<ChartTooltip<'_>>,
    #[default] legend: bool,
    #[default] attrs: Attributes,
) -> Result<impl View> {
    let count = data.len();
    let values = data
        .iter()
        .flat_map(|(_, values)| values.iter().take(config.len()));
    let (low, high) = values.fold((0.0f64, 0.0f64), |(low, high), value| {
        (low.min(*value), high.max(*value))
    });
    let scale = ValueScale::new(low, high, value_ticks);
    let y = |value: f64| 100.0 - scale.percent(value);

    let mut paths = Vec::new();
    let mut marks = Vec::new();
    let mut labels = Vec::new();
    for (series_index, series) in config.iter().enumerate() {
        let points: Vec<(f64, f64)> = data
            .iter()
            .enumerate()
            .map(|(index, (_, values))| (point_x(index, count), y(value_at(values, series_index))))
            .collect();
        if let Some(&(x0, y0)) = points.first() {
            let mut d = format!("M{x0:.3},{y0:.3}");
            curve_to(&mut d, &points, curve);
            let stroke = if colored_lines {
                series.fill()
            } else {
                "var(--color-border)".to_owned()
            };
            paths.push((d, "none".to_owned(), stroke));
        }
        for (index, ((x, y), (category, values))) in points.iter().zip(data).enumerate() {
            let color = colors
                .get(index)
                .map_or_else(|| series.fill(), |color| (*color).to_owned());
            if dot_icon.is_some() {
                marks.push((
                    MarkKind::Dot,
                    format!("left: {x:.4}%; top: {y:.4}%; color: {color}"),
                ));
            } else if dots {
                let size = dot_radius * 2.0 + 2.0;
                marks.push((
                    MarkKind::Dot,
                    format!("left: {x:.4}%; top: {y:.4}%; width: {size}px; height: {size}px; background: {color}"),
                ));
            }
            if let Some(text) =
                point_label.text(category, value_at(values, series_index), category_format)
            {
                labels.push(Placed {
                    style: format!("left: {x:.4}%; top: calc({y:.4}% - 10px); transform: translate(-50%, -100%)"),
                    text,
                });
            }
        }
    }

    let levels: Vec<Vec<f64>> = data
        .iter()
        .map(|(_, values)| {
            (0..config.len())
                .map(|index| value_at(values, index))
                .collect()
        })
        .collect();
    let columns = point_columns(
        config,
        data,
        &levels,
        colors,
        &scale,
        tooltip.as_ref(),
        (active_dot_radius, active_dot_color),
    );
    let (ticks, grid_lines, axis_width) = point_axes(
        data,
        &scale,
        category_axis,
        value_axis,
        grid,
        category_format,
        max_category_ticks,
    );
    let labelled = point_label != ChartLabel::None;
    let margins = (
        8.0 + if labelled { 20.0 } else { 0.0 },
        12.0,
        5.0 + if category_axis { 25.0 } else { 0.0 },
        12.0 + axis_width,
    );

    Ok(view! {
        chart_container(config: config, attrs: attrs,
            cartesian_plot(
                config: config,
                margins: margins,
                grid: grid_lines,
                paths: paths,
                marks: marks,
                labels: labels,
                ticks: ticks,
                columns: columns,
                bar_class: "",
                shade_column: false,
                dot_icon: dot_icon,
                tooltip: tooltip,
                legend: legend,
            )
        )
    })
}

/// The hover columns of a line or area chart. `levels` holds, for each
/// category, the height of each series in chart values, where its active dot
/// sits (the top of its band, for stacked areas).
fn point_columns(
    config: ChartConfig<'_>,
    data: &[ChartRow<'_>],
    levels: &[Vec<f64>],
    colors: &[&str],
    scale: &ValueScale,
    tooltip: Option<&ChartTooltip<'_>>,
    (radius, active_color): (f64, Option<&str>),
) -> Vec<Column> {
    let count = data.len();
    data.iter()
        .enumerate()
        .map(|(index, (category, values))| {
            let (left, width) = point_column(index, count);
            let x = point_x(index, count);
            // The point's position within its column.
            let inside = if width > 0.0 { (x - left) / width * 100.0 } else { 50.0 };
            let row_color = colors.get(index).copied();
            let heading = tooltip.and_then(|tooltip| tooltip.heading(category));
            let items = tooltip.map_or_else(Vec::new, |tooltip| tooltip_items(config, values, row_color, tooltip));
            let mut column = column(
                format!("top: 0; bottom: 0; left: {left:.4}%; width: {width:.4}%"),
                TooltipSide::beside(x),
                index,
                heading,
                items,
                tooltip,
            );
            if tooltip.is_some_and(|tooltip| tooltip.cursor) {
                column.marks.push(HoverMark {
                    style: format!("left: {inside:.4}%; top: 0; bottom: 0; width: 1px; background: {GRID_STROKE}"),
                    dot: false,
                });
            }
            for (series_index, series) in config.iter().enumerate() {
                let level = levels[index].get(series_index).copied().unwrap_or(0.0);
                let top = 100.0 - scale.percent(level);
                let color = active_color
                    .map(str::to_owned)
                    .or(row_color.map(str::to_owned))
                    .unwrap_or_else(|| series.fill());
                let size = radius * 2.0;
                column.marks.push(HoverMark {
                    style: format!(
                        "left: {inside:.4}%; top: {top:.4}%; width: {size}px; height: {size}px; background: {color}"
                    ),
                    dot: true,
                });
            }
            column
        })
        .collect()
}

/// An area chart: each series fills the space under its line, down to the
/// axis or, when `stacked`, to the series below it. `expand` stacks the
/// series as shares of each category's total (Recharts' `stackOffset="expand"`).
///
/// The first series of `config` is the bottom of a stack. Areas are filled
/// opaque with a border-colored outline, as Octane's neobrutalism override
/// does. `curve`, the axes, grid, tooltip and legend work as in
/// [`line_chart`]; active dots default to `var(--chart-active-dot)`.
///
/// `attrs` are forwarded to the chart's root, with extra classes appended.
#[component]
pub async fn area_chart(
    config: ChartConfig<'_>,
    data: &[ChartRow<'_>],
    #[default] curve: ChartCurve,
    #[default] stacked: bool,
    #[default] expand: bool,
    #[default(4.0)] active_dot_radius: f64,
    #[default(Some("var(--chart-active-dot)"))] active_dot_color: Option<&str>,
    #[default(true)] category_axis: bool,
    #[default] value_axis: bool,
    #[default(5)] value_ticks: usize,
    #[default(true)] grid: bool,
    #[default(same)] category_format: Format,
    #[default] max_category_ticks: Option<usize>,
    #[default(Some(ChartTooltip::default()))] tooltip: Option<ChartTooltip<'_>>,
    #[default] legend: bool,
    #[default] attrs: Attributes,
) -> Result<impl View> {
    let count = data.len();
    let stacked = stacked || expand;
    // The bottom and top of each series' band in each category, as values.
    let series_count = config.len();
    let bands: Vec<Vec<(f64, f64)>> = data
        .iter()
        .map(|(_, values)| {
            let total: f64 = (0..series_count).map(|index| value_at(values, index)).sum();
            let mut base = 0.0;
            (0..series_count)
                .map(|index| {
                    let mut value = value_at(values, index);
                    if expand {
                        value = if total == 0.0 { 0.0 } else { value / total };
                    }
                    if stacked {
                        let band = (base, base + value);
                        base += value;
                        band
                    } else {
                        (0.0, value)
                    }
                })
                .collect()
        })
        .collect();
    let (low, high) = bands
        .iter()
        .flatten()
        .fold((0.0f64, 0.0f64), |(low, high), (bottom, top)| {
            (low.min(*bottom).min(*top), high.max(*top))
        });
    let scale = ValueScale::new(low, high, value_ticks);
    let y = |value: f64| 100.0 - scale.percent(value);

    let paths: Vec<(String, String, String)> = config
        .iter()
        .enumerate()
        .filter(|_| count > 0)
        .map(|(series_index, series)| {
            let top: Vec<(f64, f64)> = bands
                .iter()
                .enumerate()
                .map(|(index, row)| (point_x(index, count), y(row[series_index].1)))
                .collect();
            let bottom: Vec<(f64, f64)> = bands
                .iter()
                .enumerate()
                .rev()
                .map(|(index, row)| (point_x(index, count), y(row[series_index].0)))
                .collect();
            // Along the top, down the right edge, back along the bottom.
            let mut d = format!("M{:.3},{:.3}", top[0].0, top[0].1);
            curve_to(&mut d, &top, curve);
            d.push_str(&format!(" L{:.3},{:.3}", bottom[0].0, bottom[0].1));
            curve_to(&mut d, &bottom, curve);
            d.push_str(" Z");
            (d, series.fill(), "var(--color-border)".to_owned())
        })
        .collect();

    let levels: Vec<Vec<f64>> = bands
        .iter()
        .map(|row| row.iter().map(|(_, top)| *top).collect())
        .collect();
    let columns = point_columns(
        config,
        data,
        &levels,
        &[],
        &scale,
        tooltip.as_ref(),
        (active_dot_radius, active_dot_color),
    );
    let (ticks, grid_lines, axis_width) = point_axes(
        data,
        &scale,
        category_axis,
        value_axis,
        grid,
        category_format,
        max_category_ticks,
    );
    let margins = (
        8.0,
        12.0,
        5.0 + if category_axis { 25.0 } else { 0.0 },
        12.0 + axis_width,
    );

    Ok(view! {
        chart_container(config: config, attrs: attrs,
            cartesian_plot(
                config: config,
                margins: margins,
                grid: grid_lines,
                paths: paths,
                marks: Vec::new(),
                labels: Vec::new(),
                ticks: ticks,
                columns: columns,
                bar_class: "",
                shade_column: false,
                dot_icon: None,
                tooltip: tooltip,
                legend: legend,
            )
        )
    })
}

/// One ring of a [`pie_chart`]: a value per slice, in config order, between
/// two radii. The pie is drawn in a 250-unit square, so the default outer
/// radius of 100 is Recharts' 80%.
///
/// ```ignore
/// rings: &[PieRing { inner_radius: 60.0, ..PieRing::new(&[275.0, 200.0, 187.0]) }]
/// ```
#[derive(Clone, Copy, Debug)]
pub struct PieRing<'a> {
    /// The ring's name, shown as the tooltip heading.
    pub label: &'a str,
    /// The value of each slice, in config order.
    pub values: &'a [f64],
    /// Where the slices start, from the center; 0 for a full pie.
    pub inner_radius: f64,
    /// Where the slices end.
    pub outer_radius: f64,
}

impl<'a> PieRing<'a> {
    /// A full pie of `values` with the default radius.
    #[must_use]
    pub const fn new(values: &'a [f64]) -> Self {
        Self {
            label: "",
            values,
            inner_radius: 0.0,
            outer_radius: 100.0,
        }
    }
}

/// The labels of a [`pie_chart`]'s slices.
///
/// [`Default`] is `PieLabel::None`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PieLabel {
    /// No labels.
    #[default]
    None,
    /// The value outside each slice, joined to it by a line.
    Value,
    /// The value outside each slice, without the line.
    ValueOnly,
    /// The slice's name, inside it.
    Name,
}

/// The pie's size in SVG units, and its center.
const PIE_SIZE: f64 = 250.0;
const PIE_CENTER: f64 = PIE_SIZE / 2.0;

/// The point at `radius` and `angle` degrees from the center. Angles run
/// counterclockwise from three o'clock, as in Recharts.
fn polar(radius: f64, angle: f64) -> (f64, f64) {
    let radians = angle.to_radians();
    (
        PIE_CENTER + radius * radians.cos(),
        PIE_CENTER - radius * radians.sin(),
    )
}

/// The SVG path of a slice from `start` to `end` degrees between two radii.
fn sector(inner: f64, outer: f64, start: f64, end: f64) -> String {
    // A full circle is drawn as two half arcs; one arc can't end where it
    // starts.
    let end = if end - start >= 360.0 {
        start + 359.999
    } else {
        end
    };
    let large = if end - start > 180.0 { 1 } else { 0 };
    let (x0, y0) = polar(outer, start);
    let (x1, y1) = polar(outer, end);
    // Sweep flag 0: counterclockwise on screen.
    let mut d = format!("M{x0:.3},{y0:.3} A{outer},{outer} 0 {large} 0 {x1:.3},{y1:.3}");
    if inner > 0.0 {
        let (x2, y2) = polar(inner, end);
        let (x3, y3) = polar(inner, start);
        d.push_str(&format!(
            " L{x2:.3},{y2:.3} A{inner},{inner} 0 {large} 1 {x3:.3},{y3:.3} Z"
        ));
    } else {
        d.push_str(&format!(" L{PIE_CENTER},{PIE_CENTER} Z"));
    }
    d
}

/// One drawn slice of a pie.
struct Slice {
    path: String,
    fill: String,
    /// The tooltip's position, as CSS, beside the slice's outer edge.
    tooltip_style: String,
    heading: Option<String>,
    item: TooltipItem,
}

/// A pie or donut chart: a slice per series of `config`, sized by the
/// values of each ring in `rings`.
///
/// - Each [`PieRing`] sets its radii; `inner_radius` makes a donut, and
///   several rings nest (stack) pies.
/// - `active_index` pulls that slice 10 units further out.
/// - `label` draws values or names on the slices; `center_label` draws a
///   value and a caption in the middle of a donut.
/// - `tooltip` (default on, without a heading; a tooltip that shows its
///   heading uses the ring's `label`) and `legend` (default off, with extra
///   classes from `legend_class`) add those parts.
///
/// Give the chart a square box, `aspect-square!` in `attrs`. `attrs` are
/// forwarded to the chart's root, with extra classes appended.
#[component]
pub async fn pie_chart(
    config: ChartConfig<'_>,
    rings: &[PieRing<'_>],
    #[default] active_index: Option<usize>,
    #[default] label: PieLabel,
    #[default] center_label: Option<(&str, &str)>,
    #[default(Some(ChartTooltip { hide_label: true, ..ChartTooltip::default() }))] tooltip: Option<
        ChartTooltip<'_>,
    >,
    #[default] legend: bool,
    #[default] legend_class: Option<&str>,
    #[default] attrs: Attributes,
) -> Result<impl View> {
    let mut slices = Vec::new();
    let mut texts = Vec::new();
    let mut lines = Vec::new();
    for ring in rings {
        let total: f64 = ring.values.iter().take(config.len()).sum();
        let mut angle = 0.0;
        for (index, series) in config.iter().enumerate() {
            let value = value_at(ring.values, index);
            if total <= 0.0 || value <= 0.0 {
                continue;
            }
            let sweep = value / total * 360.0;
            let (start, end) = (angle, angle + sweep);
            angle = end;
            let middle = (start + end) / 2.0;
            let outer = ring.outer_radius
                + if active_index == Some(index) {
                    10.0
                } else {
                    0.0
                };

            // Labels: outside at `outer + 20`, anchored away from the center,
            // or inside halfway across the ring.
            let (x, y) = polar(outer + 20.0, middle);
            let anchor = if x > PIE_CENTER { "start" } else { "end" };
            match label {
                PieLabel::Value | PieLabel::ValueOnly => {
                    texts.push((x, y, anchor, format_number(value), "fill-foreground"));
                    if label == PieLabel::Value {
                        let (x0, y0) = polar(outer, middle);
                        // Octane's container strokes every chart path with
                        // the border color, label lines included.
                        lines.push((x0, y0, x, y, "var(--color-border)".to_owned()));
                    }
                }
                PieLabel::Name => {
                    let (x, y) = polar((ring.inner_radius + outer) / 2.0, middle);
                    texts.push((
                        x,
                        y,
                        "middle",
                        series.label.to_owned(),
                        "fill-main-foreground",
                    ));
                }
                PieLabel::None => {}
            }

            let (tx, ty) = polar(outer, middle);
            let (left, top) = (tx / PIE_SIZE * 100.0, ty / PIE_SIZE * 100.0);
            let flip = tx < PIE_CENTER;
            slices.push(Slice {
                path: sector(ring.inner_radius, outer, start, end),
                fill: series.fill(),
                tooltip_style: if flip {
                    format!(
                        "right: calc({:.4}% + 8px); top: {top:.4}%; transform: translateY(-50%)",
                        100.0 - left
                    )
                } else {
                    format!(
                        "left: calc({left:.4}% + 8px); top: {top:.4}%; transform: translateY(-50%)"
                    )
                },
                heading: tooltip
                    .filter(|tooltip| !tooltip.hide_label && !ring.label.is_empty())
                    .map(|_| ring.label.to_owned()),
                item: TooltipItem {
                    label: tooltip
                        .and_then(|tooltip| tooltip.name)
                        .unwrap_or(series.label)
                        .to_owned(),
                    value,
                    color: series.fill(),
                    icon: series.icon.clone(),
                },
            });
        }
    }
    let tooltip_settings = tooltip.unwrap_or_default();
    let view_box = format!("0 0 {PIE_SIZE} {PIE_SIZE}");

    Ok(view! {
        chart_container(config: config, attrs: attrs,
            <div class="flex size-full flex-col">
                <div class="relative flex min-h-0 flex-1 justify-center">
                    <div data-slot="chart-plot" class="relative aspect-square h-full max-w-full">
                        #[key(index)]
                        for (index, slice) in slices.into_iter().enumerate() {
                            <svg
                                class="pointer-events-none absolute inset-0 size-full overflow-visible"
                                viewBox=(view_box.clone())
                                aria-hidden="true"
                            >
                                <path
                                    data-slot="chart-sector"
                                    class="pointer-events-auto"
                                    d=(slice.path)
                                    fill=(slice.fill)
                                    stroke="var(--color-border)"
                                    stroke-width="2"
                                    stroke-linejoin="round"
                                ></path>
                            </svg>
                            if tooltip.is_some() {
                                chart_tooltip_content(
                                    tooltip: tooltip_settings,
                                    heading: slice.heading,
                                    items: vec![slice.item],
                                    attrs: attributes! {
                                        class="pointer-events-none absolute z-10 w-max invisible [:hover+&]:visible"
                                        style=(slice.tooltip_style)
                                    },
                                )
                            }
                        }
                        <svg
                            class="pointer-events-none absolute inset-0 size-full overflow-visible"
                            viewBox=(view_box.clone())
                            aria-hidden="true"
                        >
                            for (x1, y1, x2, y2, stroke) in lines {
                                <line x1=(x1) y1=(y1) x2=(x2) y2=(y2) stroke=(stroke) stroke-width="2"></line>
                            }
                            for (x, y, anchor, text, fill) in texts {
                                <text
                                    x=(x) y=(y)
                                    text-anchor=(anchor)
                                    dominant-baseline="central"
                                    font-size="12"
                                    class=(fill)
                                >
                                    (text)
                                </text>
                            }
                            match center_label {
                                Some((value, caption)) => <text
                                    x=(PIE_CENTER) y=(PIE_CENTER)
                                    text-anchor="middle"
                                    dominant-baseline="middle"
                                >
                                    <tspan
                                        x=(PIE_CENTER) y=(PIE_CENTER)
                                        class="fill-foreground text-3xl font-bold"
                                    >
                                        (value)
                                    </tspan>
                                    <tspan x=(PIE_CENTER) y=(PIE_CENTER + 24.0) class="fill-foreground">
                                        (caption)
                                    </tspan>
                                </text>,
                                None => "",
                            }
                        </svg>
                    </div>
                </div>
                if legend {
                    chart_legend_content(config: config, attrs: attributes! { class=(legend_class) })
                }
            </div>
        )
    })
}
