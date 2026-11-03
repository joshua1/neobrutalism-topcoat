// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, StaticClass, View, class, component, view},
};

/// Classes for the input's thick border, fill, file button and focus ring.
const INPUT: StaticClass = class!(
    "flex h-10 w-full rounded-base border-2 border-border bg-secondary-background \
     selection:bg-main selection:text-main-foreground px-3 py-2 text-sm font-base \
     text-foreground file:border-0 file:bg-transparent file:text-sm file:font-heading \
     placeholder:text-foreground/50 focus-visible:outline-hidden focus-visible:ring-2 \
     focus-visible:ring-black focus-visible:ring-offset-2 disabled:cursor-not-allowed \
     disabled:opacity-50",
);

/// A neobrutalism text input: a plain native `<input>`.
///
/// Pass every input attribute (`type`, `name`, `placeholder`, `value`,
/// `disabled`, ...) and event handler through `attrs`. Extra classes are
/// appended to the input's classes, and a `data-slot` replaces the default
/// `input` slot (`input_group_input` uses this). `@input` fires on every
/// keystroke and `@change` when the value is committed.
///
/// ```ignore
/// view! {
///     input(attrs: attributes! { type="email" placeholder="Email" class="w-[200px]" })
/// }
/// ```
#[component]
pub async fn input(#[default] mut attrs: Attributes) -> Result<impl View> {
    // An element takes each attribute once, so the default slot is only
    // written when the caller did not bring its own.
    let default_slot = !attrs.contains_key("data-slot");
    Ok(view! {
        <input
            if default_slot {
                data-slot="input"
            }
            class=(class!(INPUT, attrs.remove("class")))
            (attrs)
        >
    })
}
