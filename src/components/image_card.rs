// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

/// A bordered figure with an image above a caption.
///
/// `image_url` and `caption` are required. `attrs` are forwarded to the
/// `<figure>`, with extra classes appended to its classes.
///
/// ```ignore
/// view! {
///     image_card(image_url: "/flowers.jpg", caption: "Spring")
/// }
/// ```
#[component]
pub async fn image_card(
    /// The image's address.
    image_url: &str,
    /// The text shown under the image.
    caption: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <figure
            class=(class!(
                "w-[250px] overflow-hidden rounded-base border-2 border-border bg-background \
                 font-base shadow-shadow",
                attrs.remove("class"),
            ))
            (attrs)
        >
            <img class="w-full aspect-4/3" src=(image_url) alt="image">
            <figcaption class="border-t-2 text-foreground border-border p-4">(caption)</figcaption>
        </figure>
    })
}
