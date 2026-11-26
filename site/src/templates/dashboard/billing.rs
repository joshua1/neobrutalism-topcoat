//! The billing page: plans, usage against the plan's limits, the payment
//! method and past invoices.

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    runtime::{Event, shard, signal},
    view::{View, attributes, class, component, view},
};

use crate::components::{
    badge::badge,
    button::{ButtonSize, ButtonVariant, button, button_variants},
    card::{card, card_action, card_content, card_description, card_header, card_title},
    progress::{progress, progress_label},
    table::{table, table_body, table_cell, table_head, table_header, table_row},
};

use super::{
    data::{
        CARD_EXPIRES, CARD_LAST4, INVOICES, Invoice, NEXT_INVOICE, PLANS, PRODUCTS, SEATS_USED,
        USER, format_money, format_number, orders_of, period, total,
    },
    shared::chart_card,
};

/// Percent-encodes `text` for a `data:` URL.
fn encode(text: &str) -> String {
    let mut encoded = String::new();
    for byte in text.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    encoded
}

/// An invoice as a one-row CSV file, in a `data:` URL the download link
/// points at.
fn invoice_csv(invoice: &Invoice) -> String {
    let csv = format!(
        "invoice,date,plan,total\n{},\"{}\",{},{}",
        invoice.id, invoice.date, invoice.plan, invoice.total,
    );
    format!("data:text/csv;charset=utf-8,{}", encode(&csv))
}

/// The plans and what the chosen one allows.
///
/// A shard: the chosen plan is a signal read here on the server, so switching
/// re-renders the plans, usage and next invoice.
#[shard]
async fn plans(cx: &Cx) -> Result<impl View> {
    let chosen = signal(cx, || "Growth".to_owned());
    // The browser sends the plan back: anything unknown is the first plan.
    let plan = PLANS
        .iter()
        .find(|plan| plan.name == chosen.get())
        .unwrap_or(&PLANS[0]);
    let orders_used = total(period(28).days, orders_of);
    let usage = [
        ("Orders, last 4 weeks", orders_used, plan.orders),
        ("Products", PRODUCTS.len() as f64, plan.products),
        ("Team seats", SEATS_USED, plan.seats),
    ];

    Ok(view! {
        <div class="grid gap-5 lg:grid-cols-3">
            #[key(item.name)]
            for item in &PLANS {
                let current = item.name == plan.name;
                let name = item.name;
                let variant = if current { ButtonVariant::Neutral } else { ButtonVariant::Default };
                card(attrs: attributes! { class="min-w-0 bg-secondary-background!" },
                    card_header(
                        card_title((item.name))
                        card_description((item.description))
                        if current {
                            card_action(badge(attrs: attributes! { class="bg-chart-3!" }, "Current"))
                        }
                    )
                    card_content(attrs: attributes! { class="flex min-w-0 flex-1 flex-col justify-center" },
                        <p class="text-sm">
                            <span class="font-heading text-3xl tabular-nums">(format_money(item.price))</span>
                            " per month"
                        </p>
                        <ul class="my-5 flex flex-1 flex-col gap-2 text-sm">
                            for feature in [
                                format!("{} orders per month", format_number(item.orders)),
                                format!("{} products", format_number(item.products)),
                                format!("{} team seats", item.seats),
                            ] {
                                <li class="flex items-center gap-2">
                                    icon(data: iconify_icon!("lucide:check"), attrs: attributes! { class="size-4 shrink-0" })
                                    (feature)
                                </li>
                            }
                        </ul>
                        button(
                            variant: variant,
                            attrs: attributes! {
                                // `!` beats the button's own `disabled:opacity-50`.
                                class="disabled:opacity-100! disabled:shadow-none"
                                disabled=(current)
                                @click=$(|_e: Event| chosen.set(name.to_owned()))
                            },
                            if current { "Your plan" } else { (format!("Switch to {}", item.name)) }
                        )
                    )
                )
            }
        </div>

        <div class="grid gap-5 xl:grid-cols-3">
            chart_card(
                title: "Usage",
                description: format!("Against your {} limits", plan.name),
                class: "xl:col-span-2",
                <div class="flex flex-col gap-5">
                    for (label, used, limit) in usage {
                        progress(
                            value: (used / limit * 100.0).min(100.0),
                            attrs: attributes! {
                                class=(if used > limit * 0.9 {
                                    "**:data-[slot=progress-indicator]:bg-chart-2"
                                } else {
                                    "**:data-[slot=progress-indicator]:bg-chart-4"
                                })
                                aria-label=(label)
                            },
                            progress_label((label))
                            <span class="ml-auto text-sm tabular-nums">
                                (format!("{} of {}", format_number(used), format_number(limit)))
                            </span>
                        )
                    }
                </div>
            )
            chart_card(title: "Payment method", description: "Charged every month".to_owned(),
                <div class="flex min-h-36 flex-1 flex-col justify-between rounded-base border-2 border-border bg-chart-3 p-4">
                    <div class="flex items-center justify-between gap-4">
                        <span class="font-heading">(USER.store)</span>
                        icon(data: iconify_icon!("lucide:credit-card"), attrs: attributes! { class="size-6 shrink-0" })
                    </div>
                    <p class="font-heading text-lg tracking-wider whitespace-nowrap tabular-nums">
                        <span aria-hidden="true">"**** **** **** "</span>
                        <span class="sr-only">"Card ending in "</span>
                        (CARD_LAST4)
                    </p>
                    <div class="flex items-center justify-between gap-4 text-sm">
                        <span>(USER.name)</span>
                        <span class="tabular-nums">(format!("Expires {CARD_EXPIRES}"))</span>
                    </div>
                </div>
                <p class="mt-4 flex items-center justify-between gap-4 text-sm" aria-live="polite">
                    <span>(format!("Next invoice on {NEXT_INVOICE}"))</span>
                    <span class="font-heading tabular-nums">(format_money(plan.price))</span>
                </p>
            )
        </div>
    })
}

#[component]
pub async fn billing_page(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        plans()

        chart_card(title: "Invoices", description: "Your past payments".to_owned(),
            table(attrs: attributes! { class="max-sm:[&_td]:px-2 max-sm:[&_th]:px-2" },
                table_header(
                    table_row(
                        table_head("Invoice")
                        table_head("Date")
                        table_head(attrs: attributes! { class="hidden sm:table-cell" }, "Plan")
                        table_head(attrs: attributes! { class="hidden sm:table-cell" }, "Status")
                        table_head(attrs: attributes! { class="text-right" }, "Total")
                        table_head(attrs: attributes! { class="w-12 sm:w-16" },
                            <span class="sr-only">"Download"</span>
                        )
                    )
                )
                table_body(
                    for invoice in &INVOICES {
                        table_row(attrs: attributes! { class="bg-secondary-background!" },
                            table_cell(attrs: attributes! { class="py-3 font-heading whitespace-nowrap" }, (invoice.id))
                            table_cell(attrs: attributes! { class="py-3 whitespace-nowrap" }, (invoice.date))
                            table_cell(attrs: attributes! { class="hidden py-3 sm:table-cell" }, (invoice.plan))
                            table_cell(attrs: attributes! { class="hidden py-3 sm:table-cell" },
                                badge(attrs: attributes! { class="bg-chart-4!" }, "Paid")
                            )
                            table_cell(attrs: attributes! { class="py-3 text-right font-heading tabular-nums" },
                                (format_money(invoice.total))
                            )
                            table_cell(attrs: attributes! { class="py-3" },
                                <a
                                    href=(invoice_csv(invoice))
                                    download=(format!("{}.csv", invoice.id))
                                    aria-label=(format!("Download {}", invoice.id))
                                    class=(class!(button_variants(ButtonVariant::NoShadow, ButtonSize::IconXs)))
                                >
                                    icon(data: iconify_icon!("lucide:download"))
                                </a>
                            )
                        )
                    }
                )
            )
        )
    })
}
