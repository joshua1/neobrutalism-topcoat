//! A panel that slides in from an edge of the screen, built on the native
//! `<dialog>` element like [`dialog`](super::dialog).
//!
//! The [`sheet`] element is the overlay (Base UI's `SheetPortal` and
//! `SheetOverlay`), and [`sheet_content`] is the sliding panel. Open it with a
//! [`sheet_trigger`] and close it with [`sheet_close`], Escape, or a click on
//! the overlay. No runtime is needed.

// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    runtime::Expr,
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

use super::dialog::{DIALOG_OVERLAY, LIGHT_DISMISS_SCRIPT, dialog_close, dialog_trigger};

/// Classes that fade the overlay in over 500ms and out over 300ms.
const FADE: StaticClass = class!(
    "bg-overlay opacity-0 open:opacity-100 starting:open:opacity-0 \
     transition-[opacity,display,overlay] transition-discrete duration-300 open:duration-500",
);

/// A modal sheet: the overlay that holds a [`sheet_content`] panel.
///
/// Give it an `id` and open it with a [`sheet_trigger`] naming that id, or pass
/// a runtime expression to `open` (see [`dialog`](super::dialog::dialog)).
/// Clicking the overlay closes it unless `disable_pointer_dismissal` is set.
/// `attrs` are forwarded to the `<dialog>`, with extra classes appended.
///
/// ```ignore
/// view! {
///     sheet_trigger(
///         target: "filters",
///         attrs: attributes! {
///             class=(button_variants(ButtonVariant::Default, ButtonSize::Default))
///         },
///         "Filters"
///     )
///     sheet(
///         id: "filters",
///         sheet_content(
///             side: SheetSide::Left,
///             sheet_header(
///                 sheet_title("Filters")
///                 sheet_description("Narrow the results below.")
///             )
///         )
///     )
/// }
/// ```
#[component]
pub async fn sheet(
    /// Whether the sheet shows without being opened by a trigger.
    #[into]
    #[default(Expr::from(false))]
    open: Expr<bool>,
    /// The `id` that [`sheet_trigger`]'s `target` refers to.
    #[into]
    #[default]
    id: Option<&str>,
    /// Keeps a click on the overlay from closing the sheet.
    #[default]
    disable_pointer_dismissal: bool,
    /// Extra attributes for the `<dialog>` element.
    #[default]
    mut attrs: Attributes,
    /// The sheet's content, usually a [`sheet_content`].
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    let dismiss = (!disable_pointer_dismissal).then_some(LIGHT_DISMISS_SCRIPT);
    // Let a wrapper replace `data-slot` without rendering it twice.
    let slot = (!attrs.contains_key("data-slot")).then_some("sheet");

    Ok(view! {
        <dialog
            data-slot=(slot)
            id=(id)
            :open=(open)
            onclick=(dismiss)
            class=(class!(DIALOG_OVERLAY, FADE, attrs.remove("class")))
            (attrs)
        >
            (child)
        </dialog>
    })
}

/// A button that opens the [`sheet`] whose `id` is `target`.
///
/// It is unstyled; give it button styling with `class=(button_variants(..))`.
#[component]
pub async fn sheet_trigger(
    /// The `id` of the sheet to open.
    target: &str,
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        dialog_trigger(
            target: target,
            attrs: attributes! { data-slot="sheet-trigger" (attrs) },
            (child)
        )
    })
}

/// A button that closes the [`sheet`] it is inside. It is unstyled.
#[component]
pub async fn sheet_close(
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        dialog_close(attrs: attributes! { data-slot="sheet-close" (attrs) }, (child))
    })
}

/// The edge a [`sheet_content`] slides in from.
///
/// [`Default`] is `SheetSide::Right`, used when no side is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SheetSide {
    /// Across the top edge, full width.
    Top,
    /// Along the right edge, full height.
    #[default]
    Right,
    /// Across the bottom edge, full width.
    Bottom,
    /// Along the left edge, full height.
    Left,
}

impl SheetSide {
    /// The value of the panel's `data-side` attribute.
    fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }

    /// Classes that place and size the panel against its edge.
    fn classes(self) -> StaticClass {
        match self {
            Self::Right => class!("inset-y-0 right-0 h-full w-3/4 border-l sm:max-w-sm"),
            Self::Left => class!("inset-y-0 left-0 h-full w-3/4 border-r sm:max-w-sm"),
            Self::Top => class!("inset-x-0 top-0 h-auto border-b"),
            Self::Bottom => class!("inset-x-0 bottom-0 h-auto border-t"),
        }
    }

    /// Classes that slide the panel in from, and back out to, its edge.
    fn motion(self) -> StaticClass {
        match self {
            Self::Right => class!(
                "translate-x-full group-open/dialog:translate-x-0 \
                 starting:group-open/dialog:translate-x-full",
            ),
            Self::Left => class!(
                "-translate-x-full group-open/dialog:translate-x-0 \
                 starting:group-open/dialog:-translate-x-full",
            ),
            Self::Top => class!(
                "-translate-y-full group-open/dialog:translate-y-0 \
                 starting:group-open/dialog:-translate-y-full",
            ),
            Self::Bottom => class!(
                "translate-y-full group-open/dialog:translate-y-0 \
                 starting:group-open/dialog:translate-y-full",
            ),
        }
    }
}

/// Classes shared by every side: the thick border and the slide timing (500ms
/// in, 300ms out).
const CONTENT: StaticClass = class!(
    "bg-background fixed z-50 flex flex-col gap-4 border-2 border-border outline-none \
     transition ease-in-out duration-300 group-open/dialog:duration-500",
);

/// The sliding panel inside a [`sheet`], with a close button in its corner.
///
/// `side` defaults to `Right`. Set `show_close_button: false` to leave the
/// corner button out.
#[component]
pub async fn sheet_content(
    /// The edge the panel slides in from.
    #[default]
    side: SheetSide,
    /// Whether to render the close button in the top-right corner.
    #[default(true)]
    show_close_button: bool,
    /// Extra attributes for the panel `<div>`.
    #[default]
    mut attrs: Attributes,
    /// The panel's sections.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="sheet-content"
            data-side=(side.as_str())
            class=(class!(CONTENT, side.classes(), side.motion(), attrs.remove("class")))
            (attrs)
        >
            (child)
            if show_close_button {
                sheet_close(
                    attrs: attributes! {
                        class="absolute right-4 top-4 rounded-base ring-offset-white \
                            focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 \
                            disabled:pointer-events-none"
                    },
                    icon(data: iconify_icon!("lucide:x"), attrs: attributes! { class="h-4 w-4" })
                    <span class="sr-only">"Close"</span>
                )
            }
        </div>
    })
}

/// The opening section of a [`sheet_content`], stacking a [`sheet_title`] and a
/// [`sheet_description`].
#[component]
pub async fn sheet_header(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="sheet-header"
            class=(class!("flex flex-col gap-1.5 p-4", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The actions at the bottom of a [`sheet_content`], stacked.
#[component]
pub async fn sheet_footer(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="sheet-footer"
            class=(class!("mt-auto flex flex-col gap-3 p-4", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The heading of a [`sheet`], rendered as an `<h2>`.
#[component]
pub async fn sheet_title(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <h2
            data-slot="sheet-title"
            class=(class!("text-foreground font-heading", attrs.remove("class")))
            (attrs)
        >
            (child)
        </h2>
    })
}

/// Text that explains the sheet's purpose, rendered as a `<p>`.
#[component]
pub async fn sheet_description(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p
            data-slot="sheet-description"
            class=(class!("text-sm text-foreground font-base", attrs.remove("class")))
            (attrs)
        >
            (child)
        </p>
    })
}
