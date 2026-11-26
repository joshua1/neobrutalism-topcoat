//! The dashboard's charts, drawn on the server as SVG and HTML.
//!
//! They reuse the `chart` component's container (the `--color-<key>`
//! custom properties) and legend, and lay the plot out the way the upstream
//! Recharts charts do: a 52px value axis on the left, a 30px category axis
//! below, the same margins, "nice" value ticks and money-formatted labels,
//! which the `chart` component's charts don't offer.
//!
//! As in the `chart` component, the plot is an SVG whose `0 0 100 100` view
//! box stretches to fill it, with bars, dots and labels laid over it as HTML
//! at the same percentages. Tooltips need no script: each category has a
//! hover column holding its tooltip.
//!
//! Recharts hides category labels that would collide, measuring each one at
//! the chart's current width. Here the label widths are estimated, the same
//! rule ("preserve end") is run for a range of widths, and container queries
//! show each label at the widths where it fits.

use topcoat::{
    Result,
    context::Cx,
    view::{StaticClass, View, attributes, class, component, view},
};

use crate::components::chart::{ChartConfig, chart_container, chart_legend_content};

use super::data::{format_compact, format_number, sum};

/// Formats a value for a tooltip, a label or an axis tick.
pub type Format = fn(f64) -> String;

/// One category of a chart: its label and a value per series, in config
/// order.
pub struct Row {
    pub label: String,
    pub values: Vec<f64>,
}

impl Row {
    pub fn new(label: impl Into<String>, values: Vec<f64>) -> Self {
        Self {
            label: label.into(),
            values,
        }
    }

    fn value(&self, index: usize) -> f64 {
        self.values.get(index).copied().unwrap_or(0.0)
    }
}

/// The plot's margins and axes, in pixels, as in the upstream charts.
const MARGIN_TOP: f64 = 8.0;
const MARGIN_RIGHT: f64 = 8.0;
const VALUE_AXIS: f64 = 52.0;
const CATEGORY_AXIS: f64 = 30.0;
const LEGEND: f64 = 28.0;

/// The grid line color.
const GRID: &str = "#80808080";

/// The tooltip box, shown while its column is hovered. Hidden rather than
/// invisible, so tooltips past the page's edge don't widen it.
const TOOLTIP: StaticClass = class!(
    "pointer-events-none absolute z-20 hidden w-max min-w-[8rem] items-start gap-1.5 \
     rounded-base border-2 border-border bg-secondary-background px-2.5 py-1.5 text-xs \
     group-hover/column:grid",
);

/// Rounds away floating-point noise, keeping 12 significant digits.
fn clean(value: f64) -> f64 {
    if value == 0.0 || !value.is_finite() {
        return value;
    }
    let scale = 10f64.powi(11 - value.abs().log10().floor() as i32);
    (value * scale).round() / scale
}

/// A value axis: its domain and its ticks.
struct Scale {
    min: f64,
    max: f64,
    ticks: Vec<f64>,
}

impl Scale {
    /// Recharts' `getFormatStep`: a step of 1, 1.05, ... 9.95 or 10 times a
    /// power of ten (1, 1.1, ... for single digits).
    fn step(rough: f64, allow_decimals: bool, correction: f64) -> f64 {
        if rough <= 0.0 {
            return 0.0;
        }
        let digits = rough.log10().floor() + 1.0;
        let magnitude = 10f64.powf(digits);
        let ratio_scale = if digits == 1.0 { 0.1 } else { 0.05 };
        let ratio = clean(rough / magnitude / ratio_scale);
        let step = clean((ratio.ceil() + correction) * ratio_scale * magnitude);
        if allow_decimals { step } else { step.ceil() }
    }

    /// Recharts' nice ticks for a `[0, auto]` domain: `count` ticks from 0
    /// covering `max`.
    fn nice(max: f64, count: usize) -> Self {
        let max = if max > 0.0 { max } else { 1.0 };
        let mut correction = 0.0;
        loop {
            let step = Self::step(max / (count - 1) as f64, true, correction);
            let up = clean(max / step).ceil() as usize;
            if up + 1 > count {
                correction += 1.0;
                continue;
            }
            let ticks: Vec<f64> = (0..count).map(|i| clean(i as f64 * step)).collect();
            return Self {
                min: 0.0,
                max: ticks[count - 1],
                ticks,
            };
        }
    }

    /// Recharts' ticks for a fixed domain: evenly stepped from `min`, plus
    /// `max`.
    fn fixed(min: f64, max: f64, count: usize, allow_decimals: bool) -> Self {
        let step = Self::step((max - min) / (count - 1) as f64, allow_decimals, 0.0);
        let mut ticks = Vec::new();
        let stop = max - 0.99 * step;
        let mut value = min;
        while step > 0.0 && value < stop {
            ticks.push(clean(value));
            value += step;
        }
        ticks.push(max);
        Self { min, max, ticks }
    }

    /// How far up the plot `value` sits, in percent.
    fn percent(&self, value: f64) -> f64 {
        if self.max == self.min {
            return 0.0;
        }
        (value - self.min) / (self.max - self.min) * 100.0
    }

    /// The distance of `value` from the plot's top, in percent.
    fn top(&self, value: f64) -> f64 {
        100.0 - self.percent(value)
    }
}

/// Advance widths of DM Sans at weight 500, in ems, measured in Chrome:
/// `a` to `z`, `A` to `Z` and `0` to `9`.
const LOWER: [f64; 26] = [
    0.561, 0.634, 0.585, 0.633, 0.578, 0.358, 0.575, 0.592, 0.251, 0.253, 0.528, 0.238, 0.902,
    0.589, 0.600, 0.634, 0.633, 0.391, 0.515, 0.411, 0.589, 0.546, 0.791, 0.518, 0.572, 0.476,
];
const UPPER: [f64; 26] = [
    0.679, 0.616, 0.721, 0.685, 0.573, 0.544, 0.766, 0.693, 0.248, 0.517, 0.609, 0.539, 0.850,
    0.705, 0.766, 0.593, 0.766, 0.608, 0.590, 0.574, 0.664, 0.681, 0.985, 0.625, 0.598, 0.560,
];
const DIGITS: [f64; 10] = [
    0.679, 0.341, 0.582, 0.598, 0.624, 0.613, 0.629, 0.537, 0.615, 0.629,
];

/// The width of `text` in 12px DM Sans, for laying out labels the way
/// Recharts measures them.
fn text_width(text: &str) -> f64 {
    let ems: f64 = text
        .chars()
        .map(|char| match char {
            'a'..='z' => LOWER[char as usize - 'a' as usize],
            'A'..='Z' => UPPER[char as usize - 'A' as usize],
            '0'..='9' => DIGITS[char as usize - '0' as usize],
            ' ' => 0.236,
            '.' | ',' => 0.22,
            '$' => 0.59,
            '%' => 0.838,
            '–' => 0.677,
            '-' => 0.551,
            '<' | '>' | '×' => 0.559,
            _ => 0.6,
        })
        .sum();
    ems * 12.0
}

/// Recharts' "preserve end" tick thinning: from the last tick backwards, keep
/// each label that fits between the chart's edges and `gap` pixels clear of
/// the last one kept. `coords` are the labels' centers.
fn preserve_end(coords: &[f64], widths: &[f64], start: f64, end: f64, gap: f64) -> Vec<bool> {
    let mut end = end;
    let mut shown = vec![false; coords.len()];
    for index in (0..coords.len()).rev() {
        let size = widths[index];
        let mut coord = coords[index];
        if index + 1 == coords.len() {
            let over = coord + size / 2.0 - end;
            if over > 0.0 {
                coord -= over;
            }
        }
        if coord - size / 2.0 >= start && coord + size / 2.0 <= end {
            end = coord - (size / 2.0 + gap);
            shown[index] = true;
        }
    }
    shown
}

/// The narrowest chart width considered, and the container widths from which
/// a label's visibility may change, with the classes that show or hide it
/// from there on. Each 20px step is laid out at its middle width.
const NARROWEST: f64 = 240.0;
const STEP: f64 = 20.0;
const BREAKS: [(f64, &str, &str); 48] = [
    (260.0, "@min-[260px]:block", "@min-[260px]:hidden"),
    (280.0, "@min-[280px]:block", "@min-[280px]:hidden"),
    (300.0, "@min-[300px]:block", "@min-[300px]:hidden"),
    (320.0, "@min-[320px]:block", "@min-[320px]:hidden"),
    (340.0, "@min-[340px]:block", "@min-[340px]:hidden"),
    (360.0, "@min-[360px]:block", "@min-[360px]:hidden"),
    (380.0, "@min-[380px]:block", "@min-[380px]:hidden"),
    (400.0, "@min-[400px]:block", "@min-[400px]:hidden"),
    (420.0, "@min-[420px]:block", "@min-[420px]:hidden"),
    (440.0, "@min-[440px]:block", "@min-[440px]:hidden"),
    (460.0, "@min-[460px]:block", "@min-[460px]:hidden"),
    (480.0, "@min-[480px]:block", "@min-[480px]:hidden"),
    (500.0, "@min-[500px]:block", "@min-[500px]:hidden"),
    (520.0, "@min-[520px]:block", "@min-[520px]:hidden"),
    (540.0, "@min-[540px]:block", "@min-[540px]:hidden"),
    (560.0, "@min-[560px]:block", "@min-[560px]:hidden"),
    (580.0, "@min-[580px]:block", "@min-[580px]:hidden"),
    (600.0, "@min-[600px]:block", "@min-[600px]:hidden"),
    (620.0, "@min-[620px]:block", "@min-[620px]:hidden"),
    (640.0, "@min-[640px]:block", "@min-[640px]:hidden"),
    (660.0, "@min-[660px]:block", "@min-[660px]:hidden"),
    (680.0, "@min-[680px]:block", "@min-[680px]:hidden"),
    (700.0, "@min-[700px]:block", "@min-[700px]:hidden"),
    (720.0, "@min-[720px]:block", "@min-[720px]:hidden"),
    (740.0, "@min-[740px]:block", "@min-[740px]:hidden"),
    (760.0, "@min-[760px]:block", "@min-[760px]:hidden"),
    (780.0, "@min-[780px]:block", "@min-[780px]:hidden"),
    (800.0, "@min-[800px]:block", "@min-[800px]:hidden"),
    (820.0, "@min-[820px]:block", "@min-[820px]:hidden"),
    (840.0, "@min-[840px]:block", "@min-[840px]:hidden"),
    (860.0, "@min-[860px]:block", "@min-[860px]:hidden"),
    (880.0, "@min-[880px]:block", "@min-[880px]:hidden"),
    (900.0, "@min-[900px]:block", "@min-[900px]:hidden"),
    (920.0, "@min-[920px]:block", "@min-[920px]:hidden"),
    (940.0, "@min-[940px]:block", "@min-[940px]:hidden"),
    (960.0, "@min-[960px]:block", "@min-[960px]:hidden"),
    (980.0, "@min-[980px]:block", "@min-[980px]:hidden"),
    (1000.0, "@min-[1000px]:block", "@min-[1000px]:hidden"),
    (1020.0, "@min-[1020px]:block", "@min-[1020px]:hidden"),
    (1040.0, "@min-[1040px]:block", "@min-[1040px]:hidden"),
    (1060.0, "@min-[1060px]:block", "@min-[1060px]:hidden"),
    (1080.0, "@min-[1080px]:block", "@min-[1080px]:hidden"),
    (1100.0, "@min-[1100px]:block", "@min-[1100px]:hidden"),
    (1120.0, "@min-[1120px]:block", "@min-[1120px]:hidden"),
    (1140.0, "@min-[1140px]:block", "@min-[1140px]:hidden"),
    (1160.0, "@min-[1160px]:block", "@min-[1160px]:hidden"),
    (1180.0, "@min-[1180px]:block", "@min-[1180px]:hidden"),
    (1200.0, "@min-[1200px]:block", "@min-[1200px]:hidden"),
];

/// Where the category labels sit along the axis.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Placement {
    /// In the middle of each band (bar charts).
    Band,
    /// Evenly from edge to edge (line and area charts).
    Point,
}

impl Placement {
    /// The position of category `index` of `count`, in percent of the plot.
    fn percent(self, index: usize, count: usize) -> f64 {
        match self {
            Self::Band => (index as f64 + 0.5) * 100.0 / count as f64,
            Self::Point if count > 1 => index as f64 * 100.0 / (count - 1) as f64,
            Self::Point => 50.0,
        }
    }
}

/// The visibility classes of each category label: shown where Recharts would
/// show it at that chart width.
fn label_classes(labels: &[String], placement: Placement, gap: f64) -> Vec<String> {
    let widths: Vec<f64> = labels.iter().map(|label| text_width(label)).collect();
    let count = labels.len();
    let shown_at = |width: f64| {
        let plot = width - VALUE_AXIS - MARGIN_RIGHT;
        let coords: Vec<f64> = (0..count)
            .map(|index| VALUE_AXIS + plot * placement.percent(index, count) / 100.0)
            .collect();
        preserve_end(&coords, &widths, 0.0, width, gap)
    };
    let mut previous = shown_at(NARROWEST + STEP / 2.0);
    let mut classes: Vec<Vec<&str>> = previous
        .iter()
        .map(|shown| if *shown { Vec::new() } else { vec!["hidden"] })
        .collect();
    for (width, show, hide) in BREAKS {
        let now = shown_at(width + STEP / 2.0);
        for (index, shown) in now.iter().enumerate() {
            if *shown != previous[index] {
                classes[index].push(if *shown { show } else { hide });
            }
        }
        previous = now;
    }
    classes.into_iter().map(|list| list.join(" ")).collect()
}

/// A label positioned over the plot.
struct Placed {
    style: String,
    class: String,
    lines: Vec<String>,
}

/// An SVG shape in the plot's 0 to 100 view box.
struct Shape {
    d: String,
    fill: String,
    dashed: bool,
}

/// One value in a tooltip.
struct TooltipRow {
    color: String,
    label: String,
    value: String,
}

/// One category's hover area, with what it shows while hovered.
struct Column {
    style: String,
    /// The tooltip's placement classes.
    side: &'static str,
    heading: Option<String>,
    rows: Vec<TooltipRow>,
    /// A shaded band behind the bars, rather than a line through the points.
    band: bool,
    /// The cursor line's position within the column, in percent.
    line: Option<f64>,
    /// Active dots: their style within the column.
    dots: Vec<String>,
}

/// Everything a cartesian chart draws.
struct Frame {
    config: ChartConfig<'static>,
    height: &'static str,
    min_width: Option<f64>,
    legend: bool,
    /// The plot's distance from the chart's edges: top, right, bottom, left.
    inset: (f64, f64, f64, f64),
    grid: Vec<f64>,
    shapes: Vec<Shape>,
    bars: Vec<String>,
    dots: Vec<String>,
    labels: Vec<Placed>,
    ticks: Vec<Placed>,
    columns: Vec<Column>,
}

impl Frame {
    fn new(config: ChartConfig<'static>, height: &'static str, legend: bool) -> Self {
        let bottom = CATEGORY_AXIS + if legend { LEGEND } else { 0.0 };
        Self {
            config,
            height,
            min_width: None,
            legend,
            inset: (MARGIN_TOP, MARGIN_RIGHT, bottom, VALUE_AXIS),
            grid: Vec::new(),
            shapes: Vec::new(),
            bars: Vec::new(),
            dots: Vec::new(),
            labels: Vec::new(),
            ticks: Vec::new(),
            columns: Vec::new(),
        }
    }

    /// Value ticks on the left and a grid line at each.
    fn value_axis(&mut self, scale: &Scale, format: Format) {
        for tick in &scale.ticks {
            let top = scale.top(*tick);
            self.grid.push(top);
            self.ticks.push(Placed {
                style: format!(
                    "right: calc(100% + 14px); top: {top:.4}%; transform: translateY(-50%)"
                ),
                class: String::new(),
                lines: vec![format(*tick)],
            });
        }
    }

    /// Category labels along the bottom, thinned like Recharts' (unless
    /// `all`), at least `gap` pixels apart.
    fn category_axis(&mut self, rows: &[Row], placement: Placement, gap: f64, all: bool) {
        let labels: Vec<String> = rows.iter().map(|row| row.label.clone()).collect();
        let classes = if all {
            vec![String::new(); labels.len()]
        } else {
            label_classes(&labels, placement, gap)
        };
        let count = labels.len();
        for (index, (label, class)) in labels.into_iter().zip(classes).enumerate() {
            let at = placement.percent(index, count);
            // The last point's label would pass the chart's edge: Recharts
            // moves it back inside.
            let last = placement == Placement::Point && index + 1 == count && count > 1;
            let style = if last && text_width(&label) / 2.0 > MARGIN_RIGHT {
                format!("right: -{MARGIN_RIGHT}px; top: calc(100% + 11px)")
            } else {
                format!("left: {at:.4}%; top: calc(100% + 11px); transform: translateX(-50%)")
            };
            self.ticks.push(Placed {
                style,
                class,
                lines: vec![label],
            });
        }
    }
}

/// The tooltip rows of a category.
fn tooltip_rows(config: ChartConfig<'static>, row: &Row, format: Format) -> Vec<TooltipRow> {
    config
        .iter()
        .enumerate()
        .map(|(index, series)| TooltipRow {
            color: format!("var(--color-{})", series.key),
            label: series.label.to_owned(),
            value: format(row.value(index)),
        })
        .collect()
}

/// Tooltip placement beside a column whose middle is at `middle` percent.
fn side(middle: f64) -> &'static str {
    if middle > 50.0 {
        "top-0 right-[calc(100%+8px)]"
    } else {
        "top-0 left-[calc(100%+8px)]"
    }
}

/// An SVG path through `points`, as `M x,y L x,y ...`.
fn polyline(points: &[(f64, f64)]) -> String {
    let mut d = String::new();
    for (index, (x, y)) in points.iter().enumerate() {
        let command = if index == 0 { 'M' } else { 'L' };
        d.push_str(&format!("{command}{x:.3},{y:.3} "));
    }
    d
}

/// The hover columns of a line or area chart: each from halfway to the
/// previous point to halfway to the next. `levels` holds each category's
/// active dot heights, in percent from the top.
fn point_columns(
    config: ChartConfig<'static>,
    rows: &[Row],
    levels: &[Vec<f64>],
    format: Format,
    dot: f64,
) -> Vec<Column> {
    let count = rows.len();
    let step = if count > 1 {
        100.0 / (count - 1) as f64
    } else {
        100.0
    };
    rows.iter()
        .enumerate()
        .map(|(index, row)| {
            let x = Placement::Point.percent(index, count);
            let left = (x - step / 2.0).max(0.0);
            let right = (x + step / 2.0).min(100.0);
            let width = right - left;
            let inside = if width > 0.0 {
                (x - left) / width * 100.0
            } else {
                50.0
            };
            Column {
                style: format!("left: {left:.4}%; width: {width:.4}%"),
                side: side(x),
                heading: Some(row.label.clone()),
                rows: tooltip_rows(config, row, format),
                band: false,
                line: Some(inside),
                dots: levels[index]
                    .iter()
                    .map(|top| {
                        format!(
                            "left: {inside:.4}%; top: {top:.4}%; width: {dot}px; height: {dot}px; \
                             background: var(--chart-active-dot)"
                        )
                    })
                    .collect(),
            }
        })
        .collect()
}

/// Draws a [`Frame`].
#[component]
async fn plot(cx: &Cx, frame: Frame) -> Result<impl View> {
    let (top, right, bottom, left) = frame.inset;
    let config = frame.config;
    let min_width = frame.min_width.map(|width| format!("min-width: {width}px"));
    Ok(view! {
        // A chart with a minimum width scrolls in its parent instead of
        // shrinking.
        <div style=(min_width)>
            chart_container(
                config: config,
                attrs: attributes! { class=(class!("aspect-auto! w-full", frame.height)) },
                <div class="@container relative size-full">
                    <div
                        data-slot="chart-plot"
                        class="absolute"
                        style=(format!("top: {top}px; right: {right}px; bottom: {bottom}px; left: {left}px"))
                    >
                        <svg
                            class="pointer-events-none absolute inset-0 size-full overflow-visible"
                            viewBox="0 0 100 100"
                            preserveAspectRatio="none"
                            aria-hidden="true"
                        >
                            for y in frame.grid {
                                <line
                                    x1="0" y1=(y) x2="100" y2=(y)
                                    stroke=(GRID)
                                    stroke-width="1"
                                    vector-effect="non-scaling-stroke"
                                ></line>
                            }
                        </svg>
                        for column in frame.columns {
                            <div class="group/column absolute top-0 bottom-0" style=(column.style)>
                                if column.band {
                                    <div class="pointer-events-none invisible absolute inset-0 bg-[#80808033] group-hover/column:visible"></div>
                                }
                                if let Some(at) = column.line {
                                    <div
                                        class="pointer-events-none invisible absolute top-0 bottom-0 z-10 w-px bg-border group-hover/column:visible"
                                        style=(format!("left: {at:.4}%"))
                                    ></div>
                                }
                                for dot in column.dots {
                                    <span
                                        class="pointer-events-none invisible absolute z-10 -translate-1/2 rounded-full group-hover/column:visible"
                                        style=(dot)
                                    ></span>
                                }
                                <div class=(class!(TOOLTIP, column.side))>
                                    if let Some(heading) = column.heading {
                                        <div class="font-base">(heading)</div>
                                    }
                                    <div class="grid gap-1.5">
                                        for row in column.rows {
                                            <div class="flex w-full items-center gap-2">
                                                <span
                                                    class="size-3 shrink-0 rounded-[2px] border border-border"
                                                    style=(format!("background: {}", row.color))
                                                ></span>
                                                <span>(row.label)</span>
                                                <span class="ml-auto pl-3 font-heading tabular-nums">(row.value)</span>
                                            </div>
                                        }
                                    </div>
                                </div>
                            </div>
                        }
                        <svg
                            class="pointer-events-none absolute inset-0 size-full overflow-visible"
                            viewBox="0 0 100 100"
                            preserveAspectRatio="none"
                            aria-hidden="true"
                        >
                            for shape in frame.shapes {
                                <path
                                    d=(shape.d)
                                    fill=(shape.fill)
                                    stroke="var(--border)"
                                    stroke-width="2"
                                    stroke-linejoin="round"
                                    stroke-dasharray=(shape.dashed.then_some("6 4"))
                                    vector-effect="non-scaling-stroke"
                                ></path>
                            }
                        </svg>
                        for bar in frame.bars {
                            <div class="pointer-events-none absolute border-2 border-border" style=(bar)></div>
                        }
                        for dot in frame.dots {
                            <span
                                class="pointer-events-none absolute -translate-1/2 rounded-full border border-border"
                                style=(dot)
                            ></span>
                        }
                        for label in frame.labels.into_iter().chain(frame.ticks) {
                            <span
                                class=(class!(
                                    "pointer-events-none absolute leading-4 whitespace-nowrap text-foreground",
                                    label.class,
                                ))
                                style=(label.style)
                            >
                                for (index, line) in label.lines.into_iter().enumerate() {
                                    if index > 0 {
                                        <br>
                                    }
                                    (line)
                                }
                            </span>
                        }
                    </div>
                    if frame.legend {
                        <div class="absolute inset-x-0 bottom-0">
                            chart_legend_content(config: config)
                        </div>
                    }
                </div>
            )
        </div>
    })
}

/// Stacked areas, one layer per config entry, in config order (the first at
/// the bottom).
#[component]
pub async fn stacked_area(
    cx: &Cx,
    config: ChartConfig<'static>,
    rows: Vec<Row>,
    #[default(format_number)] format: Format,
    #[default(format_compact)] axis_format: Format,
    #[default("h-72")] height: &'static str,
) -> Result<impl View> {
    let count = rows.len();
    // The top of each layer, per category.
    let tops: Vec<Vec<f64>> = rows
        .iter()
        .map(|row| {
            let mut total = 0.0;
            (0..config.len())
                .map(|index| {
                    total += row.value(index);
                    total
                })
                .collect()
        })
        .collect();
    let max = tops
        .iter()
        .filter_map(|row| row.last().copied())
        .fold(0.0, f64::max);
    let scale = Scale::nice(max, 5);

    let mut frame = Frame::new(config, height, true);
    frame.value_axis(&scale, axis_format);
    frame.category_axis(&rows, Placement::Point, 32.0, false);
    for index in 0..config.len() {
        let x = |at: usize| Placement::Point.percent(at, count);
        let mut points: Vec<(f64, f64)> = (0..count)
            .map(|at| (x(at), scale.top(tops[at][index])))
            .collect();
        let below = |at: usize| if index == 0 { 0.0 } else { tops[at][index - 1] };
        points.extend((0..count).rev().map(|at| (x(at), scale.top(below(at)))));
        frame.shapes.push(Shape {
            d: format!("{}Z", polyline(&points)),
            fill: format!("var(--color-{})", config[index].key),
            dashed: false,
        });
    }
    let levels: Vec<Vec<f64>> = tops
        .iter()
        .map(|row| row.iter().map(|top| scale.top(*top)).collect())
        .collect();
    frame.columns = point_columns(config, &rows, &levels, format, 8.0);

    Ok(view! { plot(frame: frame) })
}

const COMPARISON: ChartConfig<'static> = &[
    crate::components::chart::ChartSeries::new("current", "This period", "var(--chart-1)"),
    crate::components::chart::ChartSeries::new("previous", "Previous period", "var(--chart-3)"),
];

/// The current period filled, with the previous one as a dashed line. Rows
/// hold `[current, previous]`.
#[component]
pub async fn comparison(
    cx: &Cx,
    rows: Vec<Row>,
    #[default(format_number)] format: Format,
    #[default(format_compact)] axis_format: Format,
    #[default("h-72")] height: &'static str,
) -> Result<impl View> {
    let count = rows.len();
    let max = rows
        .iter()
        .flat_map(|row| [row.value(0), row.value(1)])
        .fold(0.0, f64::max);
    let scale = Scale::nice(max, 5);
    let x = |at: usize| Placement::Point.percent(at, count);

    let mut frame = Frame::new(COMPARISON, height, true);
    frame.value_axis(&scale, axis_format);
    frame.category_axis(&rows, Placement::Point, 32.0, false);
    let mut area: Vec<(f64, f64)> = (0..count)
        .map(|at| (x(at), scale.top(rows[at].value(0))))
        .collect();
    area.push((x(count.saturating_sub(1)), 100.0));
    area.push((x(0), 100.0));
    frame.shapes.push(Shape {
        d: format!("{}Z", polyline(&area)),
        fill: "var(--color-current)".to_owned(),
        dashed: false,
    });
    let line: Vec<(f64, f64)> = (0..count)
        .map(|at| (x(at), scale.top(rows[at].value(1))))
        .collect();
    frame.shapes.push(Shape {
        d: polyline(&line),
        fill: "none".to_owned(),
        dashed: true,
    });
    frame.dots = line
        .iter()
        .map(|(x, y)| {
            format!("left: {x:.4}%; top: {y:.4}%; width: 9px; height: 9px; background: var(--color-previous)")
        })
        .collect();
    let levels: Vec<Vec<f64>> = rows
        .iter()
        .map(|row| vec![scale.top(row.value(0))])
        .collect();
    frame.columns = point_columns(COMPARISON, &rows, &levels, format, 8.0);

    Ok(view! { plot(frame: frame) })
}

/// A single line with a dot per point. The axis fits the data instead of
/// starting at zero. `config` has one entry.
#[component]
pub async fn trend(
    cx: &Cx,
    config: ChartConfig<'static>,
    rows: Vec<Row>,
    #[default(format_number)] format: Format,
    #[default(format_compact)] axis_format: Format,
    #[default("h-72")] height: &'static str,
) -> Result<impl View> {
    let count = rows.len();
    let values: Vec<f64> = rows.iter().map(|row| row.value(0)).collect();
    let low = values.iter().copied().fold(f64::INFINITY, f64::min);
    let high = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let scale = Scale::fixed(low.floor() - 1.0, high.ceil() + 1.0, 5, false);
    let points: Vec<(f64, f64)> = values
        .iter()
        .enumerate()
        .map(|(at, value)| (Placement::Point.percent(at, count), scale.top(*value)))
        .collect();

    let mut frame = Frame::new(config, height, false);
    frame.value_axis(&scale, axis_format);
    frame.category_axis(&rows, Placement::Point, 32.0, false);
    frame.shapes.push(Shape {
        d: polyline(&points),
        fill: "none".to_owned(),
        dashed: false,
    });
    let key = config[0].key;
    frame.dots = points
        .iter()
        .map(|(x, y)| {
            format!("left: {x:.4}%; top: {y:.4}%; width: 11px; height: 11px; background: var(--color-{key})")
        })
        .collect();
    let levels: Vec<Vec<f64>> = points.iter().map(|(_, y)| vec![*y]).collect();
    frame.columns = point_columns(config, &rows, &levels, format, 12.0);

    Ok(view! { plot(frame: frame) })
}

/// Vertical bars, side by side or `stacked`, at most 48px wide. With
/// `all_labels` every category is labelled and narrow screens scroll the
/// chart instead.
#[component]
pub async fn columns(
    cx: &Cx,
    config: ChartConfig<'static>,
    rows: Vec<Row>,
    #[default(format_number)] format: Format,
    #[default(format_compact)] axis_format: Format,
    #[default] stacked: bool,
    #[default] all_labels: bool,
    #[default("h-72")] height: &'static str,
) -> Result<impl View> {
    let count = rows.len().max(1);
    let keys = config.len();
    let max = rows
        .iter()
        .map(|row| {
            if stacked {
                sum((0..keys).map(|index| row.value(index)))
            } else {
                (0..keys).map(|index| row.value(index)).fold(0.0, f64::max)
            }
        })
        .fold(0.0, f64::max);
    let scale = Scale::nice(max, 5);

    let mut frame = Frame::new(config, height, keys > 1);
    if all_labels {
        frame.min_width = Some(rows.len() as f64 * 84.0);
    }
    frame.value_axis(&scale, axis_format);
    frame.category_axis(&rows, Placement::Band, 12.0, all_labels);

    // Each band keeps 10% clear on both sides; its bars share the rest, 4px
    // apart, each at most 48px and centered in its share (Recharts'
    // `getBarPosition`).
    let band = 100.0 / count as f64;
    let slots = if stacked { 1 } else { keys.max(1) };
    let gaps = (slots - 1) * 4;
    let share = format!("(({:.4}% - {gaps}px) / {slots})", band * 0.8);
    let size = format!("min({share}, 48px)");
    for (index, row) in rows.iter().enumerate() {
        let start = index as f64 * band + band * 0.1;
        let mut base = 0.0;
        for key in 0..keys {
            let value = row.value(key);
            let slot = if stacked { 0 } else { key };
            let (from, to) = if stacked {
                base += value;
                (base - value, base)
            } else {
                (0.0, value)
            };
            if to <= from {
                continue;
            }
            let rounded = !stacked || key + 1 == keys;
            let radius = if rounded { "4px 4px 0 0" } else { "0" };
            let (bottom, height) = (scale.percent(from), scale.percent(to) - scale.percent(from));
            // The stroke is centered on the bar's edge in Recharts, so the
            // box grows by a pixel on each side.
            frame.bars.push(format!(
                "left: calc({start:.4}% + {slot} * ({share} + 4px) + ({share} - {size}) / 2 - 1px); \
                 width: calc({size} + 2px); bottom: calc({bottom:.4}% - 1px); \
                 height: calc({height:.4}% + 2px); border-radius: {radius}; \
                 background: var(--color-{})",
                config[key].key,
            ));
        }
        let middle = index as f64 * band + band / 2.0;
        frame.columns.push(Column {
            style: format!("left: {:.4}%; width: {band:.4}%", index as f64 * band),
            side: side(middle),
            heading: Some(row.label.clone()),
            rows: tooltip_rows(config, row, format),
            band: true,
            line: None,
            dots: Vec::new(),
        });
    }

    Ok(view! {
        if all_labels {
            <div class="overflow-x-auto">
                plot(frame: frame)
            </div>
        } else {
            plot(frame: frame)
        }
    })
}

/// Splits a category label into lines the way Recharts wraps a 100px axis's
/// labels.
fn wrap(label: &str) -> Vec<String> {
    const WIDTH: f64 = 76.0;
    let mut lines: Vec<String> = Vec::new();
    for word in label.split(' ') {
        match lines.last_mut() {
            Some(line) if text_width(&format!("{line} {word}")) <= WIDTH => {
                line.push(' ');
                line.push_str(word);
            }
            _ => lines.push(word.to_owned()),
        }
    }
    lines
}

/// Horizontal bars with the value printed at the end of each one. `config`
/// has one entry.
#[component]
pub async fn ranked_bars(
    cx: &Cx,
    config: ChartConfig<'static>,
    rows: Vec<Row>,
    #[default(format_number)] format: Format,
    #[default("h-72")] height: &'static str,
) -> Result<impl View> {
    let count = rows.len().max(1);
    let max = rows.iter().map(|row| row.value(0)).fold(0.0, f64::max);
    let max = if max > 0.0 { max } else { 1.0 };
    let key = config[0].key;

    let mut frame = Frame::new(config, height, false);
    frame.inset = (0.0, 64.0, 0.0, 100.0);
    let band = 100.0 / count as f64;
    // At most 28px thick, centered in 80% of the band.
    let size = format!("min({:.4}%, 28px)", band * 0.8);
    for (index, row) in rows.iter().enumerate() {
        let value = row.value(0);
        let start = index as f64 * band;
        let middle = start + band / 2.0;
        let width = value / max * 100.0;
        frame.bars.push(format!(
            "top: calc({middle:.4}% - {size} / 2 - 1px); height: calc({size} + 2px); left: -1px; \
             width: calc({width:.4}% + 2px); border-radius: 0 4px 4px 0; background: var(--color-{key})"
        ));
        frame.labels.push(Placed {
            style: format!(
                "left: calc({width:.4}% + 8px); top: {middle:.4}%; transform: translateY(-50%)"
            ),
            class: "font-heading".to_owned(),
            lines: vec![format(value)],
        });
        frame.ticks.push(Placed {
            style: format!(
                "right: calc(100% + 14px); top: {middle:.4}%; transform: translateY(-50%)"
            ),
            class: "text-right leading-3!".to_owned(),
            lines: wrap(&row.label),
        });
        frame.columns.push(Column {
            style: format!(
                "left: 0; width: 100%; top: {start:.4}%; height: {band:.4}%; bottom: auto"
            ),
            side: "top-full right-2 mt-1",
            heading: Some(row.label.clone()),
            rows: tooltip_rows(config, row, format),
            band: true,
            line: None,
            dots: Vec::new(),
        });
    }

    Ok(view! { plot(frame: frame) })
}

/// A slice of a [`donut`].
pub struct Slice {
    pub label: &'static str,
    pub value: f64,
    pub color: &'static str,
}

/// The donut's box (Tailwind's `h-52`) and radii, in pixels.
const DONUT_SIZE: f64 = 208.0;
const DONUT_INNER: f64 = 62.0;
const DONUT_OUTER: f64 = 98.0;

/// The point at `radius` and `angle` degrees from the donut's center,
/// counterclockwise from three o'clock as in Recharts.
fn polar(radius: f64, angle: f64) -> (f64, f64) {
    let center = DONUT_SIZE / 2.0;
    let radians = angle.to_radians();
    (
        center + radius * radians.cos(),
        center - radius * radians.sin(),
    )
}

/// The path of a ring slice from `start` to `end` degrees.
fn sector(start: f64, end: f64) -> String {
    let end = if end - start >= 360.0 {
        start + 359.999
    } else {
        end
    };
    let large = u8::from(end - start > 180.0);
    let (x0, y0) = polar(DONUT_OUTER, start);
    let (x1, y1) = polar(DONUT_OUTER, end);
    let (x2, y2) = polar(DONUT_INNER, end);
    let (x3, y3) = polar(DONUT_INNER, start);
    format!(
        "M{x0:.3},{y0:.3} A{DONUT_OUTER},{DONUT_OUTER} 0 {large} 0 {x1:.3},{y1:.3} \
         L{x2:.3},{y2:.3} A{DONUT_INNER},{DONUT_INNER} 0 {large} 1 {x3:.3},{y3:.3} Z"
    )
}

/// A donut with its total in the middle and a legend that lists every value
/// and its share.
#[component]
pub async fn donut(
    cx: &Cx,
    slices: Vec<Slice>,
    caption: &str,
    #[default(format_number)] format: Format,
) -> Result<impl View> {
    let all = sum(slices.iter().map(|slice| slice.value));
    let mut angle = 0.0;
    let mut drawn = Vec::new();
    for slice in &slices {
        if all <= 0.0 || slice.value <= 0.0 {
            continue;
        }
        let sweep = slice.value / all * 360.0;
        let (start, end) = (angle, angle + sweep);
        angle = end;
        let (x, y) = polar(DONUT_OUTER, (start + end) / 2.0);
        let (left, top) = (x / DONUT_SIZE * 100.0, y / DONUT_SIZE * 100.0);
        let place = if x < DONUT_SIZE / 2.0 {
            format!(
                "right: calc({:.4}% + 8px); top: {top:.4}%; transform: translateY(-50%)",
                100.0 - left
            )
        } else {
            format!("left: calc({left:.4}% + 8px); top: {top:.4}%; transform: translateY(-50%)")
        };
        drawn.push((
            sector(start, end),
            slice.color,
            place,
            slice.label,
            format(slice.value),
        ));
    }
    let view_box = format!("0 0 {DONUT_SIZE} {DONUT_SIZE}");

    Ok(view! {
        <div>
            <div class="relative mx-auto h-52 max-w-52 text-xs">
                <div class="absolute inset-0 flex flex-col items-center justify-center">
                    <span class="font-heading text-xl tabular-nums">(format(all))</span>
                    <span class="text-xs">(caption)</span>
                </div>
                for (path, color, place, label, value) in drawn {
                    <svg
                        class="pointer-events-none absolute inset-0 size-full overflow-visible"
                        viewBox=(view_box.clone())
                        aria-hidden="true"
                    >
                        <path
                            class="pointer-events-auto"
                            d=(path)
                            fill=(color)
                            stroke="var(--border)"
                            stroke-width="2"
                            stroke-linejoin="round"
                        ></path>
                    </svg>
                    <div
                        class="pointer-events-none absolute z-20 hidden w-max min-w-[8rem] rounded-base border-2 border-border bg-secondary-background px-2.5 py-1.5 [:hover+&]:grid"
                        style=(place)
                    >
                        <div class="flex w-full items-center gap-2">
                            <span
                                class="size-3 shrink-0 rounded-[2px] border border-border"
                                style=(format!("background: {color}"))
                            ></span>
                            <span>(label)</span>
                            <span class="ml-auto pl-3 font-heading tabular-nums">(value)</span>
                        </div>
                    </div>
                }
            </div>
            <ul class="mt-5 space-y-2 text-sm">
                for slice in &slices {
                    <li class="flex items-center gap-2">
                        <span
                            class="size-3.5 shrink-0 rounded-[2px] border-2 border-border"
                            style=(format!("background: {}", slice.color))
                        ></span>
                        (slice.label)
                        <span class="ml-auto tabular-nums">(format(slice.value))</span>
                        <span class="w-10 text-right font-heading tabular-nums">
                            (format!("{}%", super::data::round(slice.value / all * 100.0)))
                        </span>
                    </li>
                }
            </ul>
        </div>
    })
}

/// A small filled line for a stat card, scaled from its lowest to its
/// highest value.
#[component]
pub async fn sparkline(cx: &Cx, values: Vec<f64>, color: &str) -> Result<impl View> {
    const WIDTH: f64 = 80.0;
    const HEIGHT: f64 = 40.0;
    const MARGIN: f64 = 2.0;
    let low = values.iter().copied().fold(f64::INFINITY, f64::min);
    let high = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let count = values.len();
    let x = |index: usize| {
        let step = if count > 1 {
            (WIDTH - 2.0 * MARGIN) / (count - 1) as f64
        } else {
            0.0
        };
        MARGIN + index as f64 * step
    };
    let y = |value: f64| {
        let share = if high > low {
            (value - low) / (high - low)
        } else {
            0.5
        };
        MARGIN + (HEIGHT - 2.0 * MARGIN) * (1.0 - share)
    };
    let mut points: Vec<(f64, f64)> = values
        .iter()
        .enumerate()
        .map(|(index, value)| (x(index), y(*value)))
        .collect();
    points.push((x(count.saturating_sub(1)), HEIGHT - MARGIN));
    points.push((x(0), HEIGHT - MARGIN));
    let d = format!("{}Z", polyline(&points));

    Ok(view! {
        <svg
            class="pointer-events-none h-10 w-20 shrink-0 overflow-visible"
            viewBox="0 0 80 40"
            aria-hidden="true"
        >
            <path d=(d) fill=(color) stroke="var(--border)" stroke-width="2" stroke-linejoin="round"></path>
        </svg>
    })
}
