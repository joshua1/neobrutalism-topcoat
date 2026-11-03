//! A floating panel anchored to its trigger, built on the native `popover`
//! attribute and CSS anchor positioning.
//!
//! [`popover_trigger`] toggles the panel through `popovertarget`, so no
//! runtime is needed. The browser renders an open [`popover_content`] in the
//! top layer (Base UI's portal), closes it on Escape or a click outside, and
//! closes other open popovers. Anchor positioning places it against the
//! trigger and flips it to the other side when it would overflow. Browsers
//! without anchor positioning center it in the viewport instead.
//!
//! To open or close it from script, call `showPopover()`, `hidePopover()` or
//! `togglePopover()` on the content element.

// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

/// Returns `value` for the attribute `key`, unless the caller set it in `attrs`.
fn unless_given(attrs: &Attributes, key: &str, value: &'static str) -> Option<&'static str> {
    (!attrs.contains_key(key)).then_some(value)
}

/// Builds the inline style that makes an element the anchor for the popover
/// whose `id` is `id`: `anchor-name: --{id}`.
///
/// [`popover_trigger`] sets it on itself. Use it to anchor the panel to
/// another element instead, such as the input of a combobox:
///
/// ```ignore
/// view! {
///     <div style=(popover_anchor("fruit-list"))>
///         <input ..>
///         popover_trigger(target: "fruit-list", ..)
///     </div>
///     popover_content(id: "fruit-list", ..)
/// }
/// ```
///
/// The `id` must be a valid CSS identifier (letters, digits, `-`, `_`).
#[must_use]
#[allow(dead_code)]
pub fn popover_anchor(id: &str) -> String {
    format!("anchor-name: --{id}")
}

/// Groups a [`popover_trigger`] with its [`popover_content`]. It renders only
/// its children; the two parts are linked by the content's `id`.
#[component]
pub async fn popover(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! { (child) })
}

/// A button that opens and closes the [`popover_content`] whose `id` is
/// `target`, and anchors it.
///
/// It is unstyled; give it button styling with `class=(button_variants(..))`
/// in `attrs`. The button carries the anchor in its `style`, so put inline
/// styles on an inner element instead. Pass `anchor: false` when another
/// element carries [`popover_anchor`].
#[component]
pub async fn popover_trigger(
    /// The `id` of the popover content to toggle.
    target: &str,
    /// Whether this button is the element the content is placed against.
    #[default(true)]
    anchor: bool,
    /// Extra attributes for the `<button>` element.
    #[default]
    attrs: Attributes,
    /// The button's label.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    let slot = unless_given(&attrs, "data-slot", "popover-trigger");
    let style = anchor.then(|| popover_anchor(target));

    Ok(view! {
        <button
            type="button"
            data-slot=(slot)
            popovertarget=(target)
            style=(style)
            (attrs)
        >
            (child)
        </button>
    })
}

/// The side of the trigger a [`popover_content`] opens on.
///
/// [`Default`] is `PopoverSide::Bottom`, used when no side is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PopoverSide {
    /// Above the trigger.
    Top,
    /// To the right of the trigger.
    Right,
    /// Below the trigger.
    #[default]
    Bottom,
    /// To the left of the trigger.
    Left,
}

impl PopoverSide {
    /// The value of the content's `data-side` attribute.
    fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }

    /// Classes for the 4px gap to the trigger, the zoom origin, the flip when
    /// the panel would overflow, and the short slide away from the trigger as
    /// it opens.
    fn classes(self) -> StaticClass {
        match self {
            Self::Top => class!(
                "mb-1 origin-bottom [position-try-fallbacks:flip-block] \
                 starting:open:translate-y-2",
            ),
            Self::Right => class!(
                "ml-1 origin-left [position-try-fallbacks:flip-inline] \
                 starting:open:-translate-x-2",
            ),
            Self::Bottom => class!(
                "mt-1 origin-top [position-try-fallbacks:flip-block] \
                 starting:open:-translate-y-2",
            ),
            Self::Left => class!(
                "mr-1 origin-right [position-try-fallbacks:flip-inline] \
                 starting:open:translate-x-2",
            ),
        }
    }
}

/// How a [`popover_content`] lines up with its trigger along the side it opens
/// on.
///
/// [`Default`] is `PopoverAlign::Center`, used when no alignment is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PopoverAlign {
    /// Lines up the starting edges (left, or top on the left and right sides).
    Start,
    /// Centers the panel on the trigger.
    #[default]
    Center,
    /// Lines up the ending edges (right, or bottom on the left and right sides).
    End,
}

/// The CSS `position-area` that places the panel on `side` with `align`.
fn placement(side: PopoverSide, align: PopoverAlign) -> StaticClass {
    use PopoverAlign::{Center, End, Start};
    use PopoverSide::{Bottom, Left, Right, Top};

    match (side, align) {
        (Bottom, Start) => class!("[position-area:bottom_span-right]"),
        (Bottom, Center) => class!("[position-area:bottom_span-all]"),
        (Bottom, End) => class!("[position-area:bottom_span-left]"),
        (Top, Start) => class!("[position-area:top_span-right]"),
        (Top, Center) => class!("[position-area:top_span-all]"),
        (Top, End) => class!("[position-area:top_span-left]"),
        (Left, Start) => class!("[position-area:left_span-bottom]"),
        (Left, Center) => class!("[position-area:left_span-all]"),
        (Left, End) => class!("[position-area:left_span-top]"),
        (Right, Start) => class!("[position-area:right_span-bottom]"),
        (Right, Center) => class!("[position-area:right_span-all]"),
        (Right, End) => class!("[position-area:right_span-top]"),
    }
}

/// Classes for the panel. Display is set only while open (`open:flex`) so the
/// browser's closed state stays hidden. `overflow-visible` undoes the
/// browser's `overflow: auto` on popovers, which would clip the shadow of a
/// child such as a calendar in a `p-0` panel.
const CONTENT: StaticClass = class!(
    "z-50 open:flex w-72 flex-col gap-4 rounded-base border-2 border-border bg-background p-4 \
     text-foreground outline-none overflow-visible not-supports-[position-area:center]:m-auto",
);

/// Classes that fade and zoom the panel in and out. `transition-discrete`
/// keeps it displayed, and in the top layer, until the exit transition ends.
const MOTION: StaticClass = class!(
    "scale-95 opacity-0 open:scale-100 open:opacity-100 \
     starting:open:scale-95 starting:open:opacity-0 \
     transition-[opacity,scale,translate,display,overlay] transition-discrete \
     duration-150 ease-out",
);

/// The floating panel of a popover.
///
/// `id` links it to its [`popover_trigger`]'s `target`. `side` defaults to
/// `Bottom` and `align` to `Center`. It has `role="dialog"` unless `attrs`
/// sets another role, and `popover="auto"` (light dismiss) unless `attrs` sets
/// `popover="manual"`. To size it to its anchor, add `w-[anchor-size(width)]!`.
///
/// ```ignore
/// view! {
///     popover(
///         popover_trigger(
///             target: "dimensions",
///             attrs: attributes! {
///                 class=(button_variants(ButtonVariant::NoShadow, ButtonSize::Default))
///             },
///             "Open popover"
///         )
///         popover_content(
///             id: "dimensions",
///             align: PopoverAlign::Start,
///             popover_header(
///                 popover_title("Dimensions")
///                 popover_description("Set the dimensions for the layer.")
///             )
///         )
///     )
/// }
/// ```
#[component]
pub async fn popover_content(
    /// The `id` that [`popover_trigger`]'s `target` refers to.
    id: &str,
    /// The side of the trigger to open on.
    #[default]
    side: PopoverSide,
    /// How to line up with the trigger.
    #[default]
    align: PopoverAlign,
    /// Extra attributes for the panel `<div>`.
    #[default]
    mut attrs: Attributes,
    /// The panel's content.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    let role = unless_given(&attrs, "role", "dialog");
    let mode = unless_given(&attrs, "popover", "auto");

    Ok(view! {
        <div
            data-slot="popover-content"
            data-side=(side.as_str())
            id=(id)
            popover=(mode)
            role=(role)
            style=(format!("position-anchor: --{id}"))
            class=(class!(
                CONTENT,
                MOTION,
                side.classes(),
                placement(side, align),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The opening section of a [`popover_content`], stacking a [`popover_title`]
/// and a [`popover_description`].
#[component]
pub async fn popover_header(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="popover-header"
            class=(class!("flex flex-col gap-1", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The heading of a popover, rendered as an `<h2>`.
#[component]
pub async fn popover_title(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <h2
            data-slot="popover-title"
            class=(class!("font-heading leading-none", attrs.remove("class")))
            (attrs)
        >
            (child)
        </h2>
    })
}

/// Text that explains the popover's purpose, rendered as a `<p>`.
#[component]
pub async fn popover_description(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p
            data-slot="popover-description"
            class=(class!("text-sm font-base text-foreground", attrs.remove("class")))
            (attrs)
        >
            (child)
        </p>
    })
}
