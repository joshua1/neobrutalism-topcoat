//! "Tally", a store analytics dashboard: a collapsible sidebar and eight
//! pages (overview, revenue, marketing, orders, customers, inventory, profile
//! and billing) with date ranges, charts, heatmaps and searchable tables.
//! A port of neobrutalism.dev's dashboard template.
//!
//! Each page has its own URL under [`BASE`]: the overview at `BASE`, the
//! others at `BASE/{slug}` (see [`Page::from_slug`]). The date range is the
//! `range` query parameter (`?range=7`, `28` or `84`), which the sidebar's
//! links keep; the header's select changes it, and the export link downloads
//! that range as CSV from a route the app adds at `BASE/export.csv`:
//!
//! ```ignore
//! // src/app/dashboard.rs: GET /dashboard
//! #[page]
//! async fn dashboard(cx: &Cx) -> Result<impl View> {
//!     Ok(view! { templates::dashboard::dashboard(page: Page::Overview) })
//! }
//!
//! #[route(GET "./export.csv")]
//! async fn export(cx: &Cx) -> Result<impl IntoResponse> {
//!     Ok(templates::dashboard::export_csv(cx))
//! }
//! ```
//!
//! The latest orders (orders page) and the plans (billing page) are shards,
//! so register them with the router's `.discover()`.

mod app_sidebar;
mod billing;
mod charts;
mod customers;
mod data;
mod inventory;
mod marketing;
mod order_table;
mod orders;
mod overview;
mod profile;
mod revenue;
mod shared;

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    router::request,
    view::{View, attributes, component, view},
};

use crate::components::{
    button::{ButtonSize, ButtonVariant, button_variants},
    select::{select, select_content, select_group, select_item},
    sidebar::{sidebar_inset, sidebar_provider, sidebar_state, sidebar_trigger},
};

use self::{
    app_sidebar::dashboard_sidebar,
    billing::billing_page,
    customers::customers_page,
    data::{DEFAULT_RANGE, RANGES, period, report_csv},
    inventory::inventory_page,
    marketing::marketing_page,
    orders::orders_page,
    overview::overview_page,
    profile::profile_page,
    revenue::revenue_page,
};

/// Where the dashboard is mounted. Change it to the path of the page that
/// renders [`dashboard`].
pub const BASE: &str = "/templates/dashboard";

/// A page of the dashboard.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    Overview,
    Revenue,
    Marketing,
    Orders,
    Customers,
    Inventory,
    Profile,
    Billing,
}

impl Page {
    /// The pages served at `BASE/{slug}`, every one but the overview.
    const SUBPAGES: [Self; 7] = [
        Self::Revenue,
        Self::Marketing,
        Self::Orders,
        Self::Customers,
        Self::Inventory,
        Self::Profile,
        Self::Billing,
    ];

    /// The page's URL segment.
    pub fn slug(self) -> &'static str {
        match self {
            Self::Overview => "overview",
            Self::Revenue => "revenue",
            Self::Marketing => "marketing",
            Self::Orders => "orders",
            Self::Customers => "customers",
            Self::Inventory => "inventory",
            Self::Profile => "profile",
            Self::Billing => "billing",
        }
    }

    /// The page's title, in the header and the sidebar.
    pub fn title(self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::Revenue => "Revenue",
            Self::Marketing => "Marketing",
            Self::Orders => "Orders",
            Self::Customers => "Customers",
            Self::Inventory => "Inventory",
            Self::Profile => "Profile",
            Self::Billing => "Billing",
        }
    }

    /// The page served at `BASE/{slug}`, or `None` for an unknown slug (the
    /// overview is `BASE` itself).
    pub fn from_slug(slug: &str) -> Option<Self> {
        Self::SUBPAGES.into_iter().find(|page| page.slug() == slug)
    }

    /// Account pages have no date range or export.
    fn is_account(self) -> bool {
        matches!(self, Self::Profile | Self::Billing)
    }

    /// The page's path, without a query.
    fn path(self) -> String {
        match self {
            Self::Overview => BASE.to_owned(),
            _ => format!("{BASE}/{}", self.slug()),
        }
    }

    /// A link to the page that keeps the date range.
    pub fn href(self, range: usize) -> String {
        if range == DEFAULT_RANGE {
            self.path()
        } else {
            format!("{}?range={range}", self.path())
        }
    }
}

/// The date range of the request, in days: its `range` query parameter when
/// that is one of [`RANGES`], otherwise the default.
pub fn range(cx: &Cx) -> usize {
    request::uri(cx)
        .query()
        .and_then(|query| {
            query
                .split('&')
                .find_map(|pair| pair.strip_prefix("range="))
        })
        .and_then(|value| RANGES.iter().find(|range| range.value == value))
        .map_or(DEFAULT_RANGE, |range| range.days)
}

/// The response of the export link: the request's range as a CSV download.
pub fn export_csv(cx: &Cx) -> ([(&'static str, &'static str); 2], String) {
    (
        [
            ("content-type", "text/csv; charset=utf-8"),
            (
                "content-disposition",
                "attachment; filename=\"tally-report.csv\"",
            ),
        ],
        report_csv(&period(range(cx))),
    )
}

/// The whole dashboard showing `page`: sidebar, header and content.
#[component]
pub async fn dashboard(cx: &Cx, page: Page) -> Result<impl View> {
    let range = range(cx);
    let period = period(range);
    // The sidebar renders in the state the user last left it in.
    let open = sidebar_state(cx).unwrap_or(true);
    let action = page.path();
    let export = format!("{BASE}/export.csv?range={range}");

    Ok(view! {
        // Collapsed, the sidebar is as wide as the header is tall (h-16).
        sidebar_provider(
            attrs: attributes! { class="font-base text-foreground" style="--sidebar-width-icon: 4rem" },
            dashboard_sidebar(active: page, range: range, open: open)
            // `!` replaces the inset's own background.
            sidebar_inset(attrs: attributes! { class="min-w-0 bg-background!" },
                <header class="sticky top-0 z-10 flex h-16 shrink-0 items-center gap-2 border-b-2 border-border bg-secondary-background px-4 sm:gap-3 sm:px-6">
                    // `!` replaces the trigger's own `size-7!`.
                    sidebar_trigger(attrs: attributes! { class="size-9! shrink-0" })
                    <h1 class="truncate font-heading text-lg sm:text-xl">(page.title())</h1>
                    if !page.is_account() {
                        <div class="ml-auto flex items-center gap-2 sm:gap-3">
                            // Choosing a range reloads the page with it.
                            <form method="get" action=(action)>
                                select(attrs: attributes! {
                                    name="range"
                                    aria-label="Date range"
                                    onchange="this.form.requestSubmit()"
                                    class="w-37 [&>select]:h-9 [&>select]:bg-secondary-background"
                                },
                                    select_content(
                                        select_group(
                                            for option in RANGES {
                                                select_item(
                                                    attrs: attributes! {
                                                        value=(option.value)
                                                        selected=(option.days == range)
                                                    },
                                                    (option.label)
                                                )
                                            }
                                        )
                                    )
                                )
                            </form>
                            <a
                                href=(export)
                                download="tally-report.csv"
                                aria-label="Export CSV"
                                class=(button_variants(ButtonVariant::NoShadow, ButtonSize::Sm))
                            >
                                <span class="hidden sm:inline">"Export"</span>
                                icon(data: iconify_icon!("lucide:download"))
                            </a>
                        </div>
                    }
                </header>
                <div class="flex flex-col gap-5 p-4 pb-28 sm:p-6 sm:pb-28">
                    match page {
                        Page::Overview => overview_page(period: &period, range: range),
                        Page::Revenue => revenue_page(period: &period),
                        Page::Marketing => marketing_page(period: &period),
                        Page::Orders => orders_page(period: &period),
                        Page::Customers => customers_page(period: &period),
                        Page::Inventory => inventory_page(period: &period),
                        Page::Profile => profile_page(),
                        Page::Billing => billing_page(),
                    }
                </div>
            )
        )
    })
}
