use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{StaticClass, View, attributes, class, component, view},
};

use crate::components::{
    button::{ButtonSize, ButtonVariant, button_variants},
    table::{
        table, table_body, table_caption, table_cell, table_footer, table_head, table_header,
        table_row,
    },
};

/// One row of the invoice examples.
struct Invoice {
    invoice: &'static str,
    payment_status: &'static str,
    total_amount: &'static str,
    payment_method: &'static str,
}

const INVOICES: [Invoice; 7] = [
    Invoice {
        invoice: "INV001",
        payment_status: "Paid",
        total_amount: "$250.00",
        payment_method: "Credit Card",
    },
    Invoice {
        invoice: "INV002",
        payment_status: "Pending",
        total_amount: "$150.00",
        payment_method: "PayPal",
    },
    Invoice {
        invoice: "INV003",
        payment_status: "Unpaid",
        total_amount: "$350.00",
        payment_method: "Bank Transfer",
    },
    Invoice {
        invoice: "INV004",
        payment_status: "Paid",
        total_amount: "$450.00",
        payment_method: "Credit Card",
    },
    Invoice {
        invoice: "INV005",
        payment_status: "Paid",
        total_amount: "$550.00",
        payment_method: "PayPal",
    },
    Invoice {
        invoice: "INV006",
        payment_status: "Pending",
        total_amount: "$200.00",
        payment_method: "Bank Transfer",
    },
    Invoice {
        invoice: "INV007",
        payment_status: "Unpaid",
        total_amount: "$300.00",
        payment_method: "Credit Card",
    },
];

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        table(
            table_caption(
                attrs: attributes! { class="text-foreground" },
                "A list of your recent invoices."
            )
            table_header(
                table_row(
                    table_head(attrs: attributes! { class="w-[100px]" }, "Invoice")
                    table_head("Status")
                    table_head("Method")
                    table_head(attrs: attributes! { class="text-right" }, "Amount")
                )
            )
            table_body(
                #[key(invoice.invoice)]
                for invoice in &INVOICES {
                    table_row(
                        table_cell(attrs: attributes! { class="font-base" }, (invoice.invoice))
                        table_cell((invoice.payment_status))
                        table_cell((invoice.payment_method))
                        table_cell(attrs: attributes! { class="text-right" }, (invoice.total_amount))
                    )
                }
            )
            table_footer(
                table_row(
                    table_cell(attrs: attributes! { colspan="3" }, "Total")
                    table_cell(attrs: attributes! { class="text-right" }, "$2,500.00")
                )
            )
        )
    })
}

#[component]
pub async fn footer() -> Result<impl View> {
    Ok(view! {
        table(
            table_caption("A list of your recent invoices.")
            table_header(
                table_row(
                    table_head(attrs: attributes! { class="w-[100px]" }, "Invoice")
                    table_head("Status")
                    table_head("Method")
                    table_head(attrs: attributes! { class="text-right" }, "Amount")
                )
            )
            table_body(
                #[key(invoice.invoice)]
                for invoice in INVOICES.iter().take(3) {
                    table_row(
                        table_cell(attrs: attributes! { class="font-heading" }, (invoice.invoice))
                        table_cell((invoice.payment_status))
                        table_cell((invoice.payment_method))
                        table_cell(attrs: attributes! { class="text-right" }, (invoice.total_amount))
                    )
                }
            )
            table_footer(
                table_row(
                    table_cell(attrs: attributes! { colspan="3" }, "Total")
                    table_cell(attrs: attributes! { class="text-right" }, "$2,500.00")
                )
            )
        )
    })
}

/// The products in the actions example: name, price and a slug for ids.
const PRODUCTS: [(&str, &str, &str); 3] = [
    ("Wireless Mouse", "$29.99", "wireless-mouse"),
    ("Mechanical Keyboard", "$129.99", "mechanical-keyboard"),
    ("USB-C Hub", "$49.99", "usb-c-hub"),
];

// A stand-in for the `dropdown-menu` component: a native popover anchored
// under its trigger's end edge, using that component's classes.
const MENU: StaticClass = class!(
    "z-50 min-w-[8rem] overflow-hidden rounded-base border-2 border-border bg-background p-1 \
     text-left font-base text-foreground outline-none inset-auto m-0 mt-1 \
     [position-area:bottom_span-left]",
);
const MENU_ITEM: StaticClass = class!(
    "relative flex w-full cursor-default select-none items-center gap-2 rounded-base border-2 \
     border-transparent px-2 py-1.5 text-sm font-base outline-hidden transition-colors \
     hover:bg-main hover:text-main-foreground hover:border-border focus:bg-main \
     focus:text-main-foreground focus:border-border",
);

#[component]
pub async fn actions() -> Result<impl View> {
    // `size-8!` and `text-foreground!` beat the button's own size and color:
    // `class!` does not drop conflicting classes the way Octane's `cn` does.
    Ok(view! {
        table(
            table_header(
                table_row(
                    table_head("Product")
                    table_head("Price")
                    table_head(attrs: attributes! { class="text-right" }, "Actions")
                )
            )
            table_body(
                #[key(slug)]
                for (name, price, slug) in PRODUCTS {
                    let menu_id = format!("table-actions-{slug}");
                    let anchor = format!("--table-actions-{slug}");
                    table_row(
                        table_cell(attrs: attributes! { class="font-heading" }, (name))
                        table_cell((price))
                        table_cell(
                            attrs: attributes! { class="text-right" },
                            <button
                                type="button"
                                popovertarget=(menu_id.clone())
                                style=(format!("anchor-name: {anchor}"))
                                class=(class!(
                                    button_variants(ButtonVariant::NoShadow, ButtonSize::Icon),
                                    "size-8! bg-secondary-background! text-foreground!",
                                ))
                            >
                                icon(data: iconify_icon!("lucide:ellipsis"))
                                <span class="sr-only">"Open menu"</span>
                            </button>
                            <div
                                id=(menu_id.clone())
                                popover=""
                                role="menu"
                                style=(format!("position-anchor: {anchor}"))
                                class=(MENU)
                            >
                                for item in ["Edit", "Duplicate"] {
                                    <button
                                        type="button"
                                        role="menuitem"
                                        popovertarget=(menu_id.clone())
                                        popovertargetaction="hide"
                                        class=(MENU_ITEM)
                                    >
                                        (item)
                                    </button>
                                }
                                <div role="separator" class="-mx-1 my-1 h-0.5 bg-border"></div>
                                <button
                                    type="button"
                                    role="menuitem"
                                    popovertarget=(menu_id.clone())
                                    popovertargetaction="hide"
                                    class=(MENU_ITEM)
                                >
                                    "Delete"
                                </button>
                            </div>
                        )
                    )
                }
            )
        )
    })
}
