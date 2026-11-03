// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{IconData, iconify::iconify_icon},
    view::{Attributes, View, class, component, view},
};

/// The spinning glyph, Lucide's `loader-circle` (formerly `loader-2`).
const LOADER: IconData = iconify_icon!("lucide:loader-circle");

/// A spinning loading indicator.
///
/// It is `size-4` by default; pass another size class in `attrs` to change
/// it. It announces itself as a `status` named `label` (default `"Loading"`).
/// `attrs` are forwarded to the `<svg>`, with extra classes appended to its
/// classes.
///
/// ```ignore
/// view! {
///     button(
///         attrs: attributes! { disabled="" },
///         spinner()
///         "Please wait"
///     )
/// }
/// ```
#[component]
pub async fn spinner(
    /// The accessible name announced to assistive technology.
    #[into]
    #[default(String::from("Loading"))]
    label: String,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <svg
            data-slot="spinner"
            role="status"
            aria-label=(label)
            viewBox=(LOADER.view_box())
            class=(class!("size-4 animate-spin", attrs.remove("class")))
            (attrs)
        >
            (LOADER.into_body())
        </svg>
    })
}
