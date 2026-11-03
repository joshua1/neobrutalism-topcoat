//! Each calendar lives in a `#[shard]`: picking a day or changing the month
//! sets a signal in the browser, and the shard re-renders the month grid on
//! the server.

use topcoat::{
    Result,
    context::Cx,
    runtime::{shard, signal},
    view::{View, component, view},
};

use crate::components::calendar::{CalendarMode, Date, calendar};

/// The single-day calendar, starting on today.
#[shard]
async fn single_calendar(cx: &Cx) -> Result<impl View> {
    let today = Date::today();
    let month = signal(cx, || today.month_key());
    let selected = signal(cx, || Some(today.to_string()));

    Ok(view! {
        calendar(month: month, mode: CalendarMode::Single(selected))
    })
}

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! { single_calendar() })
}

/// The range calendar: January 12th of this year and the 30 days after it.
#[shard]
async fn range_calendar(cx: &Cx) -> Result<impl View> {
    let start = Date::new(Date::today().year(), 1, 12);
    let month = signal(cx, || start.month_key());
    let from = signal(cx, || Some(start.to_string()));
    let to = signal(cx, || Some(start.add_days(30).to_string()));

    Ok(view! {
        calendar(
            month: month,
            mode: CalendarMode::Range { from, to },
            number_of_months: 2,
            // Days after today, and before 1900, can't be picked.
            disabled: Some(|day| day > Date::today() || day < Date::new(1900, 1, 1)),
        )
    })
}

#[component]
pub async fn range() -> Result<impl View> {
    Ok(view! { range_calendar() })
}
