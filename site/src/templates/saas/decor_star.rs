//! The decorative stars every section uses, and the faint grid background.

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::components::stars::{Star, star};

/// Faint 56px grid used as the background of every white section.
pub const GRID_BACKGROUND: &str = "bg-[linear-gradient(to_right,#80808033_1px,transparent_1px),linear-gradient(to_bottom,#80808033_1px,transparent_1px)] bg-size-[56px_56px]";

/// A star outlined like the cards. The non-scaling stroke keeps every outline
/// at `stroke_width` pixels however big the star is drawn. Avoid `--chart-1`
/// for decoration: it reads as the interactive main colour.
///
/// Only the `class` of `attrs` is used; it is appended to the star's classes.
#[component]
pub async fn decor_star(
    cx: &Cx,
    shape: Star,
    #[default("var(--chart-3)")] color: &str,
    #[default(2.0)] stroke_width: f32,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        star(
            shape: shape,
            color: color,
            stroke: "var(--border)",
            stroke_width: stroke_width,
            path_class: "[vector-effect:non-scaling-stroke]",
            attrs: attributes! {
                aria-hidden="true"
                class=(class!("pointer-events-none shrink-0 overflow-visible", attrs.remove("class")))
            },
        )
    })
}

/// One of the big background stars of a section. The section needs
/// `relative isolate overflow-hidden`: `isolate` makes it a stacking context,
/// so the `-z-10` star paints above its background but below its content.
#[component]
pub async fn section_star(cx: &Cx, shape: Star, color: &str, class: &str) -> Result<impl View> {
    Ok(view! {
        decor_star(
            shape: shape,
            color: color,
            attrs: attributes! { class=(class!("absolute -z-10", class)) },
        )
    })
}

/// A tilted star with a short label on it, which turns when hovered.
#[component]
pub async fn star_sticker(
    cx: &Cx,
    shape: Star,
    color: &str,
    #[default] class: &str,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(
            "group absolute z-10 flex size-24 rotate-6 items-center justify-center text-center \
             font-heading leading-tight text-foreground",
            class,
        ))>
            decor_star(
                shape: shape,
                color: color,
                attrs: attributes! {
                    class="absolute inset-0 size-full transition-transform duration-700 ease-out group-hover:rotate-45 motion-reduce:transition-none"
                },
            )
            <span class="relative">(child)</span>
        </div>
    })
}
