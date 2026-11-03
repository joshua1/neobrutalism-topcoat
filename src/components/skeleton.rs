// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

/// Classes for a pulsing, bordered placeholder.
const SKELETON: StaticClass = class!(
    "animate-pulse rounded-base bg-secondary-background border-2 border-border",
);

/// A pulsing placeholder for content that is still loading.
///
/// Set its size and shape through classes in `attrs`, matching the content
/// it stands in for. `attrs` are forwarded to the `<div>`, with extra classes
/// appended to its classes. Children are optional.
///
/// ```ignore
/// view! {
///     <div class="space-y-2">
///         skeleton(attrs: attributes! { class="h-4 w-[250px]" })
///         skeleton(attrs: attributes! { class="h-4 w-[200px]" })
///     </div>
/// }
/// ```
#[component]
pub async fn skeleton(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div data-slot="skeleton" class=(class!(SKELETON, attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}
