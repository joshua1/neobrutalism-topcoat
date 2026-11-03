//! A month calendar, rendered on the server. It replaces react-day-picker,
//! with the date math done here in plain Rust (see [`Date`]).
//!
//! The calendar keeps its state in Topcoat runtime signals that the caller
//! creates and passes in:
//!
//! - `month`, a `Signal<String>` holding the first month shown as
//!   `YYYY-MM`. The previous and next buttons and the month and year
//!   dropdowns set it.
//! - the selection, in [`CalendarMode`]: one `Signal<Option<String>>` for a
//!   single day, or two (`from` and `to`) for a range, each holding a
//!   `YYYY-MM-DD` date.
//!
//! A click sets the signal in the browser. The calendar reads the signals on
//! the server, so a change re-renders the enclosing shard (or the page, if
//! there is none) with the new month or selection. Put the calendar, and
//! anything else that shows its state such as a date picker's trigger, in a
//! `#[shard]` so only that part re-renders:
//!
//! ```ignore
//! #[shard]
//! async fn pick_a_day(cx: &Cx) -> Result<impl View> {
//!     let today = Date::today();
//!     let month = signal(cx, || today.month_key());
//!     let selected = signal(cx, || Some(today.to_string()));
//!     Ok(view! {
//!         calendar(month: month, mode: CalendarMode::Single(selected))
//!     })
//! }
//! ```
//!
//! Values sent back by the browser are user input: the calendar parses them
//! and treats anything that is not a valid date as no date, and keeps the
//! month inside `start_month` / `end_month`. Do the same wherever else you
//! read the signals.
//!
//! Differences from react-day-picker: the arrow keys move focus between the
//! days of a month but not across months, every enabled day is a tab stop,
//! `multiple` selection and week numbers are not supported, and each change
//! waits for the server render (clicking "next" twice before the first render
//! arrives moves one month, not two). Once a range has both ends, the next
//! click starts a new range rather than moving one of its ends. "Today" is
//! the server's date in UTC unless `today` is passed.

// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use std::{
    fmt,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    runtime::{Event, Signal},
    view::{Attributes, StaticClass, View, attributes, class, component, view},
};

use super::button::{ButtonSize, ButtonVariant, button_variants};

/// English month names, January first.
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

/// English weekday names, Sunday first (the order of [`Date::weekday`]).
const WEEKDAY_NAMES: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

/// A day of the proleptic Gregorian calendar, without a time or time zone.
///
/// The fields are ordered year, month, day, so the derived `Ord` compares
/// dates chronologically. Its `Display` and `FromStr` use ISO 8601,
/// `YYYY-MM-DD`, the format the calendar's signals hold.
///
/// ```ignore
/// let date = Date::new(2025, 6, 1);
/// assert_eq!(date.to_string(), "2025-06-01");
/// assert_eq!("2025-06-01".parse(), Ok(date));
/// assert_eq!(date.add_days(30), Date::new(2025, 7, 1));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    year: i32,
    month: u32,
    day: u32,
}

/// Whether `year` has a February 29th: every fourth year, except centuries
/// that are not divisible by 400.
const fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// The number of days in `month` (1 to 12) of `year`.
const fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

#[allow(dead_code)]
impl Date {
    /// The date `year`-`month`-`day`. A `month` outside 1 to 12 and a `day`
    /// past the end of the month are clamped, so `Date::new(2025, 2, 31)` is
    /// February 28th.
    #[must_use]
    pub const fn new(year: i32, month: u32, day: u32) -> Self {
        let month = if month < 1 {
            1
        } else if month > 12 {
            12
        } else {
            month
        };
        let last = days_in_month(year, month);
        let day = if day < 1 {
            1
        } else if day > last {
            last
        } else {
            day
        };
        Self { year, month, day }
    }

    /// Today's date in UTC, from the server's clock.
    #[must_use]
    pub fn today() -> Self {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |elapsed| elapsed.as_secs());
        Self::from_days((seconds / 86_400) as i64)
    }

    /// The year, such as `2025`.
    #[must_use]
    pub const fn year(self) -> i32 {
        self.year
    }

    /// The month, from 1 (January) to 12 (December).
    #[must_use]
    pub const fn month(self) -> u32 {
        self.month
    }

    /// The day of the month, from 1.
    #[must_use]
    pub const fn day(self) -> u32 {
        self.day
    }

    /// The date `days` days after 1970-01-01 (negative for earlier dates).
    ///
    /// This is Howard Hinnant's `civil_from_days`. It counts in 400-year
    /// eras, which always have 146,097 days, and starts each year on March
    /// 1st so the leap day falls at the end of the year.
    #[must_use]
    pub const fn from_days(days: i64) -> Self {
        // Shift the epoch from 1970-01-01 to 0000-03-01.
        let z = days + 719_468;
        // `div_euclid` rounds toward negative infinity, so dates before year
        // 0 land in era -1 rather than era 0; `/` would round toward zero.
        let era = z.div_euclid(146_097);
        let day_of_era = z.rem_euclid(146_097); // 0 ..= 146_096
        // Every 4th year is a leap year, except every 100th, except every
        // 400th: undo those extra days to count whole years.
        let year_of_era =
            (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        // Months from March: 31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 28/29.
        // `(153 * m + 2) / 5` is the first day of month `m` in that order.
        let march_month = (5 * day_of_year + 2) / 153; // 0 ..= 11
        let day = day_of_year - (153 * march_month + 2) / 5 + 1;
        let month = if march_month < 10 {
            march_month + 3
        } else {
            march_month - 9
        };
        // January and February belong to the next calendar year.
        let year = year_of_era + era * 400 + if month <= 2 { 1 } else { 0 };
        Self {
            year: year as i32,
            month: month as u32,
            day: day as u32,
        }
    }

    /// The number of days from 1970-01-01 to this date, the inverse of
    /// [`Date::from_days`] (Hinnant's `days_from_civil`).
    #[must_use]
    pub const fn to_days(self) -> i64 {
        let month = self.month as i64;
        // Years start on March 1st, so January and February count as part of
        // the previous year.
        let year = self.year as i64 - if month <= 2 { 1 } else { 0 };
        let era = year.div_euclid(400);
        let year_of_era = year.rem_euclid(400); // 0 ..= 399
        let march_month = if month > 2 { month - 3 } else { month + 9 };
        let day_of_year = (153 * march_month + 2) / 5 + self.day as i64 - 1;
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
        era * 146_097 + day_of_era - 719_468
    }

    /// The day of the week, from 0 (Sunday) to 6 (Saturday).
    #[must_use]
    pub const fn weekday(self) -> u32 {
        // 1970-01-01 was a Thursday (4). `rem_euclid` keeps the result in
        // 0 ..= 6 for earlier dates too, where `%` would go negative.
        (self.to_days() + 4).rem_euclid(7) as u32
    }

    /// The date `days` days later (or earlier, if negative).
    #[must_use]
    pub const fn add_days(self, days: i64) -> Self {
        Self::from_days(self.to_days() + days)
    }

    /// The same day `months` months later (or earlier, if negative),
    /// clamped to the end of a shorter month.
    #[must_use]
    pub const fn add_months(self, months: i32) -> Self {
        let index = self.year * 12 + (self.month as i32 - 1) + months;
        Self::new(
            index.div_euclid(12),
            index.rem_euclid(12) as u32 + 1,
            self.day,
        )
    }

    /// The first day of this date's month.
    #[must_use]
    pub const fn first_of_month(self) -> Self {
        Self { day: 1, ..self }
    }

    /// The last day of this date's month.
    #[must_use]
    pub const fn last_of_month(self) -> Self {
        Self {
            day: days_in_month(self.year, self.month),
            ..self
        }
    }

    /// The month as `YYYY-MM`, the format of a calendar's `month` signal.
    #[must_use]
    pub fn month_key(self) -> String {
        format!("{:04}-{:02}", self.year, self.month)
    }

    /// Parses a `YYYY-MM` month key into the first day of that month.
    #[must_use]
    pub fn parse_month(key: &str) -> Option<Self> {
        format!("{key}-01").parse().ok()
    }

    /// The month's English name, such as `"June"`.
    #[must_use]
    pub fn month_name(self) -> &'static str {
        MONTH_NAMES[self.month as usize - 1]
    }

    /// The weekday's English name, such as `"Sunday"`.
    #[must_use]
    pub fn weekday_name(self) -> &'static str {
        WEEKDAY_NAMES[self.weekday() as usize]
    }

    /// The date as `June 1st, 2025` (date-fns' `PPP`).
    #[must_use]
    pub fn format_long(self) -> String {
        let suffix = match (self.day % 10, self.day % 100) {
            (_, 11..=13) => "th",
            (1, _) => "st",
            (2, _) => "nd",
            (3, _) => "rd",
            _ => "th",
        };
        format!("{} {}{suffix}, {}", self.month_name(), self.day, self.year)
    }
}

impl fmt::Display for Date {
    /// Formats the date as ISO 8601, `YYYY-MM-DD`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

/// The error from parsing a [`Date`] that is not a valid `YYYY-MM-DD` date.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseDateError;

impl fmt::Display for ParseDateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("expected a date as YYYY-MM-DD")
    }
}

impl std::error::Error for ParseDateError {}

impl FromStr for Date {
    type Err = ParseDateError;

    /// Parses an ISO 8601 date, `YYYY-MM-DD`. Unlike [`Date::new`], it
    /// rejects a month or day that does not exist.
    fn from_str(text: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = text.splitn(3, '-');
        let mut next = || parts.next().ok_or(ParseDateError);
        let year: i32 = next()?.parse().map_err(|_| ParseDateError)?;
        let month: u32 = next()?.parse().map_err(|_| ParseDateError)?;
        let day: u32 = next()?.parse().map_err(|_| ParseDateError)?;
        if (1..=9999).contains(&year)
            && (1..=12).contains(&month)
            && (1..=days_in_month(year, month)).contains(&day)
        {
            Ok(Self { year, month, day })
        } else {
            Err(ParseDateError)
        }
    }
}

/// What clicking a day does, and where the selection is kept.
///
/// [`Default`] is `CalendarMode::None`: the days are not buttons.
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub enum CalendarMode {
    /// Nothing can be selected.
    #[default]
    None,
    /// One day, as `YYYY-MM-DD`. Clicking the selected day clears it.
    Single(Signal<Option<String>>),
    /// A range of days. The first click sets `from`, the second sets `to`
    /// (swapping them if it is earlier), and a third starts a new range.
    Range {
        /// The first day of the range, as `YYYY-MM-DD`.
        from: Signal<Option<String>>,
        /// The last day of the range, as `YYYY-MM-DD`.
        to: Signal<Option<String>>,
    },
}

/// The first day of the week in a [`calendar`].
///
/// [`Default`] is `WeekStart::Sunday`, used when none is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum WeekStart {
    /// Weeks run Sunday to Saturday.
    #[default]
    Sunday,
    /// Weeks run Monday to Sunday.
    Monday,
}

impl WeekStart {
    /// The [`Date::weekday`] number of the first day of the week.
    const fn weekday(self) -> u32 {
        match self {
            Self::Sunday => 0,
            Self::Monday => 1,
        }
    }
}

/// How a [`calendar`] month shows its caption.
///
/// [`Default`] is `CaptionLayout::Label`, used when none is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum CaptionLayout {
    /// The month and year as text, such as "June 2025".
    #[default]
    Label,
    /// Month and year dropdowns.
    Dropdown,
    /// A month dropdown and the year as text.
    DropdownMonths,
    /// The month as text and a year dropdown.
    DropdownYears,
}

/// Classes for the calendar panel.
const ROOT: StaticClass = class!(
    "group/calendar w-fit rounded-base border-2 border-border bg-background p-3 font-heading \
     text-foreground shadow-shadow",
);

/// Classes for the row (or, on narrow screens, column) of months.
const MONTHS: StaticClass = class!("relative flex flex-col gap-2 sm:flex-row");

/// Classes for one month: its caption above its grid.
const MONTH: StaticClass = class!("flex w-full flex-col gap-4");

/// Classes for the navigation bar laid over the first caption row.
const NAV: StaticClass =
    class!("absolute inset-x-0 top-0 flex w-full items-center justify-between gap-1");

/// Classes the previous and next buttons add to a `NoShadow` icon button.
/// The `!`s beat the button's own size and colors, which `class!` does not
/// drop the way Octane's `cn` does.
const NAV_BUTTON: StaticClass = class!(
    "size-7! bg-secondary-background! p-0 text-foreground! select-none aria-disabled:opacity-50",
);

/// Classes for the caption row.
const CAPTION: StaticClass = class!("flex h-7 w-full items-center justify-center px-8");

/// Classes for the caption text.
const CAPTION_LABEL: StaticClass = class!("font-heading select-none text-sm");

/// Classes for the label drawn over a caption dropdown, with its chevron.
const DROPDOWN_LABEL: StaticClass = class!(
    "font-heading select-none flex h-7 items-center gap-1 rounded-base px-2 text-sm \
     [&>svg]:size-3.5",
);

/// Classes for the row of caption dropdowns.
const DROPDOWNS: StaticClass =
    class!("flex h-7 w-full items-center justify-center gap-1.5 text-sm font-heading");

/// Classes for the box around one dropdown.
const DROPDOWN_ROOT: StaticClass =
    class!("relative rounded-base border-2 border-border bg-secondary-background");

/// Classes for the native `<select>`: invisible, over the label, so the
/// browser's own list opens on click.
const DROPDOWN: StaticClass = class!("absolute inset-0 opacity-0");

/// Classes for a month's grid.
const GRID: StaticClass = class!("w-full border-collapse");

/// Classes for the weekday header row.
const WEEKDAYS: StaticClass = class!("flex");

/// Classes for a weekday header cell.
const WEEKDAY: StaticClass =
    class!("w-9 rounded-base text-[0.8rem] font-base text-foreground select-none");

/// Classes for a week row.
const WEEK: StaticClass = class!("mt-2 flex w-full");

/// Classes for a day cell.
const DAY: StaticClass = class!("group/day relative size-9 p-0 text-center text-sm select-none");

/// Classes a day button adds to a `NoShadow` icon button.
const DAY_BUTTON: StaticClass = class!("size-9! font-base");

/// Classes for a day of a range: it drops the border and corners on each side
/// that touches another selected day, and pads that side by the border width
/// so the number stays centered, so a row of the range shares one border.
const RANGE_SEAMS: StaticClass = class!(
    "[[data-selected]:not([data-hidden])+td>&]:rounded-l-none \
     [[data-selected]:not([data-hidden])+td>&]:border-l-0 \
     [[data-selected]:not([data-hidden])+td>&]:pl-0.5 \
     [td:has(+[data-selected]:not([data-hidden]))>&]:rounded-r-none \
     [td:has(+[data-selected]:not([data-hidden]))>&]:border-r-0 \
     [td:has(+[data-selected]:not([data-hidden]))>&]:pr-0.5",
);

/// Inline `onkeydown` for a month grid: the arrow keys move focus to the
/// previous or next day, or the same weekday of the previous or next week.
const ARROW_KEYS: &str = "const s={ArrowLeft:-1,ArrowRight:1,ArrowUp:-7,ArrowDown:7}[event.key];\
     if(!s)return;const b=[...this.querySelectorAll('td>button')],i=b.indexOf(document.activeElement);\
     if(i<0||!b[i+s])return;event.preventDefault();b[i+s].focus()";

/// The selection as dates: `from` to `to`, inclusive.
struct Selection {
    from: Option<Date>,
    to: Option<Date>,
    range: bool,
}

/// Parses a signal's `YYYY-MM-DD` value, ignoring anything invalid.
fn read_date(signal: &Signal<Option<String>>) -> Option<Date> {
    signal.get().as_deref().and_then(|text| text.parse().ok())
}

impl Selection {
    /// Reads the selection from the signals of `mode`. The reads are tracked,
    /// so a change re-renders the calendar.
    fn read(mode: &CalendarMode) -> Self {
        match mode {
            CalendarMode::None => Self {
                from: None,
                to: None,
                range: false,
            },
            CalendarMode::Single(day) => {
                let day = read_date(day);
                Self {
                    from: day,
                    to: day,
                    range: false,
                }
            }
            CalendarMode::Range { from, to } => {
                let (from, to) = match (read_date(from), read_date(to)) {
                    (Some(from), Some(to)) if to < from => (Some(to), Some(from)),
                    (Some(from), to) => (Some(from), to.or(Some(from))),
                    _ => (None, None),
                };
                Self {
                    from,
                    to,
                    range: true,
                }
            }
        }
    }

    /// Whether `date` is selected.
    fn contains(&self, date: Date) -> bool {
        match (self.from, self.to) {
            (Some(from), Some(to)) => from <= date && date <= to,
            _ => false,
        }
    }
}

/// One cell of a month grid.
struct Day {
    date: Date,
    /// The date as `YYYY-MM-DD`.
    iso: String,
    /// Whether it belongs to the previous or next month.
    outside: bool,
    /// Whether it is an outside day that is not shown.
    hidden: bool,
    disabled: bool,
    today: bool,
    selected: bool,
    range_start: bool,
    range_middle: bool,
    range_end: bool,
}

impl Day {
    /// The day button's accessible name, such as "Today, Sunday, June 1st,
    /// 2025, selected".
    fn label(&self) -> String {
        format!(
            "{}{}, {}{}",
            if self.today { "Today, " } else { "" },
            self.date.weekday_name(),
            self.date.format_long(),
            if self.selected { ", selected" } else { "" },
        )
    }
}

/// One month of the calendar.
struct MonthGrid {
    /// The first day of the month.
    first: Date,
    weeks: Vec<Vec<Day>>,
}

/// Lays out the weeks of the month starting at `first`.
fn month_grid(
    first: Date,
    week_start: WeekStart,
    show_outside_days: bool,
    today: Date,
    selection: &Selection,
    disabled: Option<fn(Date) -> bool>,
) -> MonthGrid {
    // Back up to the start of the first week, then run to the end of the
    // last one.
    let lead = (first.weekday() + 7 - week_start.weekday()) % 7;
    let start = first.add_days(-(lead as i64));
    let last = first.last_of_month();
    let trail = (week_start.weekday() + 6 - last.weekday()) % 7;
    let total = (last.to_days() + trail as i64) - start.to_days() + 1;

    let days = (0..total).map(|offset| {
        let date = start.add_days(offset);
        let outside = date.month() != first.month();
        let selected = selection.contains(date);
        let range_start = selection.range && selected && Some(date) == selection.from;
        let range_end = selection.range && selected && Some(date) == selection.to;
        Day {
            date,
            iso: date.to_string(),
            outside,
            hidden: outside && !show_outside_days,
            disabled: disabled.is_some_and(|is_disabled| is_disabled(date)),
            today: date == today,
            selected,
            range_start,
            range_middle: selection.range && selected && !range_start && !range_end,
            range_end,
        }
    });

    // `total` is a whole number of weeks; split the days into rows of 7.
    let mut weeks = Vec::new();
    let mut week = Vec::with_capacity(7);
    for day in days {
        week.push(day);
        if week.len() == 7 {
            weeks.push(std::mem::replace(&mut week, Vec::with_capacity(7)));
        }
    }
    MonthGrid { first, weeks }
}

/// The browser handler, set on a month's `<tbody>`, that updates the
/// selection of `mode` when a day is clicked. One handler serves every day:
/// it reads the date from the clicked button's `value`, which is empty for
/// clicks that land elsewhere, such as between the days.
fn on_day_click(cx: &Cx, mode: &CalendarMode) -> Attributes {
    match mode {
        CalendarMode::None => attributes! { cx => },
        CalendarMode::Single(selected) => attributes! { cx =>
            @click=$(|e: Event| {
                let day = e.target.value;
                if !day.is_empty() {
                    if selected.get().is_some() {
                        if selected.get().unwrap() == day {
                            selected.set(None);
                        } else {
                            selected.set(Some(day));
                        }
                    } else {
                        selected.set(Some(day));
                    }
                }
            })
        },
        CalendarMode::Range { from, to } => attributes! { cx =>
            @click=$(|e: Event| {
                let day = e.target.value;
                if !day.is_empty() {
                    if from.get().is_none() {
                        from.set(Some(day));
                    } else if to.get().is_some() {
                        from.set(Some(day));
                        to.set(None);
                    } else {
                        let start = from.get().unwrap();
                        if day < start {
                            from.set(Some(day));
                            to.set(Some(start));
                        } else {
                            to.set(Some(day));
                        }
                    }
                }
            })
        },
    }
}

/// One option of a caption dropdown: the `month` key it sets, its label,
/// and whether it is the current one.
type DropdownOption = (String, String, bool);

/// The month dropdown's options for the month `shown`, the `index`-th month of
/// the calendar. Each sets the calendar's first month so that this month
/// becomes the picked one.
fn month_options(
    shown: Date,
    index: i32,
    start: Option<Date>,
    end: Option<Date>,
) -> Vec<DropdownOption> {
    (1..=12)
        .map(|month| Date::new(shown.year(), month, 1))
        .filter(|month| start.is_none_or(|start| *month >= start))
        .filter(|month| end.is_none_or(|end| *month <= end))
        .map(|month| {
            let label = month.month_name()[..3].to_owned();
            (month.add_months(-index).month_key(), label, month == shown)
        })
        .collect()
}

/// The year dropdown's options for the month `shown`, the `index`-th month of
/// the calendar, from `start`'s year to `end`'s.
fn year_options(shown: Date, index: i32, start: Date, end: Date) -> Vec<DropdownOption> {
    (start.year()..=end.year())
        .map(|year| {
            let month = Date::new(year, shown.month(), 1).clamp(start, end);
            (
                month.add_months(-index).month_key(),
                year.to_string(),
                year == shown.year(),
            )
        })
        .collect()
}

/// A calendar showing one or more months, with day selection.
///
/// `month` holds the first month shown as `YYYY-MM`; the navigation buttons
/// and dropdowns set it. `mode` says where clicks store the selection and
/// defaults to `CalendarMode::None`. See the [module docs](self) for how the
/// signals re-render the calendar.
///
/// The other props match react-day-picker's: `number_of_months` (default 1),
/// `show_outside_days` (default true), `week_start` (default Sunday),
/// `caption_layout` (default `Label`), `disabled` (a function returning
/// whether a day can't be picked), `start_month` and `end_month` (limits for
/// the navigation; with a dropdown caption they default to 100 years ago and
/// the end of this year) and `today` (defaults to [`Date::today`]). `attrs`
/// are forwarded to the root `<div>`, with extra classes appended.
///
/// ```ignore
/// #[shard]
/// async fn range_calendar(cx: &Cx) -> Result<impl View> {
///     let start = Date::new(2025, 1, 12);
///     let month = signal(cx, || start.month_key());
///     let from = signal(cx, || Some(start.to_string()));
///     let to = signal(cx, || Some(start.add_days(30).to_string()));
///     Ok(view! {
///         calendar(
///             month: month,
///             mode: CalendarMode::Range { from, to },
///             number_of_months: 2,
///             disabled: Some(|day| day > Date::today()),
///         )
///     })
/// }
/// ```
#[component]
pub async fn calendar(
    cx: &Cx,
    /// The first month shown, as `YYYY-MM`.
    month: Signal<String>,
    /// What clicking a day selects.
    #[default]
    mode: CalendarMode,
    /// How many months to show side by side.
    #[default(1)]
    number_of_months: usize,
    /// Whether to show the days of the previous and next months that fill
    /// the first and last weeks.
    #[default(true)]
    show_outside_days: bool,
    /// The first day of the week.
    #[default]
    week_start: WeekStart,
    /// Whether the caption is text or dropdowns.
    #[default]
    caption_layout: CaptionLayout,
    /// Returns whether a day is disabled.
    #[default]
    disabled: Option<fn(Date) -> bool>,
    /// The earliest month to navigate to.
    #[default]
    start_month: Option<Date>,
    /// The latest month to navigate to.
    #[default]
    end_month: Option<Date>,
    /// The day marked as today.
    #[default]
    today: Option<Date>,
    /// Extra attributes for the root `<div>`.
    #[default]
    mut attrs: Attributes,
) -> Result<impl View> {
    let today = today.unwrap_or_else(Date::today);
    let count = number_of_months.clamp(1, 12) as i32;
    let dropdowns = caption_layout != CaptionLayout::Label;
    let start = start_month
        .or(dropdowns.then(|| Date::new(today.year() - 100, 1, 1)))
        .map(Date::first_of_month);
    let end = end_month
        .or(dropdowns.then(|| Date::new(today.year(), 12, 1)))
        .map(Date::first_of_month);

    // The browser sends `month` back: parse it and keep it inside the limits.
    let mut first = Date::parse_month(&month.get())
        .unwrap_or(today)
        .first_of_month();
    if let Some(end) = end {
        first = first.min(end.add_months(1 - count));
    }
    if let Some(start) = start {
        first = first.max(start);
    }
    let last = first.add_months(count - 1);
    let can_go_back = start.is_none_or(|start| first > start);
    let can_go_forward = end.is_none_or(|end| last < end);
    let previous = first.add_months(-1).month_key();
    let next = first.add_months(1).month_key();

    let selection = Selection::read(&mode);
    let range = selection.range;
    let grids: Vec<MonthGrid> = (0..count)
        .map(|index| {
            let shown = first.add_months(index);
            month_grid(
                shown,
                week_start,
                show_outside_days,
                today,
                &selection,
                disabled,
            )
        })
        .collect();

    // The weekday headers, rotated to start on `week_start`.
    let weekdays: Vec<&str> = (0..7)
        .map(|offset| WEEKDAY_NAMES[((week_start.weekday() + offset) % 7) as usize])
        .collect();

    let nav_class = class!(
        button_variants(ButtonVariant::NoShadow, ButtonSize::Icon),
        NAV_BUTTON
    );

    Ok(view! {
        <div data-slot="calendar" class=(class!(ROOT, attrs.remove("class"))) (attrs)>
            <div class=(MONTHS)>
                <nav class=(NAV) aria-label="Navigation bar">
                    <button
                        type="button"
                        class=(nav_class.clone())
                        aria-label="Go to the Previous Month"
                        aria-disabled=((!can_go_back).then_some("true"))
                        disabled=(!can_go_back)
                        @click=$(|_e: Event| month.set(previous.to_owned()))
                    >
                        icon(data: iconify_icon!("lucide:chevron-left"), attrs: attributes! { class="size-4" })
                    </button>
                    <button
                        type="button"
                        class=(nav_class)
                        aria-label="Go to the Next Month"
                        aria-disabled=((!can_go_forward).then_some("true"))
                        disabled=(!can_go_forward)
                        @click=$(|_e: Event| month.set(next.to_owned()))
                    >
                        icon(data: iconify_icon!("lucide:chevron-right"), attrs: attributes! { class="size-4" })
                    </button>
                </nav>
                #[key(grid.first.month_key())]
                for (index, grid) in grids.into_iter().enumerate() {
                    let index = index as i32;
                    let title = format!("{} {}", grid.first.month_name(), grid.first.year());
                    <div class=(MONTH)>
                        <div class=(CAPTION)>
                            if dropdowns {
                                <div class=(DROPDOWNS)>
                                    if caption_layout == CaptionLayout::DropdownYears {
                                        <span class=(CAPTION_LABEL)>(grid.first.month_name())</span>
                                    } else {
                                        caption_dropdown(
                                            month: month.clone(),
                                            label: "Choose the Month",
                                            options: month_options(grid.first, index, start, end),
                                        )
                                    }
                                    match (caption_layout, start, end) {
                                        (CaptionLayout::Dropdown | CaptionLayout::DropdownYears, Some(start), Some(end)) => {
                                            caption_dropdown(
                                                month: month.clone(),
                                                label: "Choose the Year",
                                                options: year_options(grid.first, index, start, end),
                                            )
                                        },
                                        _ => <span class=(CAPTION_LABEL)>(grid.first.year())</span>,
                                    }
                                </div>
                            } else {
                                <span class=(CAPTION_LABEL) role="status" aria-live="polite">
                                    (title.clone())
                                </span>
                            }
                        </div>
                        <table
                            role="grid"
                            class=(GRID)
                            aria-label=(title)
                            aria-multiselectable=(range.then_some("true"))
                            onkeydown=(ARROW_KEYS)
                        >
                            <thead aria-hidden="true">
                                <tr class=(WEEKDAYS)>
                                    for name in weekdays.iter() {
                                        <th scope="col" class=(WEEKDAY) aria-label=(*name)>(&name[..2])</th>
                                    }
                                </tr>
                            </thead>
                            <tbody (on_day_click(cx, &mode))>
                                #[key(week[0].iso.clone())]
                                for week in grid.weeks {
                                    <tr class=(WEEK)>
                                        #[key(day.iso.clone())]
                                        for day in week {
                                            <td
                                                role="gridcell"
                                                class=(class!(
                                                    DAY,
                                                    "rounded-base bg-main/50" if day.selected && !range,
                                                    "opacity-50" if day.outside || day.disabled,
                                                    "invisible" if day.hidden,
                                                ))
                                                data-day=(day.iso.clone())
                                                data-selected=(day.selected)
                                                data-outside=(day.outside)
                                                data-disabled=(day.disabled)
                                                data-hidden=(day.hidden)
                                                data-today=(day.today)
                                                aria-selected=(day.selected.then_some("true"))
                                            >
                                                if !day.hidden {
                                                    <button
                                                        type="button"
                                                        class=(class!(
                                                            button_variants(ButtonVariant::NoShadow, ButtonSize::Icon),
                                                            DAY_BUTTON,
                                                            // Selected days keep the variant's `bg-main`.
                                                            "bg-transparent! text-foreground!" if day.today && !day.selected,
                                                            "bg-secondary-background! text-foreground!" if !day.today && !day.selected,
                                                            RANGE_SEAMS if range && day.selected,
                                                        ))
                                                        data-day=(day.iso.clone())
                                                        data-selected-single=(day.selected && !range)
                                                        data-range-start=(day.range_start)
                                                        data-range-middle=(day.range_middle)
                                                        data-range-end=(day.range_end)
                                                        data-today=(day.today)
                                                        aria-label=(day.label())
                                                        value=(day.iso.clone())
                                                        disabled=(day.disabled)
                                                    >
                                                        (day.date.day())
                                                    </button>
                                                }
                                            </td>
                                        }
                                    </tr>
                                }
                            </tbody>
                        </table>
                    </div>
                }
            </div>
        </div>
    })
}

/// A caption dropdown: a native `<select>` laid invisibly over a label that
/// shows the current option and a chevron. Picking an option sets `month`.
#[component]
async fn caption_dropdown(
    month: Signal<String>,
    label: &str,
    options: Vec<DropdownOption>,
) -> Result<impl View> {
    let current = options
        .iter()
        .find(|(_, _, selected)| *selected)
        .map(|(_, text, _)| text.clone())
        .unwrap_or_default();
    Ok(view! {
        <span class=(DROPDOWN_ROOT)>
            <select
                class=(DROPDOWN)
                aria-label=(label)
                @change=$(|e: Event| month.set(e.target.value))
            >
                for (value, text, selected) in options {
                    <option value=(value) selected=(selected)>(text)</option>
                }
            </select>
            <span class=(DROPDOWN_LABEL) aria-hidden="true">
                (current)
                icon(data: iconify_icon!("lucide:chevron-down"))
            </span>
        </span>
    })
}
