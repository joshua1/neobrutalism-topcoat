use topcoat::{
    Result,
    view::{View, component, view},
};

use crate::components::image_card::image_card;

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        image_card(
            caption: "Image",
            image_url: "https://hips.hearstapps.com/hmg-prod/images/flowers-trees-and-bushes-reach-their-peak-of-full-bloom-in-news-photo-1678292967.jpg?resize=300:*",
        )
    })
}
