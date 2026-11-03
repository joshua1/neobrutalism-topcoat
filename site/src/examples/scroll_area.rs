use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use crate::components::scroll_area::{ScrollBarOrientation, scroll_area, scroll_bar};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        scroll_area(
            attrs: attributes! {
                class="rounded-base h-[200px] w-[350px] text-foreground border-2 border-border \
                    bg-background p-4 shadow-shadow"
            },
            "Jokester began sneaking into the castle in the middle of the night and leaving \
             jokes all over the place: under the king's pillow, in his soup, even in the royal \
             toilet. The king was furious, but he couldn't seem to stop Jokester. And then, one \
             day, the people of the kingdom discovered that the jokes left by Jokester were so \
             funny that they couldn't help but laugh. And once they started laughing, they \
             couldn't stop."
        )
    })
}

/// A photo and its artist.
struct Artwork {
    artist: &'static str,
    art: &'static str,
}

const WORKS: [Artwork; 3] = [
    Artwork {
        artist: "Ornella Binni",
        art: "https://images.unsplash.com/photo-1465869185982-5a1a7522cbcb?auto=format&fit=crop&w=300&q=80",
    },
    Artwork {
        artist: "Tom Byrom",
        art: "https://images.unsplash.com/photo-1548516173-3cabfa4607e9?auto=format&fit=crop&w=300&q=80",
    },
    Artwork {
        artist: "Vladimir Malyavko",
        art: "https://images.unsplash.com/photo-1494337480532-3725c85fd2ab?auto=format&fit=crop&w=300&q=80",
    },
];

#[component]
pub async fn horizontal() -> Result<impl View> {
    Ok(view! {
        scroll_area(
            attrs: attributes! {
                class="w-96 rounded-base border-2 border-border bg-background shadow-shadow \
                    whitespace-nowrap"
            },
            <div class="flex w-max space-x-4 p-4">
                for artwork in WORKS {
                    <figure class="shrink-0">
                        <div class="overflow-hidden rounded-base border-2 border-border">
                            <img
                                src=(artwork.art)
                                alt=(format!("Photo by {}", artwork.artist))
                                class="aspect-[3/4] h-fit w-fit object-cover"
                                width="300"
                                height="400"
                            >
                        </div>
                        <figcaption class="pt-2 text-xs font-base text-foreground">
                            "Photo by "
                            <span class="font-heading text-foreground">(artwork.artist)</span>
                        </figcaption>
                    </figure>
                }
            </div>
            scroll_bar(orientation: ScrollBarOrientation::Horizontal)
        )
    })
}
