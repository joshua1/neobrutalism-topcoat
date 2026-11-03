// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    runtime::Expr,
    view::{Attributes, Child, View, class, component, view},
};

/// A neobrutalism progress bar on a 0 to 100 scale.
///
/// `value` is the completed percentage. Pass a number for a fixed bar, or a
/// runtime expression such as `$(uploaded.get())` to update it in the
/// browser. The bar is drawn after the children, so put a
/// [`progress_label`] and a [`progress_value`] inside to show them above it.
///
/// The root `<div>` has `role="progressbar"` and keeps `aria-valuenow` in
/// sync. Give it an accessible name with `aria-label`, or `aria-labelledby`
/// pointing at the label's `id`. `attrs` are forwarded to the `<div>`, with
/// extra classes appended to its classes.
///
/// ```ignore
/// view! {
///     progress(
///         value: 56.0,
///         attrs: attributes! { class="w-full max-w-sm" aria-labelledby="upload" },
///         progress_label(attrs: attributes! { id="upload" }, "Upload progress")
///         progress_value(value: 56.0)
///     )
/// }
/// ```
#[component]
pub async fn progress(
    /// The completed percentage, from 0 to 100.
    #[into]
    value: Expr<f64>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="progress"
            role="progressbar"
            aria-valuemin="0"
            aria-valuemax="100"
            :aria-valuenow=(value.clone())
            class=(class!("flex w-full flex-wrap gap-2", attrs.remove("class")))
            (attrs)
        >
            (child)
            progress_track(progress_indicator(value: value))
        </div>
    })
}

/// The bordered groove of a [`progress`] bar, holding a
/// [`progress_indicator`]. [`progress`] renders one itself; use this part
/// only to build a bar by hand.
#[component]
pub async fn progress_track(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="progress-track"
            class=(class!(
                "relative h-4 w-full overflow-hidden rounded-base border-2 border-border \
                 bg-secondary-background",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The filled part of a [`progress_track`], `value` percent wide.
#[component]
pub async fn progress_indicator(
    /// The completed percentage, from 0 to 100.
    #[into]
    value: Expr<f64>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="progress-indicator"
            :style=$(("width: ", value, "%"))
            class=(class!(
                "h-full border-r-2 border-border bg-main transition-all",
                attrs.remove("class"),
            ))
            (attrs)
        ></div>
    })
}

/// A heading for a [`progress`] bar, shown above it at the start.
#[component]
pub async fn progress_label(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="progress-label"
            class=(class!("text-sm font-heading text-foreground", attrs.remove("class")))
            (attrs)
        >
            (child)
        </span>
    })
}

/// The percentage of a [`progress`] bar as text, such as `56%`, pushed to the
/// end of the row. Pass the same `value` as the bar.
#[component]
pub async fn progress_value(
    /// The completed percentage, from 0 to 100.
    #[into]
    value: Expr<f64>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="progress-value"
            class=(class!(
                "ms-auto text-sm font-base text-foreground tabular-nums",
                attrs.remove("class"),
            ))
            (attrs)
        >
            $((value, "%"))
        </span>
    })
}
