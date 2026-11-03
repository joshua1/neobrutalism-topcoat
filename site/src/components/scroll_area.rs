// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

/// The native scrollbar, restyled: thin, with a round thumb in the border
/// color on a transparent track. `scrollbar-*` covers Chrome and Firefox,
/// `::-webkit-scrollbar` covers Safari.
const SCROLLBAR: StaticClass = class!(
    "[scrollbar-width:thin] [scrollbar-color:var(--border)_transparent] \
     [&::-webkit-scrollbar]:size-2.5 [&::-webkit-scrollbar-track]:bg-transparent \
     [&::-webkit-scrollbar-thumb]:rounded-full [&::-webkit-scrollbar-thumb]:border \
     [&::-webkit-scrollbar-thumb]:border-transparent [&::-webkit-scrollbar-thumb]:bg-border \
     [&::-webkit-scrollbar-thumb]:bg-clip-padding [&::-webkit-scrollbar-corner]:bg-transparent",
);

/// A box whose content scrolls, with neobrutalism scrollbars.
///
/// It scrolls vertically. Add a [`scroll_bar`] with
/// `ScrollBarOrientation::Horizontal` inside it to scroll sideways too. Size
/// it, and add borders or padding, through `class` in `attrs`; the other
/// attributes go on the outer `<div>` as well.
///
/// Uses the browser's own scrollbars, so they keep native behavior (they may
/// overlay or hide on macOS) instead of Base UI's custom-drawn ones.
///
/// ```ignore
/// view! {
///     scroll_area(
///         attrs: attributes! { class="h-[200px] w-[350px] rounded-base border-2 border-border p-4" },
///         "A long story..."
///     )
/// }
/// ```
#[component]
pub async fn scroll_area(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="scroll-area"
            class=(class!("relative overflow-hidden", attrs.remove("class")))
            (attrs)
        >
            <div
                data-slot="scroll-area-viewport"
                tabindex="0"
                class=(class!(
                    "h-full w-full font-base outline-none overflow-y-auto overflow-x-hidden \
                     has-[>[data-slot=scroll-area-scrollbar][data-orientation=horizontal]]:overflow-x-auto",
                    SCROLLBAR,
                ))
            >
                (child)
            </div>
        </div>
    })
}

/// The direction a [`scroll_bar`] scrolls.
///
/// [`Default`] is `ScrollBarOrientation::Vertical`, used when none is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ScrollBarOrientation {
    /// Up and down. A [`scroll_area`] always scrolls this way.
    #[default]
    Vertical,
    /// Sideways.
    Horizontal,
}

impl ScrollBarOrientation {
    /// The `data-orientation` value the viewport looks for.
    fn as_str(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }
}

/// Turns on scrolling in one direction for the [`scroll_area`] it is placed in.
///
/// The bar itself is the browser's native one, so this renders only a hidden
/// marker the scroll area's viewport detects with CSS `:has()`. Place it as
/// a direct child of the scroll area's content.
///
/// ```ignore
/// view! {
///     scroll_area(
///         <div class="flex w-max gap-4">"..."</div>
///         scroll_bar(orientation: ScrollBarOrientation::Horizontal)
///     )
/// }
/// ```
#[component]
#[allow(dead_code)]
pub async fn scroll_bar(#[default] orientation: ScrollBarOrientation) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="scroll-area-scrollbar"
            data-orientation=(orientation.as_str())
            hidden=""
        ></span>
    })
}
