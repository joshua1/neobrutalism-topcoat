//! A section that shows and hides its content, built on native `<details>` /
//! `<summary>` elements. No runtime is needed.
//!
//! The parts are unstyled, as in Octane. The root names the
//! `group/collapsible` group, so children can style themselves by its state:
//! `group-open/collapsible:rotate-180` replaces Base UI's
//! `group-data-panel-open:rotate-180`.
//!
//! Unlike Base UI, the trigger must be a direct child of the root, and
//! everything else inside the root is hidden while it is closed. Content that
//! should stay visible goes outside the [`collapsible`]; position the trigger
//! with CSS if it has to sit elsewhere (see the docs-site examples).

// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// A collapsible section, holding a [`collapsible_trigger`] and a
/// [`collapsible_content`].
///
/// Pass `open=""` in `attrs` to open it initially. `attrs` are forwarded to
/// the `<details>`, with extra classes appended.
///
/// ```ignore
/// view! {
///     collapsible(
///         collapsible_trigger(
///             attrs: attributes! {
///                 class=(button_variants(ButtonVariant::NoShadow, ButtonSize::Default))
///             },
///             "Product details"
///         )
///         collapsible_content("This panel can be expanded or collapsed.")
///     )
/// }
/// ```
#[component]
pub async fn collapsible(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <details
            data-slot="collapsible"
            class=(class!("group/collapsible", attrs.remove("class")))
            (attrs)
        >
            (child)
        </details>
    })
}

/// The control that opens and closes a [`collapsible`], rendered as its
/// `<summary>`. Only the disclosure marker is removed; style it with
/// `class=(button_variants(..))` or your own classes.
#[component]
pub async fn collapsible_trigger(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <summary
            data-slot="collapsible-trigger"
            class=(class!("list-none [&::-webkit-details-marker]:hidden", attrs.remove("class")))
            (attrs)
        >
            (child)
        </summary>
    })
}

/// The content shown while a [`collapsible`] is open.
#[component]
pub async fn collapsible_content(
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div data-slot="collapsible-content" (attrs)>(child)</div>
    })
}
