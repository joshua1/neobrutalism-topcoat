// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// A bordered neobrutalism table inside a scrollable container.
///
/// Pass a [`table_caption`], [`table_header`], [`table_body`] and
/// [`table_footer`] as children. `attrs` are forwarded to the `<table>` (not
/// the wrapper), with extra classes appended to its classes.
///
/// ```ignore
/// view! {
///     table(
///         table_header(
///             table_row(table_head("Invoice") table_head("Amount"))
///         )
///         table_body(
///             table_row(table_cell("INV001") table_cell("$250.00"))
///         )
///     )
/// }
/// ```
#[component]
pub async fn table(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class="relative w-full overflow-auto">
            <table
                data-slot="table"
                class=(class!(
                    "w-full caption-bottom border-2 border-border text-sm",
                    attrs.remove("class"),
                ))
                (attrs)
            >
                (child)
            </table>
        </div>
    })
}

/// The heading section of a [`table`], holding the row of [`table_head`]s.
#[component]
pub async fn table_header(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <thead
            data-slot="table-header"
            class=(class!("[&_tr]:border-b-2 [&_tr]:border-border", attrs.remove("class")))
            (attrs)
        >
            (child)
        </thead>
    })
}

/// The body of a [`table`], holding the data rows. The last row drops its
/// bottom border.
#[component]
pub async fn table_body(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <tbody
            data-slot="table-body"
            class=(class!("[&_tr:last-child]:border-0", attrs.remove("class")))
            (attrs)
        >
            (child)
        </tbody>
    })
}

/// The closing section of a [`table`], for totals and other summaries.
#[component]
pub async fn table_footer(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <tfoot
            data-slot="table-footer"
            class=(class!(
                "border-t-2 border-border bg-background font-base text-foreground \
                 last:[&>tr]:border-b-0",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </tfoot>
    })
}

/// A table row. Pass `data-state="selected"` in `attrs` to fill it with the
/// main color.
#[component]
pub async fn table_row(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <tr
            data-slot="table-row"
            class=(class!(
                "border-b-2 border-border transition-colors text-foreground bg-background \
                 font-base data-[state=selected]:bg-main \
                 data-[state=selected]:text-main-foreground",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </tr>
    })
}

/// A column header (`<th>`) in a [`table_header`]'s row.
#[component]
pub async fn table_head(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <th
            data-slot="table-head"
            class=(class!(
                "h-12 px-4 text-left align-middle font-heading text-foreground \
                 [&:has([data-slot=checkbox])]:pr-0",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </th>
    })
}

/// One cell (`<td>`) of a [`table_row`]. Pass `colspan` in `attrs` to span
/// several columns.
#[component]
pub async fn table_cell(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <td
            data-slot="table-cell"
            class=(class!(
                "p-4 align-middle [&:has([data-slot=checkbox])]:pr-0",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </td>
    })
}

/// A caption describing the [`table`], shown below it.
#[component]
pub async fn table_caption(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <caption
            data-slot="table-caption"
            class=(class!("mt-4 text-sm text-foreground font-base", attrs.remove("class")))
            (attrs)
        >
            (child)
        </caption>
    })
}
