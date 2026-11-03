//! Stacked sections that expand and collapse, built on native
//! `<details>` / `<summary>` elements. No runtime is needed.
//!
//! Base UI's accordion lets one item open at a time unless `multiple` is set.
//! Natively, items open independently; give items the same `name` attribute to
//! make them exclusive. Arrow-key movement between triggers is not matched;
//! Tab moves between them.

// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

/// A group of [`accordion_item`]s.
///
/// `attrs` are forwarded to the `<div>`, with extra classes appended.
///
/// ```ignore
/// view! {
///     accordion(
///         attrs: attributes! { class="w-full max-w-xl space-y-4" },
///         for (question, answer) in questions {
///             // The shared name closes the open item when another opens.
///             // Leave it out to let several stand open at once.
///             accordion_item(
///                 attrs: attributes! { name="faq" },
///                 accordion_trigger((question))
///                 accordion_content((answer))
///             )
///         }
///     )
/// }
/// ```
#[component]
pub async fn accordion(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div data-slot="accordion" class=(class!(attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}

/// Classes that animate the panel's height as the item opens or closes.
///
/// `::details-content` wraps everything after the `<summary>`. Browsers without
/// it open and close the item without animating.
const ANIMATION: StaticClass = class!(
    "[interpolate-size:allow-keywords] [&::details-content]:h-0 \
     [&::details-content]:overflow-hidden \
     [&::details-content]:[transition:height_200ms_ease-out,content-visibility_200ms_allow-discrete] \
     [&[open]::details-content]:h-auto",
);

/// One collapsible section, holding an [`accordion_trigger`] and an
/// [`accordion_content`].
///
/// Pass `open=""` in `attrs` to open it initially, and a shared `name` to make
/// it exclusive with its siblings. `disabled` makes the item inert, so it can't
/// be opened, focused or clicked.
#[component]
pub async fn accordion_item(
    /// Whether the item is disabled.
    #[default]
    disabled: bool,
    /// Extra attributes for the `<details>` element.
    #[default]
    mut attrs: Attributes,
    /// The trigger and the content.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <details
            data-slot="accordion-item"
            data-disabled=(disabled)
            aria-disabled=(disabled.then_some("true"))
            inert=(disabled)
            class=(class!(
                "group/accordion-item rounded-base overflow-hidden border-2 border-b \
                 border-border shadow-shadow",
                ANIMATION,
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </details>
    })
}

/// The heading that opens and closes an [`accordion_item`], rendered as its
/// `<summary>`. A chevron turns over while the item is open.
#[component]
pub async fn accordion_trigger(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <summary
            data-slot="accordion-trigger"
            class=(class!(
                "flex flex-1 list-none items-center justify-between text-left text-base \
                 text-main-foreground border-border focus-visible:ring-[3px] bg-main p-4 \
                 font-heading transition-all [&::-webkit-details-marker]:hidden \
                 group-open/accordion-item:[&>svg]:rotate-180 \
                 group-open/accordion-item:rounded-b-none group-open/accordion-item:border-b-2 \
                 group-data-disabled/accordion-item:pointer-events-none \
                 group-data-disabled/accordion-item:opacity-50",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
            icon(
                data: iconify_icon!("lucide:chevron-down"),
                attrs: attributes! {
                    class="pointer-events-none size-5 shrink-0 transition-transform duration-200"
                }
            )
        </summary>
    })
}

/// The panel shown while an [`accordion_item`] is open.
///
/// `attrs` go on the padded inner `<div>`, with extra classes appended.
#[component]
pub async fn accordion_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="accordion-content"
            class="overflow-hidden rounded-b-base bg-secondary-background text-sm font-base"
        >
            <div class=(class!("p-4", attrs.remove("class"))) (attrs)>(child)</div>
        </div>
    })
}
