// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, Class, StaticClass, View, class, component, view},
};

/// The direction a [`button_group`] lays its children out in.
///
/// [`Default`] is `ButtonGroupOrientation::Horizontal`, used when no
/// orientation is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ButtonGroupOrientation {
    /// Children sit in a row, joined at their left and right edges.
    #[default]
    Horizontal,
    /// Children stack in a column, joined at their top and bottom edges.
    Vertical,
}

impl ButtonGroupOrientation {
    /// Classes that square off and merge the borders where children meet.
    fn classes(self) -> StaticClass {
        match self {
            Self::Horizontal => class!(
                "*:data-slot:rounded-r-none [&>[data-slot]:not(:has(~[data-slot]))]:rounded-r-base! \
                 [&>[data-slot]~[data-slot]]:rounded-l-none [&>[data-slot]~[data-slot]]:border-l-0",
            ),
            Self::Vertical => class!(
                "flex-col *:data-slot:rounded-b-none \
                 [&>[data-slot]:not(:has(~[data-slot]))]:rounded-b-base! \
                 [&>[data-slot]~[data-slot]]:rounded-t-none [&>[data-slot]~[data-slot]]:border-t-0",
            ),
        }
    }

    /// The `data-orientation` / `aria-orientation` value.
    fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Classes shared by both orientations. Children lose their hard shadow and
/// hover shift so the group reads as one control.
const BASE: StaticClass = class!(
    "flex w-fit items-stretch rounded-base *:shadow-none! *:hover:translate-x-0! \
     *:hover:translate-y-0! *:focus-visible:relative *:focus-visible:z-10 \
     has-[>[data-slot=button-group]]:gap-2 \
     has-[select[aria-hidden=true]:last-child]:[&>[data-slot=select-trigger]:last-of-type]:rounded-r-base \
     [&>[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&>input]:flex-1",
);

/// Builds the full class list for a button group of the given `orientation`.
#[must_use]
#[allow(dead_code)]
pub fn button_group_variants(
    orientation: ButtonGroupOrientation,
) -> Class<(StaticClass, StaticClass)> {
    class!(BASE, orientation.classes())
}

/// Related buttons (and inputs or text) joined into one control.
///
/// Children with a `data-slot` attribute, such as [`button`](super::button::button)s,
/// get squared inner corners and a single shared border. A plain element joins
/// in once it has a `data-slot`. Nest groups to space them apart. `attrs` are
/// forwarded to the `<div role="group">`, with extra classes appended to its
/// classes; add an `aria-label` describing the group.
///
/// ```ignore
/// view! {
///     button_group(attrs: attributes! { aria-label="Mail actions" },
///         button(variant: ButtonVariant::Neutral, "Archive")
///         button(variant: ButtonVariant::Neutral, "Report")
///     )
/// }
/// ```
#[component]
pub async fn button_group(
    /// Whether the children sit in a row or a column.
    #[default]
    orientation: ButtonGroupOrientation,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="group"
            data-slot="button-group"
            data-orientation=(orientation.as_str())
            class=(class!(BASE, orientation.classes(), attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A non-interactive label or value inside a [`button_group`], boxed to match
/// the buttons next to it.
///
/// Octane's `render` prop swaps the element; here, apply the same classes to
/// another element (such as a `<label>`) yourself.
#[component]
pub async fn button_group_text(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="button-group-text"
            class=(class!(
                "flex items-center gap-2 rounded-base border-2 border-border \
                 bg-secondary-background px-2.5 text-sm font-heading text-foreground \
                 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A rule between children of a [`button_group`].
///
/// `orientation` is the direction of the rule itself (default `Vertical`, for
/// a horizontal group).
#[component]
pub async fn button_group_separator(
    /// The direction of the rule.
    #[default(ButtonGroupOrientation::Vertical)]
    orientation: ButtonGroupOrientation,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="separator"
            aria-orientation=(orientation.as_str())
            data-slot="button-group-separator"
            data-orientation=(orientation.as_str())
            class=(class!(
                "relative shrink-0 self-stretch border-border \
                 data-[orientation=horizontal]:my-px data-[orientation=horizontal]:w-full \
                 data-[orientation=horizontal]:border-t-2 data-[orientation=vertical]:mx-px \
                 data-[orientation=vertical]:border-l-2",
                attrs.remove("class"),
            ))
            (attrs)
        ></div>
    })
}
