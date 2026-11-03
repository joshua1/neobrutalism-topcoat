// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, Class, StaticClass, View, class, component, view},
};

/// The visual style of a [`badge`].
///
/// [`Default`] is `BadgeVariant::Default`, used when no variant is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum BadgeVariant {
    /// A badge on the page background color.
    #[default]
    Default,
    /// A badge on the secondary (neutral) background color.
    Neutral,
}

impl BadgeVariant {
    /// Classes for the variant's fill and text color.
    fn classes(self) -> StaticClass {
        match self {
            Self::Default => class!("bg-background text-foreground"),
            Self::Neutral => class!("bg-secondary-background text-foreground"),
        }
    }
}

/// Classes shared by every variant: the thick border, radius and sizing.
const BASE: StaticClass = class!(
    "inline-flex w-fit shrink-0 items-center justify-center gap-1 overflow-hidden \
     whitespace-nowrap rounded-base border-2 border-border px-2.5 py-0.5 text-xs font-base \
     [&>svg]:pointer-events-none [&>svg]:size-3 \
     focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50",
);

/// Builds the full class list for a badge of the given `variant`.
///
/// Use it to give badge styling to another element, such as a link:
///
/// ```ignore
/// view! {
///     <a href="/releases/v2" class=(badge_variants(BadgeVariant::Neutral))>"v2.0"</a>
/// }
/// ```
#[must_use]
#[allow(dead_code)]
pub fn badge_variants(variant: BadgeVariant) -> Class<(StaticClass, StaticClass)> {
    class!(BASE, variant.classes())
}

/// A small neobrutalism label for a status or count.
///
/// `variant` defaults to `Default`. Pass the label as children and extra
/// attributes through `attrs`. Attributes go on the `<span>`, with classes
/// appended to its classes. Use [`badge_variants`] to apply the same styling
/// to another element.
///
/// ```ignore
/// view! {
///     badge(variant: BadgeVariant::Neutral, "New")
/// }
/// ```
#[component]
pub async fn badge(
    #[default] variant: BadgeVariant,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    // A wrapper (e.g. a toast action) may pass its own `data-slot`; render
    // ours only when it didn't, so the attribute never appears twice.
    let slot = (!attrs.contains_key("data-slot")).then_some("badge");

    Ok(view! {
        <span
            data-slot=(slot)
            class=(class!(BASE, variant.classes(), attrs.remove("class")))
            (attrs)
        >
            (child)
        </span>
    })
}
