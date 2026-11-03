use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, class, component, view},
};

use crate::components::{
    button::{ButtonSize, ButtonVariant, button_variants},
    data_table::{
        Column, DataTable, data_table, data_table_column_toggle, data_table_filter,
        data_table_pagination, data_table_select_all, data_table_select_row, data_table_selection,
        data_table_sort_button,
    },
    dropdown_menu::{
        DropdownMenuAlign, dropdown_menu, dropdown_menu_content, dropdown_menu_item,
        dropdown_menu_label, dropdown_menu_separator, dropdown_menu_trigger,
    },
    table::{table, table_body, table_cell, table_head, table_header, table_row},
};

// `class!` appends without merging, so a class that replaces one the
// component already sets carries Tailwind's `!` (important) modifier.

/// The state of a payment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
enum Status {
    Pending,
    Processing,
    Success,
    Failed,
}

impl Status {
    fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Processing => "processing",
            Self::Success => "success",
            Self::Failed => "failed",
        }
    }
}

struct Payment {
    id: &'static str,
    amount: f64,
    status: Status,
    email: &'static str,
}

const PAYMENTS: [Payment; 5] = [
    Payment {
        id: "m5gr84i9",
        amount: 316.0,
        status: Status::Success,
        email: "ken99@yahoo.com",
    },
    Payment {
        id: "3u1reuv4",
        amount: 242.0,
        status: Status::Success,
        email: "Abe45@gmail.com",
    },
    Payment {
        id: "derv1ws0",
        amount: 837.0,
        status: Status::Processing,
        email: "Monserrat44@gmail.com",
    },
    Payment {
        id: "5kma53ae",
        amount: 874.0,
        status: Status::Success,
        email: "Silas22@gmail.com",
    },
    Payment {
        id: "bhqecj4p",
        amount: 721.0,
        status: Status::Failed,
        email: "carmella@hotmail.com",
    },
];

/// The hideable data columns. The selection and actions columns are markup
/// only, written into the table below.
const COLUMNS: [Column<Payment>; 3] = [
    Column::new("status", "Status", |payment: &Payment| {
        payment.status.as_str().to_owned()
    }),
    Column::new("email", "Email", |payment: &Payment| {
        payment.email.to_owned()
    })
    .sortable(|a, b| a.email.to_lowercase().cmp(&b.email.to_lowercase()))
    .filterable(),
    Column::new("amount", "Amount", |payment: &Payment| usd(payment.amount)),
];

/// Formats dollars the way `Intl.NumberFormat("en-US", { style: "currency",
/// currency: "USD" })` does: `$1,234.50`.
fn usd(amount: f64) -> String {
    let cents = (amount.abs() * 100.0).round() as u64;
    let dollars = (cents / 100).to_string();
    let mut grouped = String::new();
    for (index, digit) in dollars.chars().enumerate() {
        if index > 0 && (dollars.len() - index) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    let sign = if amount < 0.0 { "-" } else { "" };
    format!("{sign}${grouped}.{:02}", cents % 100)
}

/// Copies the payment id from the item's `data-payment-id`, then closes the
/// menu and returns focus to its trigger, as other items do.
const COPY_ID_JS: &str = "navigator.clipboard.writeText(this.dataset.paymentId);\
     const d=this.closest('[data-slot=dropdown-menu]');\
     if(d){d.open=false;d.querySelector('summary').focus()}";

#[component]
pub async fn demo(cx: &Cx) -> Result<impl View> {
    // Sorting, filtering, paging and hidden columns come from the URL, so
    // `?sort=-email&filter=gmail` opens the table sorted and filtered.
    let payments = DataTable::new(cx, &COLUMNS, &PAYMENTS);

    Ok(view! {
        // Bound inside the view, which owns `payments` once it is moved in.
        let state = payments.state();
        data_table(
            <div class="flex items-center py-4">
                data_table_filter(
                    state: state,
                    attrs: attributes! { placeholder="Filter emails..." class="max-w-sm" }
                )
                data_table_column_toggle(
                    state: state,
                    columns: &COLUMNS,
                    attrs: attributes! { class="ml-auto" }
                )
            </div>
            <div>
                table(
                    table_header(attrs: attributes! { class="font-heading" },
                        table_row(attrs: attributes! { class="bg-background text-foreground" },
                            table_head(attrs: attributes! { class="text-foreground" },
                                data_table_select_all()
                            )
                            if state.is_visible("status") {
                                table_head(attrs: attributes! { class="text-foreground" }, "Status")
                            }
                            if state.is_visible("email") {
                                table_head(
                                    attrs: attributes! {
                                        class="text-foreground"
                                        aria-sort=(state.aria_sort("email"))
                                    },
                                    data_table_sort_button(state: state, column: "email", "Email")
                                )
                            }
                            if state.is_visible("amount") {
                                table_head(attrs: attributes! { class="text-foreground" },
                                    <div class="text-right">"Amount"</div>
                                )
                            }
                            table_head(attrs: attributes! { class="text-foreground" })
                        )
                    )
                    table_body(
                        if payments.rows().is_empty() {
                            table_row(
                                table_cell(attrs: attributes! { colspan="5" class="h-24 text-center" },
                                    "No results."
                                )
                            )
                        } else {
                            #[key(payment.id)]
                            for &payment in payments.rows() {
                                table_row(
                                    attrs: attributes! {
                                        class="bg-background text-foreground \
                                            data-[state=selected]:bg-main \
                                            data-[state=selected]:text-main-foreground"
                                    },
                                    table_cell(attrs: attributes! { class="px-4 py-2" },
                                        data_table_select_row(attrs: attributes! {
                                            name="payment"
                                            value=(payment.id)
                                        })
                                    )
                                    if state.is_visible("status") {
                                        table_cell(attrs: attributes! { class="px-4 py-2" },
                                            <div class="capitalize">(payment.status.as_str())</div>
                                        )
                                    }
                                    if state.is_visible("email") {
                                        table_cell(attrs: attributes! { class="px-4 py-2" },
                                            <div class="lowercase">(payment.email)</div>
                                        )
                                    }
                                    if state.is_visible("amount") {
                                        table_cell(attrs: attributes! { class="px-4 py-2" },
                                            <div class="text-right font-base">(usd(payment.amount))</div>
                                        )
                                    }
                                    table_cell(attrs: attributes! { class="px-4 py-2" },
                                        dropdown_menu(
                                            dropdown_menu_trigger(
                                                attrs: attributes! {
                                                    class=(class!(
                                                        button_variants(
                                                            ButtonVariant::NoShadow,
                                                            ButtonSize::Default,
                                                        ),
                                                        "size-8! bg-secondary-background! p-0! \
                                                         text-foreground!",
                                                    ))
                                                },
                                                <span class="sr-only">"Open menu"</span>
                                                icon(data: iconify_icon!("lucide:ellipsis"))
                                            )
                                            dropdown_menu_content(align: DropdownMenuAlign::End,
                                                dropdown_menu_label("Actions")
                                                dropdown_menu_item(
                                                    close_on_click: false,
                                                    attrs: attributes! {
                                                        data-payment-id=(payment.id)
                                                        onclick=(COPY_ID_JS)
                                                    },
                                                    "Copy payment ID"
                                                )
                                                dropdown_menu_separator()
                                                dropdown_menu_item("View customer")
                                                dropdown_menu_item("View payment details")
                                            )
                                        )
                                    )
                                )
                            }
                        }
                    )
                )
            </div>
            <div class="flex items-center justify-end space-x-2 py-4">
                data_table_selection(total: payments.filtered_count())
                data_table_pagination(state: state, page_count: payments.page_count())
            </div>
        )
    })
}
