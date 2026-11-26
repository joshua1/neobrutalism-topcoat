//! The eyebrow, title and description that open most sections.

use topcoat::{
    Result,
    context::Cx,
    view::{View, class, component, view},
};

/// How a [`section_heading`] lines up.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Align {
    #[default]
    Center,
    Left,
}

/// A section's heading. `class` is appended to the wrapper's classes; to
/// change its `mb-14`, pass the new margin with the `!` suffix.
///
/// The eyebrow's stroke is painted under its fill, so only half of the stroke
/// shows as an outline.
#[component]
pub async fn section_heading(
    cx: &Cx,
    eyebrow: &str,
    title: &str,
    #[into]
    #[default]
    description: Option<&str>,
    #[default] align: Align,
    #[default] class: &str,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(
            "mb-14 flex max-w-2xl flex-col gap-4",
            "mx-auto items-center text-center" if align == Align::Center else "items-start",
            class,
        ))>
            <p class="text-lg font-heading uppercase tracking-[0.12em] text-chart-3 [paint-order:stroke_fill] [-webkit-text-stroke:2.5px_var(--border)]">
                (eyebrow)
            </p>
            <h2 class="text-balance font-heading text-4xl leading-tight sm:text-5xl">(title)</h2>
            if let Some(description) = description {
                <p class="text-lg font-base">(description)</p>
            }
        </div>
    })
}
