// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

/// Classes for the textarea's thick border, fill, focus ring and invalid border.
const TEXTAREA: StaticClass = class!(
    "flex min-h-[80px] w-full rounded-base border-2 border-border bg-secondary-background \
     selection:bg-main selection:text-main-foreground px-3 py-2 text-sm font-base \
     text-foreground placeholder:text-foreground/50 focus-visible:outline-none \
     focus-visible:ring-2 focus-visible:ring-black focus-visible:ring-offset-2 \
     disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-red-500",
);

/// A neobrutalism multi-line text input: a plain native `<textarea>`.
///
/// Pass the initial text as children. `attrs` are forwarded to the
/// `<textarea>`, with extra classes appended to its classes, and a
/// `data-slot` replaces the default `textarea` slot. Set
/// `aria-invalid="true"` to show the red border.
///
/// ```ignore
/// view! {
///     textarea(attrs: attributes! { name="message" placeholder="Type your message here." })
/// }
/// ```
#[component]
pub async fn textarea(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    // An element takes each attribute once, so the default slot is only
    // written when the caller did not bring its own.
    let default_slot = !attrs.contains_key("data-slot");
    Ok(view! {
        <textarea
            if default_slot {
                data-slot="textarea"
            }
            class=(class!(TEXTAREA, attrs.remove("class")))
            (attrs)
        >
            (child)
        </textarea>
    })
}
