// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

/// Classes that draw a key as a small bordered box. Inside a tooltip
/// (`data-slot="tooltip-content"`) it switches to the main color.
///
/// The Octane source also lists `font-sans`, but its `cn` drops it in favor
/// of the later `font-heading`, so keys render in the browser's monospace
/// `<kbd>` font. It is left out here to match.
const KBD: StaticClass = class!(
    "pointer-events-none inline-flex h-5 w-fit min-w-5 items-center justify-center gap-1 \
     rounded-base border-2 border-border bg-secondary-background px-1 text-xs \
     font-heading text-foreground select-none \
     in-data-[slot=tooltip-content]:border-main-foreground/30 \
     in-data-[slot=tooltip-content]:bg-main in-data-[slot=tooltip-content]:text-main-foreground \
     [&_svg:not([class*='size-'])]:size-3",
);

/// A keyboard key label rendered as `<kbd>`.
///
/// Pass the key name or an icon as children, and use [`kbd_group`] for a
/// shortcut made of several keys. `attrs` are forwarded to the `<kbd>`, with
/// extra classes appended to its classes.
///
/// ```ignore
/// view! {
///     kbd_group(kbd("Ctrl") kbd("Shift") kbd("P"))
/// }
/// ```
#[component]
pub async fn kbd(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <kbd data-slot="kbd" class=(class!(KBD, attrs.remove("class"))) (attrs)>
            (child)
        </kbd>
    })
}

/// A row of [`kbd`] keys for one shortcut, itself a `<kbd>`.
#[component]
pub async fn kbd_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <kbd
            data-slot="kbd-group"
            class=(class!("inline-flex items-center gap-1", attrs.remove("class")))
            (attrs)
        >
            (child)
        </kbd>
    })
}
