// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use std::cmp::Ordering;

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    router::{query_params, request::uri},
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

use super::{
    button::{ButtonSize, ButtonVariant, button, button_variants},
    checkbox::checkbox,
    dropdown_menu::{
        DropdownMenuAlign, dropdown_menu, dropdown_menu_content, dropdown_menu_trigger,
    },
    input::input,
};

/// One column of a data table: its key, its header, and how to read, sort
/// and filter its value. TanStack Table's column definition.
///
/// Columns are plain data, so a table's columns can be a `const` array.
/// `value` and the sort function are function pointers: pass closures that
/// capture nothing, such as `|payment: &Payment| payment.email.to_owned()`
/// (the type is written out because the builder methods after `new` keep
/// Rust from inferring it).
///
/// ```ignore
/// const COLUMNS: [Column<Payment>; 2] = [
///     Column::new("status", "Status", |payment: &Payment| payment.status.to_owned()),
///     Column::new("email", "Email", |payment: &Payment| payment.email.to_owned())
///         .sortable(|a, b| a.email.to_lowercase().cmp(&b.email.to_lowercase()))
///         .filterable(),
/// ];
/// ```
pub struct Column<T> {
    /// The column's key in the URL (`?sort=email`, `?hide=email`).
    pub id: &'static str,
    /// The header text, also the label of the column's entry in
    /// [`data_table_column_toggle`].
    pub header: &'static str,
    /// The cell's value as text, which the filter searches.
    pub value: fn(&T) -> String,
    /// Orders two rows by this column, ascending. `None` when the column
    /// cannot be sorted.
    pub sort: Option<fn(&T, &T) -> Ordering>,
    /// Whether the `filter` text searches this column.
    pub filterable: bool,
    /// Whether [`data_table_column_toggle`] lists the column, so it can be
    /// hidden.
    pub hideable: bool,
}

#[allow(dead_code)]
impl<T> Column<T> {
    /// A column read by `value`, hideable, and neither sortable nor
    /// filterable.
    #[must_use]
    pub const fn new(id: &'static str, header: &'static str, value: fn(&T) -> String) -> Self {
        Self {
            id,
            header,
            value,
            sort: None,
            filterable: false,
            hideable: true,
        }
    }

    /// Makes the column sortable, ordered ascending by `compare`.
    #[must_use]
    pub const fn sortable(mut self, compare: fn(&T, &T) -> Ordering) -> Self {
        self.sort = Some(compare);
        self
    }

    /// Makes the `filter` text search this column.
    #[must_use]
    pub const fn filterable(mut self) -> Self {
        self.filterable = true;
        self
    }

    /// Sets whether the column can be hidden.
    #[must_use]
    pub const fn hideable(mut self, hideable: bool) -> Self {
        self.hideable = hideable;
        self
    }
}

/// The direction a column is sorted in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SortDirection {
    /// Smallest first.
    Asc,
    /// Largest first.
    Desc,
}

impl SortDirection {
    /// Applies the direction to an ascending comparison.
    #[must_use]
    pub fn apply(self, ordering: Ordering) -> Ordering {
        match self {
            Self::Asc => ordering,
            Self::Desc => ordering.reverse(),
        }
    }

    /// The `aria-sort` value of a header sorted in this direction.
    #[must_use]
    pub const fn aria_sort(self) -> &'static str {
        match self {
            Self::Asc => "ascending",
            Self::Desc => "descending",
        }
    }
}

/// The query parameters a data table reads. All text, so that one bad value
/// (such as `page=abc`) is ignored on its own instead of failing the whole
/// query.
#[query_params]
struct DataTableQuery {
    sort: Option<String>,
    filter: Option<String>,
    page: Option<String>,
    hide: Option<String>,
}

/// What the URL asks a data table to show, and the links that change it.
///
/// The table keeps its state in the query string: `sort=email` (or
/// `sort=-email` for descending), `filter=text`, `page=2` (from 1) and
/// `hide=status,amount`. Every control is a link or a `GET` form that loads
/// the page again with a changed query, so sorting, filtering, paging and
/// column visibility work without JavaScript, survive a reload and can be
/// shared. One table per page reads these keys.
///
/// [`DataTable`] applies the state to rows in memory. For data in a
/// database, read the state with [`DataTableState::from_query`] and pass
/// `sort`, `filter` and `page` to the query instead.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DataTableState {
    /// The path the links point at: the current page.
    pub path: String,
    /// The sorted column's id and direction.
    pub sort: Option<(String, SortDirection)>,
    /// The filter text, matched case-insensitively.
    pub filter: String,
    /// The page shown, from 1.
    pub page: usize,
    /// The ids of the hidden columns.
    pub hidden: Vec<String>,
}

#[allow(dead_code)]
impl DataTableState {
    /// Reads the state from the request's query string. A `page` that is
    /// not a number counts as the first page; a query that does not parse
    /// at all (a repeated key) gives the default state.
    #[must_use]
    pub fn from_query(cx: &Cx) -> Self {
        let path = uri(cx).path().to_owned();
        let Ok(query) = query_params::<DataTableQuery>(cx) else {
            return Self {
                path,
                page: 1,
                ..Self::default()
            };
        };
        let sort = query
            .sort
            .as_deref()
            .map(|sort| match sort.strip_prefix('-') {
                Some(id) => (id.to_owned(), SortDirection::Desc),
                None => (sort.to_owned(), SortDirection::Asc),
            });
        let hidden = query
            .hide
            .as_deref()
            .unwrap_or_default()
            .split(',')
            .filter(|id| !id.is_empty())
            .map(str::to_owned)
            .collect();
        Self {
            path,
            sort,
            filter: query.filter.clone().unwrap_or_default(),
            page: query
                .page
                .as_deref()
                .and_then(|page| page.parse().ok())
                .unwrap_or(1)
                .max(1),
            hidden,
        }
    }

    /// The direction the column `id` is sorted in, if it is the sorted one.
    #[must_use]
    pub fn sorted(&self, id: &str) -> Option<SortDirection> {
        self.sort
            .as_ref()
            .filter(|(sorted, _)| sorted == id)
            .map(|(_, direction)| *direction)
    }

    /// The `aria-sort` value for the header of the column `id`.
    #[must_use]
    pub fn aria_sort(&self, id: &str) -> Option<&'static str> {
        self.sorted(id).map(SortDirection::aria_sort)
    }

    /// Whether the column `id` is shown.
    #[must_use]
    pub fn is_visible(&self, id: &str) -> bool {
        !self.hidden.iter().any(|hidden| hidden == id)
    }

    /// The `sort` parameter's value: the column id, with `-` before it when
    /// descending.
    #[must_use]
    pub fn sort_param(&self) -> Option<String> {
        self.sort.as_ref().map(|(id, direction)| match direction {
            SortDirection::Asc => id.clone(),
            SortDirection::Desc => format!("-{id}"),
        })
    }

    /// The URL of this state: the path and its query string.
    #[must_use]
    pub fn href(&self) -> String {
        let mut pairs = Vec::new();
        if let Some(sort) = self.sort_param() {
            pairs.push(("sort", sort));
        }
        if !self.filter.is_empty() {
            pairs.push(("filter", self.filter.clone()));
        }
        if self.page > 1 {
            pairs.push(("page", self.page.to_string()));
        }
        if !self.hidden.is_empty() {
            pairs.push(("hide", self.hidden.join(",")));
        }
        if pairs.is_empty() {
            return self.path.clone();
        }
        let query: Vec<String> = pairs
            .iter()
            .map(|(key, value)| format!("{key}={}", encode(value)))
            .collect();
        format!("{}?{}", self.path, query.join("&"))
    }

    /// The link that sorts by the column `id`: ascending, or descending when
    /// it is already ascending (TanStack's `toggleSorting`). Sorting goes
    /// back to the first page.
    #[must_use]
    pub fn sort_href(&self, id: &str) -> String {
        let direction = match self.sorted(id) {
            Some(SortDirection::Asc) => SortDirection::Desc,
            _ => SortDirection::Asc,
        };
        Self {
            sort: Some((id.to_owned(), direction)),
            page: 1,
            ..self.clone()
        }
        .href()
    }

    /// The link that shows the column `id` if it is hidden, or hides it.
    #[must_use]
    pub fn toggle_column_href(&self, id: &str) -> String {
        let mut next = self.clone();
        match next.hidden.iter().position(|hidden| hidden == id) {
            Some(index) => {
                next.hidden.remove(index);
            }
            None => next.hidden.push(id.to_owned()),
        }
        next.href()
    }

    /// The link to `page` (from 1).
    #[must_use]
    pub fn page_href(&self, page: usize) -> String {
        Self {
            page,
            ..self.clone()
        }
        .href()
    }
}

/// Percent-encodes a query value, keeping only the unreserved characters.
fn encode(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(char::from(byte));
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    encoded
}

/// Rows filtered, sorted and paged in memory by the URL's
/// [`DataTableState`]: TanStack Table's row models.
///
/// The `filter` text keeps the rows where any filterable column's value
/// contains it, ignoring case. The sort is stable, so equal rows keep their
/// order. The page is clamped to the pages there are.
///
/// ```ignore
/// let payments = DataTable::new(cx, &COLUMNS, &PAYMENTS);
/// view! {
///     // Bound inside the view, which owns `payments` once it is moved in.
///     let state = payments.state();
///     for &payment in payments.rows() {
///         table_row(
///             if state.is_visible("email") {
///                 table_cell((payment.email))
///             }
///         )
///     }
/// }
/// ```
pub struct DataTable<'a, T> {
    state: DataTableState,
    rows: Vec<&'a T>,
    filtered: usize,
    page_count: usize,
}

#[allow(dead_code)]
impl<'a, T> DataTable<'a, T> {
    /// The rows of `data` that the URL's state shows, ten to a page.
    #[must_use]
    pub fn new(cx: &Cx, columns: &[Column<T>], data: &'a [T]) -> Self {
        Self::with_page_size(cx, columns, data, 10)
    }

    /// The rows of `data` that the URL's state shows, `page_size` to a page.
    #[must_use]
    pub fn with_page_size(cx: &Cx, columns: &[Column<T>], data: &'a [T], page_size: usize) -> Self {
        let mut state = DataTableState::from_query(cx);
        let needle = state.filter.to_lowercase();
        let mut rows: Vec<&'a T> = data
            .iter()
            .filter(|row| {
                needle.is_empty()
                    || columns
                        .iter()
                        .filter(|column| column.filterable)
                        .any(|column| (column.value)(row).to_lowercase().contains(&needle))
            })
            .collect();
        if let Some((id, direction)) = &state.sort
            && let Some(compare) = columns
                .iter()
                .find(|column| column.id == id)
                .and_then(|column| column.sort)
        {
            rows.sort_by(|a, b| direction.apply(compare(a, b)));
        }

        let filtered = rows.len();
        let page_size = page_size.max(1);
        let page_count = filtered.div_ceil(page_size).max(1);
        state.page = state.page.clamp(1, page_count);
        let rows = rows
            .into_iter()
            .skip((state.page - 1) * page_size)
            .take(page_size)
            .collect();
        Self {
            state,
            rows,
            filtered,
            page_count,
        }
    }

    /// The state read from the URL, with its page clamped.
    #[must_use]
    pub fn state(&self) -> &DataTableState {
        &self.state
    }

    /// The rows on the current page, filtered and sorted.
    #[must_use]
    pub fn rows(&self) -> &[&'a T] {
        &self.rows
    }

    /// How many rows pass the filter, on every page.
    #[must_use]
    pub fn filtered_count(&self) -> usize {
        self.filtered
    }

    /// How many pages the filtered rows fill (at least one).
    #[must_use]
    pub fn page_count(&self) -> usize {
        self.page_count
    }
}

/// Runs on every change inside the table. A change of the select-all box
/// sets every row box; then the select-all box turns checked, indeterminate
/// or clear to match the rows, each row's `data-state` follows its box (so
/// `table_row` fills selected rows), and the selected count updates.
const SELECTION_JS: &str = "const t=event.target;if(!t.dataset||!t.dataset.select)return;\
     const all=this.querySelector('[data-select=all]'),\
     rows=[...this.querySelectorAll('[data-select=row]')];\
     if(t===all)rows.forEach(r=>r.checked=all.checked);\
     rows.forEach(r=>{const tr=r.closest('tr');if(tr){if(r.checked)tr.dataset.state='selected';\
     else delete tr.dataset.state}});\
     const n=rows.filter(r=>r.checked).length;\
     if(all){all.checked=n>0&&n===rows.length;all.indeterminate=n>0&&n<rows.length}\
     this.querySelectorAll('[data-slot=data-table-selected-count]').forEach(e=>e.textContent=n)";

/// A data table: sorting, filtering, paging, column visibility and row
/// selection around a [`table`](super::table::table), in place of TanStack
/// Table.
///
/// The server does the table's work. Sorting, filtering, paging and column
/// visibility live in the URL (see [`DataTableState`]), and their controls
/// are links and a `GET` form, so each change loads the page again; Octane
/// changes them in place. [`DataTable`] applies them to rows held in memory.
/// Row selection stays in the browser: the [`data_table_select_all`] and
/// [`data_table_select_row`] boxes are native checkboxes, and a small inline
/// handler on this wrapper keeps the select-all box, the selected rows'
/// highlight and the [`data_table_selection`] count in step. Loading the page
/// again clears the selection.
///
/// Compose the table yourself, so each cell can hold any markup: a toolbar
/// with a [`data_table_filter`] and a [`data_table_column_toggle`], then a
/// table whose header cells use [`data_table_sort_button`] for sortable
/// columns, then a [`data_table_selection`] and a
/// [`data_table_pagination`]. `attrs` are forwarded to the wrapper `<div>`,
/// with extra classes appended to its classes.
///
/// ```ignore
/// let payments = DataTable::new(cx, &COLUMNS, &PAYMENTS);
/// view! {
///     let state = payments.state();
///     data_table(
///         <div class="flex items-center py-4">
///             data_table_filter(state: state, attrs: attributes! { placeholder="Filter emails..." class="max-w-sm" })
///             data_table_column_toggle(state: state, columns: &COLUMNS, attrs: attributes! { class="ml-auto" })
///         </div>
///         table(
///             table_header(table_row(
///                 table_head(data_table_select_all())
///                 table_head(
///                     attrs: attributes! { aria-sort=(state.aria_sort("email")) },
///                     data_table_sort_button(state: state, column: "email", "Email")
///                 )
///             ))
///             table_body(
///                 for &payment in payments.rows() {
///                     table_row(
///                         table_cell(data_table_select_row(attrs: attributes! { value=(payment.id) }))
///                         table_cell((payment.email))
///                     )
///                 }
///             )
///         )
///         <div class="flex items-center justify-end space-x-2 py-4">
///             data_table_selection(total: payments.filtered_count())
///             data_table_pagination(state: state, page_count: payments.page_count())
///         </div>
///     )
/// }
/// ```
#[component]
pub async fn data_table(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="data-table"
            onchange=(SELECTION_JS)
            class=(class!("w-full font-base text-foreground", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The text box that filters the rows: an [`input`] named `filter` in a
/// `GET` form, submitted with Enter.
///
/// Octane filters on every keystroke; here the page loads again with the
/// new `filter`, keeping the sort and hidden columns and going back to the
/// first page. `attrs` are forwarded to the [`input`] (`placeholder`,
/// classes); the form itself is `display: contents`, so the input lays out
/// as a child of the toolbar.
#[component]
pub async fn data_table_filter(
    /// The table's state, whose sort and hidden columns the form keeps.
    state: &DataTableState,
    /// Extra attributes for the `<input>`.
    #[default]
    attrs: Attributes,
) -> Result<impl View> {
    let sort = state.sort_param();
    let hidden = state.hidden.join(",");
    Ok(view! {
        <form method="get" action=(state.path.as_str()) data-slot="data-table-filter" class="contents">
            if let Some(sort) = sort {
                <input type="hidden" name="sort" value=(sort)>
            }
            if !hidden.is_empty() {
                <input type="hidden" name="hide" value=(hidden)>
            }
            input(attrs: attributes! {
                name="filter"
                value=(state.filter.as_str())
                aria-label="Filter rows"
                (attrs)
            })
        </form>
    })
}

/// Classes for a column entry: Octane's checkbox item, a link here.
const COLUMN_ITEM: StaticClass = class!(
    "relative flex cursor-default select-none items-center rounded-base border-2 \
     border-transparent gap-2 py-1.5 pl-8 pr-2 text-sm font-base outline-hidden \
     transition-colors hover:bg-main hover:text-main-foreground hover:border-border \
     focus:bg-main focus:text-main-foreground focus:border-border \
     focus-visible:bg-main focus-visible:text-main-foreground focus-visible:border-border \
     [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
);

/// Classes that repaint a no-shadow button on the secondary background, as
/// the Octane data table does. `!` because the variant sets `bg-main`.
const NEUTRAL_FILL: StaticClass = class!("bg-secondary-background! text-foreground!");

/// The "Columns" menu, listing the hideable columns by header, with a check
/// on each visible one.
///
/// Each entry is a link that hides or shows its column, so choosing one
/// loads the page again (and closes the menu, which Octane keeps open).
/// `attrs` are forwarded to the [`dropdown_menu`]; place it with classes
/// such as `ml-auto` there.
#[component]
pub async fn data_table_column_toggle<T: 'static>(
    /// The table's state, which says which columns are hidden.
    state: &DataTableState,
    /// The table's columns; those with `hideable` are listed.
    columns: &[Column<T>],
    /// Extra attributes for the menu's `<details>`.
    #[default]
    attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        dropdown_menu(attrs: attrs,
            dropdown_menu_trigger(
                attrs: attributes! {
                    class=(class!(
                        button_variants(ButtonVariant::NoShadow, ButtonSize::Default),
                        NEUTRAL_FILL,
                    ))
                },
                "Columns"
                icon(data: iconify_icon!("lucide:chevron-down"))
            )
            dropdown_menu_content(align: DropdownMenuAlign::End,
                #[key(column.id)]
                for column in columns.iter().filter(|column| column.hideable) {
                    let visible = state.is_visible(column.id);
                    <a
                        href=(state.toggle_column_href(column.id))
                        role="menuitemcheckbox"
                        aria-checked=(if visible { "true" } else { "false" })
                        data-slot="data-table-column-toggle-item"
                        class=(class!(COLUMN_ITEM, "capitalize"))
                    >
                        <span class="pointer-events-none absolute left-2 flex size-3.5 items-center justify-center">
                            if visible {
                                icon(data: iconify_icon!("lucide:check"))
                            }
                        </span>
                        // Octane lists the capitalized id; the header reads
                        // the same here and suits ids like `created_at`.
                        (column.header)
                    </a>
                }
            )
        )
    })
}

/// A sortable column's header: a link that sorts by the column, ascending
/// first and then flipping, with an arrow icon.
///
/// Pass the header text as children, and set `aria-sort` on the header cell
/// with [`DataTableState::aria_sort`]. `attrs` are forwarded to the `<a>`,
/// with extra classes appended to its classes.
#[component]
pub async fn data_table_sort_button(
    /// The table's state.
    state: &DataTableState,
    /// The id of the column this header sorts.
    column: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <a
            href=(state.sort_href(column))
            data-slot="data-table-sort-button"
            class=(class!(
                button_variants(ButtonVariant::NoShadow, ButtonSize::Sm),
                NEUTRAL_FILL,
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
            icon(data: iconify_icon!("lucide:arrow-up-down"))
        </a>
    })
}

/// Classes that keep a selection box on the secondary background when
/// checked, since a selected row is filled with the main color.
const SELECT_BOX: StaticClass =
    class!("has-checked:bg-secondary-background! has-indeterminate:bg-secondary-background!");

/// The header [`checkbox`] that selects or clears every row on the page,
/// and shows whether all, some or none are selected.
///
/// `autocomplete="off"` keeps the browser from restoring a checked state on
/// reload, which the count would not show. Attributes in `attrs` go on the
/// `<input>`, classes on the visible box.
#[component]
pub async fn data_table_select_all(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        checkbox(attrs: attributes! {
            data-select="all"
            aria-label="Select all"
            autocomplete="off"
            class=(class!(SELECT_BOX, attrs.remove("class")))
            (attrs)
        })
    })
}

/// A row's selection [`checkbox`].
///
/// Give it the row's id as `value` (and a `name`) in `attrs` to submit the
/// selection with a surrounding form. Attributes in `attrs` go on the
/// `<input>`, classes on the visible box.
#[component]
pub async fn data_table_select_row(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        checkbox(attrs: attributes! {
            data-select="row"
            aria-label="Select row"
            autocomplete="off"
            class=(class!(SELECT_BOX, attrs.remove("class")))
            (attrs)
        })
    })
}

/// "0 of 5 row(s) selected.": the count the [`data_table`] handler keeps
/// up to date, out of `total` rows.
#[component]
pub async fn data_table_selection(
    /// How many rows pass the filter.
    total: usize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="data-table-selection"
            aria-live="polite"
            class=(class!("text-foreground flex-1 text-sm", attrs.remove("class")))
            (attrs)
        >
            <span data-slot="data-table-selected-count">"0"</span>
            " of " (total) " row(s) selected."
        </div>
    })
}

/// "Previous" and "Next" links between the table's pages. On the first or
/// last page the matching one is a disabled button instead.
#[component]
pub async fn data_table_pagination(
    /// The table's state, holding the current page.
    state: &DataTableState,
    /// How many pages there are.
    page_count: usize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let previous = (state.page > 1).then(|| state.page_href(state.page - 1));
    let next = (state.page < page_count).then(|| state.page_href(state.page + 1));
    Ok(view! {
        <div
            data-slot="data-table-pagination"
            class=(class!("space-x-2", attrs.remove("class")))
            (attrs)
        >
            page_step(href: previous, rel: "prev", "Previous")
            page_step(href: next, rel: "next", "Next")
        </div>
    })
}

/// One step of [`data_table_pagination`]: a link styled as a small button,
/// or a disabled button when there is no page to go to.
#[component]
async fn page_step(
    href: Option<String>,
    rel: &str,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        if let Some(href) = href {
            <a
                href=(href)
                rel=(rel)
                class=(class!(button_variants(ButtonVariant::NoShadow, ButtonSize::Sm), NEUTRAL_FILL))
            >
                (child)
            </a>
        } else {
            button(
                variant: ButtonVariant::NoShadow,
                size: ButtonSize::Sm,
                attrs: attributes! { type="button" disabled="" class=(NEUTRAL_FILL) },
                (child)
            )
        }
    })
}
