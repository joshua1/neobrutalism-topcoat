//! A date picker is a [`popover`] holding a [`calendar`]; there is no
//! component of its own.
//!
//! Each picker is a `#[shard]`: picking a day sets a signal in the browser,
//! and the shard re-renders both the calendar and the trigger's label on the
//! server. The re-render updates the open popover in place, so it stays open
//! until it is dismissed, or until the inline `onclick` on the panel closes it
//! after a pick (as Octane's `setOpen(false)` does).

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    runtime::{Event, shard, signal},
    view::{View, attributes, class, component, view},
};

use crate::components::{
    button::{ButtonSize, ButtonVariant, button_variants},
    calendar::{CalendarMode, CaptionLayout, Date, calendar},
    field::{field, field_group, field_label},
    input_group::{
        InputGroupAddonAlign, InputGroupButtonSize, input_group, input_group_addon,
        input_group_button, input_group_input,
    },
    popover::{PopoverAlign, popover, popover_anchor, popover_content, popover_trigger},
};

/// Inline `onclick` for a picker's panel that closes it once a day is picked.
const CLOSE_ON_PICK: &str = "if(event.target.closest('button[data-day]'))this.hidePopover()";

/// Parses a `YYYY-MM-DD` signal value, ignoring anything invalid.
fn picked(value: Option<String>) -> Option<Date> {
    value?.parse().ok()
}

/// The date as `Jan 20, 2025` (date-fns' `LLL dd, y`).
fn short_date(date: Date) -> String {
    format!(
        "{} {:02}, {}",
        &date.month_name()[..3],
        date.day(),
        date.year()
    )
}

#[shard]
async fn default_picker(cx: &Cx) -> Result<impl View> {
    let month = signal(cx, || Date::today().month_key());
    let selected = signal(cx, || None::<String>);
    let date = picked(selected.get());

    // `justify-start!` beats the button's `justify-center`.
    Ok(view! {
        popover(
            popover_trigger(
                target: "date-picker-default",
                attrs: attributes! {
                    class=(class!(
                        button_variants(ButtonVariant::NoShadow, ButtonSize::Default),
                        "w-[280px] justify-start! text-left font-base",
                    ))
                },
                icon(data: iconify_icon!("lucide:calendar"))
                match date {
                    Some(date) => (date.format_long()),
                    None => <span>"Pick a date"</span>,
                }
            )
            popover_content(id: "date-picker-default", attrs: attributes! { class="w-auto! border-0! p-0!" },
                calendar(month: month, mode: CalendarMode::Single(selected))
            )
        )
    })
}

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! { default_picker() })
}

#[shard]
async fn basic_picker(cx: &Cx) -> Result<impl View> {
    let month = signal(cx, || Date::today().month_key());
    let selected = signal(cx, || None::<String>);
    let date = picked(selected.get());

    // `w-44!` beats the field's `w-full`.
    Ok(view! {
        field(attrs: attributes! { class="mx-auto w-44!" },
            field_label(attrs: attributes! { for="date-picker-simple" }, "Date")
            popover(
                popover_trigger(
                    target: "date-picker-basic",
                    attrs: attributes! {
                        id="date-picker-simple"
                        class=(class!(
                            button_variants(ButtonVariant::NoShadow, ButtonSize::Default),
                            "justify-start! font-base",
                        ))
                    },
                    match date {
                        Some(date) => (date.format_long()),
                        None => <span>"Pick a date"</span>,
                    }
                )
                popover_content(
                    id: "date-picker-basic",
                    align: PopoverAlign::Start,
                    attrs: attributes! { class="w-auto! border-0! p-0!" },
                    calendar(month: month, mode: CalendarMode::Single(selected))
                )
            )
        )
    })
}

#[component]
pub async fn basic() -> Result<impl View> {
    Ok(view! { basic_picker() })
}

#[shard]
async fn range_picker_shard(cx: &Cx) -> Result<impl View> {
    let start = Date::new(Date::today().year(), 1, 20);
    let month = signal(cx, || start.month_key());
    let from = signal(cx, || Some(start.to_string()));
    let to = signal(cx, || Some(start.add_days(20).to_string()));
    let range = (picked(from.get()), picked(to.get()));

    Ok(view! {
        field(attrs: attributes! { class="mx-auto w-60!" },
            field_label(attrs: attributes! { for="date-picker-range" }, "Date Picker Range")
            popover(
                popover_trigger(
                    target: "date-picker-range-panel",
                    attrs: attributes! {
                        id="date-picker-range"
                        class=(class!(
                            button_variants(ButtonVariant::NoShadow, ButtonSize::Default),
                            "justify-start! px-2.5! font-base",
                        ))
                    },
                    icon(data: iconify_icon!("lucide:calendar"))
                    match range {
                        (Some(from), Some(to)) => (format!("{} - {}", short_date(from), short_date(to))),
                        (Some(from), None) => (short_date(from)),
                        _ => <span>"Pick a date"</span>,
                    }
                )
                popover_content(
                    id: "date-picker-range-panel",
                    align: PopoverAlign::Start,
                    attrs: attributes! { class="w-auto! border-0! p-0!" },
                    calendar(
                        month: month,
                        mode: CalendarMode::Range { from, to },
                        number_of_months: 2,
                    )
                )
            )
        )
    })
}

#[component]
pub async fn range_picker() -> Result<impl View> {
    Ok(view! { range_picker_shard() })
}

#[shard]
async fn date_of_birth_picker(cx: &Cx) -> Result<impl View> {
    let month = signal(cx, || Date::today().month_key());
    let selected = signal(cx, || None::<String>);
    let label = match picked(selected.get()) {
        // `toLocaleDateString()` in en-US: month/day/year.
        Some(date) => format!("{}/{}/{}", date.month(), date.day(), date.year()),
        None => "Select date".to_owned(),
    };

    Ok(view! {
        field(attrs: attributes! { class="mx-auto w-44!" },
            field_label(attrs: attributes! { for="date-picker-dob" }, "Date of birth")
            popover(
                popover_trigger(
                    target: "date-picker-dob-panel",
                    attrs: attributes! {
                        id="date-picker-dob"
                        class=(class!(
                            button_variants(ButtonVariant::NoShadow, ButtonSize::Default),
                            "justify-start! font-base",
                        ))
                    },
                    (label)
                )
                popover_content(
                    id: "date-picker-dob-panel",
                    align: PopoverAlign::Start,
                    attrs: attributes! {
                        class="w-auto! overflow-hidden border-0! p-0!"
                        onclick=(CLOSE_ON_PICK)
                    },
                    calendar(
                        month: month,
                        mode: CalendarMode::Single(selected),
                        caption_layout: CaptionLayout::Dropdown,
                    )
                )
            )
        )
    })
}

#[component]
pub async fn date_of_birth() -> Result<impl View> {
    Ok(view! { date_of_birth_picker() })
}

/// Reads a typed date, as JavaScript's `new Date(text)` would for the
/// formats people type: `June 01, 2025` (or `Jun 1, 2025`), `2025-06-01`
/// and `6/1/2025`. Each format is turned into `YYYY-MM-DD` and parsed with
/// `Date`'s `FromStr`, which rejects days that don't exist.
fn parse_typed(text: &str) -> Option<Date> {
    let text = text.trim();
    // `6/1/2025`: month, day, year.
    let numbers: Vec<&str> = text.split('/').collect();
    let (year, month, day): (i32, u32, u32) = if let [month, day, year] = numbers[..] {
        (year.parse().ok()?, month.parse().ok()?, day.parse().ok()?)
    } else if let Ok(date) = text.parse::<Date>() {
        return Some(date);
    } else {
        // `June 01, 2025`: a month name (or its first three letters), day, year.
        let mut words = text.split([' ', ',']).filter(|word| !word.is_empty());
        let name = words.next()?.to_lowercase();
        let day = words.next()?.parse().ok()?;
        let year = words.next()?.parse().ok()?;
        let month = (1..=12).find(|&month| {
            let full = Date::new(2000, month, 1).month_name().to_lowercase();
            name.len() >= 3 && full.starts_with(&name)
        })?;
        (year, month, day)
    };
    format!("{year:04}-{month:02}-{day:02}").parse().ok()
}

/// The date as `June 01, 2025`, the input's format.
fn input_date(date: Date) -> String {
    format!("{} {:02}, {}", date.month_name(), date.day(), date.year())
}

/// Inline `onkeydown` for the input: ArrowDown opens the calendar.
const OPEN_ON_ARROW_DOWN: &str = "if(event.key==='ArrowDown'){event.preventDefault();\
     document.getElementById('date-picker-input-panel').showPopover()}";

/// Inline `onclick` for the panel: after a pick, it writes the day into the
/// input (firing its `input` event, which updates `text`) and closes.
const FILL_INPUT: &str = "const b=event.target.closest('button[data-day]');if(!b)return;\
     const i=document.getElementById('date-required');\
     i.value=new Date(b.value+'T00:00').toLocaleDateString('en-US',\
     {day:'2-digit',month:'long',year:'numeric'});\
     i.dispatchEvent(new Event('input',{bubbles:true}));this.hidePopover()";

#[shard]
async fn input_picker(cx: &Cx) -> Result<impl View> {
    let initial = Date::new(2025, 6, 1);
    let text = signal(cx, || input_date(initial));
    let typed = parse_typed(&text.get());

    // The typed text is the source of truth. Each valid date gets calendar
    // signals of its own (keyed by the date), which start on that date, so
    // typing one moves the calendar to it. Text that is not a date yet gets
    // a fresh, empty calendar.
    let calendar_cx = cx.keyed(typed.map(|date| date.to_string()).unwrap_or_default());
    let month = signal(&calendar_cx, || typed.unwrap_or(initial).month_key());
    let selected = signal(&calendar_cx, || typed.map(|date| date.to_string()));

    Ok(view! {
        field(attrs: attributes! { class="mx-auto w-48!" },
            field_label(attrs: attributes! { for="date-required" }, "Subscription Date")
            input_group(
                input_group_input(attrs: attributes! {
                    id="date-required"
                    placeholder="June 01, 2025"
                    :value=$(text.get())
                    @input=$(|e: Event| text.set(e.target.value))
                    onkeydown=(OPEN_ON_ARROW_DOWN)
                })
                input_group_addon(align: InputGroupAddonAlign::InlineEnd,
                    input_group_button(
                        size: InputGroupButtonSize::IconXs,
                        attrs: attributes! {
                            id="date-picker"
                            aria-label="Select date"
                            popovertarget="date-picker-input-panel"
                            style=(popover_anchor("date-picker-input-panel"))
                        },
                        icon(data: iconify_icon!("lucide:calendar"))
                        <span class="sr-only">"Select date"</span>
                    )
                )
            )
            // Outside the input group, so the addon's click-to-focus doesn't
            // reach the panel. Octane's `sideOffset={10}` is the `mt-2.5!`.
            popover_content(
                id: "date-picker-input-panel",
                align: PopoverAlign::End,
                attrs: attributes! {
                    class="w-auto! overflow-hidden border-0! p-0! mt-2.5!"
                    onclick=(FILL_INPUT)
                },
                calendar(month: month, mode: CalendarMode::Single(selected))
            )
        )
    })
}

#[component]
pub async fn input() -> Result<impl View> {
    Ok(view! { input_picker() })
}

#[shard]
async fn time_picker_shard(cx: &Cx) -> Result<impl View> {
    let month = signal(cx, || Date::today().month_key());
    let selected = signal(cx, || None::<String>);
    let label = picked(selected.get()).map_or("Select date".to_owned(), Date::format_long);

    // `flex-row!` beats the group's `flex-col`, and `justify-between!` the
    // button's `justify-center`. `input` is called by its full path because
    // this module's `input` example shadows it.
    Ok(view! {
        field_group(attrs: attributes! { class="mx-auto max-w-xs flex-row!" },
            field(
                field_label(attrs: attributes! { for="date-picker-optional" }, "Date")
                popover(
                    popover_trigger(
                        target: "date-picker-time-panel",
                        attrs: attributes! {
                            id="date-picker-optional"
                            class=(class!(
                                button_variants(ButtonVariant::NoShadow, ButtonSize::Default),
                                "w-32 justify-between! font-base",
                            ))
                        },
                        (label)
                        icon(data: iconify_icon!("lucide:chevron-down"))
                    )
                    popover_content(
                        id: "date-picker-time-panel",
                        align: PopoverAlign::Start,
                        attrs: attributes! {
                            class="w-auto! overflow-hidden border-0! p-0!"
                            onclick=(CLOSE_ON_PICK)
                        },
                        calendar(
                            month: month,
                            mode: CalendarMode::Single(selected),
                            caption_layout: CaptionLayout::Dropdown,
                        )
                    )
                )
            )
            field(attrs: attributes! { class="w-32!" },
                field_label(attrs: attributes! { for="time-picker-optional" }, "Time")
                crate::components::input::input(attrs: attributes! {
                    type="time"
                    id="time-picker-optional"
                    step="1"
                    value="10:30:00"
                    class="appearance-none [&::-webkit-calendar-picker-indicator]:hidden \
                        [&::-webkit-calendar-picker-indicator]:appearance-none"
                })
            )
        )
    })
}

#[component]
pub async fn time_picker() -> Result<impl View> {
    Ok(view! { time_picker_shard() })
}
