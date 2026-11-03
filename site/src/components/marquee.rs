// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

/// A full-width band of large text scrolling sideways forever.
///
/// Renders `items` twice: the second copy follows the first so the loop has
/// no gap. Needs the `animate-marquee` and `animate-marquee2` utilities and
/// their keyframes in the theme. `attrs` are forwarded to the outer `<div>`,
/// with extra classes appended to its classes.
///
/// ```ignore
/// view! {
///     marquee(items: &["Fast", "Typed", "Server-rendered"])
/// }
/// ```
#[component]
pub async fn marquee(
    /// The words or phrases to scroll.
    items: &[&str],
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(
                "relative flex w-full overflow-x-hidden border-b-2 border-t-2 border-border \
                 bg-secondary-background text-foreground font-base",
                attrs.remove("class"),
            ))
            (attrs)
        >
            <div class="animate-marquee whitespace-nowrap py-12">
                for item in items {
                    <span class="mx-4 text-4xl">(*item)</span>
                }
            </div>
            // Both copies are needed: the second slides in as the first leaves.
            <div class="absolute top-0 animate-marquee2 whitespace-nowrap py-12" aria-hidden="true">
                for item in items {
                    <span class="mx-4 text-4xl">(*item)</span>
                }
            </div>
        </div>
    })
}
