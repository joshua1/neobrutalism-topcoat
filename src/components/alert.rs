// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

/// The visual style of an [`alert`].
///
/// [`Default`] is `AlertVariant::Default`, used when no variant is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AlertVariant {
    /// An alert on the page background color.
    #[default]
    Default,
    /// A black alert with white text, for errors and failures.
    Destructive,
}

impl AlertVariant {
    /// Classes for the variant's fill and text color.
    fn classes(self) -> StaticClass {
        match self {
            Self::Default => class!("bg-background text-foreground"),
            Self::Destructive => class!("bg-black text-white"),
        }
    }
}

/// Classes for the alert layout. The icon column collapses to zero width when
/// the alert has no direct `<svg>` child.
const BASE: StaticClass = class!(
    "relative w-full rounded-base border-2 border-border px-4 py-3 text-sm grid \
     has-[>svg]:grid-cols-[calc(var(--spacing)*4)_1fr] grid-cols-[0_1fr] has-[>svg]:gap-x-3 \
     gap-y-0.5 items-start [&>svg]:size-4 [&>svg]:translate-y-0.5 [&>svg]:text-current \
     shadow-shadow",
);

/// A neobrutalism notice displayed within the page.
///
/// `variant` defaults to `Default`. Pass an optional icon, an [`alert_title`]
/// and an [`alert_description`] as children. `attrs` are forwarded to the
/// `<div>`, which has `role="alert"`, with extra classes appended to its
/// classes.
///
/// ```ignore
/// view! {
///     alert(
///         variant: AlertVariant::Destructive,
///         icon(data: iconify_icon!("lucide:circle-alert"))
///         alert_title("Something went wrong!")
///         alert_description("Your session has expired. Please log in again.")
///     )
/// }
/// ```
#[component]
pub async fn alert(
    #[default] variant: AlertVariant,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="alert"
            role="alert"
            class=(class!(BASE, variant.classes(), attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The heading of an [`alert`], clamped to one line.
#[component]
pub async fn alert_title(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="alert-title"
            class=(class!(
                "col-start-2 line-clamp-1 min-h-4 font-heading tracking-tight",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Text that explains an [`alert`]. Paragraphs inside it get relaxed leading.
#[component]
pub async fn alert_description(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="alert-description"
            class=(class!(
                "col-start-2 grid justify-items-start gap-1 text-sm font-base \
                 [&_p]:leading-relaxed",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}
