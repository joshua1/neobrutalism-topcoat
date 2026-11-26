//! Demo data for the dashboard. Everything is derived from one seeded set of
//! daily figures, so totals agree across pages and every render shows the
//! same numbers. Replace [`days`] and the lists below with your own data.
//!
//! The generator, its seed and the order of its draws are the upstream
//! template's, and the arithmetic follows JavaScript's (`Math.round`,
//! `toFixed`), so the figures match neobrutalism.dev's dashboard.

use std::sync::LazyLock;

/// A calendar day, as days since 1970-01-01 (UTC).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Date(i64);

const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

impl Date {
    /// The day `year-month-day`.
    pub const fn from_ymd(year: i64, month: i64, day: i64) -> Self {
        // Howard Hinnant's `days_from_civil`.
        let year = if month <= 2 { year - 1 } else { year };
        let era = if year >= 0 { year } else { year - 399 } / 400;
        let year_of_era = year - era * 400;
        let shifted = if month > 2 { month - 3 } else { month + 9 };
        let day_of_year = (153 * shifted + 2) / 5 + day - 1;
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
        Self(era * 146_097 + day_of_era - 719_468)
    }

    /// The day `days` later (or earlier, when negative).
    pub const fn add_days(self, days: i64) -> Self {
        Self(self.0 + days)
    }

    /// The day of the week, Sunday first (JavaScript's `getUTCDay`).
    pub fn weekday(self) -> usize {
        // 1970-01-01 was a Thursday.
        (self.0 + 4).rem_euclid(7) as usize
    }

    /// The year, month (1 to 12) and day of the month.
    fn ymd(self) -> (i64, usize, i64) {
        // Howard Hinnant's `civil_from_days`.
        let z = self.0 + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let day_of_era = z - era * 146_097;
        let year_of_era =
            (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let shifted = (5 * day_of_year + 2) / 153;
        let day = day_of_year - (153 * shifted + 2) / 5 + 1;
        let month = if shifted < 10 {
            shifted + 3
        } else {
            shifted - 9
        };
        let year = year_of_era + era * 400 + i64::from(month <= 2);
        (year, month as usize, day)
    }

    /// The short form used on axes and in tables, such as `Nov 22`.
    pub fn short(self) -> String {
        let (_, month, day) = self.ymd();
        format!("{} {day}", MONTHS[month - 1])
    }

    /// The ISO form, such as `2026-11-22`.
    pub fn iso(self) -> String {
        let (year, month, day) = self.ymd();
        format!("{year:04}-{month:02}-{day:02}")
    }
}

/// The sales channels, in the order of [`Day::revenue`] and [`Day::orders`].
pub const CHANNELS: [&str; 3] = ["online", "marketplace", "retail"];
/// The traffic sources, in the order of [`Day::sessions`].
pub const SOURCES: [&str; 4] = ["organic", "paid", "social", "email"];

/// One day of figures.
#[derive(Clone, Debug)]
pub struct Day {
    pub date: Date,
    /// Revenue per channel, in [`CHANNELS`] order.
    pub revenue: [f64; 3],
    /// Orders per channel, in [`CHANNELS`] order.
    pub orders: [f64; 3],
    pub refunds: f64,
    pub new_customers: f64,
    pub returning: f64,
    /// Online store sessions per source, in [`SOURCES`] order.
    pub sessions: [f64; 4],
}

/// A date range the header's select offers.
pub struct Range {
    pub days: usize,
    pub value: &'static str,
    pub label: &'static str,
}

pub const RANGES: [Range; 3] = [
    Range {
        days: 7,
        value: "7",
        label: "Last 7 days",
    },
    Range {
        days: 28,
        value: "28",
        label: "Last 4 weeks",
    },
    Range {
        days: 84,
        value: "84",
        label: "Last 12 weeks",
    },
];

/// The range shown when none is chosen.
pub const DEFAULT_RANGE: usize = 28;

/// The longest range plus its comparison period.
const TOTAL_DAYS: usize = 168;
const LAST_DAY: Date = Date::from_ymd(2026, 11, 22);

/// The `mulberry32` generator the upstream data is drawn from.
struct Random(u32);

impl Random {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x6d2b_79f5);
        let seed = self.0;
        let mut t = (seed ^ (seed >> 15)).wrapping_mul(1 | seed);
        t = t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t)) ^ t;
        f64::from(t ^ (t >> 14)) / 4_294_967_296.0
    }

    /// A factor within `amount` of 1.
    fn jitter(&mut self, amount: f64) -> f64 {
        1.0 + (self.next() - 0.5) * 2.0 * amount
    }
}

/// JavaScript's `Math.round`: halves round up, also below zero.
pub fn round(value: f64) -> f64 {
    let rounded = value.round();
    if value - rounded == 0.5 {
        rounded + 1.0
    } else {
        rounded
    }
}

pub fn sum(values: impl IntoIterator<Item = f64>) -> f64 {
    values.into_iter().fold(0.0, |total, value| total + value)
}

/// Splits a count by shares, with the rounding remainder on the largest one
/// so the parts always add up.
pub fn split(count: f64, shares: &[f64]) -> Vec<f64> {
    let mut parts: Vec<f64> = shares.iter().map(|share| round(count * share)).collect();
    let largest =
        shares.iter().enumerate().fold(
            0,
            |best, (index, share)| if *share > shares[best] { index } else { best },
        );
    let rest = count - sum(parts.iter().copied());
    parts[largest] += rest;
    parts
}

/// Sunday first, matching [`Date::weekday`].
const WEEKDAY: [f64; 7] = [0.86, 0.94, 0.97, 1.0, 1.06, 1.18, 1.09];

/// The generated days and the latest orders. They share one generator, so
/// they are drawn together, in the upstream order.
static DATA: LazyLock<(Vec<Day>, Vec<Order>)> = LazyLock::new(|| {
    let mut random = Random(42);
    let days = (0..TOTAL_DAYS)
        .map(|i| {
            let index = i as f64;
            let date = LAST_DAY.add_days(i as i64 - (TOTAL_DAYS as i64 - 1));
            let base = (118.0 + index * 0.32) * WEEKDAY[date.weekday()];
            let orders = [
                round(base * 0.56 * random.jitter(0.12)),
                round(base * 0.27 * random.jitter(0.15)),
                round(base * 0.17 * random.jitter(0.18)),
            ];
            let revenue = [
                round(orders[0] * 64.0 * random.jitter(0.06)),
                round(orders[1] * 51.0 * random.jitter(0.06)),
                round(orders[2] * 78.0 * random.jitter(0.08)),
            ];
            let total = sum(revenue);
            let count = sum(orders);
            let new_customers = round(count * 0.36 * random.jitter(0.14));
            let visits = (orders[0] / (0.029 + index * 0.00002)) * random.jitter(0.05);
            let refunds = round(total * 0.042 * random.jitter(0.5));
            let sessions = [
                round(visits * 0.38 * random.jitter(0.08)),
                round(visits * 0.28 * random.jitter(0.12)),
                round(visits * 0.2 * random.jitter(0.16)),
                round(visits * 0.14 * random.jitter(0.2)),
            ];
            Day {
                date,
                revenue,
                orders,
                refunds,
                new_customers,
                returning: count - new_customers,
                sessions,
            }
        })
        .collect();

    // The latest orders, all from the last day, so the newest are still
    // processing.
    let orders = (0..48)
        .map(|i| {
            let product = &PRODUCTS[(random.next() * PRODUCTS.len() as f64).floor() as usize];
            let quantity = 1.0 + (random.next() * 3.0).floor();
            let roll = random.next();
            let settled = if i as f64 + random.next() * 16.0 < 18.0 {
                Status::Processing
            } else if i < 38 {
                Status::Shipped
            } else {
                Status::Delivered
            };
            let customer = NAMES[(random.next() * NAMES.len() as f64).floor() as usize];
            Order {
                id: format!("#{}", 5248 - i),
                customer,
                product: if quantity > 1.0 {
                    format!("{} × {quantity}", product.name)
                } else {
                    product.name.to_owned()
                },
                total: product.price * quantity,
                status: if roll < 0.06 {
                    Status::Cancelled
                } else {
                    settled
                },
                date: LAST_DAY,
            }
        })
        .collect();

    (days, orders)
});

/// Every generated day, oldest first.
pub fn days() -> &'static [Day] {
    &DATA.0
}

/// The 48 latest orders, newest first.
pub fn orders() -> &'static [Order] {
    &DATA.1
}

pub fn revenue_of(day: &Day) -> f64 {
    sum(day.revenue)
}

pub fn orders_of(day: &Day) -> f64 {
    sum(day.orders)
}

pub fn sessions_of(day: &Day) -> f64 {
    sum(day.sessions)
}

/// The sum of `pick` over `days`.
pub fn total(days: &[Day], pick: impl Fn(&Day) -> f64) -> f64 {
    sum(days.iter().map(pick))
}

/// Groups digits in threes, like `toLocaleString("en-US")` of a whole number.
fn group(whole: &str) -> String {
    let mut grouped = String::new();
    for (index, digit) in whole.chars().enumerate() {
        if index > 0 && (whole.len() - index) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    grouped
}

/// JavaScript's `toFixed`: rounds the exact value, with ties away from zero
/// (Rust's formatting rounds ties to even).
pub fn to_fixed(value: f64, digits: usize) -> String {
    let exact = format!("{:.*}", digits + 30, value.abs());
    let cut = exact.len() - 30;
    let tie = exact[cut..].starts_with('5') && exact[cut + 1..].bytes().all(|byte| byte == b'0');
    let magnitude = if tie {
        let step = 10f64.powi(-(digits as i32));
        format!("{:.*}", digits, value.abs() + step / 2.0)
    } else {
        format!("{:.*}", digits, value.abs())
    };
    if value < 0.0 && magnitude.bytes().any(|byte| (b'1'..=b'9').contains(&byte)) {
        format!("-{magnitude}")
    } else {
        magnitude
    }
}

pub fn format_number(value: f64) -> String {
    let rounded = round(value);
    let grouped = group(&format!("{}", rounded.abs()));
    if rounded < 0.0 {
        format!("-{grouped}")
    } else {
        grouped
    }
}

pub fn format_money(value: f64) -> String {
    format!("${}", format_number(value))
}

pub fn format_percent(value: f64) -> String {
    format!("{}%", to_fixed(value, 1))
}

/// Thousands as `k`, such as `12.5k`.
pub fn format_compact(value: f64) -> String {
    if value >= 1000.0 {
        let text = to_fixed(value / 1000.0, 1);
        format!("{}k", text.strip_suffix(".0").unwrap_or(&text))
    } else {
        format!("{}", round(value))
    }
}

pub fn format_compact_money(value: f64) -> String {
    format!("${}", format_compact(value))
}

/// Dollars and cents, such as `$62.77`.
pub fn format_cents(value: f64) -> String {
    format!("${}", to_fixed(value, 2))
}

/// A run of days shown as one point or bar: a day, or a week in the longest
/// range.
pub struct Bucket {
    pub label: String,
    pub days: &'static [Day],
}

/// The days of the chosen range and of the same span before it.
pub struct Period {
    pub days: &'static [Day],
    pub previous: &'static [Day],
    /// Daily buckets for short ranges, weekly ones for the longest.
    pub buckets: Vec<Bucket>,
    pub previous_buckets: Vec<Bucket>,
    /// `"day"` or `"week"`.
    pub unit: &'static str,
}

fn bucket(days: &'static [Day], size: usize) -> Vec<Bucket> {
    days.chunks(size)
        .map(|slice| Bucket {
            label: slice[0].date.short(),
            days: slice,
        })
        .collect()
}

/// The period of the last `length` days (one of [`RANGES`]).
pub fn period(length: usize) -> Period {
    let all = days();
    let end = all.len();
    let days = &all[end - length..];
    let previous = &all[end - length * 2..end - length];
    let size = if length > 28 { 7 } else { 1 };
    Period {
        days,
        previous,
        buckets: bucket(days, size),
        previous_buckets: bucket(previous, size),
        unit: if size == 7 { "week" } else { "day" },
    }
}

/// The figures of each day in `period`, as CSV.
pub fn report_csv(period: &Period) -> String {
    let mut csv = String::from("date,revenue,orders,refunds,new_customers,sessions");
    for day in period.days {
        csv.push_str(&format!(
            "\n{},{},{},{},{},{}",
            day.date.iso(),
            revenue_of(day),
            orders_of(day),
            day.refunds,
            day.new_customers,
            sessions_of(day),
        ));
    }
    csv
}

pub struct Product {
    pub name: &'static str,
    pub category: &'static str,
    /// Share of total revenue; the shares add up to 1.
    pub share: f64,
    pub price: f64,
    pub stock: f64,
    pub reorder: f64,
}

pub const PRODUCTS: [Product; 8] = [
    Product {
        name: "Canvas Tote",
        category: "Bags",
        share: 0.19,
        price: 38.0,
        stock: 1460.0,
        reorder: 700.0,
    },
    Product {
        name: "Block Hoodie",
        category: "Apparel",
        share: 0.17,
        price: 72.0,
        stock: 286.0,
        reorder: 340.0,
    },
    Product {
        name: "Notebook",
        category: "Stationery",
        share: 0.14,
        price: 18.0,
        stock: 2140.0,
        reorder: 1100.0,
    },
    Product {
        name: "Enamel Mug",
        category: "Home",
        share: 0.12,
        price: 24.0,
        stock: 1028.0,
        reorder: 700.0,
    },
    Product {
        name: "Desk Lamp",
        category: "Home",
        share: 0.11,
        price: 89.0,
        stock: 141.0,
        reorder: 180.0,
    },
    Product {
        name: "Graphic Tee",
        category: "Apparel",
        share: 0.1,
        price: 34.0,
        stock: 865.0,
        reorder: 420.0,
    },
    Product {
        name: "Weekender",
        category: "Bags",
        share: 0.09,
        price: 128.0,
        stock: 62.0,
        reorder: 100.0,
    },
    Product {
        name: "Brush Pens",
        category: "Stationery",
        share: 0.08,
        price: 29.0,
        stock: 717.0,
        reorder: 400.0,
    },
];

/// Shares of revenue by shipping country.
pub const COUNTRIES: [(&str, f64); 6] = [
    ("United States", 0.41),
    ("Germany", 0.17),
    ("United Kingdom", 0.14),
    ("Croatia", 0.11),
    ("France", 0.09),
    ("Other", 0.08),
];

/// Where an order is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Delivered,
    Processing,
    Cancelled,
    Shipped,
}

pub const STATUSES: [Status; 4] = [
    Status::Delivered,
    Status::Processing,
    Status::Cancelled,
    Status::Shipped,
];

impl Status {
    pub fn label(self) -> &'static str {
        match self {
            Self::Delivered => "Delivered",
            Self::Processing => "Processing",
            Self::Cancelled => "Cancelled",
            Self::Shipped => "Shipped",
        }
    }

    pub fn color(self) -> &'static str {
        match self {
            Self::Delivered => "var(--chart-4)",
            Self::Processing => "var(--chart-3)",
            Self::Cancelled => "var(--chart-2)",
            Self::Shipped => "var(--chart-5)",
        }
    }

    /// Share of a period's orders in this status.
    pub fn share(self) -> f64 {
        match self {
            Self::Delivered => 0.72,
            Self::Processing => 0.09,
            Self::Cancelled => 0.05,
            Self::Shipped => 0.14,
        }
    }

    /// The status called `label`, if any.
    pub fn from_label(label: &str) -> Option<Self> {
        STATUSES.into_iter().find(|status| status.label() == label)
    }
}

#[derive(Clone, Debug)]
pub struct Order {
    pub id: String,
    pub customer: &'static str,
    pub product: String,
    pub total: f64,
    pub status: Status,
    pub date: Date,
}

const NAMES: [&str; 12] = [
    "Ana Kovač",
    "Liam Porter",
    "Mei Tanaka",
    "Omar Haddad",
    "Sofia Rossi",
    "Noah Fischer",
    "Priya Nair",
    "Lucas Martin",
    "Ivy Chen",
    "Diego Ramos",
    "Freya Olsen",
    "Tom Walsh",
];

/// The first hour of each two-hour slot.
pub const HOURS: [u32; 12] = [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22];
pub const WEEKDAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

/// Percent of a day's orders in each two-hour slot; weekends shift later.
const WORKDAY_HOURS: [f64; 12] = [
    2.0, 1.0, 1.0, 3.0, 8.0, 12.0, 13.0, 11.0, 12.0, 14.0, 15.0, 8.0,
];
const WEEKEND_HOURS: [f64; 12] = [
    4.0, 2.0, 1.0, 1.0, 4.0, 9.0, 12.0, 13.0, 14.0, 15.0, 15.0, 10.0,
];

/// Orders per weekday (Monday first) and two-hour slot.
pub fn order_heatmap(days: &[Day]) -> Vec<(&'static str, Vec<f64>)> {
    WEEKDAYS
        .iter()
        .enumerate()
        .map(|(i, weekday)| {
            let count: f64 = sum(days
                .iter()
                .filter(|day| day.date.weekday() == (i + 1) % 7)
                .map(orders_of));
            let hours = if i > 4 { WEEKEND_HOURS } else { WORKDAY_HOURS };
            (
                *weekday,
                hours
                    .iter()
                    .map(|share| round((count * share) / 100.0))
                    .collect(),
            )
        })
        .collect()
}

/// Share of orders by hours between payment and dispatch.
pub const DISPATCH: [(&str, f64); 5] = [
    ("<6h", 0.18),
    ("6–12h", 0.31),
    ("12–24h", 0.27),
    ("1–2d", 0.16),
    (">2d", 0.08),
];

/// Percent of each cohort that ordered again, by months since first order.
pub struct Cohort {
    pub month: &'static str,
    pub size: f64,
    pub retention: &'static [f64],
}

pub const COHORTS: [Cohort; 6] = [
    Cohort {
        month: "Jun",
        size: 1184.0,
        retention: &[100.0, 42.0, 35.0, 31.0, 28.0, 26.0],
    },
    Cohort {
        month: "Jul",
        size: 1246.0,
        retention: &[100.0, 44.0, 37.0, 33.0, 30.0],
    },
    Cohort {
        month: "Aug",
        size: 1302.0,
        retention: &[100.0, 43.0, 38.0, 34.0],
    },
    Cohort {
        month: "Sep",
        size: 1395.0,
        retention: &[100.0, 47.0, 40.0],
    },
    Cohort {
        month: "Oct",
        size: 1467.0,
        retention: &[100.0, 49.0],
    },
    Cohort {
        month: "Nov",
        size: 1120.0,
        retention: &[100.0],
    },
];

pub struct Segment {
    pub label: &'static str,
    pub share: f64,
    pub color: &'static str,
}

pub const SEGMENTS: [Segment; 4] = [
    Segment {
        label: "VIP",
        share: 0.12,
        color: "var(--chart-1)",
    },
    Segment {
        label: "Loyal",
        share: 0.31,
        color: "var(--chart-3)",
    },
    Segment {
        label: "Occasional",
        share: 0.38,
        color: "var(--chart-2)",
    },
    Segment {
        label: "One-time",
        share: 0.19,
        color: "var(--chart-4)",
    },
];

pub struct Customer {
    pub name: &'static str,
    pub country: &'static str,
    pub orders: u32,
    pub spent: f64,
}

pub const TOP_CUSTOMERS: [Customer; 5] = [
    Customer {
        name: "Mei Tanaka",
        country: "United States",
        orders: 34,
        spent: 2864.0,
    },
    Customer {
        name: "Noah Fischer",
        country: "Germany",
        orders: 29,
        spent: 2417.0,
    },
    Customer {
        name: "Ana Kovač",
        country: "Croatia",
        orders: 27,
        spent: 2190.0,
    },
    Customer {
        name: "Tom Walsh",
        country: "United Kingdom",
        orders: 22,
        spent: 1975.0,
    },
    Customer {
        name: "Lucas Martin",
        country: "France",
        orders: 21,
        spent: 1748.0,
    },
];

/// A campaign: its share of online revenue and its daily spend.
pub struct Campaign {
    pub name: &'static str,
    pub channel: &'static str,
    pub share: f64,
    pub spend: f64,
}

pub const CAMPAIGNS: [Campaign; 5] = [
    Campaign {
        name: "Holiday search",
        channel: "Paid",
        share: 0.17,
        spend: 290.0,
    },
    Campaign {
        name: "Autumn drop",
        channel: "Social",
        share: 0.11,
        spend: 225.0,
    },
    Campaign {
        name: "Win-back",
        channel: "Email",
        share: 0.09,
        spend: 48.0,
    },
    Campaign {
        name: "Creator collab",
        channel: "Social",
        share: 0.06,
        spend: 340.0,
    },
    Campaign {
        name: "Retargeting",
        channel: "Paid",
        share: 0.08,
        spend: 140.0,
    },
];

/// Share of sessions that reach each step before ordering.
pub const FUNNEL: [(&str, f64); 4] = [
    ("Sessions", 1.0),
    ("Product views", 0.54),
    ("Added to cart", 0.14),
    ("Checkout", 0.062),
];

pub struct User {
    pub name: &'static str,
    pub initials: &'static str,
    pub email: &'static str,
    pub role: &'static str,
    pub store: &'static str,
    pub since: &'static str,
}

pub const USER: User = User {
    name: "Ella Novak",
    initials: "EN",
    email: "ella@tally.example",
    role: "Store owner",
    store: "Tally Goods",
    since: "March 2024",
};

pub struct Notification {
    pub id: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub enabled: bool,
}

pub const NOTIFICATIONS: [Notification; 4] = [
    Notification {
        id: "orders",
        label: "New orders",
        description: "Every time an order comes in",
        enabled: false,
    },
    Notification {
        id: "stock",
        label: "Low stock",
        description: "When a product needs a reorder",
        enabled: true,
    },
    Notification {
        id: "refunds",
        label: "Refunds",
        description: "When a customer asks for one",
        enabled: true,
    },
    Notification {
        id: "report",
        label: "Weekly report",
        description: "A summary every Monday",
        enabled: true,
    },
];

pub struct Plan {
    pub name: &'static str,
    pub description: &'static str,
    pub price: f64,
    pub orders: f64,
    pub products: f64,
    pub seats: f64,
}

pub const PLANS: [Plan; 3] = [
    Plan {
        name: "Starter",
        description: "For new stores",
        price: 29.0,
        orders: 5000.0,
        products: 25.0,
        seats: 3.0,
    },
    Plan {
        name: "Growth",
        description: "For stores with steady sales",
        price: 79.0,
        orders: 10000.0,
        products: 250.0,
        seats: 5.0,
    },
    Plan {
        name: "Scale",
        description: "For large teams",
        price: 199.0,
        orders: 50000.0,
        products: 2500.0,
        seats: 20.0,
    },
];

pub const SEATS_USED: f64 = 3.0;
pub const CARD_LAST4: &str = "4242";
pub const CARD_EXPIRES: &str = "08/28";
pub const NEXT_INVOICE: &str = "Dec 1, 2026";

pub struct Invoice {
    pub id: &'static str,
    pub date: &'static str,
    pub plan: &'static str,
    pub total: f64,
}

pub const INVOICES: [Invoice; 6] = [
    Invoice {
        id: "INV-0032",
        date: "Nov 1, 2026",
        plan: "Growth",
        total: 79.0,
    },
    Invoice {
        id: "INV-0031",
        date: "Oct 1, 2026",
        plan: "Growth",
        total: 79.0,
    },
    Invoice {
        id: "INV-0030",
        date: "Sep 1, 2026",
        plan: "Growth",
        total: 79.0,
    },
    Invoice {
        id: "INV-0029",
        date: "Aug 1, 2026",
        plan: "Growth",
        total: 79.0,
    },
    Invoice {
        id: "INV-0028",
        date: "Jul 1, 2026",
        plan: "Starter",
        total: 29.0,
    },
    Invoice {
        id: "INV-0027",
        date: "Jun 1, 2026",
        plan: "Starter",
        total: 29.0,
    },
];
